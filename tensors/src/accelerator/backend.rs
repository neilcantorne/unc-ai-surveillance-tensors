use crate::error::ErrorVariants;

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

#[repr(u64)]
#[allow(unused)]
pub enum DeviceType {
    Default = (1 << 0),
    Cpu = (1 << 1),
    Gpu = (1 << 2),
    Accelerator = (1 << 3),
    All = 0xffffffff,
}

#[repr(i32)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[allow(unused)]
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

impl OpenClErrorCode {
    pub fn to_result(&self) -> Result<(), crate::Error> {
        match self {
            OpenClErrorCode::Success => Ok(()),
            _ => Err(crate::Error(ErrorVariants::OpenClError(*self)))
        }
    }
}

impl std::fmt::Debug for OpenClErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Success => write!(f, "Success"),
            Self::DeviceNotFound => write!(f, "Device not Found"),
            Self::DeviceNotAvailable => write!(f, "Device not Available"),
            Self::CompilerNotAvailable => write!(f, "Compiler not Available"),
            Self::MemObjectAllocationFailure => write!(f, " Memory Object Allocation Failure"),
            Self::OutOfResources => write!(f, "Out of Resources"),
            Self::OutOfHostMemory => write!(f, "Out of Host Memory"),
            Self::ProfilingInfoNotAvailable => write!(f, "Profiling Information not Available"),
            Self::MemCopyOverlap => write!(f, " Memory Copy Overlap"),
            Self::ImageFormatMismatch => write!(f, "Image Format Mismatch"),
            Self::ImageFormatNotSupported => write!(f, "Image Format not Supported"),
            Self::BuildProgramFailure => write!(f, "Build Program Failure"),
            Self::MapFailure => write!(f, "Map Failure"),
            Self::MisalignedSubBufferOffset => write!(f, "Misaligned Sub Buffer Offset"),
            Self::ExecStatusErrorForEventsInWaitList => write!(f, "Execute Status Error for Events in Wait List"),
            Self::CompileProgramFailure => write!(f, "Compile Program Failure"),
            Self::LinkerNotAvailable => write!(f, "Linker not Available"),
            Self::LinkProgramFailure => write!(f, "Link Program Failure"),
            Self::DevicePartitionFailed => write!(f, "Device Partition Failed"),
            Self::KernelArgInfoNotAvailable => write!(f, "Kernel Argument Information not Available"),
            Self::InvalidValue => write!(f, "Invalid Value"),
            Self::InvalidDeviceType => write!(f, "Invalid Device Type"),
            Self::InvalidPlatform => write!(f, "Invalid Platform"),
            Self::InvalidDevice => write!(f, "Invalid Device"),
            Self::InvalidContext => write!(f, "Invalid Context"),
            Self::InvalidQueueProperties => write!(f, "Invalid Queue Properties"),
            Self::InvalidCommandQueue => write!(f, "Invalid Command Queue"),
            Self::InvalidHostPtr => write!(f, "Invalid Host Ptr"),
            Self::InvalidMemObject => write!(f, "Invalid  Memory Object"),
            Self::InvalidImageFormatDescriptor => write!(f, "Invalid Image Format Descriptor"),
            Self::InvalidImageSize => write!(f, "Invalid Image Size"),
            Self::InvalidSampler => write!(f, "Invalid Sampler"),
            Self::InvalidBinary => write!(f, "Invalid Binary"),
            Self::InvalidBuildOptions => write!(f, "Invalid Build Options"),
            Self::InvalidProgram => write!(f, "Invalid Program"),
            Self::InvalidProgramExecutable => write!(f, "Invalid Program Executable"),
            Self::InvalidKernelName => write!(f, "Invalid Kernel Name"),
            Self::InvalidKernelDefinition => write!(f, "Invalid Kernel Definition"),
            Self::InvalidKernel => write!(f, "Invalid Kernel"),
            Self::InvalidArgIndex => write!(f, "Invalid Argument Index"),
            Self::InvalidArgValue => write!(f, "Invalid Argument Value"),
            Self::InvalidArgSize => write!(f, "Invalid Argument Size"),
            Self::InvalidKernelArgs => write!(f, "Invalid Kernel Arguments"),
            Self::InvalidWorkDimension => write!(f, "Invalid Work Dimension"),
            Self::InvalidWorkGroupSize => write!(f, "Invalid Work Group Size"),
            Self::InvalidWorkItemSize => write!(f, "Invalid Work Item Size"),
            Self::InvalidGlobalOffset => write!(f, "Invalid Global Offset"),
            Self::InvalidEventWaitList => write!(f, "Invalid Event Wait List"),
            Self::InvalidEvent => write!(f, "Invalid Event"),
            Self::InvalidOperation => write!(f, "Invalid Operation"),
            Self::InvalidGlObject => write!(f, "Invalid GL Object"),
            Self::InvalidBufferSize => write!(f, "Invalid Buffer Size"),
            Self::InvalidMipLevel => write!(f, "Invalid Mip Level"),
            Self::InvalidGlobalWorkSize => write!(f, "Invalid Global Work Size"),
            Self::InvalidProperty => write!(f, "Invalid Property"),
            Self::InvalidImageDescriptor => write!(f, "Invalid Image Descriptor"),
            Self::InvalidCompilerOptions => write!(f, "Invalid Compiler Options"),
            Self::InvalidLinkerOptions => write!(f, "Invalid Linker Options"),
            Self::InvalidDevicePartitionCount => write!(f, "Invalid Device Partition Count"),
            Self::InvalidPipeSize => write!(f, "Invalid Pipe Size"),
            Self::InvalidDeviceQueue => write!(f, "Invalid Device Queue"),
            Self::InvalidSpecId => write!(f, "Invalid Specified ID"),
            Self::MaxSizeRestrictionExceeded => write!(f, "Max Size Restriction Exceeded"),
        }
    }
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[allow(unused)]
pub enum ParamName {
    DeviceType = 0x1000,
    DeviceMaxComputeUnits = 0x1002,
    DeviceMaxClockFrequency = 0x100C,
    DeviceAddressBits = 0x100D,
    DeviceName = 0x102B,
    DeviceVendor = 0x102C,
    DriverVersion = 0x102D,
    DeviceVersion = 0x102F,
}

