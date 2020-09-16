use std::mem::{MaybeUninit};
use std::ops::{self, Index, IndexMut};
use std::fmt;


#[derive(Debug, PartialEq, Clone)]
pub struct Matrix<T> {
    mat: Vec<T>,
    n_rows: usize,
    n_cols: usize
}


impl<T> Matrix<T>
where
    T: Copy
{

    pub fn from_vec(vec: Vec<T>, n_rows: usize, n_cols: usize) -> Self {
        Self {
            mat: vec,
            n_rows,
            n_cols
        }
    }

    pub fn new(fill: T, n_rows: usize, n_cols: usize) -> Self {
        Self {
            mat: vec![fill; n_rows * n_cols],
            n_rows,
            n_cols
        }
    }
}

impl<T> Matrix<T>
where
    T: Copy + Default + ops::AddAssign + ops::Mul<Output = T>
{
    pub fn mul(&self, other: &Matrix<T>) -> Self {
        assert_eq!(self.n_cols, other.n_rows);

        //let mut mat = Matrix::<MaybeUninit<T>>::new(MaybeUninit::uninit(), self.n_rows, self.n_cols);
        let mut mat = Matrix::<T>::new(*self.mat.first().unwrap(), self.n_rows, other.n_cols);

        for irow in 0..self.n_rows {
            for icol in 0..other.n_cols {

                let mut sum: T = T::default();
                for k in 0..self.n_cols {
                    sum += self[(irow, k)] * other[(k, icol)];
                }
                mat[(irow, icol)] = sum;
            }
        }

        mat
    }

    pub fn pow(&self, deg: usize) -> Self {
        let mut res = self.clone();
        for _ in 0..deg-1 {
            res = res.mul(self);
        }
        res
    }
}

impl<'a, T> ops::AddAssign<&'a Self> for Matrix<T>
where
    T: Copy + ops::AddAssign
{
    fn add_assign(&mut self, other: &Self) {
        assert_eq!(self.n_rows, other.n_rows);
        assert_eq!(self.n_cols, other.n_cols);

        for (l, r) in self.mat.iter_mut().zip(other.mat.iter()) {
            *l += *r;
        }
    }
}


impl<T> Index<(usize, usize)> for Matrix<T> {
    type Output = T;

    fn index(&self, (i, j) : (usize, usize)) -> &Self::Output {
        &self.mat[i * self.n_cols + j]
    }
}

impl<T> IndexMut<(usize, usize)> for Matrix<T> {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        &mut self.mat[i * self.n_cols + j]
    }
}

impl<T> fmt::Display for Matrix<T>
where
    T: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.n_rows {
            for j in 0..self.n_cols {
                write!(f, "{:>6} ", self.index((i, j)))?
            }
            writeln!(f)?
        }
        Ok(())
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_identity_1() {
        let a = Matrix::<usize>::from_vec(
            vec![
                1,0,0,
                0,1,0,
                0,0,1,
            ], 3, 3);

        let b = Matrix::<usize>::from_vec(
            vec![
                1,0,0,
                0,1,0,
                0,0,1,
            ], 3, 3);

        let res = a.mul(&b);
        assert_eq!(res, a);
    }

    #[test]
    fn test_mul_identity_2() {
        let a = Matrix::<usize>::from_vec(
            vec![
                1,2,3,
                4,5,6,
                7,8,9,
            ], 3, 3);

        let b = Matrix::<usize>::from_vec(
            vec![
                1,0,0,
                0,1,0,
                0,0,1,
            ], 3, 3);

        let res = a.mul(&b);
        assert_eq!(res, a);
    }

    #[test]
    fn test_mul_same() {
        let a = Matrix::<usize>::from_vec(
            vec![
                1,2,3,
                4,5,6,
                7,8,9,
            ], 3, 3);

        let res = a.mul(&a);
        assert_eq!(res, Matrix::<usize>::from_vec(
            vec![
                30, 36, 42,
                66, 81, 96,
                102, 126, 150,
            ],
            3, 3
        ));
    }
}