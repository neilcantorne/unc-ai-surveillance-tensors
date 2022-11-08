#[cfg(target_family = "unix")]
#[link(name = "dl", kind = "dylib")]
extern "C" {
    fn dlopen(file: *const i8, mode: i32) -> *const ();
    fn dlsym(handle: *const (), symbol: *const u8) ->  *const();
    fn dlclose(handle: *const());
}

#[cfg(target_os = "windows")]
#[link(name = "kernel32", kind = "dylib")]
extern "C" {
    fn LoadLibrary(file: *const i8) -> *const ();
    fn GetProcAddress(handle: *const (), symbol: *const u8) ->  *const();
    fn FreeLibrary(handle: *const());
}