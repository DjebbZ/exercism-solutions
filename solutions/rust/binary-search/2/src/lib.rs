use std::cmp::Ordering;

pub fn find(array: &[i32], key: i32) -> Option<usize> {
    inner_find(array, key, array)
}

fn inner_find(array: &[i32], key: i32, original_array: &[i32]) -> Option<usize> {
    if array.is_empty() {
        return None
    }

    let middle = array.len() / 2;

    match Some(&key).cmp(&array.get(middle)) {
        Ordering::Equal => Some(original_array.iter().position(|&x| x == key).unwrap()),
        Ordering::Less => { inner_find(&array[..middle], key, original_array) },
        Ordering::Greater => { inner_find(&array[middle + 1..], key, original_array) },
    }
}