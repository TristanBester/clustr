#[test]
fn test_basic_correctness() {
    let inputs = vec!["a", "b", "c", "a", "b", "c"];
    let expected = vec![vec!["a", "a"], vec!["b", "b"], vec!["c", "c"]];
    let results = clustr::cluster_strings(&inputs, 0.0, 6).unwrap();
    // cluster ordering nondeterministic
    for e in expected {
        assert!(results.contains(&e));
    }
}

#[test]
fn test_reject_max_edit_frac_below_zero() {
    let inputs = vec!["a", "b", "c"];
    let expected = Err(clustr::ValueError::InvalidFraction);
    let results = clustr::cluster_strings(&inputs, -0.1, 1);
    assert_eq!(expected, results);
}

#[test]
fn test_reject_edit_frac_above_one() {
    let inputs = vec!["a", "b", "c"];
    let expected = Err(clustr::ValueError::InvalidFraction);
    let results = clustr::cluster_strings(&inputs, 1.1, 1);
    assert_eq!(expected, results);
}

#[test]
fn test_edit_frac_zero_accepted() {
    let inputs = vec!["a", "b", "c", "d"];
    let expected = vec![vec!["a"], vec!["b"], vec!["c"], vec!["d"]];
    let results = clustr::cluster_strings(&inputs, 0.0, 4).unwrap();
    // cluster ordering nondeterministic
    for e in expected {
        assert!(results.contains(&e));
    }
}

#[test]
fn test_edit_frac_one_accepted() {
    let inputs = vec!["a", "b", "c", "d"];
    let expected = vec![vec!["a", "b", "c", "d"]];
    let results = clustr::cluster_strings(&inputs, 1.0, 1).unwrap();
    // cluster ordering nondeterministic
    for e in expected {
        assert!(results.contains(&e));
    }
}

#[test]
fn test_edit_frac_applied_correctly() {
    let inputs = vec!["aaaa", "aaax", "bbbb", "bbbz"];
    let expected = vec![vec!["aaaa", "aaax"], vec!["bbbb", "bbbz"]];
    let results = clustr::cluster_strings(&inputs, 0.25, 2).unwrap();
    // cluster ordering nondeterministic
    for e in expected {
        assert!(results.contains(&e));
    }
}

#[test]
fn test_reject_empty_vector() {
    let inputs = Vec::new();
    let expected = Err(clustr::ValueError::EmptyVector);
    let results = clustr::cluster_strings(&inputs, 0.0, 1);
    assert_eq!(results, expected);
}

#[test]
fn test_reject_more_threads_than_strings() {
    let inputs = vec!["a", "b", "b"];
    let expected = Err(clustr::ValueError::InsufficientWork);
    let results = clustr::cluster_strings(&inputs, 0.0, 4);
    assert_eq!(expected, results);
}

#[test]
fn test_reject_zero_threads_commissioned() {
    let inputs = vec!["a", "b", "c", "d"];
    let expected = Err(clustr::ValueError::InsufficientThreadCount);
    let results = clustr::cluster_strings(&inputs, 0.0, 0);
    assert_eq!(expected, results);
}

#[test]
fn test_one_input_string_accepted() {
    let inputs = vec!["a"];
    let expected = vec![vec!["a"]];
    let results = clustr::cluster_strings(&inputs, 0.0, 1).unwrap();
    assert_eq!(results, expected);
}

#[test]
fn test_output_consistent_across_thread_counts() {
    let inputs = vec![
        "aaaa", "aaax", "bbbb", "bbby", "cccc", "cccz", "dddd", "dddw",
    ];
    let expected = vec![
        vec!["aaaa", "aaax"],
        vec!["bbbb", "bbby"],
        vec!["cccc", "cccz"],
        vec!["dddd", "dddw"],
        vec!["aaax", "aaaa"],
        vec!["bbby", "bbbb"],
        vec!["cccz", "cccc"],
        vec!["dddw", "dddd"],
    ];

    for n_threads in 1..=8 {
        let results = clustr::cluster_strings(&inputs, 0.25, n_threads).unwrap();
        // cluster ordering nondeterministic
        for r in results {
            // string ordering within cluster nondeterministic
            assert!(expected.contains(&r));
        }
    }
}
