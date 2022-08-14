mod clustering;
mod metric;
mod threading;

use threading::aggregation::aggregate_results;
use threading::formation::form_clusters;

pub fn cluster_strings<'a>(
    inputs: &'a Vec<&'a str>,
    max_edit_frac: f32,
    n_threads: usize,
) -> Vec<Vec<&'a str>> {
    let clusters = form_clusters(inputs, max_edit_frac, n_threads);
    let result = aggregate_results(clusters, max_edit_frac);

    result
}
