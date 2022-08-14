mod clustering;
mod metric;
mod threading;

use threading::aggregation::aggregate_results;
use threading::formation::form_clusters;

#[derive(PartialEq, Debug)]
pub enum ValueError {
    InvalidFraction,
    EmptyVector,
    InsufficientWork,
}

pub fn cluster_strings<'a>(
    inputs: &'a Vec<&'a str>,
    max_edit_frac: f32,
    n_threads: usize,
) -> Result<Vec<Vec<&'a str>>, ValueError> {
    // Validation here as error propagation cannot be done from within threads
    if inputs.len() == 0 {
        return Err(ValueError::EmptyVector);
    }
    if max_edit_frac < 0.0 || max_edit_frac > 1.0 {
        return Err(ValueError::InvalidFraction);
    }
    if n_threads > inputs.len() {
        return Err(ValueError::InsufficientWork);
    }

    let clusters = form_clusters(inputs, max_edit_frac, n_threads);
    let result = aggregate_results(clusters, max_edit_frac);

    Ok(result)
}
