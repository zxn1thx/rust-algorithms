use rand::rng;
use rand::Rng;
use std::time::{SystemTime};
use rust_xlsxwriter::{Workbook, XlsxError};
// cargo add rand
// cargo add 
// cargo add rust_xlsxwriter

fn main() -> Result<(), XlsxError>{
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    // run user test
    for i in 0..1_000{
        let x = run_test(0);
        worksheet.write(i, 0, x)?;
    }
    // run rust test
    for j in 0..1_000{
        let y = run_test(1);
        worksheet.write(j, 1, y)?;
    }
    workbook.save("results.xlsx")?;
    Ok(())
}
fn run_test(instance: i32) -> f64{
    let start = SystemTime::now();
    // picks number of times to run test
        let mut rng = rng();
        // generate each array
        let mut array = [0; 100_000];
        for j in 0..100_000{
            array[j] = rng.random_range(i32::MIN..=i32::MAX);
        }
        // each sorting algorithm depending on input
        if instance == 1{
            array.sort();
        } else{
            let high = (array.len() - 1) as isize;
            quicksort(&mut array, 0, high);
        }
        // generating a random one to look for
        let target_index: usize = rng.random_range(0..100_000);
        let target_value: i32 = array[target_index];
        // ensure that the results are valid
        if instance == 1{
            let x = array.binary_search(&target_value).unwrap_or(usize::MAX);
            assert!(x != usize::MAX);
            assert_eq!(x, target_index as usize);
        } else{
            let x: i32 = binary_search(&array, target_value);
            assert!(x != -1);
            assert_eq!(x, target_index as i32);
        }
    // calculating the runtime
    let end = SystemTime::now();
    let duration = end.duration_since(start)
        .expect("Time went backwards");
    // returning the data
    duration.as_secs() as f64 + duration.subsec_nanos() as f64 * 1e-9
}
fn binary_search(array: &[i32], target: i32) -> i32{
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
// merge sort, quicksort
fn quicksort(array: &mut[i32], low: isize, high: isize){
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