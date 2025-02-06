#![warn(missing_docs)]

//! # Multithreaded String Clustering
//! This crate provides a scalable string clustering implementation.
//!
//! Strings are aggregated into clusters based on pairwise Levenshtein
//! distance. If the distance is below a set fraction of the shorter string's
//! length, the strings are added to the same cluster.
//!
//! # Multithreading model
//! * The input strings are evenly paritioned across the set of allocated
//! threads.
//! * Once each thread has clustered its associated input strings, result aggregation is
//! started.
//! * Clusters are merged in pairs accross multiple threads in a manner that is similar to
//! traversing a binary tree from the leaves up to the root. The root of the tree is the final
//! clustering.
//! * Thus, if there are N threads allocated, there will be ceil(log2(N)) merge operations.
//!
//! # Optimisation
//! A key optimisation made to significantly improve the performance of the implementation is the use
//! of transitivity in the clustering operation.
//!
//! If string A is clustered with string B and string B clustered with string C, strings A and C
//! will be clustered together. This optimisation often provides significant runtime performance benefits with
//! negligible impact on clustering performance.
//!
//! There are a number of instances in which this optimisation will result in poor clustering performance.
//! As a result, if this property cannot be exploited on the desired input data, another implementation
//! should be used.
//!
//! # Installation
//!
//! Add this to your Cargo.toml
//!
//! ```toml
//! [dependencies]
//! clustr = "0.1.2"
//! ```
//!
//! # Examples
//! Basic usage:
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), clustr::ValueError> {
//! let inputs = vec!["aaaa", "aaax", "bbbb", "bbbz"];
//! let expected = vec![vec!["aaaa", "aaax"], vec!["bbbb", "bbbz"]];
//!
//! let clusters = clustr::cluster_strings(&inputs, 0.25, 1)?;
//!
//! assert_eq!(clusters, expected);
//! #
//! # Ok(())
//! # }
//! ```
//!
//! # Multiple threads:
//! ```
//! # use std::error::Error;
//! #
//! # fn main() -> Result<(), clustr::ValueError> {
//! let inputs = vec!["aa", "bb", "aa", "bb"];
//! let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];
//!
//! // Cluster using 4 threads
//! let results = clustr::cluster_strings(&inputs, 0.0, 4)?;
//!  
//! // Order of returned clusters nondeterministic
//! for e in expected {
//!     assert!(results.contains(&e));
//! }
//! #
//! # Ok(())
//! # }
//! ```

mod clustering;
mod metric;
mod threading;

use threading::aggregation::aggregate_results;
use threading::formation::form_clusters;

/// Validation errors. Errors associated with invalid function argument values.
#[derive(PartialEq, Debug)]
pub enum ValueError {
    /// Fraction value outside of closed interval \[0,1\].
    InvalidFraction,
    /// Input vector empty.
    EmptyVector,
    /// More threads allocated than input strings.
    InsufficientWork,
    /// Thread count less than one.
    InsufficientThreadCount,
}

/// Group similar input strings into clusters.
///
/// Strings will be grouped into a cluster if the Levenshtein distance between the
/// strings is below 'max_edit_frac' of the shorter string's length.
///
/// # Examples
/// Basic usage:
/// ```
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), clustr::ValueError> {
/// let inputs = vec!["aaaa", "aaax", "bbbb", "bbbz"];
/// let expected = vec![vec!["aaaa", "aaax"], vec!["bbbb", "bbbz"]];
///
/// let clusters = clustr::cluster_strings(&inputs, 0.25, 1)?;
///
/// assert_eq!(clusters, expected);
/// #
/// # Ok(())
/// # }
/// ```
///
/// # Multiple threads:
/// ```
/// # use std::error::Error;
/// #
/// # fn main() -> Result<(), clustr::ValueError> {
/// let inputs = vec!["aa", "bb", "aa", "bb"];
/// let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];
///
/// let results = clustr::cluster_strings(&inputs, 0.0, 4)?;
///  
/// // Order of returned clusters nondeterministic
/// for e in expected {
///     assert!(results.contains(&e));
/// }
/// #
/// # Ok(())
/// # }
/// ```
pub fn cluster_strings<'a>(
    inputs: &'a Vec<&'a str>,
    max_edit_frac: f32,
    n_threads: usize,
) -> Result<Vec<Vec<&'a str>>, ValueError> {
    // Validation here to avoid having to propagate errors out of threads
    if inputs.len() == 0 {
        return Err(ValueError::EmptyVector);
    }
    if max_edit_frac < 0.0 || max_edit_frac > 1.0 {
        return Err(ValueError::InvalidFraction);
    }
    if n_threads > inputs.len() {
        return Err(ValueError::InsufficientWork);
    }
    if n_threads <= 0 {
        return Err(ValueError::InsufficientThreadCount);
    }

    let clusters = form_clusters(inputs, max_edit_frac, n_threads);
    let result = aggregate_results(clusters, max_edit_frac);
    Ok(result)
}
