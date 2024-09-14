use std::{
    fmt::{Debug, Display},
    ops,
};

use crate::utils::float_equals;

// type MatrixDimensions<const M: usize, const N: usize>;

#[derive(Debug, Clone, Copy, PartialOrd)]
pub struct Matrix<const M: usize, const N: usize, T: Into<f64> + Clone> {
    matrix: [[T; N]; M],
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone> Matrix<M, N, T> {
    pub fn new(matrix: [[T; N]; M]) -> Self {
        Self::from(matrix)
    }

    pub fn multiply<const M2: usize, const N2: usize>(&self, _other: &Matrix<M2, N2, T>) -> Self {
        self.clone()
    }
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone> From<[[T; N]; M]> for Matrix<M, N, T> {
    fn from(value: [[T; N]; M]) -> Self {
        Self { matrix: value }
    }
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone> ops::Index<usize> for Matrix<M, N, T> {
    type Output = [T; N];

    fn index(&self, index: usize) -> &Self::Output {
        &self.matrix[index]
    }
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone> ops::IndexMut<usize>
    for Matrix<M, N, T>
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.matrix[index]
    }
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone> PartialEq for Matrix<M, N, T> {
    fn eq(&self, other: &Self) -> bool {
        for i in 0..M {
            for j in 0..N {
                let a = self[i][j].clone().into();
                let b = other[i][j].clone().into();
                if !float_equals(&a, &b) {
                    return false;
                }
            }
        }

        true
    }
}

impl<const M: usize, const N: usize, T: Into<f64> + Clone + Display> Display for Matrix<M, N, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_string_length = self
            .matrix
            .as_flattened()
            .iter()
            .map(|item: &T| format!("{:.3}", item).len())
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

            if i != N - 1 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::Matrix;

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

        let matrix_2x2 = Matrix::new([[-3, 5], [1, -2]]);

        assert_eq!(matrix_2x2[0][0], -3);
        assert_eq!(matrix_2x2[0][1], 5);
        assert_eq!(matrix_2x2[1][0], 1);
        assert_eq!(matrix_2x2[1][1], -2);

        let matrix_3x3 = Matrix::new([[-3, 5, 0], [1, -2, -7], [0, 1, 1]]);

        assert_eq!(matrix_3x3[0][0], -3);
        assert_eq!(matrix_3x3[1][1], -2);
        assert_eq!(matrix_3x3[2][2], 1);
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
}
