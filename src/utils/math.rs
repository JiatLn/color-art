pub type Matrix = Vec<Vec<f64>>;

/// <https://www.w3.org/TR/css-color-4/multiply-matrices.js>
///
/// a is m x n. b is n x p. product is m x p.
///
/// a:
/// ```bash
/// | 1, 2, 3 |
/// | 4, 5, 6 |
/// | 7, 8, 9 |
/// ```
/// b:
/// ```bash
/// | 1 |
/// | 2 |
/// | 3 |
/// ```
/// product:
/// ```bash
/// | 14 |
/// | 32 |
/// | 50 |
/// ```
pub fn multiply_matrices(a: Matrix, b: Matrix) -> Matrix {
    // TODO: check if a and b are valid matrices
    (0..a.len())
        .map(|i| {
            (0..b[i].len()).fold(Vec::with_capacity(b[i].len()), |mut acc, j| {
                acc.push((0..a[i].len()).map(|k| a[i][k] * b[k][j]).fold(0.0, |acc, x| acc + x));
                acc
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply_matrices() {
        let a = vec![vec![1.0, 2.0, 3.0];3];
        let b = vec![vec![1.0], vec![2.0], vec![3.0]];
        let c = multiply_matrices(a, b);
        assert_eq!(c, vec![vec![14.0], vec![14.0], vec![14.0]]);
    }
}
