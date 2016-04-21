use std::cmp::Ordering;

/// This function provides a binary search
///
/// #Examples
///
/// ```
/// use rosalind_algo::bins::bsearch;
/// println!("{}", bsearch(25, &vec!(1,123,25)));
/// ```
pub fn bsearch(number: i32, sorted_values: &Vec<i32>) -> isize {
    let mut index: isize = -1;
    let mut range: (usize, usize) = (0, sorted_values.len() - 1);
    let mut offset: usize = (range.1 - range.0) / 2;

    if sorted_values[range.0] == number {
        return range.0 as isize;
    }

    if sorted_values[range.1] == number {
        return range.1 as isize;
    }

    while offset > 0 {
        match number.cmp(&sorted_values[range.0+offset]) {
            Ordering::Equal     => { index = (range.0+offset) as isize; break }
            Ordering::Greater   => range = (range.0 + offset, range.1),
            Ordering::Less      => range = (range.0, range.0 + offset)
        }
        offset = (range.1 - range.0) / 2;
    }

    return index;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_for_list_of_2_elements() {
        let values = vec!(2,5);
        assert_eq!(bsearch(3, &values), -1);
        assert_eq!(bsearch(2, &values), 0);
        assert_eq!(bsearch(5, &values), 1);
    }

    #[test]
    fn it_works_for_list_of_3_elements() {
        let values = vec!(2,5,6);
        assert_eq!(bsearch(3, &values), -1);
        assert_eq!(bsearch(2, &values), 0);
        assert_eq!(bsearch(5, &values), 1);
        assert_eq!(bsearch(6, &values), 2);
    }

    #[test]
    fn it_works_for_list_of_4_elements() {
        let values = vec!(2,5,7,8);
        assert_eq!(bsearch(3, &values), -1);
        assert_eq!(bsearch(2, &values), 0);
        assert_eq!(bsearch(5, &values), 1);
        assert_eq!(bsearch(7, &values), 2);
        assert_eq!(bsearch(8, &values), 3);
    }

    #[test]
    fn it_works_for_list_of_5_elements() {
        let values = vec!(2,5,7,8,9);
        assert_eq!(bsearch(3, &values), -1);
        assert_eq!(bsearch(2, &values), 0);
        assert_eq!(bsearch(5, &values), 1);
        assert_eq!(bsearch(7, &values), 2);
        assert_eq!(bsearch(8, &values), 3);
        assert_eq!(bsearch(9, &values), 4);
    }
}
