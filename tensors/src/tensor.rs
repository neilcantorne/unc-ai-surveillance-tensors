use std::{fmt::{Debug, Display, Write}, ops::{Index, IndexMut}};

#[derive(Clone, Copy)]
pub struct Tensor<T, const ROW: usize, const COLUMN: usize> {
    elements: [[T; COLUMN]; ROW]
}

impl<T, const ROW: usize, const COLUMN: usize> Tensor<T, ROW, COLUMN> 
    where T: Copy {

    pub fn filled_with(value: T) -> Self {
        Self {
            elements: [[value; COLUMN]; ROW]
        }
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

impl<T, const ROW: usize, const COLUMN: usize> Display for Tensor<T, ROW, COLUMN>
    where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut r = 0usize;
        let mut c = 0usize;

        f.write_char('[')?;

        loop {
            self.elements[r][c].fmt(f)?;
            
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
        Self { 
            elements: [[T::default(); COLUMN]; ROW]
        }
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Index<(usize, usize)> for Tensor<T, ROW, COLUMN> {
    type Output = T;

    #[inline]
    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.elements[index.0][index.1]
    }
}

impl<T, const ROW: usize, const COLUMN: usize> IndexMut<(usize, usize)> for Tensor<T, ROW, COLUMN> {
    #[inline]
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        &mut self.elements[index.0][index.1]
    }
}

impl<T, const COLUMN: usize> Index<usize> for Tensor<T, 1, COLUMN> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.elements[0][index]
    }
}

impl<T, const COLUMN: usize> IndexMut<usize> for Tensor<T, 1, COLUMN> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.elements[0][index]
    }
}

impl<T, const ROW: usize, const COLUMN: usize> From<[[T; COLUMN]; ROW]> for Tensor<T, ROW, COLUMN>
    where T: Copy {
    
    fn from(array: [[T; COLUMN]; ROW]) -> Self {
        Self { elements: array }
    }
}

// Operators
macro_rules! from_expression {
    ($num_type:ty, $row_count:ident, $column_count:ident, $r:ident, $c:ident, $expression:expr) => {
        {
            let mut $r = 0usize;
            let mut $c = 0usize;

            let mut result = Tensor::<T, $row_count, $column_count> {
                elements: unsafe { [[std::mem::MaybeUninit::uninit().assume_init(); $column_count]; $row_count] }
            };

            while $r < $row_count {
                while $c < $column_count {
                    result.elements[$r][$c] = {$expression};
                    $c += 1;
                }
                $c = 0;
                $r += 1;
            }

            result
        }
    };
}

impl<T, const ROW: usize, const COLUMN: usize> crate::ops::HadamardProduct for Tensor<T, ROW, COLUMN>
    where T: std::ops::Mul<T, Output = T> + Sized + Copy {
    type Output = Self;

    fn hadamard_product(self, operand: Self) -> Self::Output {
        from_expression!(T, ROW, COLUMN, r, c, {
            self.elements[r][c] * operand.elements[r][c]
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
            &self.tensor.elements[self.index_r][self.index_c],
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
            &self.tensor.elements[self.index_r][self.index_c],
            self.index_r,
            self.index_c
        ));

        self.index_r += 1;
        
        return next;
    }
}