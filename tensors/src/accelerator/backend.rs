#[allow(unused)]
#[cfg(target_family = "unix")]
#[link(name = "dl", kind = "dylib")]
extern "C" {
    fn dlopen(file: *const i8, mode: i32) -> usize;
    fn dlsym(handle: usize, symbol: *const u8) -> *const();
    fn dlclose(handle: usize);
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32", kind = "dylib")]
extern "C" {
    fn LoadLibrary(file: *const i8) -> usize;
    fn GetProcAddress(handle: usize, symbol: *const u8) -> *const();
    fn FreeLibrary(handle: usize);
}

#[derive(Clone)]
pub struct BackendError {
    message: String
}

impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

macro_rules! failed_load {
    ($symbol:ident) => {
        return Err(BackendError { message: format!("Failed to load {}", stringify!($symbol)) })
    };
}

macro_rules! backend_err {
    ($msg:expr) => {
        BackendError { message: {$msg}.to_string() }
    };
}
