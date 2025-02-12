/*!
# Matrix-rs: A generic matrix library in Rust
Matrix-rs is a simple matrix library designed to be easy to use.

# Disclaimer
This crate should not be considered mature enough for *professional use*, check alternatives like [cgmath](https://github.com/brendanzab/cgmath) or [nalgebra](https://github.com/sebcrozet/nalgebra) if you are in that case.

If you are still interested, feel free to continue!

# Usage
Link it in your project's `Cargo.toml` file:
```text
# Example Cargo.toml

[dependencies]
simple-matrix = "0.1"
```

Then, you can use it in your project:

```
use simple_matrix::Matrix;

let mat: Matrix<i32> = Matrix::zero(2, 3);
```

# Example: Basic matrix usage
```
use simple_matrix::Matrix;

// Create a matrix of default cells
let zero: Matrix<u32> = Matrix::zero(2, 4);

// Create a 2x4 matrix from an iterator (fill it row by row)
let mat1: Matrix<u32> = Matrix::from_iter(2, 4, 0..);

// Clone a matrix
let mat2 = mat1.clone();


// Add by reference (do not consume them)
let mut add = &mat1 + &mat2;

// Subtract by value (consume them)
let mut sub = mat1 - mat2;

// OpAssign are also available
sub += &zero;
sub -= zero;


// Get cells
let val: u32 = add.get(0, 3).unwrap();

// Set cells
add.set(0, 3, 0);

// Iterate through the matrix (row by row)
for val in add {
    print!("{} ", val);
}
```

# Example: Dot product
```
use simple_matrix::Matrix;

let mat: Matrix<f64> = Matrix::zero(2, 4);

// Construct the transposed matrix
let mat_t = mat.transpose();

// Construct the dot product
let dot = mat * mat_t;
```

# Features
- *Features are extensions of the library left to opt-in by the user.*
- *They can increase compilation time and library size.*

To include a feature, add it to your `Cargo.toml` file:
```text
# Example Cargo.toml with added feature (replace values with your own)

[dependencies]
simple-matrix = { version = "0.1", features = ["impl_from"] }
```

Current available features are listed below with a little description:
## impl_from
Implements the *From* Trait for basic numeric types.

```ignore
use simple_matrix::Matrix;

let m1: Matrix<i8> = Matrix::zero(3, 5);
let m2: Matrix<i64> = m1.into();
```
*/

#![deny(missing_docs)]

mod matrix;

pub use crate::matrix::*;
