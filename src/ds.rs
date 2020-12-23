pub struct UnionByRank {
    entries: Vec<Entry>,
}

struct Entry {
    parent: usize,
    rank: usize,
}

impl Entry {
    fn new(i: usize) -> Self {
        Entry { parent: i, rank: 0 }
    }
}

impl UnionByRank {
    pub fn new(n: usize) -> Self {
        Self {
            entries: (0..n).map(|i| Entry::new(i)).collect(),
        }
    }

    pub fn find(&self, i: usize) -> usize {
        let mut k = i;
        while self.entries[k].parent != k {
            k = self.entries[k].parent;
        }
        k
    }

    pub fn find_and_compress(&mut self, i: usize) -> usize {
        let mut k = i;
        while self.entries[k].parent != k {
            k = self.entries[k].parent;
        }
        let mut j = i;
        while self.entries[j].parent != j {
            j = std::mem::replace(&mut self.entries[j].parent, k);
        }
        k
    }

    pub fn merge(&mut self, i: usize, k: usize) -> usize {
        use std::cmp::Ordering::*;
        let leader_i = self.find_and_compress(i);
        let leader_k = self.find_and_compress(k);
        match self.entries[leader_i]
            .rank
            .cmp(&self.entries[leader_k].rank)
        {
            Less => {
                self.entries[leader_i].parent = leader_k;
                leader_k
            }
            Greater => {
                self.entries[leader_k].parent = leader_i;
                leader_i
            }
            Equal => {
                self.entries[leader_i].parent = leader_k;
                self.entries[leader_i].rank += 1;
                leader_k
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_union_by_rank() {
        let mut u = UnionByRank::new(10);
        for i in 0..10 {
            assert_eq!(i, u.find_and_compress(i));
        }
        u.merge(0, 1);
        assert_eq!(u.find_and_compress(0), u.find_and_compress(1));
        u.merge(5, 6);
        assert_eq!(u.find_and_compress(5), u.find_and_compress(6));
        assert_ne!(u.find_and_compress(0), u.find_and_compress(6));
        u.merge(1, 5);
        assert_eq!(u.find_and_compress(0), u.find_and_compress(6));
    }
}
