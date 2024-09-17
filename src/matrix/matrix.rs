use anyhow::{Error, Result};
use std::{
    fmt::{Debug, Display},
    ops,
};

use crate::{spatial::Tuple, utils::float_equals};

#[derive(Debug, Clone, Copy)]
/// Representation of a Matrix of dimension `M x N`
/// containing [f64] values
///
/// `M` = Total # of rows
/// `N` = Total # of columns
pub struct Matrix<const M: usize, const N: usize> {
    matrix: [[f64; N]; M],
}

impl<const M: usize, const N: usize> Matrix<M, N> {
    /// Build a new default matrix with all 0.0 values:
    ///
    /// `M` = Total # of rows in the matrix
    /// `N` = Total # of columns in the matrix    
    fn new() -> Self {
        Self::default()
    }

    /// Get identity matrix of size S
    ///
    /// ```
    /// use raytracer::matrix::Matrix;
    ///
    /// let identity_2x2 = match Matrix::<2,2>::identity() {
    ///                         Ok(m) => m,
    ///                         Err(_e) => panic!("this should be a valid identity matrix construction")
    ///                   };
    ///
    /// // Multiplying with an identity matrix gives us back the original one
    /// let matrix = Matrix::from([[2.0, 4.0], [4.0, 9.0]]);
    /// match matrix.multiply(&identity_2x2) {
    ///     Ok(m) => assert_eq!(m, matrix.clone()),
    ///     Err(_e) => panic!("this should not happen, since it's a valid multiplication")
    /// };
    ///
    /// let identity_2x3_invalid = Matrix::<2,3>::identity();
    ///
    /// match identity_2x3_invalid {
    ///     Ok(_m) => panic!("this should be an invalid construction"),
    ///     Err(_e) => (),
    /// };
    /// ```
    pub fn identity() -> Result<Self> {
        if M != N {
            return Err(Error::msg(
                "# of rows should equal # of columns for an identity matrix",
            ));
        }

        let mut matrix = Matrix::<M, N>::new();

        let mut i = 0;
        let mut j = 0;

        while i < M && j < N {
            matrix[i][j] = 1.0;
            i += 1;
            j += 1;
        }

        Ok(matrix)
    }

    /// Performs the multiplication of two matrices.
    ///
    /// `result = left_matrix x this matrix`
    ///
    /// The requirement is that the number of columns of
    /// the `left_matrix` should be equal to the number
    /// of rows of this matrix.
    ///
    /// _Remember_, that the order matters when multiplying
    /// matrices, and this operation assumes you pass in
    /// the matrix that goes on the *left* side of the operation.
    /// i.e., assume this matrix is B, and the input is A,
    /// this performs the multiplication `A x B`.
    ///
    /// ```
    /// use raytracer::matrix::Matrix;
    ///
    /// let a = Matrix::from([
    ///     [1.0, 0.0],
    ///     [2.0, 5.0],
    /// ]);
    ///
    /// let b = Matrix::from([
    ///     [4.0, 3.0],
    ///     [2.0, 9.0],
    /// ]);
    ///
    /// let ab = b.multiply(&a);
    ///
    /// match ab {
    ///     Ok(m) => assert_eq!(m, Matrix::from([[4.0, 3.0],[18.0, 51.0]])),
    ///     Err(_) => panic!("this should not fail")
    /// };
    /// ```
    pub fn multiply<const M2: usize, const N2: usize>(
        &self,
        left_matrix: &Matrix<M2, N2>,
    ) -> Result<Matrix<M2, N>> {
        if M != N2 {
            Err(Error::msg("Invalid indices for multiplication"))
        } else {
            let mut matrix = Matrix::new();

            for i in 0..M2 {
                for j in 0..N {
                    for k in 0..M {
                        matrix[i][j] += left_matrix[i][k] * self[k][j];
                    }
                }
            }

            Ok(matrix)
        }
    }

    /// When you transpose a matrix, its rows turn into columns
    /// and its columns into rows
    ///
    /// ```
    /// use raytracer::matrix::Matrix;
    ///
    /// let m = Matrix::from([
    ///     [2.0, 4.0],
    ///     [7.0, 1.0],
    /// ]);
    ///
    /// assert_eq!(m.transpose(), Matrix::from([
    ///     [2.0, 7.0],
    ///     [4.0, 1.0],
    /// ]));
    /// ```
    pub fn transpose(&self) -> Matrix<N, M> {
        let mut transposed = Matrix::<N, M>::new();

        for i in 0..M {
            for j in 0..N {
                transposed[j][i] = self[i][j];
            }
        }

        transposed
    }
}

impl<const M: usize, const N: usize> From<[[f64; N]; M]> for Matrix<M, N> {
    fn from(value: [[f64; N]; M]) -> Self {
        Self { matrix: value }
    }
}

