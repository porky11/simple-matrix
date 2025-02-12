#[cfg(feature = "impl_from")]
mod from;
mod iter;
mod std_ops;

use num_traits::{One, Zero};

use std::ops::{Deref, Div, Index, IndexMut, Mul, Sub};

/// A 2-Dimensional, non-resizable container.
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd)]
pub struct Matrix<T> {
    rows: usize,
    cols: usize,
    data: Vec<T>,
}

impl<T> Matrix<T> {
    /// Constructs a new Matrix<T> from a 2D array.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<i32> = Matrix::new([[1, 2], [3, 4], [5, 6]]);
    /// ```
    pub fn new<const R: usize, const C: usize>(values: [[T; R]; C]) -> Matrix<T>
    where
        T: Zero,
    {
        Matrix::from_iter(R, C, values.into_iter().flatten())
    }

    /// Constructs a new Matrix<T> where cells are set to zero.
    /// Use `Matrix::from_iter` if you want to set the matrix from an iterator.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mut mat: Matrix<i32> = Matrix::zero(3, 6);
    /// ```
    pub fn zero(rows: usize, cols: usize) -> Matrix<T>
    where
        T: Zero,
    {
        Matrix::from_iter(rows, cols, (0..).map(|_| T::zero()))
    }

    /// Constructs a new identity Matrix<T> of a specified size.
    ///
    /// # Panics
    /// Panics if `size` is equal to `0`
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::identity(3);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 1);
    /// assert_eq!(mat.get(0, 1).unwrap(), 0);
    /// assert_eq!(mat.get(2, 1).unwrap(), 0);
    /// assert_eq!(mat.get(2, 2).unwrap(), 1);
    /// ```
    pub fn identity(size: usize) -> Matrix<T>
    where
        T: Zero + One,
    {
        let mut result = Self::zero(size, size);
        for i in 0..size {
            result.set(i, i, T::one());
        }
        result
    }

    /// Constructs a new, non-empty Matrix<T> where cells are set from an iterator.  
    /// The matrix cells are set row by row.  
    /// The iterator can be infinite, this method only consume `rows * cols`
    /// values from the iterator.
    ///
    /// # Panics
    /// Panics if either `rows` or `cols` are equal to `0`.  
    /// Panics if the iterator does not have `rows * cols` values
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(0, 1).unwrap(), 1);
    /// assert_eq!(mat.get(1, 0).unwrap(), 6);
    /// ```
    pub fn from_iter(rows: usize, cols: usize, data: impl IntoIterator<Item = T>) -> Matrix<T> {
        assert!(rows > 0 && cols > 0);

        Matrix {
            rows,
            cols,
            data: {
                let data: Vec<_> = data.into_iter().take(rows * cols).collect();
                assert_eq!(data.len(), rows * cols);
                data
            },
        }
    }

    /// Returns the number of rows in the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.rows(), 3);
    /// ```
    pub fn rows(&self) -> usize {
        self.rows
    }

    /// Returns the number of columns in the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.cols(), 6);
    /// ```
    pub fn cols(&self) -> usize {
        self.cols
    }

    /// Try to get the value at given row & column.  
    /// Returns `None` if `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(2, 5).unwrap(), 17);
    ///
    /// assert!(mat.get(10, 2).is_none());
    /// ```
    pub fn get(&self, row: usize, col: usize) -> Option<T>
    where
        T: Clone,
    {
        if row < self.rows && col < self.cols {
            Some(self.data[col + row * self.cols].clone())
        } else {
            None
        }
    }

    /// Try to get a reference to the value at given row & column.  
    /// Returns `None` if `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.get_ref(0, 0).unwrap(), &0);
    /// assert_eq!(mat.get_ref(2, 5).unwrap(), &17);
    ///
    /// assert!(mat.get(10, 2).is_none());
    /// ```
    pub fn get_ref(&self, row: usize, col: usize) -> Option<&T> {
        if row < self.rows && col < self.cols {
            Some(&self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Try to get a mutable reference to the cell at given row & column.  
    /// Returns `None` if `row` or `col` is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mut mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    ///
    /// let cell = mat.get_mut(0, 0).unwrap();
    /// *cell = 5;
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 5);
    /// ```
    pub fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut T> {
        if row < self.rows && col < self.cols {
            Some(&mut self.data[col + row * self.cols])
        } else {
            None
        }
    }

    /// Try to set the cell at given row & column to the given value.  
    /// Returns `false` if `row` or `col` is outside of the matrix.  
    /// Returns `true` if the cell has been modified.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mut mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    ///
    /// mat.set(0, 0, 5);
    /// assert_eq!(mat.get(0, 0).unwrap(), 5);
    /// ```
    pub fn set(&mut self, row: usize, col: usize, value: T) -> bool {
        if let Some(cell) = self.get_mut(row, col) {
            *cell = value;
            true
        } else {
            false
        }
    }

    /// Try to get an iterator of all cells of the requested row.  
    /// Returns `None` if given row is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.get_row(1).unwrap().cloned().collect::<Vec<usize>>(), vec![6, 7, 8, 9, 10, 11]);
    ///
    /// assert!(mat.get_row(5).is_none());
    /// ```
    pub fn get_row(&self, row: usize) -> Option<impl Iterator<Item = &T>> {
        if row < self.rows {
            Some((0..self.cols).map(move |col| self.get_ref(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Try to get an iterator of all cells of the requested column.  
    /// Returns `None` if given row is outside of the matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    ///
    /// assert_eq!(mat.get_col(1).unwrap().cloned().collect::<Vec<usize>>(), vec![1, 7, 13]);
    ///
    /// assert!(mat.get_col(10).is_none());
    /// ```
    pub fn get_col(&self, col: usize) -> Option<impl Iterator<Item = &T>> {
        if col < self.cols {
            Some((0..self.rows).map(move |row| self.get_ref(row, col).unwrap()))
        } else {
            None
        }
    }

    /// Swaps row at the specified indices.
    pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        for col in 0..self.cols {
            self.data
                .swap(col + row1 * self.cols, col + row2 * self.cols);
        }
    }

    /// Swaps columns at the specified indices.
    pub fn swap_cols(&mut self, col1: usize, col2: usize) {
        for row in 0..self.rows {
            self.data
                .swap(col1 + row * self.cols, col2 + row * self.cols);
        }
    }

    /// Take a *M*x*N* Matrix and construct the transposed *N*x*M* Matrix.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// let mat_t = mat.transpose();
    ///
    /// assert_eq!(mat.rows(), mat_t.cols());
    /// assert_eq!(mat.cols(), mat_t.rows());
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), mat_t.get(0, 0).unwrap());
    /// assert_eq!(mat.get(1, 2).unwrap(), mat_t.get(2, 1).unwrap());
    /// ```
    pub fn transpose(&self) -> Matrix<T>
    where
        T: Clone,
    {
        Matrix {
            rows: self.cols,
            cols: self.rows,
            data: {
                let mut data = Vec::with_capacity(self.cols * self.rows);
                for col in 0..self.cols {
                    for val in self.get_col(col).unwrap() {
                        data.push(val.clone());
                    }
                }
                data
            },
        }
    }

    /// Take a *N*x*N* Matrix and construct the inverse of it.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// let mat: Matrix<f32> = Matrix::new([
    ///     [1.0, 0.0, 2.0, 0.0],
    ///     [0.0, 3.0, 0.0, 4.0],
    ///     [5.0, 0.0, 6.0, 0.0],
    ///     [0.0, 7.0, 0.0, 8.0],
    /// ]);
    /// let inverse = mat.inverse().unwrap();
    ///
    /// for (value, expected) in inverse.into_iter().zip(Matrix::new([
    ///     [-1.5, 0.0, 0.5, 0.0],
    ///     [0.0, -2.0, 0.0, 1.0],
    ///     [1.25, 0.0, -0.25, 0.0],
    ///     [0.0, 1.75, 0.0, -0.75],
    /// ])) {
    ///     assert!((value - expected).abs() < 0.01);
    /// }
    /// ```
    pub fn inverse(&self) -> Option<Matrix<T>>
    where
        T: Clone + Zero + One + Sub<Output = T> + Mul<Output = T> + Div<Output = T>,
    {
        if self.rows != self.cols {
            return None;
        }

        let len = self.rows;
        let mut matrix: Matrix<T> = Matrix::zero(len, len * 2);
        for i in 0..len {
            for j in 0..len {
                matrix.set(i, j, self.get(i, j).unwrap());
            }
            matrix.set(i, i + len, T::one());
        }

        let mut lead = 0;

        for r in 0..matrix.rows {
            if matrix.cols <= lead {
                break;
            }
            let mut i = r;
            while matrix.get_ref(i, lead).unwrap().is_zero() {
                i = i + 1;
                if matrix.rows == i {
                    i = r;
                    lead = lead + 1;
                    if matrix.cols == lead {
                        break;
                    }
                }
            }

            matrix.swap_rows(i, r);

            let div = matrix.get_ref(r, lead).unwrap();
            if !div.is_zero() {
                let div = div.clone();
                for j in 0..matrix.cols {
                    let value = matrix.get_mut(r, j).unwrap();
                    *value = value.clone() / div.clone();
                }
            }

            for k in 0..matrix.rows {
                if k != r {
                    let mul = matrix.get(k, lead).unwrap();
                    for j in 0..matrix.cols {
                        let subtracted = matrix.get(r, j).unwrap() * mul.clone();
                        let value = matrix.get_mut(k, j).unwrap();
                        *value = value.clone() - subtracted;
                    }
                }
            }

            lead += 1;
        }

        let mut result: Matrix<T> = Matrix::zero(len, len);
        for i in 0..len {
            for j in 0..len {
                result.set(i, j, matrix.get(i, j + len).unwrap());
            }
        }
        Some(result)
    }

    /// Apply a function to all cells of the matrix.  
    /// Cells are provided as immutable references to the function,
    /// if you want to modify the cells, use `apply_mut`.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// // Get the sum of all cells
    /// let mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// let mut sum = 0;
    /// mat.apply(|n| sum += *n);
    ///
    /// assert_eq!(sum, 153);
    /// ```
    pub fn apply<F: FnMut(&T)>(&self, mut func: F) {
        self.data.iter().for_each(|n| func(n));
    }

    /// Apply a function to all cells of the matrix.  
    /// Cells are provided as mutable references to the function,
    /// and can therefore be modified.
    ///
    /// # Examples
    /// ```
    /// use simple_matrix::Matrix;
    ///
    /// // Modify all cells with a function
    /// let mut mat: Matrix<usize> = Matrix::from_iter(3, 6, 0..);
    /// mat.apply_mut(|n| *n *= 2);
    ///
    /// assert_eq!(mat.get(0, 0).unwrap(), 0);
    /// assert_eq!(mat.get(0, 1).unwrap(), 2);
    /// assert_eq!(mat.get(0, 2).unwrap(), 4);
    /// ```
    pub fn apply_mut<F: FnMut(&mut T)>(&mut self, mut func: F) {
        self.data.iter_mut().for_each(|n| func(n));
    }
}

impl<T> Deref for Matrix<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl<T> Index<[usize; 2]> for Matrix<T> {
    type Output = T;

    fn index(&self, [row, col]: [usize; 2]) -> &Self::Output {
        &self.data[col + row * self.cols]
    }
}

impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (row, col): (usize, usize)) -> &Self::Output {
        &self.data[col + row * self.cols]
    }
}

impl<T> IndexMut<[usize; 2]> for Matrix<T> {
    fn index_mut(&mut self, [row, col]: [usize; 2]) -> &mut Self::Output {
        &mut self.data[col + row * self.cols]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (row, col): (usize, usize)) -> &mut Self::Output {
        &mut self.data[col + row * self.cols]
    }
}
