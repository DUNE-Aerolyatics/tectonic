use crate::{xetex_ini, xetex_output};

#[inline]
pub(crate) unsafe fn is_non_discardable_node(p: i32) -> bool {
    use xetex_ini::MEM;
    (MEM[p as usize].b16.s1 as i32) < 9
}

#[inline]
pub(crate) unsafe fn is_char_node(p: i32) -> bool {
    p >= xetex_ini::hi_mem_min
}

#[inline]
pub(crate) unsafe fn print_c_string(mut str: *const i8) {
    while *str != 0 {
        let fresh0 = str;
        str = str.offset(1);
        xetex_output::print_char(*fresh0 as i32);
    }
}
