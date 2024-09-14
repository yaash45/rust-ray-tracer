use crate::utils::float_equals;
use anyhow::{Error, Result};
use core::fmt::Display;

#[derive(Debug, Clone)]
/// Representation of a Matrix of dimension `m x n`
/// containing [f64] values
pub struct MatrixBuggy {
    m: usize,
    n: usize,
    inner: Vec<f64>,
}

impl MatrixBuggy {
    /// This helper function is meant to extract the input vectors into
    /// a 1D-vector representing the values of the matrix.
    ///
    /// Before extraction, the input vectors are validated.
    /// to ensure that we have the correct number of values
    /// to construct a matrix of size `m x n`
    fn validate_and_get_inner_matrix_representation(
        m: usize,
        n: usize,
        vals: Vec<Vec<f64>>,
    ) -> Result<Vec<f64>> {
        if !vals.is_empty()
            && vals.len() == m
            && vals
                .clone()
                .into_iter()
                .filter(|item: &Vec<f64>| item.len() != n)
                .count()
                == 0
        {
            let mut inner = vec![0_f64; m * n];

            let mut k = 0;

            (0..m).for_each(|i| {
                for j in 0..n {
                    inner[k] = vals[i][j];
                    k += 1;
                }
            });

            println!("{:?}", inner);

            Ok(inner)
        } else {
            Err(Error::msg("blah"))
        }
    }

    /// Build a new matrix with three inputs:
    ///
    /// `m`: Total # of rows in the matrix
    ///
    /// `n`: Total # of columns in the matrix
    ///
    /// `vals`: a vector of vectors representing the matrix values
    ///
    /// The length of `vals` should be equal to `m`, and each individual
    /// vector in `vals` has to be of length `n`. If this constraint
    /// is not met, then an error is returned in the Result.
    ///
    ///
    /// ```
    /// use raytracer::matrix::Matrix;
    ///
    /// let m: usize = 2;
    /// let n: usize = 2;
    /// let vals: Vec<Vec<f64>> = [
    ///     vec![1.0, 2.0],
    ///     vec![-3.0, 4.0],
    /// ].to_vec();
    ///
    /// let matrix = Matrix::new(m, n, vals);
    ///
    /// match matrix {
    ///     Ok(m) => println!("{}", m),
    ///     Err(_e) => panic!("Something went wrong during matrix creation")
    /// }
    ///
    /// ```
    pub fn new(m: usize, n: usize, vals: Vec<Vec<f64>>) -> Result<Self> {
        let inner = Self::validate_and_get_inner_matrix_representation(m, n, vals)?;
        Ok(Self { m, n, inner })
    }

    /// Helper function to map an input row and column index
    /// to the index of our 1D array representing the matrix
    fn map_index(&self, row: usize, col: usize) -> usize {
        dbg!((dbg!(self.m) * dbg!(row)) + dbg!(col))
    }

    /// Validates that the index `[row, col]` is a valid index
    /// for the current matrix.
    ///
    /// i.e. `row` < `self.m` and `col` < `self.n`
    fn validate_index(&self, row: usize, col: usize) -> Result<()> {
        if row < self.m && col < self.n {
            Ok(())
        } else {
            Err(Error::msg(format!(
                "Invalid index [{},{}] for matrix of size {} x {}",
                row, col, self.m, self.n,
            )))
        }
    }

    /// Gets the value at a specific matrix index, returning
    /// an error if the requested indices fall outside the
    /// range of the matrix dimensions
    pub fn get(&self, row: usize, col: usize) -> Result<f64> {
        self.validate_index(row, col)?;
        Ok(self.inner[self.map_index(row, col)])
    }

    fn write_at(&mut self, row: usize, col: usize, value: f64) -> Result<()> {
        self.validate_index(row, col)?;
        let clone = self.clone();
        self.inner[clone.map_index(row, col)] = value;
        Ok(())
    }

    fn multiplication_dimensions_are_valid(&self, other: &MatrixBuggy) -> bool {
        other.n == self.m
    }

