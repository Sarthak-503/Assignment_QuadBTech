fn find_median(arr: &[i32]) -> Option<f64> {
    let len = arr.len();
    if len == 0 {
        return None;
    }
    let mid = len / 2;
    if len % 2 == 1 {
        // If the length is odd, return the middle element
        Some(arr[mid] as f64)
    } else {
        // If the length is even, return the average of the two middle elements
        let median = (arr[mid - 1] + arr[mid]) as f64 / 2.0;
        Some(median)
    }
}

fn main() {
    let arr_odd = [1, 2, 3, 4, 15];
    let arr_even = [1, 2, 3, 4];
    
    if let Some(median_odd) = find_median(&arr_odd) {
        println!("Median of odd length array: {}", median_odd);
    } else {
        println!("Array is empty.");
    }
    
    if let Some(median_even) = find_median(&arr_even) {
        println!("Median of even length array: {}", median_even);
    } else {
        println!("Array is empty.");
    }
}
