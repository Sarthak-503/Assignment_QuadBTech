fn binary_search(arr: &[i32], target: i32) -> Option<usize> {
    let mut left = 0;
    let mut right = arr.len() - 1;

    while left <= right {
        let mid = left + (right - left) / 2;

        if arr[mid] == target {
            // Found the target, now check if it's the first occurrence
            let mut first_occurrence = mid;
            while first_occurrence > 0 && arr[first_occurrence - 1] == target {
                first_occurrence -= 1;
            }
            return Some(first_occurrence);
        } else if arr[mid] < target {
            left = mid + 1;
        } else {
            right = mid - 1;
        }
    }

    None
}

fn main() {
    let arr = [1, 2, 3, 3, 3, 4, 5, 6, 7];
    let target = 3;
    if let Some(index) = binary_search(&arr, target) {
        println!("The index of the first occurrence of {} is {}", target, index);
    } else {
        println!("{} not found in the array.", target);
    }
}
