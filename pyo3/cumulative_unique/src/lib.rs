use pyo3::prelude::*;
use ndarray::Array1;
use std::collections::HashSet;

/// Calculates the cumulative unique mask
#[pyfunction]
fn cunique(arr: Vec<i64>) -> PyResult<Vec<bool>> {
    let arr = Array1::from(arr);
    let len = arr.len();
    let mut unique_mask = vec![true; len];
    let mut unique_set = HashSet::new();

    for (i, &item) in arr.iter().enumerate() {
        if unique_set.contains(&item) {
            unique_mask[i] = false;
        } else {
            unique_set.insert(item);
        }
    }

    Ok(unique_mask)
}

#[pymodule]
fn cumulative_unique(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cunique, m)?)?;
    Ok(())
}

