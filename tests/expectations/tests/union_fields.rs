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
#[derive(Debug, Copy)]
pub struct _bindgen_ty_1 {
    pub mInt: __BindgenUnionField<::std::os::raw::c_int>,
    pub mFloat: __BindgenUnionField<f32>,
    pub mPointer: __BindgenUnionField<*mut ::std::os::raw::c_void>,
    pub bindgen_union_field: u64,
}
#[test]
fn bindgen_test_layout__bindgen_ty_1() {
    assert_eq!(::std::mem::size_of::<_bindgen_ty_1>() , 8usize);
    assert_eq!(::std::mem::align_of::<_bindgen_ty_1>() , 8usize);
}
impl Clone for _bindgen_ty_1 {
    fn clone(&self) -> Self { *self }
}
pub type nsStyleUnion = _bindgen_ty_1;
