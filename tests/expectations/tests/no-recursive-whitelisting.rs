/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]

pub enum Bar {}

#[repr(C)]
#[derive(Debug, Copy)]
pub struct Foo {
    pub baz: *mut Bar,
}
#[test]
fn bindgen_test_layout_Foo() {
    assert_eq!(::std::mem::size_of::<Foo>() , 8usize);
    assert_eq!(::std::mem::align_of::<Foo>() , 8usize);
}
impl Clone for Foo {
    fn clone(&self) -> Self { *self }
}