    pub fn multiply_matrix(&self, other: &MatrixBuggy) -> Result<MatrixBuggy> {
        if !self.multiplication_dimensions_are_valid(other) {
            let error_message = format!(
                "Attempted invalid matrix multiplication. \
                The number of columns of the input matrix should \
                match the number of rows of this matrix. \
                # cols of input: {}, # rows of current: {}
                ",
                other.n, self.m
            );
            Err(Error::msg(error_message))
        } else {
            // TODO: See if this can be cleaned up
            let mut matrix = Self {
                m: other.m,
                n: self.n,
                inner: vec![0_f64; other.m * self.n],
            };

            for i in 0..self.m {
                for j in 0..other.n {
                    let mut cur_sum = 0_f64;
                    for k in 0..self.m {
                        let other_val = other.get(j, k)?;
                        let self_val = self.get(k, i)?;
                        println!(
                            "other = [j:{}, k:{}] = {}, self = [k:{}, i:{}] = {}",
                            j, k, other_val, k, i, self_val
                        );

                        cur_sum += other_val * self_val;
                    }
                    matrix.write_at(i, j, cur_sum)?;
                }
            }

            Ok(matrix)
        }
    }
}

impl PartialEq for MatrixBuggy {
    fn eq(&self, other: &Self) -> bool {
        if self.m == other.m && self.n == other.n {
            for i in 0..self.m {
                for j in 0..self.n {
                    let this_val = self.get(i, j).expect("index out of bounds");
                    let other_val = other.get(i, j).expect("index out of bounds");

                    if !float_equals(&this_val, &other_val) {
                        return false;
                    }
                }
            }
            true
        } else {
            false
        }
    }
}

impl Display for MatrixBuggy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Ensure that the precision used to determine this max string length
        // matches the one used below to assign val_as_string
        let max_string_length = self
            .inner
            .clone()
            .into_iter()
            .map(|item: f64| (format!("{:.3}", item)).len())
            .reduce(usize::max)
            .unwrap();

        for i in 0..self.m {
            write!(f, "| ")?;
            for j in 0..self.n {
                let val = self.get(i, j).expect("invalid index accessed");
                let val_as_string = format!("{:.3}", val);

                for _c in 0..(max_string_length - val_as_string.len()) {
                    write!(f, " ")?;
                }

                write!(f, "{:.3} | ", val)?;
            }
            if i != self.n - 1 {
                writeln!(f)?;
            }
        }
        writeln!(f)
    }
}

#[cfg(test)]
mod tests {
    use super::MatrixBuggy;
    use anyhow::Result;

    #[test]
    fn constructing_valid_matrices() -> Result<()> {
        // Case 1: Construct a 4x4 matrix and test getting values at indices
        let four_by_four_vals = [
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5],
        ]
        .to_vec();
        let four_by_four_matrix = MatrixBuggy::new(4, 4, four_by_four_vals)?;

        assert_eq!(four_by_four_matrix.get(0, 0)?, 1.0);
        assert_eq!(four_by_four_matrix.get(0, 3)?, 4.0);
        assert_eq!(four_by_four_matrix.get(1, 0)?, 5.5);
        assert_eq!(four_by_four_matrix.get(1, 2)?, 7.5);
        assert_eq!(four_by_four_matrix.get(2, 2)?, 11.0);
        assert_eq!(four_by_four_matrix.get(3, 0)?, 13.5);
        assert_eq!(four_by_four_matrix.get(3, 2)?, 15.5);

        // Case 2: Construct a 2x2 matrix and test getting values at indices
        let two_by_two_vals = [vec![-3.0, 5.0], vec![1.0, -2.0]].to_vec();
        let two_by_two_matrix = MatrixBuggy::new(2, 2, two_by_two_vals)?;

        assert_eq!(two_by_two_matrix.get(0, 0)?, -3.0);
        assert_eq!(two_by_two_matrix.get(0, 1)?, 5.0);
        assert_eq!(two_by_two_matrix.get(1, 0)?, 1.0);
        assert_eq!(two_by_two_matrix.get(1, 1)?, -2.0);

