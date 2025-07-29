use rand::rng; // cargo add rand
use rand::Rng;
use std::time::{SystemTime};
use rust_xlsxwriter::{Workbook, XlsxError}; // cargo add rust_xlsxwriter

// other files
mod quicksort;
mod binary_search;

fn main() -> Result<(), XlsxError>{
    const ARRAY_SIZE: usize = 100_000;
    let mut rng = rng();

    // creating the excel document
    let mut workbook = Workbook::new();
    let worksheet = workbook.add_worksheet();

    // run the tests and output the data excel for each line
    for i in 0..10{
        // generating and filling a random array
        let mut array: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
        for j in 0..ARRAY_SIZE{
            array[j] = rng.random_range(i32::MIN..=i32::MAX);
        }

        let x = run_test(0, &mut array);
        worksheet.write(i, 0, x)?;

        let y = run_test(1, &mut array);
        worksheet.write(i, 1, y)?;
    }

    // save the excel document and name it
    workbook.save("results.xlsx")?;
    Ok(())
}
fn run_test(instance: i32, array: &mut[i32]) -> f64{
    let mut rng = rng();
    let start = SystemTime::now();

    // each sorting algorithm depending on input
    if instance == 1{
        array.sort();
    } else{
        let high = (array.len() - 1) as isize;
        quicksort::quicksort(array, 0, high);
    }
    // generating a random one to look for
    let target_index: usize = rng.random_range(0..array.len());
    let target_value: i32 = array[target_index];

    // checking whether the algorithm was correct
    if instance == 1{
        let x = array.binary_search(&target_value).unwrap_or(usize::MAX);
        assert!(x != usize::MAX);
        assert_eq!(x, target_index as usize);
    } else{
        let x: i32 = binary_search::binary_search(&array, target_value);
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