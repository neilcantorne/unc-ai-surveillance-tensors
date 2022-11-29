pub struct Code {
    pub(crate) inner: Box<dyn CodeInner>
}

pub(crate) trait CodeInner: Drop {

}

pub(crate) struct OpenClCode {
    open_cl: super::backend::OpenCl,
    program: usize,
}

impl CodeInner for OpenClCode {

}

impl Drop for OpenClCode {
    fn drop(&mut self) {
        unsafe { self.open_cl.release_program(self.program).to_result().unwrap() }
    }
}