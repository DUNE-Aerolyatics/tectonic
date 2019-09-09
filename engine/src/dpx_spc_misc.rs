#![allow(dead_code,
         mutable_transmutes,
         non_camel_case_types,
         non_snake_case,
         non_upper_case_globals,
         unused_assignments,
         unused_mut)]

extern crate libc;
extern "C" {
    pub type pdf_obj;
    #[no_mangle]
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    #[no_mangle]
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    #[no_mangle]
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn strncmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong) -> libc::c_int;
    #[no_mangle]
    fn ttstub_input_open(
        path: *const libc::c_char,
        format: tt_input_format_type,
        is_gz: libc::c_int,
    ) -> rust_input_handle_t;
    #[no_mangle]
    fn ttstub_input_close(handle: rust_input_handle_t) -> libc::c_int;
    #[no_mangle]
    fn spc_warn(spe: *mut spc_env, fmt: *const libc::c_char, _: ...);
    #[no_mangle]
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /* Tectonic-enabled versions */
    #[no_mangle]
    fn tt_mfgets(
        buffer: *mut libc::c_char,
        length: libc::c_int,
        file: rust_input_handle_t,
    ) -> *mut libc::c_char;
    #[no_mangle]
    fn pdf_dev_put_image(
        xobj_id: libc::c_int,
        p: *mut transform_info,
        ref_x: libc::c_double,
        ref_y: libc::c_double,
    ) -> libc::c_int;
    /* Please use different interface than findresource...
     * This is not intended to be used for specifying page number and others.
     * Only pdf:image special in spc_pdfm.c want optinal dict!
     */
    #[no_mangle]
    fn pdf_ximage_findresource(ident: *const libc::c_char, options: load_options) -> libc::c_int;
    #[no_mangle]
    fn mps_scan_bbox(
        pp: *mut *const libc::c_char,
        endptr: *const libc::c_char,
        bbox: *mut pdf_rect,
    ) -> libc::c_int;
    #[no_mangle]
    fn transform_info_clear(info: *mut transform_info);
    #[no_mangle]
    fn skip_white(start: *mut *const libc::c_char, end: *const libc::c_char);
}
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
pub type size_t = libc::c_ulong;
/* The weird enum values are historical and could be rationalized. But it is
 * good to write them explicitly since they must be kept in sync with
 * `src/engines/mod.rs`.
 */
