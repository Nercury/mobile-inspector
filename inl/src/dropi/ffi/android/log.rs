use libc;

pub type __va_list_tag = ();

/* automatically generated by rust-bindgen */

#[derive(Clone, Copy)]
#[repr(i32)]
pub enum LogPriority {
    UNKNOWN = 0,
    DEFAULT = 1,
    VERBOSE = 2,
    DEBUG = 3,
    INFO = 4,
    WARN = 5,
    ERROR = 6,
    FATAL = 7,
    SILENT = 8,
}

extern "C" {
    pub fn __android_log_write(prio: libc::c_int,
                               tag: *const libc::c_char,
                               text: *const libc::c_char)
     -> libc::c_int;
    pub fn __android_log_print(prio: libc::c_int,
                               tag: *const libc::c_char,
                               fmt: *const libc::c_char, ...)
     -> libc::c_int;
    pub fn __android_log_vprint(prio: libc::c_int,
                                tag: *const libc::c_char,
                                fmt: *const libc::c_char,
                                ap: *mut __va_list_tag) -> libc::c_int;
    pub fn __android_log_assert(cond: *const libc::c_char,
                                tag: *const libc::c_char,
                                fmt: *const libc::c_char, ...);
}