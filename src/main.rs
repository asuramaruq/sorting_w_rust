extern crate sorting_w_rust;

use sorting_w_rust::quick_sort;
use sorting_w_rust::selection_sort;
use sorting_w_rust::insertion_sort;
use sorting_w_rust::merge_sort;

fn main() {
    let mut arr = [3, 2, 1];

    quick_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = [3, 2, 1];
    selection_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = [3, 2, 1];
    insertion_sort(&mut arr);
    println!("{:?}", arr);

    let mut arr = [3, 2, 1];
    merge_sort(&mut arr);
    println!("{:?}", arr);
}