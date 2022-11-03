use std::fmt::{Debug, Display, Write};

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

impl<T, const ROW: usize, const COLUMN: usize> From<[[T; COLUMN]; ROW]> for Tensor<T, ROW, COLUMN>
    where T: Copy {
    
    fn from(array: [[T; COLUMN]; ROW]) -> Self {
        Self { elements: array }
    }
}