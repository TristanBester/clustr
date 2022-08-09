use edit_distance::edit_distance;

fn get_max_edit_dist(a: &String, b: &String, tol: f32) -> usize {
    let max_edit: f32;
    let l_a = a.len() as f32;
    let l_b = b.len() as f32;

    if l_a < l_b {
        max_edit = l_a * tol;
    } else {
        max_edit = l_b * tol;
    }
    max_edit as usize
}

fn is_similar(a: &String, b: &String, tol: f32) -> bool {
    let max_edit = get_max_edit_dist(a, b, tol) as i32;
    let len_diff = (a.len() as i32 - b.len() as i32).abs();

    // If difference in length between string is greater than max
    // edit distance it is not possible for the strings to be similar
    if len_diff > max_edit {
        return false;
    }

    let dist = edit_distance(a, b);
    dist <= max_edit as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_similar_accepts_below_max_edit() {
        let a = "aaaa".to_string();
        let b = "aaab".to_string();

        let result = is_similar(&a, &b, 0.5);
        assert!(result);
    }

    #[test]
    fn is_similar_accepts_max_edit() {
        let a = "aaaa".to_string();
        let b = "aabb".to_string();

        let result = is_similar(&a, &b, 0.5);
        assert!(result);
    }

    #[test]
    fn is_similar_rejects_above_max_edit() {
        let a = "a".to_string();
        let b = "abbb".to_string();

        let result = is_similar(&a, &b, 1.0);
        assert!(!result);
    }

    #[test]
    fn max_edit_calculation_correct() {
        let a = "aa".to_string();
        let b = "bbb".to_string();

        let result = get_max_edit_dist(&a, &b, 0.5);
        assert_eq!(result, 1);
    }

    #[test]
    fn max_edit_calculation_applies_floor() {
        let a = "aaaa".to_string();
        let b = "bbbb".to_string();

        let result = get_max_edit_dist(&a, &b, 0.49);
        assert_eq!(result, 1);
    }

    #[test]
    fn max_edit_shorted_string_selected() {
        let a = "aa".to_string();
        let b = "bbb".to_string();

        let op_one = get_max_edit_dist(&a, &b, 0.5);
        let op_two = get_max_edit_dist(&b, &a, 0.5);
        assert_eq!(op_one, op_two);
    }

    #[test]
    fn max_edit_handles_empty() {
        let a = "".to_string();
        let b = "bbb".to_string();

        let result = get_max_edit_dist(&a, &b, 0.5);
        assert_eq!(result, 0);
    }
}

// #[cfg(test)]
// mod tests {
//     #[test]
//     fn it_works() {
//         let result = 2 + 2;
//         assert_eq!(result, 4);
//     }
// }
