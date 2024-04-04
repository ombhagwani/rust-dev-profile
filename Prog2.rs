fn first_occurrence(arr: &[i32], target: i32) -> Option<usize> {
    arr.iter().position(|&x| x == target)
}

fn main() {
    let arr = [1, 2, 2, 3, 4, 4, 4, 5];
    let target = 4;
    if let Some(index) = first_occurrence(&arr, target) {
        println!("The first occurrence of {} is at index {}.", target, index);
    } else {
        println!("{} is not found in the array.", target);
    }
}
