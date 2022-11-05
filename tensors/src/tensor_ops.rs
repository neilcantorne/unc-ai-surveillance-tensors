pub trait HadamardProduct {
    type Output: Sized;

    fn hadamard_product(self, operand: Self) -> Self::Output;
}

pub trait CrossProduct {
    type Output: Sized;

    fn cross(self, operand: Self) -> Self::Output;
}

pub trait DotProduct {
    type Output: Sized;

    fn dot(self, operand: Self) -> Self::Output;
}

pub trait Transpose {
    type Output: Sized;

    fn transpose(self) -> Self::Output;
}