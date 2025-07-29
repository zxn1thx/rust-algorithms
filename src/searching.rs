pub fn binary_search(array: &[i32], target: i32) -> i32{
    let mut left: usize = 0;
    let mut right: usize = array.len();
    while left < right {
        let mid = (left + right) / 2;
        if array[mid] == target {
            return mid as i32;
        } else if array[mid] < target{
            left = mid + 1;
        } else {
            right = mid;
        }
    }
    -1
}