mod utils;
use crate::utils::is_similar;

fn init_container<'a>(inputs: &Vec<&'a str>) -> Vec<Vec<&'a str>> {
    if inputs.len() == 0 {
        panic!("inputs cannot be empty.");
    }

    let mut container = vec![Vec::new(); inputs.len()];

    for (i, s) in inputs.iter().enumerate() {
        container[i].push(*s);
    }
    container
}

fn form_clusters<'a>(inputs: &Vec<&'a str>, tol: f32) -> Vec<Vec<&'a str>> {
    if inputs.len() == 0 {
        panic!("inputs cannot be empty.");
    }

    let mut container = init_container(inputs);

    // Store if value has been moved into a cluster
    let mut moved = vec![false; inputs.len()];
    // Store if value is cluster representative
    let mut is_repr = vec![false; inputs.len()];

    for i in 0..inputs.len() {
        if moved[i] {
            // A value cannot be moved into multiple clusters
            continue;
        } else {
            // If value has not been moved into any of the previous clusters
            // it forms a new cluster
            is_repr[i] = true;
        }

        for j in i + 1..inputs.len() {
            if is_similar(container[i][0], container[j][0], tol) {
                let str_ref = container[j][0];
                container[i].push(str_ref);
                moved[j] = true;
            }
        }
    }

    // Delete single value vectors remaining after values moved into cluster
    for (i, v) in moved.iter().enumerate().rev() {
        if *v {
            container.remove(i);
        }
    }
    container
}

#[cfg(test)]
mod tests {
    mod init_container {
        use crate::init_container;

        #[test]
        fn test_structure_correct() {
            let inputs = vec!["a", "b"];
            let expected = vec![vec!["a"], vec!["b"]];

            let results = init_container(&inputs);
            assert_eq!(results, expected);
        }

        #[test]
        #[should_panic(expected = "inputs cannot be empty.")]
        fn test_reject_empty() {
            let inputs = Vec::new();
            init_container(&inputs);
        }
    }

    mod form_clusters {
        use crate::form_clusters;

        #[test]
        fn test_cluster_correct() {
            let inputs = vec!["a", "a", "b", "b"];
            let expected = vec![vec!["a", "a"], vec!["b", "b"]];

            let results = form_clusters(&inputs, 0.0);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_clusters_formed_below_tol() {
            let inputs = vec!["aaa", "aac", "bbb", "bbc"];
            let expected = vec![vec!["aaa", "aac"], vec!["bbb", "bbc"]];

            let results = form_clusters(&inputs, 0.34);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_clusters_formed_equal_tol() {
            let inputs = vec!["aa", "ab", "cc", "cd"];
            let expected = vec![vec!["aa", "ab"], vec!["cc", "cd"]];

            let results = form_clusters(&inputs, 0.5);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_no_clusters() {
            let inputs = vec!["a", "b", "c"];
            let expected = vec![vec!["a"], vec!["b"], vec!["c"]];

            let results = form_clusters(&inputs, 0.0);
            assert_eq!(results, expected);
        }

        #[test]
        #[should_panic(expected = "inputs cannot be empty.")]
        fn test_reject_empty() {
            let inputs = Vec::new();
            form_clusters(&inputs, 0.0);
        }
    }
}