impl From<Tuple> for Matrix<4, 1> {
    fn from(value: Tuple) -> Self {
        Self::from([
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

impl<const M: usize, const N: usize> Default for Matrix<M, N> {
    fn default() -> Self {
        Self::from([[0.0; N]; M])
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;
    use crate::spatial::Tuple;
    use anyhow::Result;

    #[test]
    fn constructing_matrices_works() {
        // Case 1: Construct a 4x4 matrix and test getting values at indices
        let matrix_4x4 = Matrix::from([
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

        // Case 2: Construct a 2x2 matrix and test getting values at indices
        let matrix_2x2 = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        assert_eq!(matrix_2x2[0][0], -3.0);
        assert_eq!(matrix_2x2[0][1], 5.0);
        assert_eq!(matrix_2x2[1][0], 1.0);
        assert_eq!(matrix_2x2[1][1], -2.0);

        let matrix_3x3 = Matrix::from([[-3.0, 5.0, 0.0], [1.0, -2.0, -7.0], [0.0, 1.0, 1.0]]);

        assert_eq!(matrix_3x3[0][0], -3.0);
        assert_eq!(matrix_3x3[1][1], -2.0);
        assert_eq!(matrix_3x3[2][2], 1.0);
    }

    #[test]
    fn that_matrix_equality_works() {
        let m_a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let mut m_b = Matrix::from([
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
        let m_a = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let m_b = Matrix::from([
            [-2.0, 1.0, 2.0, 3.0],
            [3.0, 2.0, 1.0, -1.0],
            [4.0, 3.0, 6.0, 5.0],
            [1.0, 2.0, 7.0, 8.0],
        ]);

        let expected = Matrix::from([
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
        let matrix_4x4 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_2x2 = Matrix::from([[-3.0, 5.0], [1.0, -2.0]]);

        match matrix_2x2.multiply(&matrix_4x4) {
            Ok(_m) => panic!("We should have an invalid dimension error"),
            Err(_e) => (),
        }

        Ok(())
    }

    #[test]
    fn matrix_multiplication_4x4_times_4x3_works() -> Result<()> {
        let matrix_4x4 = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [5.0, 6.0, 7.0, 8.0],
            [9.0, 8.0, 7.0, 6.0],
            [5.0, 4.0, 3.0, 2.0],
        ]);

        let matrix_4x3 = Matrix::from([
            [-2.0, 1.0, 2.0],
            [3.0, 2.0, 1.0],
            [4.0, 3.0, 6.0],
            [1.0, 2.0, 7.0],
        ]);

        let expected = Matrix::from([
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
        let matrix = Matrix::from([
            [1.0, 2.0, 3.0, 4.0],
            [2.0, 4.0, 4.0, 2.0],
            [8.0, 6.0, 4.0, 1.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let tuple_matrix = Matrix::from(Tuple::from((1.0, 2.0, 3.0, 1.0)));

        let expected = Tuple::new_point(18, 24, 33);

        let actual: Tuple = tuple_matrix.multiply(&matrix)?.into();

        assert_eq!(actual, expected);

        Ok(())
    }

    #[test]
    fn construct_and_use_identity_matrix() -> Result<()> {
        let identity_2x2 = Matrix::<2, 2>::identity()?;
        assert_eq!(identity_2x2, Matrix::from([[1.0, 0.0], [0.0, 1.0]]));

        let identity_3x3 = Matrix::<3, 3>::identity()?;
        assert_eq!(
            identity_3x3,
            Matrix::from([[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]])
        );

        match Matrix::<2, 3>::identity() {
            Ok(_m) => panic!("2x3 is not a valid identity matrix size"),
            Err(_e) => (),
        };

        Ok(())
    }

    #[test]
    fn identity_matrix_multiplication_works_with_tuples() -> Result<()> {
        let tuple = Tuple::new_point(1.0, 2.0, 3.0);
        let tuple_matrix = Matrix::from(Tuple::new_point(1.0, 2.0, 3.0));
        let identity_4x4 = Matrix::<4, 4>::identity()?;

        assert_eq!(
            Tuple::from(tuple_matrix.multiply(&identity_4x4)?),
            tuple.clone()
        );
        Ok(())
    }

    #[test]
    fn transpose_operation_works() -> Result<()> {
        let m = Matrix::from([
            [0.0, 9.0, 3.0, 0.0],
            [9.0, 8.0, 0.0, 8.0],
            [1.0, 8.0, 5.0, 3.0],
            [0.0, 0.0, 5.0, 8.0],
        ]);

        let m_t = Matrix::from([
            [0.0, 9.0, 1.0, 0.0],
            [9.0, 8.0, 8.0, 0.0],
            [3.0, 0.0, 5.0, 5.0],
            [0.0, 8.0, 3.0, 8.0],
        ]);

        assert_eq!(m.transpose(), m_t);

        // The identity matrix transposed is the same as the original matrix
        let identity_4x4 = Matrix::<4, 4>::identity()?;
        assert_eq!(identity_4x4.transpose(), identity_4x4.clone());

        Ok(())
    }
}
