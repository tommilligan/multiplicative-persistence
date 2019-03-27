/// An iterator for all the `n`-length combinations of a collection, with replacement.
#[derive(Debug, Clone)]
pub struct CombinationsWithReplacement<T: Copy> {
    // Derived value, whether the iterator will have no values
    empty: bool,
    // Collection of (probably) unique elements
    pool: Vec<T>,
    // Vector of indexes of pool
    mask: Vec<usize>,
    // The length of our output combination
    mask_length: usize,
    // The max value of any single int in our mask
    mask_max_value: usize,
    // Whether this is the first iteration
    starting: bool,
}

impl<T: Copy> CombinationsWithReplacement<T> {
    /// Create a new CombinationsWithReplacement iterator from a `Vec` of `Copy`-able elements.
    pub fn new(pool: Vec<T>, n: usize) -> CombinationsWithReplacement<T> {
        // If either the pool is empty or the size is zero, return None immediately
        let empty = n == 0 || pool.len() == 0;
        CombinationsWithReplacement {
            mask_max_value: if empty { 0 } else { pool.len() - 1 },

            empty,
            pool,
            mask: vec![0; n],
            mask_length: n,
            starting: true,
        }
    }

    /// Map the current mask over the pool to get an output combination
    fn current(&self) -> Vec<T> {
        self.mask.iter().map(|i| self.pool[*i]).collect()
    }
}

impl<T: Copy> Iterator for CombinationsWithReplacement<T> {
    type Item = Vec<T>;

    fn next(&mut self) -> Option<Vec<T>> {
        // If this is the first iteration
        if self.starting {
            // In empty edge cases, stop iterating immediately
            return if self.empty {
                None
            // Otherwise, yield the initial state
            } else {
                self.starting = false;
                Some(self.current())
            };
        }

        // Work out where we need to update our mask
        let mut increment: Option<(usize, usize)> = None;
        for (i, mask_int) in self.mask.iter().enumerate().rev() {
            if mask_int < &self.mask_max_value {
                increment = Some((i, mask_int + 1));
                break;
            }
        }

        match increment {
            Some((increment_from, increment_value)) => {
                // We need to update the rightmost non-max value
                // and all those to the right
                for mask_index in increment_from..self.mask_length {
                    self.mask[mask_index] = increment_value
                }
                Some(self.current())
            }
            // If we have nothing to increment, we're done
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_combinations_with_replacement() {
        // Pool smaller than size
        let mut combinations = CombinationsWithReplacement::new(vec!['A'], 2);
        assert_eq!(combinations.next(), Some(vec!['A', 'A']));
        assert_eq!(combinations.next(), None);
        assert_eq!(combinations.next(), None);

        // Pool larger than size
        let combinations: Vec<Vec<char>> =
            CombinationsWithReplacement::new(vec!['A', 'B', 'C'], 2).collect();
        assert_eq!(
            combinations,
            vec![
                vec!['A', 'A'],
                vec!['A', 'B'],
                vec!['A', 'C'],
                vec!['B', 'B'],
                vec!['B', 'C'],
                vec!['C', 'C'],
            ]
        );

        let empty_char_combinations: Vec<Vec<char>> = vec![];
        // Zero size
        let combinations: Vec<Vec<char>> = CombinationsWithReplacement::new(vec!['A'], 0).collect();
        assert_eq!(combinations, empty_char_combinations);

        // Empty pool
        let combinations: Vec<Vec<char>> = CombinationsWithReplacement::new(vec![], 2).collect();
        assert_eq!(combinations, empty_char_combinations);
    }
}