#[repr(u32)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[allow(unused)]
pub enum ContextProperty {
    ContextPlatform = 0x1084
}

#[repr(u64)]
#[derive(Copy, Clone)]
#[derive(PartialEq)]
#[allow(unused)]
pub enum CommandQueueProperties {
    QueueOutOfOrderExecModeEnable = (1 << 0),
    QueueProfilingEnable = (1 << 1)
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

    #[symbol(clGetDeviceInfo)]
    pub fn get_device_info(
        device: usize,
        param_name: ParamName,
        param_value_size: usize,
        param_value: *mut (),
        param_value_size_ret: *mut usize) -> OpenClErrorCode;

    #[symbol(clCreateContext)]
    pub fn create_context(
        properties: *const ContextProperty,
        num_devices: u32,
        device_ids: *const usize,
        fn_notify: usize,
        user_data: usize,
        error: *mut OpenClErrorCode
    ) -> usize;

    #[symbol(clCreateCommandQueue)]
    pub fn create_command_queue(
        context: *const ContextProperty,
        device: usize,
        properties: CommandQueueProperties,
        error: *mut OpenClErrorCode
    ) -> usize;

    #[symbol(clCreateKernel)]
    pub fn create_kernel(
        program: usize,
        kernel_name: *const i8,
        error: *mut OpenClErrorCode) -> usize;

    #[symbol(clSetKernelArg)]
    pub fn set_kernel_arg (kernel: usize, index: u32, size: usize, buffer: *const ());

    #[symbol(clCreateProgramWithIL)]
    pub fn create_program_with_il(
        context: usize,
        il: *const u8,
        size: usize,
        error: *mut OpenClErrorCode) -> usize;

    #[symbol(clReleaseContext)]
    pub fn release_context(context: usize) -> OpenClErrorCode;

    #[symbol(clReleaseProgram)]
    pub fn release_program(program: usize) -> OpenClErrorCode;
}
