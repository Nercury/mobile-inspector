/* automatically generated by rust-bindgen */

#[repr(C)]
#[derive(Copy)]
pub struct ARect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}
impl ::std::clone::Clone for ARect {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for ARect {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}