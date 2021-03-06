/* automatically generated by rust-bindgen */


#![allow(non_snake_case)]


#[repr(C)]
pub struct __BindgenUnionField<T>(::std::marker::PhantomData<T>);
impl <T> __BindgenUnionField<T> {
    #[inline]
    pub fn new() -> Self { __BindgenUnionField(::std::marker::PhantomData) }
    #[inline]
    pub unsafe fn as_ref(&self) -> &T { ::std::mem::transmute(self) }
    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut T { ::std::mem::transmute(self) }
}
impl <T> ::std::default::Default for __BindgenUnionField<T> {
    #[inline]
    fn default() -> Self { Self::new() }
}
impl <T> ::std::clone::Clone for __BindgenUnionField<T> {
    #[inline]
    fn clone(&self) -> Self { Self::new() }
}
impl <T> ::std::marker::Copy for __BindgenUnionField<T> { }
impl <T> ::std::fmt::Debug for __BindgenUnionField<T> {
    fn fmt(&self, fmt: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        fmt.write_str("__BindgenUnionField")
    }
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NastyStruct<T> {
    pub mIsSome: bool,
    pub mStorage: NastyStruct__bindgen_ty_1<T>,
    pub __bindgen_anon_1: NastyStruct__bindgen_ty_2<T>,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NastyStruct__bindgen_ty_1<T> {
    pub mFoo: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mDummy: __BindgenUnionField<::std::os::raw::c_ulong>,
    pub bindgen_union_field: u64,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct NastyStruct__bindgen_ty_2<T> {
    pub wat: __BindgenUnionField<::std::os::raw::c_short>,
    pub wut: __BindgenUnionField<*mut ::std::os::raw::c_int>,
    pub bindgen_union_field: u64,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Whatever<T> {
    pub mTPtr: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub mInt: __BindgenUnionField<::std::os::raw::c_int>,
    pub bindgen_union_field: u64,
    pub _phantom_0: ::std::marker::PhantomData<T>,
}