pub type tt_input_format_type = libc::c_uint;
pub const TTIF_TECTONIC_PRIMARY: tt_input_format_type = 59;
pub const TTIF_OPENTYPE: tt_input_format_type = 47;
pub const TTIF_SFD: tt_input_format_type = 46;
pub const TTIF_CMAP: tt_input_format_type = 45;
pub const TTIF_ENC: tt_input_format_type = 44;
pub const TTIF_MISCFONTS: tt_input_format_type = 41;
pub const TTIF_BINARY: tt_input_format_type = 40;
pub const TTIF_TRUETYPE: tt_input_format_type = 36;
pub const TTIF_VF: tt_input_format_type = 33;
pub const TTIF_TYPE1: tt_input_format_type = 32;
pub const TTIF_TEX_PS_HEADER: tt_input_format_type = 30;
pub const TTIF_TEX: tt_input_format_type = 26;
pub const TTIF_PICT: tt_input_format_type = 25;
pub const TTIF_OVF: tt_input_format_type = 23;
pub const TTIF_OFM: tt_input_format_type = 20;
pub const TTIF_FONTMAP: tt_input_format_type = 11;
pub const TTIF_FORMAT: tt_input_format_type = 10;
pub const TTIF_CNF: tt_input_format_type = 8;
pub const TTIF_BST: tt_input_format_type = 7;
pub const TTIF_BIB: tt_input_format_type = 6;
pub const TTIF_AFM: tt_input_format_type = 4;
pub const TTIF_TFM: tt_input_format_type = 3;
pub type rust_input_handle_t = *mut libc::c_void;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_env {
    pub x_user: libc::c_double,
    pub y_user: libc::c_double,
    pub mag: libc::c_double,
    pub pg: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_arg {
    pub curptr: *const libc::c_char,
    pub endptr: *const libc::c_char,
    pub base: *const libc::c_char,
    pub command: *const libc::c_char,
}
pub type spc_handler_fn_ptr =
    Option<unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct spc_handler {
    pub key: *const libc::c_char,
    pub exec: spc_handler_fn_ptr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct transform_info {
    pub width: libc::c_double,
    pub height: libc::c_double,
    pub depth: libc::c_double,
    pub matrix: pdf_tmatrix,
    pub bbox: pdf_rect,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_rect {
    pub llx: libc::c_double,
    pub lly: libc::c_double,
    pub urx: libc::c_double,
    pub ury: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pdf_tmatrix {
    pub a: libc::c_double,
    pub b: libc::c_double,
    pub c: libc::c_double,
    pub d: libc::c_double,
    pub e: libc::c_double,
    pub f: libc::c_double,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct load_options {
    pub page_no: libc::c_int,
    pub bbox_type: libc::c_int,
    pub dict: *mut pdf_obj,
}
/* quasi-hack to get the primary input */
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

   Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
   the dvipdfmx project team.

   Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

   This program is free software; you can redistribute it and/or modify
   it under the terms of the GNU General Public License as published by
   the Free Software Foundation; either version 2 of the License, or
   (at your option) any later version.

   This program is distributed in the hope that it will be useful,
   but WITHOUT ANY WARRANTY; without even the implied warranty of
   MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
   GNU General Public License for more details.

   You should have received a copy of the GNU General Public License
   along with this program; if not, write to the Free Software
   Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
unsafe extern "C" fn spc_handler_postscriptbox(
    mut spe: *mut spc_env,
    mut ap: *mut spc_arg,
) -> libc::c_int {
    let mut form_id: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut ti: transform_info = transform_info {
        width: 0.,
        height: 0.,
        depth: 0.,
        matrix: pdf_tmatrix {
            a: 0.,
            b: 0.,
            c: 0.,
            d: 0.,
            e: 0.,
            f: 0.,
        },
        bbox: pdf_rect {
            llx: 0.,
            lly: 0.,
            urx: 0.,
            ury: 0.,
        },
        flags: 0,
    };
    let mut options: load_options = {
        let mut init = load_options {
            page_no: 1i32,
            bbox_type: 0i32,
            dict: 0 as *mut pdf_obj,
        };
        init
    };
    let mut filename: [libc::c_char; 256] = [0; 256];
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut handle: rust_input_handle_t = 0 as *mut libc::c_void;
    if !spe.is_null() && !ap.is_null() {
    } else {
        __assert_fail(
            b"spe && ap\x00" as *const u8 as *const libc::c_char,
            b"dpx-spc_misc.c\x00" as *const u8 as *const libc::c_char,
            51i32 as libc::c_uint,
            (*::std::mem::transmute::<&[u8; 66], &[libc::c_char; 66]>(
                b"int spc_handler_postscriptbox(struct spc_env *, struct spc_arg *)\x00",
            ))
            .as_ptr(),
        );
    }
    if (*ap).curptr >= (*ap).endptr {
        spc_warn(
            spe,
            b"No width/height/filename given for postscriptbox special.\x00" as *const u8
                as *const libc::c_char,
        );
        return -1i32;
    }
    /* input is not NULL terminated */
    len = (*ap).endptr.wrapping_offset_from((*ap).curptr) as libc::c_long as libc::c_int;
    len = if 511i32 < len { 511i32 } else { len };
    memcpy(
        buf.as_mut_ptr() as *mut libc::c_void,
        (*ap).curptr as *const libc::c_void,
        len as libc::c_ulong,
    );
    buf[len as usize] = '\u{0}' as i32 as libc::c_char;
    transform_info_clear(&mut ti);
    spc_warn(
        spe,
        b"%s\x00" as *const u8 as *const libc::c_char,
        buf.as_mut_ptr(),
    );
    if sscanf(
        buf.as_mut_ptr(),
        b"{%lfpt}{%lfpt}{%255[^}]}\x00" as *const u8 as *const libc::c_char,
        &mut ti.width as *mut libc::c_double,
        &mut ti.height as *mut libc::c_double,
        filename.as_mut_ptr(),
    ) != 3i32
    {
        spc_warn(
            spe,
            b"Syntax error in postscriptbox special?\x00" as *const u8 as *const libc::c_char,
        );
        return -1i32;
    }
    (*ap).curptr = (*ap).endptr;
    ti.width *= 72.0f64 / 72.27f64;
    ti.height *= 72.0f64 / 72.27f64;
    handle = ttstub_input_open(filename.as_mut_ptr(), TTIF_PICT, 0i32);
    if handle.is_null() {
        spc_warn(
            spe,
            b"Could not open image file: %s\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return -1i32;
    }
    ti.flags |= 1i32 << 1i32 | 1i32 << 2i32;
    loop {
        let mut p: *const libc::c_char = tt_mfgets(buf.as_mut_ptr(), 512i32, handle);
        if p.is_null() {
            break;
        }
        if !(mps_scan_bbox(&mut p, p.offset(strlen(p) as isize), &mut ti.bbox) >= 0i32) {
            continue;
        }
        ti.flags |= 1i32 << 0i32;
        break;
    }
    ttstub_input_close(handle);
    form_id = pdf_ximage_findresource(filename.as_mut_ptr(), options);
    if form_id < 0i32 {
        spc_warn(
            spe,
            b"Failed to load image file: %s\x00" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        return -1i32;
    }
    pdf_dev_put_image(form_id, &mut ti, (*spe).x_user, (*spe).y_user);
    return 0i32;
}
unsafe extern "C" fn spc_handler_null(
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    (*args).curptr = (*args).endptr;
    return 0i32;
}
static mut misc_handlers: [spc_handler; 6] = unsafe {
    [
        {
            let mut init = spc_handler {
                key: b"postscriptbox\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_postscriptbox
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"landscape\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"papersize\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"src:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"pos:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
        {
            let mut init = spc_handler {
                key: b"om:\x00" as *const u8 as *const libc::c_char,
                exec: Some(
                    spc_handler_null
                        as unsafe extern "C" fn(_: *mut spc_env, _: *mut spc_arg) -> libc::c_int,
                ),
            };
            init
        },
    ]
};
#[no_mangle]
pub unsafe extern "C" fn spc_misc_check_special(
    mut buffer: *const libc::c_char,
    mut size: libc::c_int,
) -> bool {
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut endptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut i: size_t = 0;
    p = buffer;
    endptr = p.offset(size as isize);
    skip_white(&mut p, endptr);
    size = endptr.wrapping_offset_from(p) as libc::c_long as libc::c_int;
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as libc::c_ulong)
    {
        if size as libc::c_ulong >= strlen(misc_handlers[i as usize].key)
            && strncmp(
                p,
                misc_handlers[i as usize].key,
                strlen(misc_handlers[i as usize].key),
            ) == 0
        {
            return 1i32 != 0;
        }
        i = i.wrapping_add(1)
    }
    return 0i32 != 0;
}
/* This is dvipdfmx, an eXtended version of dvipdfm by Mark A. Wicks.

    Copyright (C) 2002-2016 by Jin-Hwan Cho and Shunsaku Hirata,
    the dvipdfmx project team.

    Copyright (C) 1998, 1999 by Mark A. Wicks <mwicks@kettering.edu>

    This program is free software; you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation; either version 2 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program; if not, write to the Free Software
    Foundation, Inc., 59 Temple Place, Suite 330, Boston, MA 02111-1307 USA.
*/
#[no_mangle]
pub unsafe extern "C" fn spc_misc_setup_handler(
    mut handle: *mut spc_handler,
    mut spe: *mut spc_env,
    mut args: *mut spc_arg,
) -> libc::c_int {
    let mut key: *const libc::c_char = 0 as *const libc::c_char;
    let mut keylen: libc::c_int = 0;
    let mut i: size_t = 0;
    if !handle.is_null() && !spe.is_null() && !args.is_null() {
    } else {
        __assert_fail(b"handle && spe && args\x00" as *const u8 as
                          *const libc::c_char,
                      b"dpx-spc_misc.c\x00" as *const u8 as
                          *const libc::c_char, 156i32 as libc::c_uint,
                      (*::std::mem::transmute::<&[u8; 85],
                                                &[libc::c_char; 85]>(b"int spc_misc_setup_handler(struct spc_handler *, struct spc_env *, struct spc_arg *)\x00")).as_ptr());
    }
    skip_white(&mut (*args).curptr, (*args).endptr);
    key = (*args).curptr;
    while (*args).curptr < (*args).endptr
        && *(*__ctype_b_loc())
            .offset(*(*args).curptr.offset(0) as libc::c_uchar as libc::c_int as isize)
            as libc::c_int
            & _ISalpha as libc::c_int as libc::c_ushort as libc::c_int
            != 0
    {
        (*args).curptr = (*args).curptr.offset(1)
    }
    if (*args).curptr < (*args).endptr && *(*args).curptr.offset(0) as libc::c_int == ':' as i32 {
        (*args).curptr = (*args).curptr.offset(1)
    }
    keylen = (*args).curptr.wrapping_offset_from(key) as libc::c_long as libc::c_int;
    if keylen < 1i32 {
        return -1i32;
    }
    i = 0i32 as size_t;
    while i
        < (::std::mem::size_of::<[spc_handler; 6]>() as libc::c_ulong)
            .wrapping_div(::std::mem::size_of::<spc_handler>() as libc::c_ulong)
    {
        if keylen as libc::c_ulong == strlen(misc_handlers[i as usize].key)
            && strncmp(key, misc_handlers[i as usize].key, keylen as libc::c_ulong) == 0
        {
            skip_white(&mut (*args).curptr, (*args).endptr);
            (*args).command = misc_handlers[i as usize].key;
            (*handle).key = b"???:\x00" as *const u8 as *const libc::c_char;
            (*handle).exec = misc_handlers[i as usize].exec;
            return 0i32;
        }
        i = i.wrapping_add(1)
    }
    return -1i32;
}