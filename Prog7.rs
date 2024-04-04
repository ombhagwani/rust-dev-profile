fn kth_smallest(arr: &[i32], k: usize) -> Option<i32> {
    if k > arr.len() {
        return None;
    }
    
    let mut sorted_arr = arr.to_vec();
    sorted_arr.sort();

    Some(sorted_arr[k - 1])
}

fn main() {
    let array = [4, 2, 7, 1, 9, 5];
    let k = 3;

    if let Some(kth_smallest_element) = kth_smallest(&array, k) {
        println!("The {}th smallest element in the array is: {}", k, kth_smallest_element);
    } else {
        println!("The array does not have enough elements.");
    }
}
