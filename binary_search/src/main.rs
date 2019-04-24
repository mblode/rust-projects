use std::cmp::Ordering::*;

fn binary_search<T: Ord>(data: &[T], target: &T) -> Option<usize> {
    let mut high = data.len();
    let mut low = 0;
    let mut mid = high / 2;

    while low < high {
        let _result = match target.cmp(&data[mid]) {
            Less => {
                high = mid;
            }
            Greater => {
                low = mid;
            }
            Equal => return Some(mid),
        };
        mid = (high + low) / 2;
    }
    None
}

fn main() {
    println!("Binary search");

    let numbers: [i32; 5] = [1, 2, 3, 4, 5];

    let target: i32 = 4;
    let result = binary_search(&numbers, &target);

    match result {
        // The division was valid
        Some(index) => println!("\'{}\' is present at index: {}", target, index),
        // The division was invalid
        None => println!("\'{}\' is not present in array", target),
    }
}
