pub trait Comparable {
    fn compare(&self, other: &Self) -> bool;
}

impl<T: PartialOrd> Comparable for T {
    fn compare(&self, other: &Self) -> bool {
        self <= other
    }
}

pub fn quick_sort<T: Comparable + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let pivot_index = partition(arr);
    if pivot_index > 0 {
        quick_sort(&mut arr[0..pivot_index]);
    }
    if pivot_index + 1 < len {
        quick_sort(&mut arr[pivot_index + 1..]);
    }
}

fn partition<T: Comparable + Copy>(arr: &mut [T]) -> usize {
    let len = arr.len();
    let pivot_index = len / 2;
    let pivot = arr[pivot_index];
    arr.swap(pivot_index, len - 1);
    let mut i = 0;

    for j in 0..len - 1 {
        if arr[j].compare(&pivot) {
            arr.swap(i, j);
            i += 1;
        }
    }

    arr.swap(i, len - 1);
    i
}

pub fn selection_sort<T: Comparable>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut smallest = i;
        for j in i + 1..len {
            if arr[j].compare(&arr[smallest]) {
                smallest = j;
            }
        }
        arr.swap(smallest, i);
    }
}

pub fn insertion_sort<T: Comparable>(arr: &mut [T]) {
    let len = arr.len();
    for i in 1..len {
        let mut j = i;
        while j > 0 && arr[j].compare(&arr[j - 1]) {
            arr.swap(j, j - 1);
            j -= 1;
        }
    }
}

pub fn merge_sort<T: Comparable + Copy>(arr: &mut [T]) {
    let len = arr.len();
    if len < 2 {
        return;
    }

    let mid = len / 2;
    merge_sort(&mut arr[0..mid]);
    merge_sort(&mut arr[mid..]);

    let mut merged = arr.to_vec();
    merge(&arr[0..mid], &arr[mid..], &mut merged);
    arr.copy_from_slice(&merged);
}

fn merge<T: Comparable + Copy>(a: &[T], b: &[T], merged: &mut [T]) {
    let (mut i, mut j, mut k) = (0, 0, 0);

    while i < a.len() && j < b.len() {
        if a[i].compare(&b[j]) {
            merged[k] = a[i];
            i += 1;
        } else {
            merged[k] = b[j];
            j += 1;
        }
        k += 1;
    }

    if i < a.len() {
        merged[k..].copy_from_slice(&a[i..]);
    }
    if j < b.len() {
        merged[k..].copy_from_slice(&b[j..]);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut arr = [3, 2, 1];
        quick_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_selection_sort() {
        let mut arr = [3, 2, 1];
        selection_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_insertion_sort() {
        let mut arr = [3, 2, 1];
        insertion_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }

    #[test]
    fn test_merge_sort() {
        let mut arr = [3, 2, 1];
        merge_sort(&mut arr);
        assert_eq!(arr, [1, 2, 3]);
    }
}