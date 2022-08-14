mod clustering;
mod similarity;

use crate::clustering::{cluster, merge_clusters};
use crossbeam;
use fast_math::log2_raw;
use std::sync::{Arc, Mutex};

fn form_clusters<'a>(
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

pub fn aggregate_results<'a>(
    results: Vec<Vec<Vec<&'a str>>>,
    max_edit_frac: f32,
) -> Vec<Vec<&'a str>> {
    let mut results = Arc::new(results);
    let n_aggregations = log2_raw(results.len() as f32).ceil() as usize;

    let aggregations = Arc::new(Mutex::new(Vec::new()));

    for _ in 0..n_aggregations {
        crossbeam::scope(|s| {
            for j in (0..results.len()).step_by(2) {
                let results = Arc::clone(&results);
                let aggregations = Arc::clone(&aggregations);

                s.spawn(move |_| {
                    let x = &mut results[j].clone();

                    if j + 1 == results.len() {
                        aggregations.lock().unwrap().push(results[j].clone());
                    } else {
                        let y = &mut results[j + 1].clone();
                        let agg = merge_clusters(x, y, max_edit_frac);
                        aggregations.lock().unwrap().push(agg);
                    }
                });
            }
        })
        .unwrap();

        results = Arc::new(aggregations.lock().unwrap().clone());
        aggregations.lock().unwrap().clear();
    }
    // return aggregations.lock().unwrap()[0].clone();
    return results.to_vec()[0].clone();
}

mod tests {

    mod aggregate_results {
        use crate::aggregate_results;

        #[test]
        fn test_one_merge() {
            let input = vec![vec![vec!["aa"], vec!["bb"]], vec![vec!["aa"], vec!["bb"]]];
            let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];
            let results = aggregate_results(input, 0.0);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_three_merge() {
            let input = vec![
                vec![vec!["aa"]],
                vec![vec!["bb"]],
                vec![vec!["aa"]],
                vec![vec!["bb"]],
            ];
            let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];
            let results = aggregate_results(input, 0.0);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_two_merge_one_pass() {
            let input = vec![vec![vec!["aa"]], vec![vec!["bb"]], vec![vec!["aa"]]];
            let expected = vec![vec!["aa", "aa"], vec!["bb"]];
            let results = aggregate_results(input, 0.0);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_six_merge_one_pass() {
            let input = vec![
                vec![vec!["aa"]],
                vec![vec!["bb"]],
                vec![vec!["aa"]],
                vec![vec!["bb"]],
                vec![vec!["aa"]],
                vec![vec!["bb"]],
                vec![vec!["aa"]],
            ];
            let expected = vec![vec!["aa", "aa", "aa", "aa"], vec!["bb", "bb", "bb"]];
            let results = aggregate_results(input, 0.0);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_two_merge_multiple_element_clusters() {
            let input = vec![
                vec![vec!["aaa", "aaa"], vec!["bbb", "bbb"], vec!["ccc", "ccc"]],
                vec![vec!["aaa", "aaa"], vec!["bbb", "bbb"], vec!["ccc", "ccc"]],
                vec![vec!["aaa", "aaa"], vec!["bbb", "bbb"], vec!["ccc", "ccc"]],
            ];
            let expected = vec![
                vec!["aaa", "aaa", "aaa", "aaa", "aaa", "aaa"],
                vec!["bbb", "bbb", "bbb", "bbb", "bbb", "bbb"],
                vec!["ccc", "ccc", "ccc", "ccc", "ccc", "ccc"],
            ];
            let results = aggregate_results(input, 0.0);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_non_zero_max_edit_frac_one_merge() {
            let input = vec![
                vec![vec!["aaaa", "aaaa"], vec!["bbbb", "bbbb"]],
                vec![vec!["aaax", "aaax"], vec!["bbbz", "bbbz"]],
            ];
            let expected = vec![
                vec!["aaaa", "aaaa", "aaax", "aaax"],
                vec!["bbbb", "bbbb", "bbbz", "bbbz"],
            ];
            let results = aggregate_results(input, 0.25);

            for e in expected {
                assert!(results.contains(&e));
            }
        }

        #[test]
        fn test_no_merge() {
            let input = vec![vec![vec!["aa", "aa"]]];
            let expected = vec![vec!["aa", "aa"]];
            let results = aggregate_results(input, 0.0);

            assert_eq!(results, expected);
        }
    }

    mod form_clusters {
        use crate::form_clusters;

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
}
