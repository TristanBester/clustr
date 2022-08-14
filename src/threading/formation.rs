use super::cluster;
use std::sync::{Arc, Mutex};

pub fn form_clusters<'a>(
    inputs: &'a Vec<&'a str>,
    max_edit_frac: f32,
    n_threads: usize,
) -> Vec<Vec<Vec<&'a str>>> {
    if n_threads > inputs.len() {
        panic!("Not enough work per thread. Ensure n_threads is not greater than the number of input strings.")
    }

    let inputs_per_thread = inputs.len() / n_threads as usize;
    let results = Arc::new(Mutex::new(Vec::new()));

    crossbeam::scope(|s| {
        for i in 0..n_threads {
            let results = Arc::clone(&results);
            let start = i * inputs_per_thread;
            let end;

            if i + 1 != n_threads {
                end = (i + 1) * inputs_per_thread;
            } else {
                end = inputs.len()
            }

            s.spawn(move |_| {
                let clusters = cluster(&inputs[start..end], max_edit_frac);
                {
                    results.lock().unwrap().push(clusters);
                }
            });
        }
    })
    .unwrap();
    return results.lock().unwrap().clone();
}

#[cfg(test)]
mod tests {
    use super::form_clusters;

    #[test]
    fn test_correct_equal_work_per_thread() {
        let data = vec!["aa", "aa", "bb", "bb"];
        let expected = vec![vec![vec!["aa", "aa"]], vec![vec!["bb", "bb"]]];
        let result = form_clusters(&data, 0.0, 2);

        // Order of objects in result is nondeterministic
        for e in expected {
            assert!(result.contains(&e))
        }
    }

    #[test]
    fn test_unequal_work_per_thread() {
        let data = vec!["aa", "aa", "bb", "bb"];
        let expected = vec![vec![vec!["aa"]], vec![vec!["aa"]], vec![vec!["bb", "bb"]]];
        let result = form_clusters(&data, 0.0, 3);

        // Order of objects in result is nondeterministic
        for e in expected {
            assert!(result.contains(&e))
        }
    }

    #[test]
    fn test_equal_threads_and_inputs() {
        let data = vec!["aa", "aa", "bb", "bb"];
        let expected = vec![
            vec![vec!["aa"]],
            vec![vec!["aa"]],
            vec![vec!["bb"]],
            vec![vec!["bb"]],
        ];
        let result = form_clusters(&data, 0.0, 4);

        // Order of objects in result is nondeterministic
        for e in expected {
            assert!(result.contains(&e))
        }
    }

    #[test]
    #[should_panic(
        expected = "Not enough work per thread. Ensure n_threads is not greater than the number of input strings."
    )]
    fn test_more_threads_than_inputs() {
        let data = vec!["aa", "aa", "bb", "bb"];
        form_clusters(&data, 0.0, 5);
    }
}
