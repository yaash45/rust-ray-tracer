use anyhow::{Error, Result};
use std::{
    fmt::{Debug, Display},
    ops,
};

use crate::{spatial::Tuple, utils::float_equals};

#[derive(Debug, Clone, Copy)]
pub struct Matrix<const M: usize, const N: usize> {
    matrix: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    pub fn new(matrix: [[f64; N]; M]) -> Self {
        Self::from(matrix)
    }

    pub fn multiply<const M2: usize, const N2: usize>(
        &self,
        other: &Matrix<M2, N2>,
    ) -> Result<Matrix<M2, N>> {
        if M != N2 {
            Err(Error::msg("Invalid indices for multiplication"))
        } else {
            let mut matrix = Matrix::new([[0.0; N]; M2]);

            for i in 0..M2 {
                for j in 0..N {
                    for k in 0..M {
                        matrix[i][j] += other[i][k] * self[k][j];
                    }
                }
            }

            Ok(matrix)
        }
    }
}

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for Matrix<M, N> {
    fn from(value: [[f64; N]; M]) -> Self {
        Self { matrix: value }
    }
}

impl From<Tuple> for Matrix<4, 1> {
    fn from(value: Tuple) -> Self {
        Self::new([
            [value.get_x()],
            [value.get_y()],
            [value.get_z()],
            [value.get_w()],
        ])
    }
}

impl From<Matrix<4, 1>> for Tuple {
    fn from(value: Matrix<4, 1>) -> Self {
        Self::from((value[0][0], value[1][0], value[2][0], value[3][0]))
    }
}

impl<const M: usize, const N: usize> ops::Index<usize> for Matrix<M, N> {
    type Output = [f64; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<const M: usize, const N: usize> ops::IndexMut<usize> for Matrix<M, N> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<const M: usize, const N: usize> PartialEq for Matrix<M, N> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                let a = self[i][j];
                let b = other[i][j];
                if !float_equals(&a, &b) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const M: usize, const N: usize> Display for Matrix<M, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_string_length = self
            .matrix
            .as_flattened()
            .iter()
            .map(|item: &f64| format!("{:.3}", item).len())
            .reduce(usize::max)
            .unwrap();

        for i in 0..M {
            write!(f, "| ")?;
            for j in 0..N {
                let val = &self[i][j];
                let val_as_string = format!("{:.3}", val);

                (0..(max_string_length - val_as_string.len()))
                    .for_each(|_c| write!(f, " ").expect("Could not add padding in matrix cells"));

                write!(f, "{:.3} | ", val)?;
            }
            writeln!(f)?;
        }
        write!(f, "")
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn constructing_matrices_works() {
        let matrix_4x4 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.5, 6.5, 7.5, 8.5],
            [9.0, 10.0, 11.0, 12.0],
            [13.5, 14.5, 15.5, 16.5],
        ]);

        assert_eq!(matrix_4x4[0][0], 1.0);
        assert_eq!(matrix_4x4[0][3], 4.0);
        assert_eq!(matrix_4x4[1][0], 5.5);
        assert_eq!(matrix_4x4[1][2], 7.5);
        assert_eq!(matrix_4x4[3][0], 13.5);
        assert_eq!(matrix_4x4[3][2], 15.5);

        let matrix_2x2 = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(matrix_2x2[0][0], -3.0);
        assert_eq!(matrix_2x2[0][1], 5.0);
        assert_eq!(matrix_2x2[1][0], 1.0);
        assert_eq!(matrix_2x2[1][1], -2.0);

        let matrix_3x3 = Matrix::new([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(matrix_3x3[0][0], -3.0);
        assert_eq!(matrix_3x3[1][1], -2.0);
        assert_eq!(matrix_3x3[2][2], 1.0);
    }

    #[test]
    fn that_matrix_equality_works() {
        let m_a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let mut m_b = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        assert_eq!(m_a, m_b);

        m_b[0][0] *= 0.2;

        assert_ne!(m_a, m_b);
    }

    #[test]
    fn matrix_multiplication_4x4_times_4x4_works() -> Result<()> {
        let m_a = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m_b = Matrix::new([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::new([
            [20.0, 22.0, 50.0, 48.0],
            [44.0, 54.0, 114.0, 108.0],
            [40.0, 58.0, 110.0, 102.0],
            [16.0, 26.0, 46.0, 42.0],
        ]);

        let actual = m_b.multiply(&m_a)?;

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn matrix_multiplication_with_invalid_dimensions_fails() -> Result<()> {
        let matrix_4x4 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_2x2 = Matrix::new([[-3.0, 5.0], [1.0, -2.0]]);

        match matrix_2x2.multiply(&matrix_4x4) {
            Ok(_m) => panic!("We should have an invalid dimension error"),
            Err(_e) => (),
        }

        Ok(())
    }

    #[test]
    fn matrix_multiplication_4x4_times_4x3_works() -> Result<()> {
        let matrix_4x4 = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_4x3 = Matrix::new([
            [-2.0, 1.0, 2.0],
            [3.0, 2.0, 1.0],
            [4.0, 3.0, 6.0],
            [1.0, 2.0, 7.0],
        ]);

        let expected = Matrix::new([
            [20.0, 22.0, 50.0],
            [44.0, 54.0, 114.0],
            [40.0, 58.0, 110.0],
            [16.0, 26.0, 46.0],
        ]);

        let actual = matrix_4x3.multiply(&matrix_4x4)?;

        assert_eq!(expected, actual);

        Ok(())
    }

    #[test]
    fn matrix_and_tuple_multiplication_works() -> Result<()> {
        let matrix = Matrix::new([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple_matrix = Matrix::from(Tuple::from((1.0, 2.0, 3.0, 1.0)));

        let actual: Tuple = tuple_matrix.multiply(&matrix)?.into();

        println!("{:?}", actual);

        Ok(())
    }
}
