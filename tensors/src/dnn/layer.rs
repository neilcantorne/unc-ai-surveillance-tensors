pub trait LayerBuilder {
    fn build(&self, builder: &mut super::ModelBuilder)
        -> crate::Result<()>;
}
