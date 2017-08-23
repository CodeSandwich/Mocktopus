pub unsafe fn as_mut<'a, T>(t_ref: &'a T) -> &'a mut T {
    &mut *(t_ref as *const T as *mut T)
}
