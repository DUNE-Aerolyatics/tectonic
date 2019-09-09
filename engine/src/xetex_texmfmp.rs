#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    #[no_mangle]
    fn free(__ptr: *mut libc::c_void);
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* The internal, C/C++ interface: */
    #[no_mangle]
    fn _tt_abort(format: *const libc::c_char, _: ...) -> !;
    #[no_mangle]
    fn ttstub_get_file_md5(path: *const libc::c_char, digest: *mut libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn ttstub_get_data_md5(
        data: *const libc::c_char,
        len: size_t,
        digest: *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn xmalloc(size: size_t) -> *mut libc::c_void;
    #[no_mangle]
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    static mut pool_size: int32_t;
    #[no_mangle]
    static mut str_pool: *mut packed_UTF16_code;
    #[no_mangle]
    static mut str_start: *mut pool_pointer;
    #[no_mangle]
    static mut pool_ptr: pool_pointer;
    #[no_mangle]
    static firstByteMark: [uint8_t; 7];
    #[no_mangle]
    static offsetsFromUTF8: [uint32_t; 6];
    #[no_mangle]
    static bytesFromUTF8: [uint8_t; 256];
    #[no_mangle]
    fn make_string() -> str_number;
    #[no_mangle]
    fn time(__timer: *mut time_t) -> time_t;
    #[no_mangle]
    fn localtime(__timer: *const time_t) -> *mut tm;
}
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __time_t = libc::c_long;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type str_number = int32_t;
pub type packed_UTF16_code = libc::c_ushort;
pub type UInt32 = libc::c_uint;
pub type pool_pointer = int32_t;
pub type UInt16 = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
/* texmfmp.c: Hand-coded routines for TeX or Metafont in C.  Originally
written by Tim Morgan, drawing from other Unix ports of TeX.  This is
a collection of miscellany, everything that's easier (or only
possible) to do in C.

This file is public domain.  */
/* For `struct tm'.  Moved here for Visual Studio 2005.  */
static mut last_source_name: *mut libc::c_char = 0 as *const libc::c_char as *mut libc::c_char;
static mut last_lineno: libc::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn get_date_and_time(
    mut minutes: *mut int32_t,
    mut day: *mut int32_t,
    mut month: *mut int32_t,
    mut year: *mut int32_t,
) {
    let mut tmptr: *mut tm = 0 as *mut tm; /* in the XeTeX case, this may be more than enough */
    let mut myclock: time_t = time(0 as *mut time_t);
    tmptr = localtime(&mut myclock);
    *minutes = (*tmptr).tm_hour * 60i32 + (*tmptr).tm_min;
    *day = (*tmptr).tm_mday;
    *month = (*tmptr).tm_mon + 1i32;
    *year = (*tmptr).tm_year + 1900i32;
}
unsafe extern "C" fn checkpool_pointer(mut pool_ptr_0: pool_pointer, mut len: size_t) {
    if (pool_ptr_0 as libc::c_ulong).wrapping_add(len) >= pool_size as libc::c_ulong {
        _tt_abort(
            b"string pool overflow [%i bytes]\x00" as *const u8 as *const libc::c_char,
            pool_size,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn maketexstring(mut s: *const libc::c_char) -> libc::c_int {
    let mut len: size_t = 0;
    let mut rval: UInt32 = 0;
    let mut cp: *const libc::c_uchar = s as *const libc::c_uchar;
    if s.is_null() || *s as libc::c_int == 0i32 {
        return (65536 + 1i32 as libc::c_long) as libc::c_int;
    }
    len = strlen(s);
    checkpool_pointer(pool_ptr, len);
    loop {
        let fresh0 = cp;
        cp = cp.offset(1);
        rval = *fresh0 as UInt32;
        if !(rval != 0i32 as libc::c_uint) {
            break;
        }
        let mut extraBytes: UInt16 = bytesFromUTF8[rval as usize] as UInt16;
        let mut current_block_19: u64;
        match extraBytes as libc::c_int {
            5 => {
                /* note: code falls through cases! */
                rval <<= 6i32; /* max UTF16->UTF8 expansion
                               (code units, not bytes) */
                if *cp != 0 {
                    let fresh1 = cp;
                    cp = cp.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh1 as libc::c_uint) as UInt32
                        as UInt32
                }
                current_block_19 = 15420705083065539194;
            }
            4 => {
                current_block_19 = 15420705083065539194;
            }
            3 => {
                current_block_19 = 17593909170536150684;
            }
            2 => {
                current_block_19 = 9565569445570550704;
            }
            1 => {
                current_block_19 = 4209676304665092873;
            }
            0 | _ => {
                current_block_19 = 11194104282611034094;
            }
        }
        match current_block_19 {
            15420705083065539194 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh2 = cp;
                    cp = cp.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh2 as libc::c_uint) as UInt32
                        as UInt32
                }
                current_block_19 = 17593909170536150684;
            }
            _ => {}
        }
        match current_block_19 {
            17593909170536150684 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh3 = cp;
                    cp = cp.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh3 as libc::c_uint) as UInt32
                        as UInt32
                }
                current_block_19 = 9565569445570550704;
            }
            _ => {}
        }
        match current_block_19 {
            9565569445570550704 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh4 = cp;
                    cp = cp.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh4 as libc::c_uint) as UInt32
                        as UInt32
                }
                current_block_19 = 4209676304665092873;
            }
            _ => {}
        }
        match current_block_19 {
            4209676304665092873 => {
                rval <<= 6i32;
                if *cp != 0 {
                    let fresh5 = cp;
                    cp = cp.offset(1);
                    rval = (rval as libc::c_uint).wrapping_add(*fresh5 as libc::c_uint) as UInt32
                        as UInt32
                }
            }
            _ => {}
        }
        rval = (rval as libc::c_uint).wrapping_sub(offsetsFromUTF8[extraBytes as usize]) as UInt32
            as UInt32;
        if rval > 0xffffi32 as libc::c_uint {
            rval =
                (rval as libc::c_uint).wrapping_sub(0x10000i32 as libc::c_uint) as UInt32 as UInt32;
            let fresh6 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh6 as isize) = (0xd800i32 as libc::c_uint)
                .wrapping_add(rval.wrapping_div(0x400i32 as libc::c_uint))
                as packed_UTF16_code;
            let fresh7 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh7 as isize) = (0xdc00i32 as libc::c_uint)
                .wrapping_add(rval.wrapping_rem(0x400i32 as libc::c_uint))
                as packed_UTF16_code
        } else {
            let fresh8 = pool_ptr;
            pool_ptr = pool_ptr + 1;
            *str_pool.offset(fresh8 as isize) = rval as packed_UTF16_code
        }
    }
    return make_string();
}
#[no_mangle]
pub unsafe extern "C" fn gettexstring(mut s: str_number) -> *mut libc::c_char {
    let mut bytesToWrite: libc::c_uint = 0i32 as libc::c_uint;
    let mut len: pool_pointer = 0;
    let mut i: pool_pointer = 0;
    let mut j: pool_pointer = 0;
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    if s as libc::c_long >= 65536 {
        len = *str_start.offset(((s + 1i32) as libc::c_long - 65536) as isize)
            - *str_start.offset((s as libc::c_long - 65536) as isize)
    } else {
        len = 0i32
    }
    name = xmalloc((len * 3i32 + 1i32) as size_t) as *mut libc::c_char;
    i = 0i32;
    j = 0i32;
    while i < len {
        let mut c: uint32_t = *str_pool
            .offset((i + *str_start.offset((s as libc::c_long - 65536) as isize)) as isize)
            as uint32_t;
        if c >= 0xd800i32 as libc::c_uint && c <= 0xdbffi32 as libc::c_uint {
            i += 1;
            let mut lo: uint32_t = *str_pool
                .offset((i + *str_start.offset((s as libc::c_long - 65536) as isize)) as isize)
                as uint32_t;
            if lo >= 0xdc00i32 as libc::c_uint && lo <= 0xdfffi32 as libc::c_uint {
                c = c
                    .wrapping_sub(0xd800i32 as libc::c_uint)
                    .wrapping_mul(0x400i32 as libc::c_uint)
                    .wrapping_add(lo)
                    .wrapping_sub(0xdc00i32 as libc::c_uint)
                    .wrapping_add(0x10000i32 as libc::c_uint)
            } else {
                c = 0xfffdi32 as uint32_t
            }
        }
        if c < 0x80i32 as libc::c_uint {
            bytesToWrite = 1i32 as libc::c_uint
        } else if c < 0x800i32 as libc::c_uint {
            bytesToWrite = 2i32 as libc::c_uint
        } else if c < 0x10000i32 as libc::c_uint {
            bytesToWrite = 3i32 as libc::c_uint
        } else if c < 0x110000i32 as libc::c_uint {
            bytesToWrite = 4i32 as libc::c_uint
        } else {
            bytesToWrite = 3i32 as libc::c_uint;
            c = 0xfffdi32 as uint32_t
        }
        j = (j as libc::c_uint).wrapping_add(bytesToWrite) as pool_pointer as pool_pointer;
        let mut current_block_28: u64;
        match bytesToWrite {
            4 => {
                /* note: everything falls through. */
                j -= 1;
                *name.offset(j as isize) =
                    ((c | 0x80i32 as libc::c_uint) & 0xbfi32 as libc::c_uint) as libc::c_char;
                c >>= 6i32;
                current_block_28 = 9281751456159701257;
            }
            3 => {
                current_block_28 = 9281751456159701257;
            }
            2 => {
                current_block_28 = 13645261163415976511;
            }
            1 => {
                current_block_28 = 4925739576308592327;
            }
            _ => {
                current_block_28 = 2891135413264362348;
            }
        }
        match current_block_28 {
            9281751456159701257 => {
                j -= 1;
                *name.offset(j as isize) =
                    ((c | 0x80i32 as libc::c_uint) & 0xbfi32 as libc::c_uint) as libc::c_char;
                c >>= 6i32;
                current_block_28 = 13645261163415976511;
            }
            _ => {}
        }
        match current_block_28 {
            13645261163415976511 => {
                j -= 1;
                *name.offset(j as isize) =
                    ((c | 0x80i32 as libc::c_uint) & 0xbfi32 as libc::c_uint) as libc::c_char;
                c >>= 6i32;
                current_block_28 = 4925739576308592327;
            }
            _ => {}
        }
        match current_block_28 {
            4925739576308592327 => {
                j -= 1;
                *name.offset(j as isize) =
                    (c | firstByteMark[bytesToWrite as usize] as libc::c_uint) as libc::c_char
            }
            _ => {}
        }
        j = (j as libc::c_uint).wrapping_add(bytesToWrite) as pool_pointer as pool_pointer;
        i += 1
    }
    *name.offset(j as isize) = 0i32 as libc::c_char;
    return name;
}
unsafe extern "C" fn compare_paths(
    mut p1: *const libc::c_char,
    mut p2: *const libc::c_char,
) -> libc::c_int {
    let mut ret: libc::c_int = 0;
    loop {
        ret = *p1 as libc::c_int - *p2 as libc::c_int;
        if !(ret == 0i32 && *p2 as libc::c_int != 0i32
            || *p1 as libc::c_int == '/' as i32 && *p2 as libc::c_int == '/' as i32)
        {
            break;
        }
        p1 = p1.offset(1);
        p2 = p2.offset(1)
    }
    ret = if ret < 0i32 {
        -1i32
    } else if ret > 0i32 {
        1i32
    } else {
        0i32
    };
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn is_new_source(
    mut srcfilename: str_number,
    mut lineno: libc::c_int,
) -> bool {
    let mut name: *mut libc::c_char = gettexstring(srcfilename);
    return compare_paths(name, last_source_name) != 0i32 || lineno != last_lineno;
}
#[no_mangle]
pub unsafe extern "C" fn remember_source_info(
    mut srcfilename: str_number,
    mut lineno: libc::c_int,
) {
    free(last_source_name as *mut libc::c_void);
    last_source_name = gettexstring(srcfilename);
    last_lineno = lineno;
}
#[no_mangle]
pub unsafe extern "C" fn make_src_special(
    mut srcfilename: str_number,
    mut lineno: libc::c_int,
) -> pool_pointer {
    let mut oldpool_ptr: pool_pointer = pool_ptr;
    let mut filename: *mut libc::c_char = gettexstring(srcfilename);
    /* FIXME: Magic number. */
    let mut buf: [libc::c_char; 40] = [0; 40];
    let mut s: *mut libc::c_char = buf.as_mut_ptr();
    /* Always put a space after the number, which makes things easier
     * to parse.
     */
    sprintf(
        buf.as_mut_ptr(),
        b"src:%d \x00" as *const u8 as *const libc::c_char,
        lineno,
    );
    if (pool_ptr as libc::c_ulong)
        .wrapping_add(strlen(buf.as_mut_ptr()))
        .wrapping_add(strlen(filename))
        >= pool_size as size_t
    {
        _tt_abort(b"string pool overflow\x00" as *const u8 as *const libc::c_char);
    }
    s = buf.as_mut_ptr();
    while *s != 0 {
        let fresh9 = s;
        s = s.offset(1);
        let fresh10 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh10 as isize) = *fresh9 as packed_UTF16_code
    }
    s = filename;
    while *s != 0 {
        let fresh11 = s;
        s = s.offset(1);
        let fresh12 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh12 as isize) = *fresh11 as packed_UTF16_code
    }
    return oldpool_ptr;
}
/* Converts any given string in into an allowed PDF string which is
 * hexadecimal encoded;
 * sizeof(out) should be at least lin*2+1.
 */
