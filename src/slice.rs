pub struct Permutations<'a, T: Ord> {
    elements: &'a mut [T],
}

impl<'a, T: Ord> Permutations<'a, T> {
    pub fn new(elements: &'a mut [T]) -> Self {
        Self { elements }
    }

    pub fn get(&self) -> &[T] {
        self.elements
    }

    pub fn get_mut(&mut self) -> &mut [T] {
        self.elements
    }

    #[allow(clippy::should_implement_trait)]
    pub fn next(&mut self) -> bool {
        if self.elements.len() < 2 {
            return false;
        }

        let mut i = self.elements.len() - 1;
        loop {
            i -= 1;
            if self.elements[i] < self.elements[i + 1] {
                let mut k = self.elements.len() - 1;
                while self.elements[i] >= self.elements[k] {
                    k -= 1;
                }
                self.elements.swap(i, k);
                self.elements[i + 1..].reverse();
                return true;
            }
            if i == 0 {
                self.elements.reverse();
                return false;
            }
        }
    }

    pub fn previous(&mut self) -> bool {
        if self.elements.len() < 2 {
            return false;
        }

        let mut i = self.elements.len() - 1;
        loop {
            i -= 1;
            if self.elements[i] > self.elements[i + 1] {
                let mut k = self.elements.len() - 1;
                while self.elements[i] <= self.elements[k] {
                    k -= 1;
                }
                self.elements.swap(i, k);
                self.elements[i + 1..].reverse();
                return true;
            }
            if i == 0 {
                self.elements.reverse();
                return false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_permutation() {
        let mut nums = vec![1, 2, 3, 3];
        let mut p = Permutations::new(&mut nums);
        assert!(p.next());
        assert_eq!(p.get(), [1, 3, 2, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [1, 3, 3, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 1, 3, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 3, 1, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 3, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 2, 1, 3]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 2, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 3, 1, 2]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 3, 2, 1]);
        assert!(!p.next());
        assert_eq!(p.get(), [1, 2, 3, 3]);
    }

    #[test]
    fn test_previous_permutation() {
        let mut nums = vec![3, 3, 2, 1];
        let mut p = Permutations::new(&mut nums);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 3, 1, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 2, 3, 1]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 2, 1, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 1, 3, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.next());
        assert_eq!(p.get(), [3, 1, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 3, 1]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 3, 1, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [2, 1, 3, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 3, 3, 2]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 3, 2, 3]);
        assert!(p.previous());
        assert_eq!(p.get(), [1, 2, 3, 3]);
        assert!(!p.previous());
        assert_eq!(p.get(), [3, 3, 2, 1]);
    }
}
