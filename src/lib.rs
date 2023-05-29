use std::cmp::Ordering;

trait BinarySearch<T> {
    fn bsearch(&self, niddle: T) -> Option<usize>;
}

impl<T> BinarySearch<T> for &[T]
where
    T: Ord,
{
    fn bsearch(&self, niddle: T) -> Option<usize> {
        if self.is_empty() {
            return None;
        }

        let mid = self.len() / 2;
        match niddle.cmp(&self[mid]) {
            Ordering::Less => (&self[..mid]).bsearch(niddle),
            Ordering::Equal => Some(mid),
            Ordering::Greater => (&self[mid + 1..]).bsearch(niddle).map(|idx| mid + 1 + idx),
        }
    }
}

impl<T> BinarySearch<T> for Vec<T>
where
    T: Ord,
{
    fn bsearch(&self, niddle: T) -> Option<usize> {
        self.as_slice().bsearch(niddle)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn initialize_vec() -> Vec<u32> {
        let mut vec = Vec::with_capacity(10000);
        for i in 0..10_000 {
            vec.push(i * 2);
        }

        vec
    }

    fn find_and_check(niddle: u32) {
        let vec = initialize_vec();

        let expected = vec
            .iter()
            .enumerate()
            .find_map(|(i, &x)| (x == niddle).then_some(i));
        let actual = vec.bsearch(niddle);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_existing() {
        find_and_check(876);
        find_and_check(10152);
    }

    #[test]
    fn dont_find_nonexisting() {
        find_and_check(1051);
    }

    #[test]
    fn dont_find_out_of_range() {
        find_and_check(30152);
    }
}
