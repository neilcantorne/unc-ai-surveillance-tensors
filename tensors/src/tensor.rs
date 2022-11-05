use std::{fmt::{Debug, Display, Write}, ops::{Index, IndexMut}};

#[derive(Clone, Copy)]
pub struct Tensor<T, const ROW: usize, const COLUMN: usize>([[T; COLUMN]; ROW]);

impl<T, const ROW: usize, const COLUMN: usize> Tensor<T, ROW, COLUMN> 
    where T: Copy {

    pub fn filled_with(value: T) -> Self {
        Self([[value; COLUMN]; ROW])
    }

    pub fn hr_iter<'a>(&'a self) -> crate::TensorHorizontalIter<'a, T, ROW, COLUMN> {
        crate::TensorHorizontalIter::<'a, T, ROW, COLUMN> {
            tensor: self,
            index_r: 0,
            index_c: 0,
        }
    }

    pub fn vr_iter<'a>(&'a self) -> crate::TensorVerticalIter<'a, T, ROW, COLUMN> {
        crate::TensorVerticalIter::<'a, T, ROW, COLUMN> {
            tensor: self,
            index_r: 0,
            index_c: 0,
        }
    }
}

impl<T> Tensor<T, 1, 2> where T: Copy {
    #[inline]
    pub fn x(&self) -> T { self.0[0][1] }
    #[inline]
    pub fn y(&self) -> T { self.0[0][2] }
}

impl<T> Tensor<T, 1, 3> where T: Copy {
    #[inline]
    pub fn x(&self) -> T { self.0[0][1] }
    #[inline]
    pub fn y(&self) -> T { self.0[0][2] }
    #[inline]
    pub fn z(&self) -> T { self.0[0][3] }
}

impl<T, const ROW: usize, const COLUMN: usize> Display for Tensor<T, ROW, COLUMN>
    where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = 0usize;
        let mut c = 0usize;

        f.write_char('[')?;

        loop {
            self.0[r][c].fmt(f)?;
            
            c += 1;

            if c < COLUMN {
                f.write_str(", ")?;
            }
            else if r < ROW - 1 {
                r += 1; c = 0;
                f.write_str(" | ")?;
            }
            else { return f.write_char(']'); }
        }
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Debug for Tensor<T, ROW, COLUMN> 
    where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(&self, f)
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Default for Tensor<T, ROW, COLUMN>
    where T: Default + Copy + Sized {
    fn default() -> Self {
        Self([[T::default(); COLUMN]; ROW])
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Index<(usize, usize)> for Tensor<T, ROW, COLUMN> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.0[index.0][index.1]
    }
}

impl<T, const ROW: usize, const COLUMN: usize> IndexMut<(usize, usize)> for Tensor<T, ROW, COLUMN> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.0[index.0][index.1]
    }
}

impl<T, const COLUMN: usize> Index<usize> for Tensor<T, 1, COLUMN> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[0][index]
    }
}

impl<T, const COLUMN: usize> IndexMut<usize> for Tensor<T, 1, COLUMN> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[0][index]
    }
}

impl<T, const ROW: usize, const COLUMN: usize> From<[[T; COLUMN]; ROW]> for Tensor<T, ROW, COLUMN>
    where T: Copy {
    
    fn from(array: [[T; COLUMN]; ROW]) -> Self {
        Self (array)
    }
}

// Operators
macro_rules! from_expression {
    ($num_type:ty, $row_count:ident, $column_count:ident, $r:ident, $c:ident, $expression:expr) => {
        {
            let mut $r = 0usize;
            let mut $c = 0usize;

            let mut result = Tensor::<T, $row_count, $column_count> (
                unsafe { [[std::mem::MaybeUninit::uninit().assume_init(); $column_count]; $row_count] }
            );

            while $r < $row_count {
                while $c < $column_count {
                    result.0[$r][$c] = {$expression};
                    $c += 1;
                }
                $c = 0;
                $r += 1;
            }

            result
        }
    };
}

impl<T, const ROW: usize, const COLUMN: usize> crate::ops::DotProduct for Tensor<T, ROW, COLUMN>
    where T: std::ops::Mul<T, Output = T> +
        std::ops::AddAssign<T> +
        Default + Sized + Copy {
    type Output = T;

    fn dot(self, operand: Self) -> Self::Output {
        let mut sum = T::default();

        let mut r = 0usize;
        let mut c = 0usize;

        while r < ROW {
            while c < COLUMN {
                sum += self.0[r][c] * operand.0[r][c];
                c += 1;
            }
            c = 0;
            r += 1;
        }

        sum
    }
}

impl<T> crate::ops::CrossProduct for Tensor<T, 1, 3>
    where T: std::ops::Mul<T, Output = T> +
    std::ops::Sub<T, Output = T> + Sized + Copy {
    type Output = Self;

    fn cross(self, operand: Self) -> Self::Output {
        Self(
            [[
                self.y() * operand.z() - operand.z() * self.y(),
                self.z() * operand.x() - operand.x() * self.z(),
                self.x() * operand.y() - operand.y() * self.x() 
            ]]
        )
    }
}

impl<T, const ROW: usize, const COLUMN: usize> crate::ops::HadamardProduct for Tensor<T, ROW, COLUMN>
    where T: std::ops::Mul<T, Output = T> + Sized + Copy {
    type Output = Self;

    fn hadamard_product(self, operand: Self) -> Self::Output {
        from_expression!(T, ROW, COLUMN, r, c, {
            self.0[r][c] * operand.0[r][c]
        })
    }
}

// Elements Iterator
pub struct HorizontalIter<'a, T, const ROW: usize, const COLUMN: usize> {
    pub(self) tensor: &'a Tensor<T, ROW, COLUMN>,
    pub(self) index_r: usize,
    pub(self) index_c: usize,
}

impl<'a, T, const ROW: usize, const COLUMN: usize> Iterator for HorizontalIter<'a, T, ROW, COLUMN> {
    type Item = (&'a T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

        if self.index_c >= COLUMN {
            self.index_c = 0;
            self.index_r += 1;
            
            if self.index_r >= ROW { return None; }
        }

        let next =  Some((
            &self.tensor.0[self.index_r][self.index_c],
            self.index_r,
            self.index_c
        ));

        self.index_c += 1;
        
        return next;
    }
}

pub struct VerticalIter<'a, T, const ROW: usize, const COLUMN: usize> {
    pub(self) tensor: &'a Tensor<T, ROW, COLUMN>,
    pub(self) index_r: usize,
    pub(self) index_c: usize,
}

impl<'a, T, const ROW: usize, const COLUMN: usize> Iterator for VerticalIter<'a, T, ROW, COLUMN> {
    type Item = (&'a T, usize, usize);

    fn next(&mut self) -> Option<Self::Item> {

        if self.index_r >= ROW {
            self.index_r = 0;
            self.index_c += 1;
            
            if self.index_c >= COLUMN { return None; }
        }

        let next =  Some((
            &self.tensor.0[self.index_r][self.index_c],
            self.index_r,
            self.index_c
        ));

        self.index_r += 1;
        
        return next;
    }
}