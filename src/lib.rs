use std::cmp::Ordering;

pub fn bsearch(slice: &[u32], niddle: u32) -> Option<usize> {
    if slice.is_empty() {
        return None;
    }

    let mid = slice.len() / 2;
    match niddle.cmp(&slice[mid]) {
        Ordering::Less => bsearch(&slice[..mid - 1], niddle),
        Ordering::Equal => Some(mid),
        Ordering::Greater => bsearch(&slice[mid..], niddle).map(|idx| mid + idx),
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
        let actual = bsearch(&vec, niddle);

        assert_eq!(expected, actual);
    }

    #[test]
    fn find_existing() {
        find_and_check(876);
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
