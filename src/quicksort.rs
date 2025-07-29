pub fn quicksort(array: &mut[i32], low: isize, high: isize){
    if low < high {
        let p = partition(array, low, high);
        quicksort(array, low, p - 1);
        quicksort(array, p + 1, high);
    }
}
fn partition(array: &mut[i32], low: isize, high: isize) -> isize{
    let pivot = array[high as usize];
    let mut i = low - 1;
    for j in low..high{
        if array[j as usize] < pivot{
            i += 1;
            array.swap(i as usize, j as usize);
        }
    }
    array.swap((i + 1) as usize, high as usize);
    i + 1
}