fn max_subarray_sum(arr: &[i32]) -> i32 {
    let mut max = 0;
    let mut currMax = i32::MIN;

    for &num in arr {
        max = max + num;
        currMax = currMax.max(max_ending);

        if max < 0 {
            max = 0;
        }
    }

    maxCurr
}

fn main() {
    let arr = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let max_sum = max_subarray_sum(&arr);
    
    println!("Maximum subarray sum: {}", max_sum);
}
