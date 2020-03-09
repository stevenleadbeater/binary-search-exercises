struct BinarySearch {
    sorted_array: Vec<i64>,
}

impl BinarySearch {
    fn new(array: Vec<i64>) -> BinarySearch {
        BinarySearch {
            sorted_array: array,
        }
    }

    fn search(&self, target: i64) -> i64 {
        let mut local_array = self.sorted_array.clone();
        let mut basis = 0;
        loop {
            let index = (local_array.len() / 2);
            if local_array[index] == target {
                return (basis + index) as i64;
            } else if local_array[index] > target {
                local_array = local_array.split_at(index).0.to_vec();
            } else if local_array[index] < target {
                local_array = local_array.split_at(index).1.to_vec();
                basis += index;
            }
            if local_array.len() == 1 && local_array[0] != target {
                break;
            }
        }
        -1
    }
}

#[cfg(test)]
mod tests {
    use crate::binary_search::BinarySearch;

    #[test]
    fn center() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(53), 7);
    }

    #[test]
    fn left() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(33), 4);
    }

    #[test]
    fn right() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(84), 10);
    }

    #[test]
    fn not_found_left() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(34), -1);
    }

    #[test]
    fn not_found_right() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(85), -1);
    }

    #[test]
    fn far_left() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(6), 0);
    }

    #[test]
    fn far_right() {
        let searcher = BinarySearch::new(vec![6, 13, 14, 25, 33, 43, 51, 53, 64, 72, 84, 93, 95, 96, 97]);
        assert_eq!(searcher.search(97), 14);
    }

}