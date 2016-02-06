// automatically generated by rust-bindgen

use std::os::raw::{c_char, c_int};
use std::str::Utf8Error;
use std::ffi::CStr;
use std::ffi::CString;
use std::fmt::Display;

pub fn ptr_to_string(str_ptr: *const c_char) -> Result<String, Utf8Error> {
    let str_slice: &str = try!(unsafe { CStr::from_ptr(str_ptr) }.to_str());
    Ok(str_slice.to_string())
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum SioError {
    None = 0,
    NoMem = 1,
    InitAudioBackend = 2,
    SystemResources = 3,
    OpeningDevice = 4,
    NoSuchDevice = 5,
    Invalid = 6,
    BackendUnavailable = 7,
    Streaming = 8,
    IncompatibleDevice = 9,
    NoSuchClient = 10,
    IncompatibleBackend = 11,
    BackendDisconnected = 12,
    Interrupted = 13,
    Underflow = 14,
    EncodingString = 15,
}
impl Display for SioError {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { soundio_strerror(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy, PartialEq, Debug)]
#[repr(u32)]
pub enum SioChannelId {
    Invalid = 0,
    FrontLeft = 1,
    FrontRight = 2,
    FrontCenter = 3,
    Lfe = 4,
    BackLeft = 5,
    BackRight = 6,
    FrontLeftCenter = 7,
    FrontRightCenter = 8,
    BackCenter = 9,
    SideLeft = 10,
    SideRight = 11,
    TopCenter = 12,
    TopFrontLeft = 13,
    TopFrontCenter = 14,
    TopFrontRight = 15,
    TopBackLeft = 16,
    TopBackCenter = 17,
    TopBackRight = 18,
    BackLeftCenter = 19,
    BackRightCenter = 20,
    FrontLeftWide = 21,
    FrontRightWide = 22,
    FrontLeftHigh = 23,
    FrontCenterHigh = 24,
    FrontRightHigh = 25,
    TopFrontLeftCenter = 26,
    TopFrontRightCenter = 27,
    TopSideLeft = 28,
    TopSideRight = 29,
    LeftLfe = 30,
    RightLfe = 31,
    Lfe2 = 32,
    BottomCenter = 33,
    BottomLeftCenter = 34,
    BottomRightCenter = 35,
    MsMid = 36,
    MsSide = 37,
    AmbisonicW = 38,
    AmbisonicX = 39,
    AmbisonicY = 40,
    AmbisonicZ = 41,
    XyX = 42,
    XyY = 43,
    HeadphonesLeft = 44,
    HeadphonesRight = 45,
    ClickTrack = 46,
    ForeignLanguage = 47,
    HearingImpaired = 48,
    Narration = 49,
    Haptic = 50,
    DialogCentricMix = 51,
    Aux = 52,
    Aux0 = 53,
    Aux1 = 54,
    Aux2 = 55,
    Aux3 = 56,
    Aux4 = 57,
    Aux5 = 58,
    Aux6 = 59,
    Aux7 = 60,
    Aux8 = 61,
    Aux9 = 62,
    Aux10 = 63,
    Aux11 = 64,
    Aux12 = 65,
    Aux13 = 66,
    Aux14 = 67,
    Aux15 = 68,
}
impl Display for SioChannelId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { soundio_get_channel_name(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}
impl From<String> for SioChannelId {
    fn from(id: String) -> Self {
        let str_len = id.len() as i32;
        let cstr = CString::new(id).unwrap();
        unsafe { soundio_parse_channel_id(cstr.as_ptr(), str_len) }
    }
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioChannelLayoutId {
    Mono = 0,
    Stereo = 1,
    TwoPointOne = 2,
    ThreePointZero = 3,
    ThreePointZeroBack = 4,
    ThreePointOne = 5,
    FourPointZero = 6,
    Quad = 7,
    QuadSide = 8,
    FourPointOne = 9,
    FivePointZeroBack = 10,
    FivePointZeroSide = 11,
    FivePointOne = 12,
    FivePointOneBack = 13,
    SixPointZeroSide = 14,
    SixPointZeroFront = 15,
    Hexagonal = 16,
    SixPointOne = 17,
    SixPointOneBack = 18,
    SixPointOneFront = 19,
    SevenPointZero = 20,
    SevenPointZeroFront = 21,
    SevenPointOne = 22,
    SevenPointOneWide = 23,
    SevenPointOneWideBack = 24,
    Octagonal = 25,
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioBackend {
    SoundIoBackendNone = 0,
    SoundIoBackendJack = 1,
    SoundIoBackendPulseAudio = 2,
    SoundIoBackendAlsa = 3,
    SoundIoBackendCoreAudio = 4,
    SoundIoBackendWasapi = 5,
    SoundIoBackendDummy = 6,
}
impl Display for SioBackend {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { soundio_backend_name(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioDeviceAim {
    SoundIoDeviceAimInput = 0,
    SoundIoDeviceAimOutput = 1,
}

#[allow(dead_code,non_camel_case_types)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum SioFormat {
    SoundIoFormatInvalid = 0,
    SoundIoFormatS8 = 1,
    SoundIoFormatU8 = 2,
    SoundIoFormatS16LE = 3,
    SoundIoFormatS16BE = 4,
    SoundIoFormatU16LE = 5,
    SoundIoFormatU16BE = 6,
    SoundIoFormatS24LE = 7,
    SoundIoFormatS24BE = 8,
    SoundIoFormatU24LE = 9,
    SoundIoFormatU24BE = 10,
    SoundIoFormatS32LE = 11,
    SoundIoFormatS32BE = 12,
    SoundIoFormatU32LE = 13,
    SoundIoFormatU32BE = 14,
    SoundIoFormatFloat32LE = 15,
    SoundIoFormatFloat32BE = 16,
    SoundIoFormatFloat64LE = 17,
    SoundIoFormatFloat64BE = 18,
}
impl Display for SioFormat {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let str_ptr = unsafe { soundio_format_string(*self) };
        write!(f, "{}", ptr_to_string(str_ptr).unwrap())
    }
}
impl SioFormat {
    pub fn get_bytes_per_sample(self) -> i32 {
        unsafe { soundio_get_bytes_per_sample(self) as i32 }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoChannelLayout {
    pub name: *const c_char,
    pub channel_count: c_int,
    pub channels: [SioChannelId; 24usize],
}
impl ::std::clone::Clone for Struct_SoundIoChannelLayout {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoChannelLayout {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoSampleRateRange {
    pub min: c_int,
    pub max: c_int,
}
impl ::std::clone::Clone for Struct_SoundIoSampleRateRange {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoSampleRateRange {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoChannelArea {
    pub ptr: *mut c_char,
    pub step: c_int,
}
impl ::std::clone::Clone for Struct_SoundIoChannelArea {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoChannelArea {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIo {
    pub userdata: *mut ::std::os::raw::c_void,
    pub on_devices_change: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                          *mut Struct_SoundIo)>,
    pub on_backend_disconnect: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                              *mut Struct_SoundIo,
                                                                          err:
                                                                              c_int)>,
    pub on_events_signal: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                         *mut Struct_SoundIo)>,
    pub current_backend: SioBackend,
    pub app_name: *const c_char,
    pub emit_rtprio_warning: ::std::option::Option<extern "C" fn()>,
    pub jack_info_callback: ::std::option::Option<unsafe extern "C" fn(msg:
                                                                           *const c_char)>,
    pub jack_error_callback: ::std::option::Option<unsafe extern "C" fn(msg:
                                                                            *const c_char)>,
}
impl ::std::clone::Clone for Struct_SoundIo {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIo {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoDevice {
    pub soundio: *mut Struct_SoundIo,
    pub id: *mut c_char,
    pub name: *mut c_char,
    pub aim: SioDeviceAim,
    pub layouts: *mut Struct_SoundIoChannelLayout,
    pub layout_count: c_int,
    pub current_layout: Struct_SoundIoChannelLayout,
    pub formats: *mut SioFormat,
    pub format_count: c_int,
    pub current_format: SioFormat,
    pub sample_rates: *mut Struct_SoundIoSampleRateRange,
    pub sample_rate_count: c_int,
    pub sample_rate_current: c_int,
    pub software_latency_min: ::std::os::raw::c_double,
    pub software_latency_max: ::std::os::raw::c_double,
    pub software_latency_current: ::std::os::raw::c_double,
    pub is_raw: u8,
    pub ref_count: c_int,
    pub probe_error: c_int,
}
impl ::std::clone::Clone for Struct_SoundIoDevice {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoDevice {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoOutStream {
    pub device: *mut Struct_SoundIoDevice,
    pub format: SioFormat,
    pub sample_rate: c_int,
    pub layout: Struct_SoundIoChannelLayout,
    pub software_latency: ::std::os::raw::c_double,
    pub userdata: *mut ::std::os::raw::c_void,
    pub write_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                       *mut Struct_SoundIoOutStream,
                                                                   frame_count_min:
                                                                       c_int,
                                                                   frame_count_max:
                                                                       c_int)>,
    pub underflow_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                           *mut Struct_SoundIoOutStream)>,
    pub error_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                       *mut Struct_SoundIoOutStream,
                                                                   err:
                                                                       c_int)>,
    pub name: *const c_char,
    pub non_terminal_hint: u8,
    pub bytes_per_frame: c_int,
    pub bytes_per_sample: c_int,
    pub layout_error: c_int,
}
impl ::std::clone::Clone for Struct_SoundIoOutStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoOutStream {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}
#[repr(C)]
#[derive(Copy)]
pub struct Struct_SoundIoInStream {
    pub device: *mut Struct_SoundIoDevice,
    pub format: SioFormat,
    pub sample_rate: c_int,
    pub layout: Struct_SoundIoChannelLayout,
    pub software_latency: ::std::os::raw::c_double,
    pub userdata: *mut ::std::os::raw::c_void,
    pub read_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                      *mut Struct_SoundIoInStream,
                                                                  frame_count_min:
                                                                      c_int,
                                                                  frame_count_max:
                                                                      c_int)>,
    pub overflow_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                          *mut Struct_SoundIoInStream)>,
    pub error_callback: ::std::option::Option<unsafe extern "C" fn(arg1:
                                                                       *mut Struct_SoundIoInStream,
                                                                   err:
                                                                       c_int)>,
    pub name: *const c_char,
    pub non_terminal_hint: u8,
    pub bytes_per_frame: c_int,
    pub bytes_per_sample: c_int,
    pub layout_error: c_int,
}
impl ::std::clone::Clone for Struct_SoundIoInStream {
    fn clone(&self) -> Self {
        *self
    }
}
impl ::std::default::Default for Struct_SoundIoInStream {
    fn default() -> Self {
        unsafe { ::std::mem::zeroed() }
    }
}

#[allow(dead_code,non_camel_case_types)]
pub enum Struct_SoundIoRingBuffer { }

#[link(name = "soundio")]
extern "C" {
    pub fn soundio_create() -> *mut Struct_SoundIo;
    pub fn soundio_destroy(soundio: *mut Struct_SoundIo);
    pub fn soundio_connect(soundio: *mut Struct_SoundIo) -> SioError;
    pub fn soundio_connect_backend(soundio: *mut Struct_SoundIo,
                                   backend: SioBackend)
                                   -> SioError;
    pub fn soundio_disconnect(soundio: *mut Struct_SoundIo);
    pub fn soundio_strerror(error: SioError) -> *const c_char;
    pub fn soundio_backend_name(backend: SioBackend) -> *const c_char;
    pub fn soundio_backend_count(soundio: *mut Struct_SoundIo) -> c_int;
    pub fn soundio_get_backend(soundio: *mut Struct_SoundIo, index: c_int) -> SioBackend;
    pub fn soundio_have_backend(backend: SioBackend) -> u8;
    pub fn soundio_flush_events(soundio: *mut Struct_SoundIo);
    pub fn soundio_wait_events(soundio: *mut Struct_SoundIo);
    pub fn soundio_wakeup(soundio: *mut Struct_SoundIo);
    pub fn soundio_force_device_scan(soundio: *mut Struct_SoundIo);
    pub fn soundio_channel_layout_equal(a: *const Struct_SoundIoChannelLayout,
                                        b: *const Struct_SoundIoChannelLayout)
                                        -> u8;
    pub fn soundio_get_channel_name(id: SioChannelId) -> *const c_char;
    pub fn soundio_parse_channel_id(str: *const c_char, str_len: c_int) -> SioChannelId;
    pub fn soundio_channel_layout_builtin_count() -> c_int;
    pub fn soundio_channel_layout_get_builtin(index: c_int) -> *const Struct_SoundIoChannelLayout;
    pub fn soundio_channel_layout_get_default(channel_count: c_int)
                                              -> *const Struct_SoundIoChannelLayout;
    pub fn soundio_channel_layout_find_channel(layout: *const Struct_SoundIoChannelLayout,
                                               channel: SioChannelId)
                                               -> c_int;
    pub fn soundio_channel_layout_detect_builtin(layout: *mut Struct_SoundIoChannelLayout) -> u8;
    pub fn soundio_best_matching_channel_layout(preferred_layouts:
                                                    *const Struct_SoundIoChannelLayout,
                                                preferred_layout_count:
                                                    c_int,
                                                available_layouts:
                                                    *const Struct_SoundIoChannelLayout,
                                                available_layout_count:
                                                    c_int)
     -> *const Struct_SoundIoChannelLayout;
    // TODO: I am not sure if I should implement the sort method. The benefit does not
    // justify the amount of work to get it done. `ChannelLayout` contains only
    // `*const Struct_SoundIoChannelLayout` pointer, so `transmute` must be used to
    // make the `*mut` pointer.
    pub fn soundio_sort_channel_layouts(layouts: *mut Struct_SoundIoChannelLayout,
                                        layout_count: c_int);
    pub fn soundio_get_bytes_per_sample(format: SioFormat) -> c_int;
    pub fn soundio_format_string(format: SioFormat) -> *const c_char;
    pub fn soundio_input_device_count(soundio: *mut Struct_SoundIo) -> c_int;
    pub fn soundio_output_device_count(soundio: *mut Struct_SoundIo) -> c_int;
    pub fn soundio_get_input_device(soundio: *mut Struct_SoundIo,
                                    index: c_int)
                                    -> *mut Struct_SoundIoDevice;
    pub fn soundio_get_output_device(soundio: *mut Struct_SoundIo,
                                     index: c_int)
                                     -> *mut Struct_SoundIoDevice;
    pub fn soundio_default_input_device_index(soundio: *mut Struct_SoundIo) -> c_int;
    pub fn soundio_default_output_device_index(soundio: *mut Struct_SoundIo) -> c_int;
    pub fn soundio_device_ref(device: *mut Struct_SoundIoDevice);
    pub fn soundio_device_unref(device: *mut Struct_SoundIoDevice);
    pub fn soundio_device_equal(a: *const Struct_SoundIoDevice,
                                b: *const Struct_SoundIoDevice)
                                -> u8;
    pub fn soundio_device_sort_channel_layouts(device: *mut Struct_SoundIoDevice);
    pub fn soundio_device_supports_format(device: *mut Struct_SoundIoDevice,
                                          format: SioFormat)
                                          -> u8;
    pub fn soundio_device_supports_layout(device: *mut Struct_SoundIoDevice,
                                          layout: *const Struct_SoundIoChannelLayout)
                                          -> u8;
    pub fn soundio_device_supports_sample_rate(device: *mut Struct_SoundIoDevice,
                                               sample_rate: c_int)
                                               -> u8;
    pub fn soundio_device_nearest_sample_rate(device: *mut Struct_SoundIoDevice,
                                              sample_rate: c_int)
                                              -> c_int;
    pub fn soundio_outstream_create(device: *mut Struct_SoundIoDevice)
                                    -> *mut Struct_SoundIoOutStream;
    pub fn soundio_outstream_destroy(outstream: *mut Struct_SoundIoOutStream);
    pub fn soundio_outstream_open(outstream: *mut Struct_SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_start(outstream: *mut Struct_SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_begin_write(outstream: *mut Struct_SoundIoOutStream,
                                         areas: *mut *mut Struct_SoundIoChannelArea,
                                         frame_count: *mut c_int)
                                         -> SioError;
    pub fn soundio_outstream_end_write(outstream: *mut Struct_SoundIoOutStream) -> SioError;
    pub fn soundio_outstream_clear_buffer(outstream: *mut Struct_SoundIoOutStream)
                                          -> SioError;
    pub fn soundio_outstream_pause(outstream: *mut Struct_SoundIoOutStream,
                                   pause: u8)
                                   -> SioError;
    pub fn soundio_outstream_get_latency(outstream: *mut Struct_SoundIoOutStream,
                                         out_latency: *mut ::std::os::raw::c_double)
                                         -> SioError;
    pub fn soundio_instream_create(device: *mut Struct_SoundIoDevice) -> *mut Struct_SoundIoInStream;
    pub fn soundio_instream_destroy(instream: *mut Struct_SoundIoInStream);
    pub fn soundio_instream_open(instream: *mut Struct_SoundIoInStream) -> c_int;
    pub fn soundio_instream_start(instream: *mut Struct_SoundIoInStream) -> c_int;
    pub fn soundio_instream_begin_read(instream: *mut Struct_SoundIoInStream,
                                       areas: *mut *mut Struct_SoundIoChannelArea,
                                       frame_count: *mut c_int)
                                       -> c_int;
    pub fn soundio_instream_end_read(instream: *mut Struct_SoundIoInStream) -> c_int;
    pub fn soundio_instream_pause(instream: *mut Struct_SoundIoInStream, pause: u8) -> c_int;
    pub fn soundio_instream_get_latency(instream: *mut Struct_SoundIoInStream,
                                        out_latency: *mut ::std::os::raw::c_double)
                                        -> c_int;
    pub fn soundio_ring_buffer_create(soundio: *mut Struct_SoundIo,
                                      requested_capacity: c_int)
                                      -> *mut Struct_SoundIoRingBuffer;
    pub fn soundio_ring_buffer_destroy(ring_buffer: *mut Struct_SoundIoRingBuffer);
    pub fn soundio_ring_buffer_capacity(ring_buffer: *mut Struct_SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_write_ptr(ring_buffer: *mut Struct_SoundIoRingBuffer) -> *mut c_char;
    pub fn soundio_ring_buffer_advance_write_ptr(ring_buffer: *mut Struct_SoundIoRingBuffer,
                                                 count: c_int);
    pub fn soundio_ring_buffer_read_ptr(ring_buffer: *mut Struct_SoundIoRingBuffer) -> *mut c_char;
    pub fn soundio_ring_buffer_advance_read_ptr(ring_buffer: *mut Struct_SoundIoRingBuffer,
                                                count: c_int);
    pub fn soundio_ring_buffer_fill_count(ring_buffer: *mut Struct_SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_free_count(ring_buffer: *mut Struct_SoundIoRingBuffer) -> c_int;
    pub fn soundio_ring_buffer_clear(ring_buffer: *mut Struct_SoundIoRingBuffer);
}
