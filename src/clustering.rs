use crate::similarity::is_similar;

pub fn cluster<'a>(inputs: &[&'a str], max_edit_frac: f32) -> Vec<Vec<&'a str>> {
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
            if is_similar(container[i][0], container[j][0], max_edit_frac) {
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

pub fn merge_clusters<'a>(
    set_one: &mut Vec<Vec<&'a str>>,
    set_two: &mut Vec<Vec<&'a str>>,
    max_edit_frac: f32,
) -> Vec<Vec<&'a str>> {
    let mut moved = vec![false; set_two.len()];

    for i in 0..set_one.len() {
        for j in 0..set_two.len() {
            if moved[j] {
                continue;
            }

            if is_similar(set_one[i][0], set_two[j][0], max_edit_frac) {
                set_one[i].append(&mut set_two[j]);
                moved[j] = true;
            }
        }
    }

    for (i, m) in moved.iter().enumerate() {
        if !*m {
            set_one.push(set_two[i].clone());
        }
    }
    set_one.clone()
}

fn init_container<'a>(inputs: &[&'a str]) -> Vec<Vec<&'a str>> {
    if inputs.len() == 0 {
        panic!("inputs cannot be empty.");
    }

    let mut container = vec![Vec::new(); inputs.len()];

    for (i, s) in inputs.iter().enumerate() {
        container[i].push(*s);
    }
    container
}

#[cfg(test)]
mod tests {
    mod clusters {
        use crate::cluster;

        #[test]
        fn test_cluster_correct() {
            let inputs = vec!["a", "a", "b", "b"];
            let expected = vec![vec!["a", "a"], vec!["b", "b"]];

            let results = cluster(&inputs, 0.0);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_clusters_formed_below_max_edit_frac() {
            let inputs = vec!["aaa", "aac", "bbb", "bbc"];
            let expected = vec![vec!["aaa", "aac"], vec!["bbb", "bbc"]];

            let results = cluster(&inputs, 0.34);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_clusters_formed_equal_max_edit_frac() {
            let inputs = vec!["aa", "ab", "cc", "cd"];
            let expected = vec![vec!["aa", "ab"], vec!["cc", "cd"]];

            let results = cluster(&inputs, 0.5);
            assert_eq!(results, expected);
        }

        #[test]
        fn test_no_clusters() {
            let inputs = vec!["a", "b", "c"];
            let expected = vec![vec!["a"], vec!["b"], vec!["c"]];

            let results = cluster(&inputs, 0.0);
            assert_eq!(results, expected);
        }

        #[test]
        #[should_panic(expected = "inputs cannot be empty.")]
        fn test_reject_empty() {
            let inputs = Vec::new();
            cluster(&inputs, 0.0);
        }
    }

    mod merge_clusters {
        use crate::merge_clusters;

        #[test]
        fn test_merge_no_overlap() {
            let mut set_one = vec![vec!["a"], vec!["b"]];
            let mut set_two = vec![vec!["c"], vec!["d"]];
            let expected = vec![vec!["a"], vec!["b"], vec!["c"], vec!["d"]];

            let result = merge_clusters(&mut set_one, &mut set_two, 0.0);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_merge_full_overlap() {
            let mut set_one = vec![vec!["aa"], vec!["bb"]];
            let mut set_two = vec![vec!["aa"], vec!["bb"]];
            let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"]];

            let result = merge_clusters(&mut set_one, &mut set_two, 0.5);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_partial_overlap() {
            let mut set_one = vec![vec!["aa"], vec!["bb"]];
            let mut set_two = vec![vec!["aa"], vec!["bb"], vec!["cc"]];
            let expected = vec![vec!["aa", "aa"], vec!["bb", "bb"], vec!["cc"]];

            let result = merge_clusters(&mut set_one, &mut set_two, 0.5);
            assert_eq!(result, expected);
        }

        #[test]
        fn test_merge_max_edit_frac_correct() {
            let mut set_one = vec![vec!["aa"], vec!["cc"]];
            let mut set_two = vec![vec!["ab"], vec!["cd"]];
            let expected = vec![vec!["aa", "ab"], vec!["cc", "cd"]];

            let result = merge_clusters(&mut set_one, &mut set_two, 0.5);
            assert_eq!(result, expected);
        }
    }

    mod init_container {
        use super::super::init_container;

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
}
