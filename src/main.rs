use rand::rng;
use rand::Rng;
use std::time::{SystemTime};
use rust_xlsxwriter::{Workbook, XlsxError};
// cargo add rand
// cargo add 
// cargo add rust_xlsxwriter

// other files
mod sorting;
mod searching;

fn main() -> Result<(), XlsxError>{
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();
    // run user test
    for i in 0..10{
        let x = run_test(0);
        worksheet.write(i, 0, x)?;
    }
    // run rust test
    for j in 0..10{
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
            sorting::quicksort(&mut array, 0, high);
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
            let x: i32 = searching::binary_search(&array, target_value);
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