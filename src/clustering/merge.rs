use super::is_similar;

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

#[cfg(test)]
mod tests {
    use super::merge_clusters;

    mod merge_clusters {
        use super::merge_clusters;

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
}