unsafe extern "C" fn convertStringToHexString(
    mut in_0: *const libc::c_char,
    mut out: *mut libc::c_char,
    mut lin: libc::c_int,
) {
    static mut hexchars: [libc::c_char; 17] = [
        48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 65, 66, 67, 68, 69, 70, 0,
    ];
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    j = 0i32;
    i = 0i32;
    while i < lin {
        let mut c: libc::c_uchar = *in_0.offset(i as isize) as libc::c_uchar;
        let fresh13 = j;
        j = j + 1;
        *out.offset(fresh13 as isize) = hexchars[(c as libc::c_int >> 4i32 & 0xfi32) as usize];
        let fresh14 = j;
        j = j + 1;
        *out.offset(fresh14 as isize) = hexchars[(c as libc::c_int & 0xfi32) as usize];
        i += 1
    }
    *out.offset(j as isize) = '\u{0}' as i32 as libc::c_char;
}
/* Functions originating in texmfmp.c */
#[no_mangle]
pub unsafe extern "C" fn getmd5sum(mut s: str_number, mut file: bool) {
    let mut digest: [libc::c_char; 16] = [0; 16];
    let mut outbuf: [libc::c_char; 33] = [0; 33];
    let mut xname: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut ret: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    xname = gettexstring(s);
    if file {
        ret = ttstub_get_file_md5(xname, digest.as_mut_ptr())
    } else {
        ret = ttstub_get_data_md5(xname, strlen(xname), digest.as_mut_ptr())
    }
    free(xname as *mut libc::c_void);
    if ret != 0 {
        return;
    }
    if pool_ptr + 2i32 * 16i32 >= pool_size {
        /* error by str_toks that calls str_room(1) */
        return;
    }
    convertStringToHexString(digest.as_mut_ptr(), outbuf.as_mut_ptr(), 16i32);
    i = 0i32;
    while i < 2i32 * 16i32 {
        let fresh15 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        *str_pool.offset(fresh15 as isize) = outbuf[i as usize] as uint16_t;
        i += 1
    }
}