pub struct KernelArgsStack<'a> {
    pub(in crate::accelerator) inner: &'a mut dyn KernelArgsStackInner<'a>
}

impl KernelArgsStack<'_> {
    #[inline(always)]
    pub fn push_c_buffer(&mut self, address: *const(), size: usize) {
        self.inner.push_c_buffer(address, size)
    }

    #[inline(always)]
    pub fn push_u8(&mut self, value: u8) { self.inner.push_u8(value) }

    #[inline(always)]
    pub fn push_u16(&mut self, value: u16) { self.inner.push_u16(value) }

    #[inline(always)]
    pub fn push_u32(&mut self, value: u32) { self.inner.push_u32(value) }

    #[inline(always)]
    pub fn push_u64(&mut self, value: u64) { self.inner.push_u64(value) }

    #[inline(always)]
    pub fn push_i8(&mut self, value: i8) { self.inner.push_i8(value) }

    #[inline(always)]
    pub fn push_i16(&mut self, value: i16) { self.inner.push_i16(value) }

    #[inline(always)]
    pub fn push_i32(&mut self, value: i32) { self.inner.push_i32(value) }

    #[inline(always)]
    pub fn push_i64(&mut self, value: i64) { self.inner.push_i64(value) }
}

pub(in crate::accelerator) trait KernelArgsStackInner<'a> {
    fn push_c_buffer(&mut self, address: *const(), size: usize);
    fn push_u8(&mut self, value: u8);
    fn push_u16(&mut self, value: u16);
    fn push_u32(&mut self, value: u32);
    fn push_u64(&mut self, value: u64);
    fn push_i8(&mut self, value: i8);
    fn push_i16(&mut self, value: i16);
    fn push_i32(&mut self, value: i32);
    fn push_i64(&mut self, value: i64);
}

pub(in crate::accelerator) struct OpenClKernelArgsStack<'a> {
    pub(super) kernel: &'a super::kernel::OpenClKernel,
    pub(super) index_counter: u32,
}

impl<'a> KernelArgsStackInner<'a> for OpenClKernelArgsStack<'a> {
    fn push_c_buffer(&mut self, address: *const(), size: usize) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                size,
                address);
        };

        self.index_counter += 1;
    }

    fn push_u8(&mut self, value: u8) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                1,
                &value as *const u8 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_u16(&mut self, value: u16) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                2,
                &value as *const u16 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_u32(&mut self, value: u32) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                4,
                &value as *const u32 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_u64(&mut self, value: u64) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                8,
                &value as *const u64 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_i8(&mut self, value: i8) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                1,
                &value as *const i8 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_i16(&mut self, value: i16) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                2,
                &value as *const i16 as *const ());
        };

        self.index_counter += 1;
    }

    fn push_i32(&mut self, value: i32) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                4,
                &value as *const i32 as *const ());
        };
    }

    fn push_i64(&mut self, value: i64) {
        unsafe {
            self.kernel.open_cl.set_kernel_arg(
                self.kernel.kernel,
                self.index_counter,
                8,
                &value as *const i64 as *const ());
        };
    }
}
