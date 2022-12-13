pub struct Rgb<T>
    where T: Sized + Default {
    pub r: T,
    pub g: T,
    pub b: T,
}

impl<T> Default for Rgb<T>
    where T: Sized + Default {
    fn default() -> Self {
        Self { r: Default::default(), g: Default::default(), b: Default::default() }
    }
}

impl<T: Sized + Default> crate::accelerator::AsKernelArg for Rgb<T> {

}
