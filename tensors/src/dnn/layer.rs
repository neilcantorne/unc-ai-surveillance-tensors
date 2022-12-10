pub trait Layer {
    fn build(&self, builder: &mut super::ModelBuilder) -> crate::Result<()>;
}
