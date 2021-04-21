/* automatically generated by rust-bindgen 0.58.1 */

pub const GBM_FORMAT_BIG_ENDIAN: u32 = 2147483648;
pub const GBM_BO_IMPORT_WL_BUFFER: u32 = 21761;
pub const GBM_BO_IMPORT_EGL_IMAGE: u32 = 21762;
pub const GBM_BO_IMPORT_FD: u32 = 21763;
pub const GBM_BO_IMPORT_FD_MODIFIER: u32 = 21764;
pub const GBM_MAX_PLANES: u32 = 4;
pub type size_t = libc::c_ulong;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
#[doc = " \\file gbm.h"]
#[doc = " \\brief Generic Buffer Manager"]
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_device {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_bo {
    _unused: [u8; 0],
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_surface {
    _unused: [u8; 0],
}
#[doc = " Abstraction representing the handle to a buffer allocated by the"]
#[doc = " manager"]
#[repr(C)]
#[derive(Copy, Clone)]
pub union gbm_bo_handle {
    pub ptr: *mut libc::c_void,
    pub s32: i32,
    pub u32_: u32,
    pub s64: i64,
    pub u64_: u64,
}
#[test]
fn bindgen_test_layout_gbm_bo_handle() {
    assert_eq!(
        ::std::mem::size_of::<gbm_bo_handle>(),
        8usize,
        concat!("Size of: ", stringify!(gbm_bo_handle))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_bo_handle>(),
        8usize,
        concat!("Alignment of ", stringify!(gbm_bo_handle))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).ptr as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(ptr)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).s32 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(s32)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).u32_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(u32_)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).s64 as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(s64)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_bo_handle>())).u64_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_bo_handle),
            "::",
            stringify!(u64_)
        )
    );
}
pub mod gbm_bo_format {
    #[doc = " Format of the allocated buffer"]
    pub type Type = libc::c_uint;
    #[doc = " RGB with 8 bits per channel in a 32 bit value"]
    pub const GBM_BO_FORMAT_XRGB8888: Type = 0;
    #[doc = " ARGB with 8 bits per channel in a 32 bit value"]
    pub const GBM_BO_FORMAT_ARGB8888: Type = 1;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_format_name_desc {
    pub name: [libc::c_char; 5usize],
}
#[test]
fn bindgen_test_layout_gbm_format_name_desc() {
    assert_eq!(
        ::std::mem::size_of::<gbm_format_name_desc>(),
        5usize,
        concat!("Size of: ", stringify!(gbm_format_name_desc))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_format_name_desc>(),
        1usize,
        concat!("Alignment of ", stringify!(gbm_format_name_desc))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_format_name_desc>())).name as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_format_name_desc),
            "::",
            stringify!(name)
        )
    );
}
pub mod gbm_bo_flags {
    #[doc = " Flags to indicate the intended use for the buffer - these are passed into"]
    #[doc = " gbm_bo_create(). The caller must set the union of all the flags that are"]
    #[doc = " appropriate"]
    #[doc = ""]
    #[doc = " \\sa Use gbm_device_is_format_supported() to check if the combination of format"]
    #[doc = " and use flags are supported"]
    pub type Type = libc::c_uint;
    #[doc = " Buffer is going to be presented to the screen using an API such as KMS"]
    pub const GBM_BO_USE_SCANOUT: Type = 1;
    #[doc = " Buffer is going to be used as cursor"]
    pub const GBM_BO_USE_CURSOR: Type = 2;
    #[doc = " Deprecated"]
    pub const GBM_BO_USE_CURSOR_64X64: Type = 2;
    #[doc = " Buffer is to be used for rendering - for example it is going to be used"]
    #[doc = " as the storage for a color buffer"]
    pub const GBM_BO_USE_RENDERING: Type = 4;
    #[doc = " Buffer can be used for gbm_bo_write.  This is guaranteed to work"]
    #[doc = " with GBM_BO_USE_CURSOR, but may not work for other combinations."]
    pub const GBM_BO_USE_WRITE: Type = 8;
    #[doc = " Buffer is linear, i.e. not tiled."]
    pub const GBM_BO_USE_LINEAR: Type = 16;
    #[doc = " Buffer is protected, i.e. encrypted and not readable by CPU or any"]
    #[doc = " other non-secure / non-trusted components nor by non-trusted OpenGL,"]
    #[doc = " OpenCL, and Vulkan applications."]
    pub const GBM_BO_USE_PROTECTED: Type = 32;
}
extern "C" {
    pub fn gbm_device_get_fd(gbm: *mut gbm_device) -> libc::c_int;
}
extern "C" {
    pub fn gbm_device_get_backend_name(gbm: *mut gbm_device) -> *const libc::c_char;
}
extern "C" {
    pub fn gbm_device_is_format_supported(
        gbm: *mut gbm_device,
        format: u32,
        usage: u32,
    ) -> libc::c_int;
}
extern "C" {
    pub fn gbm_device_get_format_modifier_plane_count(
        gbm: *mut gbm_device,
        format: u32,
        modifier: u64,
    ) -> libc::c_int;
}
extern "C" {
    pub fn gbm_device_destroy(gbm: *mut gbm_device);
}
extern "C" {
    pub fn gbm_create_device(fd: libc::c_int) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_create(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        flags: u32,
    ) -> *mut gbm_bo;
}
extern "C" {
    pub fn gbm_bo_create_with_modifiers(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        modifiers: *const u64,
        count: libc::c_uint,
    ) -> *mut gbm_bo;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_import_fd_data {
    pub fd: libc::c_int,
    pub width: u32,
    pub height: u32,
    pub stride: u32,
    pub format: u32,
}
#[test]
fn bindgen_test_layout_gbm_import_fd_data() {
    assert_eq!(
        ::std::mem::size_of::<gbm_import_fd_data>(),
        20usize,
        concat!("Size of: ", stringify!(gbm_import_fd_data))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_import_fd_data>(),
        4usize,
        concat!("Alignment of ", stringify!(gbm_import_fd_data))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).fd as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(fd)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).width as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).height as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).stride as *const _ as usize },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(stride)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_data>())).format as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_data),
            "::",
            stringify!(format)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct gbm_import_fd_modifier_data {
    pub width: u32,
    pub height: u32,
    pub format: u32,
    pub num_fds: u32,
    pub fds: [libc::c_int; 4usize],
    pub strides: [libc::c_int; 4usize],
    pub offsets: [libc::c_int; 4usize],
    pub modifier: u64,
}
#[test]
fn bindgen_test_layout_gbm_import_fd_modifier_data() {
    assert_eq!(
        ::std::mem::size_of::<gbm_import_fd_modifier_data>(),
        72usize,
        concat!("Size of: ", stringify!(gbm_import_fd_modifier_data))
    );
    assert_eq!(
        ::std::mem::align_of::<gbm_import_fd_modifier_data>(),
        8usize,
        concat!("Alignment of ", stringify!(gbm_import_fd_modifier_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).width as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(width)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).height as *const _ as usize
        },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(height)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).format as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(format)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).num_fds as *const _ as usize
        },
        12usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(num_fds)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).fds as *const _ as usize },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(fds)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).strides as *const _ as usize
        },
        32usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(strides)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).offsets as *const _ as usize
        },
        48usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(offsets)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<gbm_import_fd_modifier_data>())).modifier as *const _ as usize
        },
        64usize,
        concat!(
            "Offset of field: ",
            stringify!(gbm_import_fd_modifier_data),
            "::",
            stringify!(modifier)
        )
    );
}
extern "C" {
    pub fn gbm_bo_import(
        gbm: *mut gbm_device,
        type_: u32,
        buffer: *mut libc::c_void,
        usage: u32,
    ) -> *mut gbm_bo;
}
pub mod gbm_bo_transfer_flags {
    #[doc = " Flags to indicate the type of mapping for the buffer - these are"]
    #[doc = " passed into gbm_bo_map(). The caller must set the union of all the"]
    #[doc = " flags that are appropriate."]
    #[doc = ""]
    #[doc = " These flags are independent of the GBM_BO_USE_* creation flags. However,"]
    #[doc = " mapping the buffer may require copying to/from a staging buffer."]
    #[doc = ""]
    #[doc = " See also: pipe_map_flags"]
    pub type Type = libc::c_uint;
    #[doc = " Buffer contents read back (or accessed directly) at transfer"]
    #[doc = " create time."]
    pub const GBM_BO_TRANSFER_READ: Type = 1;
    #[doc = " Buffer contents will be written back at unmap time"]
    #[doc = " (or modified as a result of being accessed directly)."]
    pub const GBM_BO_TRANSFER_WRITE: Type = 2;
    #[doc = " Read/modify/write"]
    pub const GBM_BO_TRANSFER_READ_WRITE: Type = 3;
}
extern "C" {
    pub fn gbm_bo_map(
        bo: *mut gbm_bo,
        x: u32,
        y: u32,
        width: u32,
        height: u32,
        flags: u32,
        stride: *mut u32,
        map_data: *mut *mut libc::c_void,
    ) -> *mut libc::c_void;
}
extern "C" {
    pub fn gbm_bo_unmap(bo: *mut gbm_bo, map_data: *mut libc::c_void);
}
extern "C" {
    pub fn gbm_bo_get_width(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_height(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_stride(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_stride_for_plane(bo: *mut gbm_bo, plane: libc::c_int) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_format(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_bpp(bo: *mut gbm_bo) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_offset(bo: *mut gbm_bo, plane: libc::c_int) -> u32;
}
extern "C" {
    pub fn gbm_bo_get_device(bo: *mut gbm_bo) -> *mut gbm_device;
}
extern "C" {
    pub fn gbm_bo_get_handle(bo: *mut gbm_bo) -> gbm_bo_handle;
}
extern "C" {
    pub fn gbm_bo_get_fd(bo: *mut gbm_bo) -> libc::c_int;
}
extern "C" {
    pub fn gbm_bo_get_modifier(bo: *mut gbm_bo) -> u64;
}
extern "C" {
    pub fn gbm_bo_get_plane_count(bo: *mut gbm_bo) -> libc::c_int;
}
extern "C" {
    pub fn gbm_bo_get_handle_for_plane(bo: *mut gbm_bo, plane: libc::c_int) -> gbm_bo_handle;
}
extern "C" {
    pub fn gbm_bo_write(bo: *mut gbm_bo, buf: *const libc::c_void, count: size_t) -> libc::c_int;
}
extern "C" {
    pub fn gbm_bo_set_user_data(
        bo: *mut gbm_bo,
        data: *mut libc::c_void,
        destroy_user_data: ::std::option::Option<
            unsafe extern "C" fn(arg1: *mut gbm_bo, arg2: *mut libc::c_void),
        >,
    );
}
extern "C" {
    pub fn gbm_bo_get_user_data(bo: *mut gbm_bo) -> *mut libc::c_void;
}
extern "C" {
    pub fn gbm_bo_destroy(bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_create(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        flags: u32,
    ) -> *mut gbm_surface;
}
extern "C" {
    pub fn gbm_surface_create_with_modifiers(
        gbm: *mut gbm_device,
        width: u32,
        height: u32,
        format: u32,
        modifiers: *const u64,
        count: libc::c_uint,
    ) -> *mut gbm_surface;
}
extern "C" {
    pub fn gbm_surface_lock_front_buffer(surface: *mut gbm_surface) -> *mut gbm_bo;
}
extern "C" {
    pub fn gbm_surface_release_buffer(surface: *mut gbm_surface, bo: *mut gbm_bo);
}
extern "C" {
    pub fn gbm_surface_has_free_buffers(surface: *mut gbm_surface) -> libc::c_int;
}
extern "C" {
    pub fn gbm_surface_destroy(surface: *mut gbm_surface);
}
extern "C" {
    pub fn gbm_format_get_name(
        gbm_format: u32,
        desc: *mut gbm_format_name_desc,
    ) -> *mut libc::c_char;
}
