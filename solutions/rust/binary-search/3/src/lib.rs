use std::cmp::Ordering;

pub fn find<S: AsRef<[T]>, T: Ord>(array: S, key: T) -> Option<usize> {
    inner_find(array, key, 0)
}

fn inner_find<S: AsRef<[T]>, T: Ord>(array: S, key: T, offset: usize) -> Option<usize> {
    let array = array.as_ref();
    if array.is_empty() {
        return None
    }
    let middle = array.len() / 2;

    match Some(&key).cmp(&array.get(middle)) {
        Ordering::Equal => Some(middle + offset),
        Ordering::Less => { inner_find(&array[..middle], key, offset) },
        Ordering::Greater => { inner_find(&array[middle + 1..], key, offset + middle + 1) },
    }
}