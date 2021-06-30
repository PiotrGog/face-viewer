#![feature(test)]

extern crate test;

use test::Bencher;

fn test_multiplication_ndarray(a: &ndarray::Array2<f32>, b: &ndarray::Array2<f32>) {
    let c = a.dot(b);
}

fn test_multiplication_nalgebra(a: &nalgebra::DMatrix<f32>, b: &nalgebra::DMatrix<f32>) {
    let c = a * b;
}

#[bench]
fn nalgebra_matrix_multiplication(b: &mut Bencher) {
    let a_array = nalgebra::DMatrix::<f32>::repeat(15_000, 200, 1.2f32);
    let b_array = nalgebra::DMatrix::<f32>::repeat(200, 1, 3.2f32);
    b.iter(move || test_multiplication_nalgebra(&a_array, &b_array))
}

#[bench]
fn ndarray_matrix_multiplication(b: &mut Bencher) {
    let a_array = ndarray::Array::from_elem((15_000, 200), 1.2f32);
    let b_array = ndarray::Array::from_elem((200, 1), 3.2f32);
    b.iter(move || test_multiplication_ndarray(&a_array, &b_array))
}

#[cfg(test)]
fn test_nalgebra_matrix_multiplication() {
    let a_array = nalgebra::DMatrix::<f32>::repeat(15_000, 200, 2.0f32);
    let b_array = nalgebra::DMatrix::<f32>::repeat(200, 1, 3.2f32);
    test_multiplication_nalgebra(&a_array, &b_array)
}

#[cfg(test)]
fn test_ndarray_matrix_multiplication() {
    let a_array = ndarray::Array::from_elem((15_000, 200), 1.2f32);
    let b_array = ndarray::Array::from_elem((200, 1), 3.2f32);
    test_multiplication_ndarray(&a_array, &b_array)
}
