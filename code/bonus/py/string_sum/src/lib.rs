use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

#[pyfunction]
fn say_hello() -> PyResult<()> {
    println!("Hello, world!");
    Ok(())
}

#[pyclass]
struct MyClass {
    #[pyo3(get)]
    data: i32,
}

#[pymethods]
impl MyClass {
    #[new]
    pub fn new(data: i32) -> MyClass {
        MyClass { data }
    }
}

fn fibo(n: u64) -> u64 {
    match n {
        0 => 1,
        1 => 1,
        _ => fibo(n - 1) + fibo(n - 2),
    }
}

#[pyfunction]
fn recur_fibo(n: u64) -> PyResult<u64> {
    Ok(fibo(n))
}

#[pyfunction]
fn fibo_range(n: u64) -> PyResult<Vec<u64>> {
    use rayon::prelude::*;

    let targets: Vec<u64> = (0 .. n).collect();
    let results: Vec<u64> = targets
        .par_iter()
        .map(|n| fibo(*n))
        .collect();
    Ok(results)
}

/// A Python module implemented in Rust.
#[pymodule]
fn string_sum(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<MyClass>()?;
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(say_hello, m)?)?;
    m.add_function(wrap_pyfunction!(recur_fibo, m)?)?;
    m.add_function(wrap_pyfunction!(fibo_range, m)?)?;
    Ok(())
}
