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
        if self.sorted_array.len() == 0 {
            return -1;
        }
        let mut low = 0;
        let mut high = self.sorted_array.len() - 1;
        let mut mid = high / 2;
        while low <= high {
            mid = low + ((high - low) / 2);
            if self.sorted_array[mid] > target {
                high = mid - 1;
            } else if self.sorted_array[mid] < target {
                low = mid + 1;
            } else {
                return mid as i64;
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