        Ok(())
    }

    #[test]
    fn matrix_invalid_values_returns_errors() -> Result<()> {
        // Case 1: Accessing invalid index on the matrix
        let two_by_two_vals = [vec![-3.0, 5.0], vec![1.0, -2.0]].to_vec();
        let two_by_two_matrix = MatrixBuggy::new(2, 2, two_by_two_vals)?;
        match two_by_two_matrix.get(1, 4) {
            Ok(_) => panic!("Invalid indices resulted in Ok return value"),
            Err(_e) => (),
        }
        match two_by_two_matrix.get(4, 0) {
            Ok(_) => panic!("Invalid indices resulted in Ok return value"),
            Err(_e) => (),
        }

        // Case 2: Trying to create matrix with invalid input values (inconsistent row sizes)
        let invalid_vals = [vec![-3.0, 5.0], vec![1.0, -2.0, 1.0]].to_vec();
        match MatrixBuggy::new(2, 2, invalid_vals) {
            Ok(_) => panic!("Matrix creation succeeded with inconsistent row value count"),
            Err(_e) => (),
        }

        // Case 3: The values are a valid matrix, but their dimensions do not match the
        // input dimensions m and n
        let vals = [vec![-3.0, 5.0], vec![1.0, -2.0]].to_vec();
        match MatrixBuggy::new(3, 3, vals) {
            Ok(_) => {
                panic!(
                    "Matrix creation succeeded with even though input\
                    values had different dimensions than the input m and/or n"
                )
            }
            Err(_e) => (),
        }

        Ok(())
    }

    #[test]
    fn matrix_equality_works() -> Result<()> {
        let matrix_a = MatrixBuggy::new(
            4,
            4,
            [
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
            .to_vec(),
        )?;
        let matrix_b = matrix_a.clone();
        let matrix_c = MatrixBuggy::new(2, 2, [vec![1.0, 2.0], vec![2.0, 4.0]].to_vec())?;
        let matrix_d = MatrixBuggy::new(
            4,
            4,
            [
                vec![2.0, 3.0, 4.0, 5.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
            .to_vec(),
        )?;

        // Case 1: matrix A and B are clones of each other, so they should be equal
        assert_eq!(matrix_a, matrix_b);

        // Case 2: matrix A and C cannot be equal, since their dimensions do not match
        assert_ne!(matrix_a, matrix_c);

        // Case 3: matrix A and D cannot be equal, since even with the same dimensions,
        // they have different values
        assert_ne!(matrix_a, matrix_d);

        Ok(())
    }

    #[test]
    fn matrix_multiplication_with_invalid_dimensions_throws_error() -> Result<()> {
        let matrix_a = MatrixBuggy::new(2, 2, [vec![1.0, 2.0], vec![2.0, 4.0]].to_vec())?;
        let matrix_b = MatrixBuggy::new(
            4,
            4,
            [
                vec![2.0, 3.0, 4.0, 5.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
            .to_vec(),
        )?;

        match matrix_a.multiply_matrix(&matrix_b) {
            Ok(_m) => panic!("We should have gotten an error for invalid dimensions"),
            Err(_e) => (),
        }

        Ok(())
    }

    #[test]
    fn matrix_multiplication_4x4_times_4x4_works() -> Result<()> {
        let matrix_a = MatrixBuggy::new(
            4,
            4,
            [
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
            .to_vec(),
        )?;

        let matrix_b = MatrixBuggy::new(
            4,
            4,
            [
                vec![-2.0, 1.0, 2.0, 3.0],
                vec![3.0, 2.0, 1.0, -1.0],
                vec![4.0, 3.0, 6.0, 5.0],
                vec![1.0, 2.0, 7.0, 8.0],
            ]
            .to_vec(),
        )?;

        let expected = MatrixBuggy::new(
            4,
            4,
            [
                vec![20.0, 22.0, 50.0, 48.0],
                vec![44.0, 54.0, 114.0, 108.0],
                vec![40.0, 58.0, 110.0, 102.0],
                vec![16.0, 26.0, 46.0, 42.0],
            ]
            .to_vec(),
        )?;

        let res = matrix_b.multiply_matrix(&matrix_a)?;

        assert_eq!(res, expected);

        Ok(())
    }

    #[test]
    fn matrix_multiplication_4x4_times_4x3_works() -> Result<()> {
        let matrix_a = MatrixBuggy::new(
            4,
            4,
            [
                vec![1.0, 2.0, 3.0, 4.0],
                vec![5.0, 6.0, 7.0, 8.0],
                vec![9.0, 8.0, 7.0, 6.0],
                vec![5.0, 4.0, 3.0, 2.0],
            ]
            .to_vec(),
        )?;

        println!("{}", matrix_a);

        let matrix_b = MatrixBuggy::new(
            4,
            3,
            [
                vec![-2.0, 1.0, 2.0],
                vec![3.0, 2.0, 1.0],
                vec![4.0, 3.0, 6.0],
                vec![1.0, 2.0, 7.0],
            ]
            .to_vec(),
        )?;

        println!("{}", matrix_b);

        let expected = MatrixBuggy::new(
            4,
            3,
            [
                vec![20.0, 22.0, 50.0],
                vec![44.0, 54.0, 114.0],
                vec![40.0, 58.0, 110.0],
                vec![16.0, 26.0, 46.0],
            ]
            .to_vec(),
        )?;

        let res = matrix_b.multiply_matrix(&matrix_a)?;

        assert_eq!(res, expected);
        Ok(())
    }
}
