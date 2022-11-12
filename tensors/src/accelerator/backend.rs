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

#[repr(u32)]
pub enum DeviceType {
    Default = (1 << 0),
    Cpu = (1 << 1),
    Gpu = (1 << 2),
    Accelerator = (1 << 3),
    All = 0xffffffff,
}

#[repr(i32)]
pub enum OpenClErrorCode {
    Success = 0,
    DeviceNotFound = -1,
    DeviceNotAvailable = -2,
    CompilerNotAvailable = -3,
    MemObjectAllocationFailure = -4,
    OutOfResources = -5,
    OutOfHostMemory = -6,
    ProfilingInfoNotAvailable = -7,
    MemCopyOverlap = -8,
    ImageFormatMismatch = -9,
    ImageFormatNotSupported = -10,
    BuildProgramFailure = -11,
    MapFailure = -12,
    MisalignedSubBufferOffset = -13,
    ExecStatusErrorForEventsInWaitList = -14,
    CompileProgramFailure = -15,
    LinkerNotAvailable = -16,
    LinkProgramFailure = -17,
    DevicePartitionFailed = -18,
    KernelArgInfoNotAvailable = -19,
    InvalidValue = -30,
    InvalidDeviceType = -31,
    InvalidPlatform = -32,
    InvalidDevice = -33,
    InvalidContext = -34,
    InvalidQueueProperties = -35,
    InvalidCommandQueue = -36,
    InvalidHostPtr = -37,
    InvalidMemObject = -38,
    InvalidImageFormatDescriptor = -39,
    InvalidImageSize = -40,
    InvalidSampler = -41,
    InvalidBinary = -42,
    InvalidBuildOptions = -43,
    InvalidProgram = -44,
    InvalidProgramExecutable = -45,
    InvalidKernelName = -46,
    InvalidKernelDefinition = -47,
    InvalidKernel = -48,
    InvalidArgIndex = -49,
    InvalidArgValue = -50,
    InvalidArgSize = -51,
    InvalidKernelArgs = -52,
    InvalidWorkDimension = -53,
    InvalidWorkGroupSize = -54,
    InvalidWorkItemSize = -55,
    InvalidGlobalOffset = -56,
    InvalidEventWaitList = -57,
    InvalidEvent = -58,
    InvalidOperation = -59,
    InvalidGlObject = -60,
    InvalidBufferSize = -61,
    InvalidMipLevel = -62,
    InvalidGlobalWorkSize = -63,
    InvalidProperty = -64,
    InvalidImageDescriptor = -65,
    InvalidCompilerOptions = -66,
    InvalidLinkerOptions = -67,
    InvalidDevicePartitionCount = -68,
    InvalidPipeSize = -69,
    InvalidDeviceQueue = -70,
    InvalidSpecId = -71,
    MaxSizeRestrictionExceeded = -72,
}

#[macros::bind_open_cl]
extern "C" {
    #[symbol(clGetDeviceIDs)]
    pub fn get_device_id(
        platform: usize,
        device_type: DeviceType,
        num_entries: u32,
        devices: *mut usize,
        num_devices: *mut u32) -> OpenClErrorCode;
}