use super::Matrix;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::Neg;
use std::ops::Sub;
use std::ops::SubAssign;

macro_rules! impl_op {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait<Output = T>> $trait for Matrix<T> {
            type Output = Matrix<T>;

            fn $func(self, rhs: Self) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .into_iter()
                        .zip(rhs.into_iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }

        impl<'a: 'b, 'b, T> $trait for &'a Matrix<T>
        where
            &'a T: $trait<&'b T, Output = T>,
        {
            type Output = Matrix<T>;

            fn $func(self, rhs: &'b Matrix<T>) -> Self::Output {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                Matrix {
                    rows: self.rows,
                    cols: self.cols,
                    data: self
                        .iter()
                        .zip(rhs.iter())
                        .map(|(a, b)| a $op b)
                        .collect(),
                }
            }
        }
    }
}

macro_rules! impl_op_assign {
    ($trait:ident, $func:ident, $op:tt) => {
        impl<T: $trait> $trait for Matrix<T> {
            fn $func(&mut self, rhs: Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.into_iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }

        impl<'a, T: $trait<&'a T>> $trait<&'a Matrix<T>> for Matrix<T> {
            fn $func(&mut self, rhs: &'a Self) {
                assert!(self.rows == rhs.rows);
                assert!(self.cols == rhs.cols);

                self.data.iter_mut()
                    .zip(rhs.iter())
                    .for_each(|(a, b)| *a $op b);
            }
        }
    }
}

// Macro-ed impl

impl_op!(Add, add, +);
impl_op!(Sub, sub, -);
impl_op_assign!(AddAssign, add_assign, +=);
impl_op_assign!(SubAssign, sub_assign, -=);

// Neg implementation

impl<T: Neg<Output = T>> Neg for Matrix<T> {
    type Output = Matrix<T>;

    fn neg(self) -> Self::Output {
        Matrix {
            rows: self.rows,
            cols: self.cols,
            data: self.into_iter().map(|a| -a).collect(),
        }
    }
}

// Mul implementation

impl<T> Mul<Matrix<T>> for Matrix<T>
where
    T: Mul<Output = T> + Add<Output = T> + Copy,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let row = self.get_row(row).unwrap();
                        let col = rhs.get_col(col).unwrap();

                        let mut iter = row.zip(col);
                        let (a, b) = iter.next().unwrap();
                        let mut acc = *a * *b;

                        for (a, b) in iter {
                            acc = acc + *a * *b;
                        }

                        data.push(acc);
                    }
                }

                data
            },
        }
    }
}

impl<'a, 'b, T: Add<Output = T>> Mul<&'b Matrix<T>> for &'a Matrix<T>
where
    &'a T: Mul<&'b T, Output = T>,
{
    type Output = Matrix<T>;

    fn mul(self, rhs: &'b Matrix<T>) -> Self::Output {
        assert!(self.cols == rhs.rows);

        Matrix {
            rows: self.rows,
            cols: rhs.cols,
            data: {
                let mut data = Vec::with_capacity(self.rows * rhs.cols);

                for row in 0..self.rows {
                    for col in 0..rhs.cols {
                        let row = self.get_row(row).unwrap();
                        let col = rhs.get_col(col).unwrap();

                        let mut iter = row.zip(col);
                        let (a, b) = iter.next().unwrap();
                        let mut acc = a * b;

                        for (a, b) in iter {
                            acc = acc + a * b;
                        }

                        data.push(acc);
                    }
                }

                data
            },
        }
    }
}
