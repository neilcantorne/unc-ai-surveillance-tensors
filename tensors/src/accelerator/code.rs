pub struct Code {
    pub(crate) inner: Box<dyn CodeInner>
}

pub(crate) trait CodeInner: Drop {

}

