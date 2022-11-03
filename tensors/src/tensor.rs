#[derive(Clone, Copy)]
pub struct Tensor<T, const ROW: usize, const COLUMN: usize> {
    elements: [[T; ROW]; COLUMN]
}

impl<T, const ROW: usize, const COLUMN: usize> Tensor<T, ROW, COLUMN> 
    where T: Copy {

    pub fn filled_with(value: T) -> Self {
        Self {
            elements: [[value; ROW]; COLUMN]
        }
    }
}

impl<T, const ROW: usize, const COLUMN: usize> Default for Tensor<T, ROW, COLUMN>
    where T: Default + Copy + Sized {
    fn default() -> Self {
        Self { 
            elements: [[T::default(); ROW]; COLUMN]
        }
    }
}

impl<T, const ROW: usize, const COLUMN: usize> From<[[T; ROW]; COLUMN]> for Tensor<T, ROW, COLUMN>
    where T: Copy {
    
    fn from(array: [[T; ROW]; COLUMN]) -> Self {
        Self { elements: array }
    }
}