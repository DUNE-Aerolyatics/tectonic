#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use bridge::DisplayExt;
use std::convert::TryFrom;
use std::ffi::CStr;
use std::io::Write;
use std::ptr;

use super::xetex_ini::Selector;
use super::xetex_io::{
    bytesFromUTF8, make_utf16_name, name_of_input_file, offsetsFromUTF8, tt_xetex_open_input,
    u_open_in,
};
use crate::core_memory::{mfree, xmalloc_array, xrealloc};
#[cfg(target_os = "macos")]
use crate::xetex_aatfont as aat;
use crate::xetex_consts::*;
use crate::xetex_errors::{confusion, error, fatal_error, overflow, pdf_error};
use crate::xetex_ext::{
    apply_mapping, apply_tfm_font_mapping, check_for_tfm_font_mapping, find_native_font,
    get_encoding_mode_and_info, get_font_char_range, get_glyph_bounds,
    get_native_char_height_depth, get_native_char_sidebearings, getnativechardp, getnativecharht,
    getnativecharic, getnativecharwd, gr_font_get_named, gr_font_get_named_1, gr_print_font_name,
    linebreak_next, linebreak_start, load_tfm_font_mapping, map_char_to_glyph, map_glyph_to_index,
    measure_native_glyph, measure_native_node, ot_font_get, ot_font_get_1, ot_font_get_2,
    ot_font_get_3, ot_get_font_metrics, print_glyph_name, print_utf8_str,
    real_get_native_glyph_italic_correction, real_get_native_italic_correction,
    real_get_native_word_cp, release_font_engine, AAT_FONT_FLAG, OTGR_FONT_FLAG,
};
use crate::xetex_ini::{
    _xeq_level_array, active_width, adjust_tail, after_token, align_ptr, align_state,
    area_delimiter, arith_error, avail, bchar, best_height_plus_depth, breadth_max,
    cancel_boundary, cond_ptr, cur_align, cur_area, cur_boundary, cur_box, cur_chr, cur_cmd,
    cur_cs, cur_dir, cur_ext, cur_group, cur_head, cur_if, cur_input, cur_l, cur_lang, cur_level,
    cur_list, cur_loop, cur_mark, cur_name, cur_order, cur_pre_head, cur_pre_tail, cur_ptr, cur_q,
    cur_r, cur_span, cur_tail, cur_tok, cur_val, cur_val1, cur_val_level, dead_cycles, def_ref,
    deletions_allowed, depth_threshold, dig, disc_ptr, empty, error_count, error_line,
    expand_depth, expand_depth_count, ext_delimiter, false_bchar, file_line_error_style_p,
    file_name_quote_char, file_offset, first, first_count, fmem_ptr, font_in_short_display,
    font_used, force_eof, gave_char_warning_help, half_error_line, hash, hash_extra, hash_high,
    hash_used, help_line, help_ptr, hi_mem_min, history, if_limit, if_line, init_pool_ptr,
    init_str_ptr, ins_disc, insert_penalties, insert_src_special_auto,
    insert_src_special_every_par, insert_src_special_every_vbox, interaction, is_hyph,
    is_in_csname, job_name, last, last_badness, last_glue, last_kern, last_leftmost_char,
    last_node_type, last_penalty, last_rightmost_char, lft_hit, lig_stack, ligature_present, line,
    lo_mem_max, loaded_font_design_size, loaded_font_flags, loaded_font_letter_space,
    loaded_font_mapping, log_file, log_opened, long_help_seen, long_state, mag_set, main_f, main_h,
    main_i, main_j, main_k, main_p, main_pp, main_ppp, main_s, mapped_text, max_buf_stack,
    max_print_line, max_reg_help_line, max_reg_num, max_strings, mem_end, name_in_progress,
    name_length, name_length16, name_of_file, name_of_file16, native_font_type_flag, native_len,
    native_text, native_text_size, no_new_control_sequence, old_setting, open_parens,
    output_active, pack_begin_line, page_contents, page_so_far, page_tail, par_loc, par_token,
    pdf_last_x_pos, pdf_last_y_pos, pool_ptr, pool_size, pre_adjust_tail, prev_class, prim,
    prim_eqtb, prim_used, pseudo_files, pstack, quoted_filename, radix, read_file, read_open,
    rover, rt_hit, rust_stdout, sa_chain, sa_level, sa_null, sa_root, save_native_len,
    scanner_status, selector, set_box_allowed, shown_mode, skip_line, space_class, stop_at_space,
    str_pool, str_ptr, str_start, tally, temp_ptr, term_offset, tex_remainder, texmf_log_name,
    total_shrink, total_stretch, trick_buf, trick_count, use_err_help, used_tectonic_coda_tokens,
    warning_index, write_file, write_open, xtx_ligature_present, LR_problems, LR_ptr, BASE_PTR,
    BCHAR_LABEL, BUFFER, BUF_SIZE, CHAR_BASE, DEPTH_BASE, EOF_SEEN, EQTB, EQTB_TOP, EXTEN_BASE,
    FONT_AREA, FONT_BC, FONT_BCHAR, FONT_CHECK, FONT_DSIZE, FONT_EC, FONT_FALSE_BCHAR, FONT_FLAGS,
    FONT_GLUE, FONT_INFO, FONT_LAYOUT_ENGINE, FONT_LETTER_SPACE, FONT_MAPPING, FONT_MAX,
    FONT_MEM_SIZE, FONT_NAME, FONT_PARAMS, FONT_PTR, FONT_SIZE, FULL_SOURCE_FILENAME_STACK,
    GRP_STACK, HEIGHT_BASE, HYPHEN_CHAR, IF_STACK, INPUT_FILE, INPUT_PTR, INPUT_STACK, IN_OPEN,
    ITALIC_BASE, KERN_BASE, LIG_KERN_BASE, LINE_STACK, MAX_IN_OPEN, MAX_IN_STACK, MAX_NEST_STACK,
    MAX_PARAM_STACK, MAX_SAVE_STACK, MEM, NEST, NEST_PTR, NEST_SIZE, PARAM_BASE, PARAM_PTR,
    PARAM_SIZE, PARAM_STACK, SAVE_PTR, SAVE_SIZE, SAVE_STACK, SKEW_CHAR, SOURCE_FILENAME_STACK,
    STACK_SIZE, WIDTH_BASE,
};
use crate::xetex_ini::{b16x4, b32x2, memory_word, prefixed_command};
use crate::xetex_io::{input_line, open_or_close_in, set_input_file_encoding, u_close};
use crate::xetex_layout_interface::*;
use crate::xetex_linebreak::line_break;
use crate::xetex_math::{
    after_math, append_choices, build_choices, fin_mlist, flush_math, init_math, math_ac,
    math_fraction, math_left_right, math_limit_switch, math_radical, resume_after_display,
    start_eq_no, sub_sup,
};
use crate::xetex_output::{
    print, print_char, print_cs, print_cstr, print_current_string, print_esc, print_esc_cstr,
    print_file_line, print_file_name, print_hex, print_int, print_ln, print_native_word, print_nl,
    print_nl_cstr, print_raw_char, print_roman_int, print_sa_num, print_scaled, print_size,
    print_write_whatsit, sprint_cs,
};
use crate::xetex_pagebuilder::build_page;
use crate::xetex_pic::{count_pdf_file_pages, load_picture};
use crate::xetex_scaledmath::{mult_and_add, round_xn_over_d, tex_round, x_over_n, xn_over_d};
use crate::xetex_shipout::{finalize_dvi_file, new_edge, out_what, ship_out};
use crate::xetex_stringpool::EMPTY_STRING;
use crate::xetex_stringpool::{
    append_str, length, make_string, search_string, slow_make_string, str_eq_buf, str_eq_str,
};
use crate::xetex_synctex::{synctex_start_input, synctex_terminate};
use crate::xetex_texmfmp::{
    getmd5sum, gettexstring, is_new_source, make_src_special, maketexstring, remember_source_info,
};
use crate::xetex_xetexd::*;
use bridge::{
    ttstub_input_close, ttstub_input_getc, ttstub_issue_warning, ttstub_output_close,
    ttstub_output_open, ttstub_output_putc,
};

use bridge::{TTHistory, TTInputFormat};

use libc::{free, memcpy, strcat, strcpy, strlen};

use bridge::InputHandleWrapper;
pub(crate) type scaled_t = i32;
pub(crate) type CFDictionaryRef = *mut libc::c_void;

pub(crate) type UTF16_code = u16;
pub(crate) type UTF8_code = u8;
pub(crate) type UnicodeScalar = i32;
pub(crate) type eight_bits = u8;
pub(crate) type pool_pointer = i32;
pub(crate) type str_number = i32;
pub(crate) type packed_UTF16_code = u16;
pub(crate) type small_number = i16;
pub(crate) type glue_ord = u8;
pub(crate) type group_code = u8;
pub(crate) type internal_font_number = usize;
pub(crate) type font_index = i32;
pub(crate) type nine_bits = i32;
pub(crate) type save_pointer = i32;

#[inline]
pub(crate) unsafe fn cur_length() -> pool_pointer {
    pool_ptr - str_start[(str_ptr - 65536) as usize]
}

fn IS_LC_HEX(c: i32) -> bool {
    (c >= ('0' as i32) && c <= ('9' as i32)) || (c >= ('a' as i32) && c <= ('f' as i32))
}

unsafe fn int_error(mut n: i32) {
    print_cstr(b" (");
    print_int(n);
    print_char(')' as i32);
    error();
}
pub(crate) unsafe fn badness(mut t: scaled_t, mut s: scaled_t) -> i32 {
    if t == 0 {
        return 0;
    }
    if s <= 0 {
        return INF_BAD;
    }
    let r;
    if t <= 7230584 {
        /* magic constant */
        r = (t * 297) / s
    } else if s >= 1663497 {
        /* magic constant */
        r = t / (s / 297)
    } else {
        r = t
    }
    if r > 1290 {
        /* magic constant */
        return INF_BAD;
    }
    (r * r * r + 0x20000i32) / 0x40000i32
}

/*:112*/
/*118:*/
pub(crate) unsafe fn show_token_list(mut p: i32, mut q: i32, mut l: i32) {
    let mut n: UTF16_code = 0;
    let mut match_chr = '#' as i32;
    n = '0' as UTF16_code;
    tally = 0;
    while !p.is_texnull() && tally < l {
        /*332:*/
        if p == q {
            first_count = tally;
            trick_count = tally + 1 + error_line - half_error_line;
            if trick_count < error_line {
                trick_count = error_line
            }
        }
        if p < hi_mem_min || p > mem_end {
            print_esc_cstr(b"CLOBBERED.");
            return;
        }
        if MEM[p as usize].b32.s0 >= CS_TOKEN_FLAG {
            print_cs(MEM[p as usize].b32.s0 - CS_TOKEN_FLAG);
        } else {
            let m = MEM[p as usize].b32.s0 / MAX_CHAR_VAL;
            let c = MEM[p as usize].b32.s0 % MAX_CHAR_VAL;
            if MEM[p as usize].b32.s0 < 0 {
                print_esc_cstr(b"BAD.");
            } else {
                /*306:*/
                match m as u16 {
                    LEFT_BRACE | RIGHT_BRACE | MATH_SHIFT | TAB_MARK | SUP_MARK | SUB_MARK
                    | SPACER | LETTER | OTHER_CHAR => print_char(c),
                    MAC_PARAM => {
                        print_char(c);
                        print_char(c);
                    }
                    OUT_PARAM => {
                        print_char(match_chr);
                        if c <= 9i32 {
                            print_char(c + 48i32);
                        } else {
                            print_char('!' as i32);
                            return;
                        }
                    }
                    MATCH => {
                        match_chr = c;
                        print_char(c);
                        n = n.wrapping_add(1);
                        print_char(n as i32);
                        if n as i32 > '9' as i32 {
                            return;
                        }
                    }
                    END_MATCH => {
                        if c == 0 {
                            print_cstr(b"->");
                        }
                    }
                    _ => print_esc_cstr(b"BAD."),
                }
            }
        }
        p = *LLIST_link(p as usize);
    }
    if !p.is_texnull() {
        print_esc_cstr(b"ETC.");
    };
}
pub(crate) unsafe fn runaway() {
    let mut p = TEX_NULL;
    if scanner_status as u16 > SKIPPING {
        match scanner_status as u16 {
            DEFINING => {
                print_nl_cstr(b"Runaway definition");
                p = def_ref as i32;
            }
            MATCHING => {
                print_nl_cstr(b"Runaway argument");
                p = TEMP_HEAD as i32;
            }
            ALIGNING => {
                print_nl_cstr(b"Runaway preamble");
                p = HOLD_HEAD as i32;
            }
            ABSORBING => {
                print_nl_cstr(b"Runaway text");
                p = def_ref as i32;
            }
            _ => {}
        }
        print_char('?' as i32);
        print_ln();
        show_token_list(MEM[p as usize].b32.s1, TEX_NULL, error_line - 10);
    };
}
pub(crate) unsafe fn get_avail() -> usize {
    let mut p = avail;
    if !p.is_texnull() {
        avail = *LLIST_link(avail as usize);
    } else if mem_end < MEM_TOP as i32 {
        mem_end += 1;
        p = mem_end;
    } else {
        hi_mem_min -= 1;
        p = hi_mem_min;
        if is_char_node(lo_mem_max) {
            runaway();
            overflow(b"main memory size", MEM_TOP + 1);
        }
    }
    let p = p as usize;
    MEM[p].b32.s1 = TEX_NULL;
    p
}
pub(crate) unsafe fn flush_list(p: Option<usize>) {
    if let Some(p) = p {
        let mut r = p;
        let mut q;
        loop {
            q = r;

            if let Some(r_) = MEM[r as usize].b32.s1.opt() {
                r = r_;
            } else {
                break;
            }
        }
        MEM[q].b32.s1 = avail;
        avail = p as i32;
    };
}
pub(crate) unsafe fn get_node(mut s: i32) -> usize {
    'restart: loop {
        let mut p = rover;
        loop {
            /*131: */
            let mut q = p + MEM[p as usize].b32.s0;
            while MEM[q as usize].b32.s1 == MAX_HALFWORD {
                let t = MEM[(q + 1) as usize].b32.s1;
                if q == rover {
                    rover = t
                }
                MEM[(t + 1) as usize].b32.s0 = MEM[(q + 1) as usize].b32.s0;
                MEM[(MEM[(q + 1) as usize].b32.s0 + 1) as usize].b32.s1 = t;
                q = q + MEM[q as usize].b32.s0
            }
            let r = q - s;
            if r > p + 1 {
                /*132: */
                MEM[p as usize].b32.s0 = r - p;
                rover = p;
                return found(r as usize, s as usize);
            }
            if r == p {
                if MEM[(p + 1) as usize].b32.s1 != p {
                    /*133: */
                    rover = MEM[(p + 1) as usize].b32.s1;
                    let t = MEM[(p + 1) as usize].b32.s0;
                    MEM[(rover + 1) as usize].b32.s0 = t;
                    MEM[(t + 1) as usize].b32.s1 = rover;
                    return found(r as usize, s as usize);
                }
            }
            MEM[p as usize].b32.s0 = q - p;
            p = MEM[(p + 1) as usize].b32.s1;
            if p == rover {
                break;
            }
        }
        if s == 0x40000000 {
            return MAX_HALFWORD as usize;
        }
        if lo_mem_max + 2 < hi_mem_min {
            if lo_mem_max + 2 <= MAX_HALFWORD {
                /*130: */
                let mut t = if hi_mem_min - lo_mem_max >= 1998 {
                    lo_mem_max + 1000
                } else {
                    lo_mem_max + 1 + (hi_mem_min - lo_mem_max) / 2
                };
                p = MEM[(rover + 1) as usize].b32.s0;
                let q = lo_mem_max;
                MEM[(p + 1) as usize].b32.s1 = q;
                MEM[(rover + 1) as usize].b32.s0 = q;
                if t > MAX_HALFWORD {
                    t = MAX_HALFWORD
                }
                MEM[(q + 1) as usize].b32.s1 = rover;
                MEM[(q + 1) as usize].b32.s0 = p;
                MEM[q as usize].b32.s1 = MAX_HALFWORD;
                MEM[q as usize].b32.s0 = t - lo_mem_max;
                lo_mem_max = t;
                MEM[lo_mem_max as usize].b32.s1 = TEX_NULL;
                MEM[lo_mem_max as usize].b32.s0 = TEX_NULL;
                rover = q;
                continue 'restart;
            }
        }
        break 'restart;
    }
    overflow(b"main memory size", MEM_TOP + 1);

    unsafe fn found(r: usize, s: usize) -> usize {
        MEM[r].b32.s1 = TEX_NULL;
        if s >= MEDIUM_NODE_SIZE as usize {
            MEM[(r + s - 1)].b32.s0 = cur_input.synctex_tag;
            MEM[(r + s - 1)].b32.s1 = line
        }
        r
    }
}
pub(crate) unsafe fn free_node(p: usize, mut s: i32) {
    MEM[p].b32.s0 = s;
    MEM[p].b32.s1 = MAX_HALFWORD;
    let q = MEM[(rover + 1) as usize].b32.s0;
    MEM[p + 1].b32.s0 = q;
    MEM[p + 1].b32.s1 = rover;
    MEM[(rover + 1) as usize].b32.s0 = p as i32;
    MEM[(q + 1) as usize].b32.s1 = p as i32;
}
pub(crate) unsafe fn new_null_box() -> usize {
    let p = get_node(BOX_NODE_SIZE);
    *NODE_type(p) = HLIST_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s1 = 0;
    MEM[p + 2].b32.s1 = 0;
    MEM[p + 3].b32.s1 = 0;
    MEM[p + 4].b32.s1 = 0;
    MEM[p + 5].b32.s1 = TEX_NULL;
    MEM[p + 5].b16.s1 = NORMAL as u16;
    MEM[p + 5].b16.s0 = NORMAL as u16;
    *BOX_glue_set(p) = 0.;
    p
}
pub(crate) unsafe fn new_rule() -> usize {
    let p = get_node(RULE_NODE_SIZE);
    *NODE_type(p) = RULE_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s1 = NULL_FLAG;
    MEM[p + 2].b32.s1 = NULL_FLAG;
    MEM[p + 3].b32.s1 = NULL_FLAG;
    p
}
pub(crate) unsafe fn new_ligature(mut f: internal_font_number, mut c: u16, mut q: i32) -> usize {
    let p = get_node(SMALL_NODE_SIZE);
    *NODE_type(p) = LIGATURE_NODE;
    MEM[p + 1].b16.s1 = f as u16;
    MEM[p + 1].b16.s0 = c;
    MEM[p + 1].b32.s1 = q;
    MEM[p].b16.s0 = 0;
    p
}
pub(crate) unsafe fn new_lig_item(mut c: u16) -> usize {
    let p = get_node(SMALL_NODE_SIZE);
    MEM[p].b16.s0 = c;
    MEM[p + 1].b32.s1 = TEX_NULL;
    p
}
pub(crate) unsafe fn new_disc() -> usize {
    let p = get_node(SMALL_NODE_SIZE);
    *NODE_type(p) = DISC_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s0 = TEX_NULL;
    MEM[p + 1].b32.s1 = TEX_NULL;
    p
}
pub(crate) unsafe fn copy_native_glyph_info(src: usize, dest: usize) {
    let mut glyph_count: i32 = 0;
    if !MEM[src + 5].ptr.is_null() {
        glyph_count = MEM[src + 4].b16.s0 as i32;
        MEM[dest + 5].ptr =
            xmalloc_array::<libc::c_char>(glyph_count as usize * NATIVE_GLYPH_INFO_SIZE as usize)
                as *mut _;
        memcpy(
            MEM[dest + 5].ptr,
            MEM[src + 5].ptr,
            (glyph_count * NATIVE_GLYPH_INFO_SIZE) as usize,
        );
        MEM[dest + 4].b16.s0 = glyph_count as u16
    };
}
pub(crate) unsafe fn new_math(mut w: scaled_t, mut s: small_number) -> usize {
    let p = get_node(MEDIUM_NODE_SIZE);
    *NODE_type(p) = MATH_NODE;
    MEM[p].b16.s0 = s as u16;
    MEM[p + 1].b32.s1 = w;
    p
}
pub(crate) unsafe fn new_spec(p: usize) -> usize {
    let q = get_node(GLUE_SPEC_SIZE);
    MEM[q] = MEM[p];
    MEM[q].b32.s1 = TEX_NULL;
    MEM[q + 1].b32.s1 = MEM[p + 1].b32.s1;
    MEM[q + 2].b32.s1 = MEM[p + 2].b32.s1;
    MEM[q + 3].b32.s1 = MEM[p + 3].b32.s1;
    q
}
pub(crate) unsafe fn new_param_glue(mut n: small_number) -> usize {
    let p = get_node(MEDIUM_NODE_SIZE);
    *NODE_type(p) = GLUE_NODE;
    MEM[p].b16.s0 = n as u16 + 1;
    MEM[p + 1].b32.s1 = TEX_NULL;
    let q = EQTB[GLUE_BASE + n as usize].b32.s1 as usize;
    MEM[p + 1].b32.s0 = q as i32;
    MEM[q].b32.s1 += 1;
    p
}
pub(crate) unsafe fn new_glue(mut q: i32) -> usize {
    let p = get_node(MEDIUM_NODE_SIZE);
    *NODE_type(p) = GLUE_NODE;
    *GLUE_SPEC_shrink_order(p) = NORMAL;
    MEM[p + 1].b32.s1 = TEX_NULL;
    MEM[p + 1].b32.s0 = q;
    *GLUE_SPEC_ref_count(q as usize) += 1;
    p
}
pub(crate) unsafe fn new_skip_param(mut n: small_number) -> usize {
    temp_ptr = new_spec(EQTB[(GLUE_BASE as i32 + n as i32) as usize].b32.s1 as usize) as i32; // 232
    let p = new_glue(temp_ptr);
    MEM[temp_ptr as usize].b32.s1 = TEX_NULL;
    MEM[p].b16.s0 = n as u16 + 1;
    p
}
pub(crate) unsafe fn new_kern(mut w: scaled_t) -> usize {
    let p = get_node(MEDIUM_NODE_SIZE);
    *NODE_type(p) = KERN_NODE;
    MEM[p].b16.s0 = NORMAL;
    MEM[p + 1].b32.s1 = w;
    p
}
pub(crate) unsafe fn new_penalty(mut m: i32) -> usize {
    let p = get_node(MEDIUM_NODE_SIZE);
    *NODE_type(p) = PENALTY_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s1 = m;
    p
}
/*:165*/
pub(crate) unsafe fn prev_rightmost(mut s: i32, mut e: i32) -> i32 {
    let mut p = s;
    if p.is_texnull() {
        return TEX_NULL;
    }
    while MEM[p as usize].b32.s1 != e {
        p = *LLIST_link(p as usize);
        if p.is_texnull() {
            return TEX_NULL;
        }
    }
    p
}
pub(crate) unsafe fn short_display(mut p: i32) {
    while p > 0 {
        if is_char_node(p) {
            if p <= mem_end {
                if MEM[p as usize].b16.s1 as i32 != font_in_short_display {
                    if MEM[p as usize].b16.s1 as i32 > FONT_MAX as i32 {
                        print_char('*' as i32);
                    } else {
                        /*279:*/
                        print_esc(
                            (*hash.offset(
                                (FONT_ID_BASE as i32 + MEM[p as usize].b16.s1 as i32) as isize,
                            ))
                            .s1,
                        );
                    }
                    print_char(' ' as i32);
                    font_in_short_display = MEM[p as usize].b16.s1 as i32
                }
                print(MEM[p as usize].b16.s0 as i32);
            }
        } else {
            /*183:*/
            match MEM[p as usize].b16.s1 {
                HLIST_NODE | VLIST_NODE | INS_NODE | MARK_NODE | ADJUST_NODE | UNSET_NODE => {
                    print_cstr(b"[]")
                }
                WHATSIT_NODE => match MEM[p as usize].b16.s0 {
                    NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                        if MEM[(p + 4) as usize].b16.s2 as i32 != font_in_short_display {
                            print_esc(
                                (*hash.offset(
                                    (FONT_ID_BASE as i32 + MEM[(p + 4) as usize].b16.s2 as i32)
                                        as isize,
                                ))
                                .s1,
                            );
                            print_char(' ' as i32);
                            font_in_short_display = MEM[(p + 4) as usize].b16.s2 as i32
                        }
                        print_native_word(p as usize);
                    }
                    _ => print_cstr(b"[]"),
                },
                RULE_NODE => print_char('|' as i32),
                GLUE_NODE => {
                    if MEM[(p + 1) as usize].b32.s0 != 0 {
                        print_char(' ' as i32);
                    }
                }
                MATH_NODE => {
                    if MEM[p as usize].b16.s0 >= L_CODE {
                        print_cstr(b"[]");
                    } else {
                        print_char('$' as i32);
                    }
                }
                LIGATURE_NODE => short_display(MEM[(p + 1) as usize].b32.s1),
                DISC_NODE => {
                    short_display(MEM[(p + 1) as usize].b32.s0);
                    short_display(MEM[(p + 1) as usize].b32.s1);
                    let mut n = MEM[p as usize].b16.s0 as i32;
                    while n > 0 {
                        if !MEM[p as usize].b32.s1.is_texnull() {
                            p = *LLIST_link(p as usize)
                        }
                        n -= 1;
                    }
                }
                _ => {}
            }
        }
        p = *LLIST_link(p as usize)
    }
}
pub(crate) unsafe fn print_font_and_char(mut p: i32) {
    if p > mem_end {
        print_esc_cstr(b"CLOBBERED.");
    } else {
        if MEM[p as usize].b16.s1 as i32 > FONT_MAX as i32 {
            print_char('*' as i32);
        } else {
            /*279: */
            print_esc(
                (*hash.offset((FONT_ID_BASE as i32 + MEM[p as usize].b16.s1 as i32) as isize)).s1,
            );
        }
        print_char(' ' as i32);
        print(MEM[p as usize].b16.s0 as i32);
    };
}
pub(crate) unsafe fn print_mark(mut p: i32) {
    print_char('{' as i32);
    if p < hi_mem_min || p > mem_end {
        print_esc_cstr(b"CLOBBERED.");
    } else {
        show_token_list(MEM[p as usize].b32.s1, TEX_NULL, max_print_line - 10);
    }
    print_char('}' as i32);
}
pub(crate) unsafe fn print_rule_dimen(mut d: scaled_t) {
    if d == NULL_FLAG {
        print_char('*' as i32);
    } else {
        print_scaled(d);
    };
}
pub(crate) unsafe fn print_glue(mut d: scaled_t, mut order: i32, s: &[u8]) {
    print_scaled(d);
    if order < NORMAL as i32 || order > FILLL {
        print_cstr(b"foul");
    } else if order > NORMAL as i32 {
        print_cstr(b"fil");
        while order > FIL {
            print_char('l' as i32);
            order -= 1
        }
    } else if !s.is_empty() {
        print_cstr(s);
    };
}
pub(crate) unsafe fn print_spec(p: i32, s: &[u8]) {
    if p < 0 || p >= lo_mem_max {
        print_char('*' as i32);
    } else {
        let p = p as usize;
        print_scaled(MEM[p + 1].b32.s1);
        if !s.is_empty() {
            print_cstr(s);
        }
        if MEM[p + 2].b32.s1 != 0 {
            print_cstr(b" plus ");
            print_glue(MEM[p + 2].b32.s1, MEM[p].b16.s1 as i32, s);
        }
        if MEM[p + 3].b32.s1 != 0 {
            print_cstr(b" minus ");
            print_glue(MEM[p + 3].b32.s1, MEM[p].b16.s0 as i32, s);
        }
    };
}
pub(crate) unsafe fn print_fam_and_char(p: usize) {
    let mut c: i32 = 0;
    print_esc_cstr(b"fam");
    print_int(MEM[p].b16.s1 as i32 % 256 % 256);
    print_char(' ' as i32);
    c = (MEM[p].b16.s0 as i64 + (MEM[p].b16.s1 as i32 / 256) as i64 * 65536) as i32;
    if (c as i64) < 65536 {
        print(c);
    } else {
        print_char(c);
    };
}
pub(crate) unsafe fn print_delimiter(p: usize) {
    let mut a: i32 = 0;
    a = ((MEM[p].b16.s3 as i32 % 256 * 256) as i64
        + (MEM[p].b16.s2 as i64 + (MEM[p].b16.s3 as i32 / 256) as i64 * 65536)) as i32;
    a = ((a * 4096i32 + MEM[p].b16.s1 as i32 % 256 * 256) as i64
        + (MEM[p].b16.s0 as i64 + (MEM[p].b16.s1 as i32 / 256) as i64 * 65536)) as i32;
    if a < 0i32 {
        print_int(a);
    } else {
        print_hex(a);
    };
}
pub(crate) unsafe fn print_subsidiary_data(p: usize, mut c: UTF16_code) {
    if cur_length() >= depth_threshold {
        if MEM[p as usize].b32.s1 != 0 {
            print_cstr(b" []");
        }
    } else {
        str_pool[pool_ptr as usize] = c;
        pool_ptr += 1;
        temp_ptr = p as i32;
        match MEM[p].b32.s1 {
            MATH_CHAR => {
                print_ln();
                print_current_string();
                print_fam_and_char(p);
            }
            SUB_BOX => show_info(),
            SUB_MLIST => {
                if MEM[p].b32.s0.is_texnull() {
                    print_ln();
                    print_current_string();
                    print_cstr(b"{}");
                } else {
                    show_info();
                }
            }
            _ => {}
        }
        pool_ptr -= 1
    };
}
pub(crate) unsafe fn print_style(mut c: i32) {
    match c / 2 {
        // DISPLAY_STYLE / 2
        0 => print_esc_cstr(b"displaystyle"),
        // TEXT_STYLE / 2
        1 => print_esc_cstr(b"textstyle"),
        // SCRIPT_STYLE / 2
        2 => print_esc_cstr(b"scriptstyle"),
        // SCRIPT_SCRIPT_STYLE / 2
        3 => print_esc_cstr(b"scriptscriptstyle"),
        _ => print_cstr(b"Unknown style!"),
    };
}
pub(crate) unsafe fn print_skip_param(n: GluePar) {
    use GluePar::*;
    match n {
        line_skip => print_esc_cstr(b"lineskip"),
        baseline_skip => print_esc_cstr(b"baselineskip"),
        par_skip => print_esc_cstr(b"parskip"),
        above_display_skip => print_esc_cstr(b"abovedisplayskip"),
        below_display_skip => print_esc_cstr(b"belowdisplayskip"),
        above_display_short_skip => print_esc_cstr(b"abovedisplayshortskip"),
        below_display_short_skip => print_esc_cstr(b"belowdisplayshortskip"),
        left_skip => print_esc_cstr(b"leftskip"),
        right_skip => print_esc_cstr(b"rightskip"),
        top_skip => print_esc_cstr(b"topskip"),
        split_top_skip => print_esc_cstr(b"splittopskip"),
        tab_skip => print_esc_cstr(b"tabskip"),
        space_skip => print_esc_cstr(b"spaceskip"),
        xspace_skip => print_esc_cstr(b"xspaceskip"),
        par_fill_skip => print_esc_cstr(b"parfillskip"),
        xetex_linebreak_skip => print_esc_cstr(b"XeTeXlinebreakskip"),
        thin_mu_skip => print_esc_cstr(b"thinmuskip"),
        med_mu_skip => print_esc_cstr(b"medmuskip"),
        thick_mu_skip => print_esc_cstr(b"thickmuskip"),
    };
}
pub(crate) unsafe fn show_node_list(mut p: i32) {
    let mut n: i32 = 0;
    let mut g: f64 = 0.;
    if cur_length() > depth_threshold {
        if p > TEX_NULL {
            print_cstr(b" []");
        }
        return;
    }
    n = 0;
    while p > 0 {
        print_ln();
        print_current_string();
        if p > mem_end {
            print_cstr(b"Bad link, display aborted.");
            return;
        }
        n += 1;
        if n > breadth_max {
            print_cstr(b"etc.");
            return;
        }
        if is_char_node(p) {
            print_font_and_char(p);
        } else {
            let p = p as usize;
            match MEM[p].b16.s1 {
                HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                    if MEM[p].b16.s1 == HLIST_NODE {
                        print_esc('h' as i32);
                    } else if MEM[p].b16.s1 == VLIST_NODE {
                        print_esc('v' as i32);
                    } else {
                        print_esc_cstr(b"unset");
                    }
                    print_cstr(b"box(");
                    print_scaled(MEM[p + 3].b32.s1);
                    print_char('+' as i32);
                    print_scaled(MEM[p + 2].b32.s1);
                    print_cstr(b")x");
                    print_scaled(MEM[p + 1].b32.s1);
                    if MEM[p].b16.s1 == UNSET_NODE {
                        /*193:*/
                        if MEM[p].b16.s0 != 0 {
                            print_cstr(b" (");
                            print_int(MEM[p].b16.s0 as i32 + 1);
                            print_cstr(b" columns)");
                        }
                        if MEM[p + 6].b32.s1 != 0 {
                            print_cstr(b", stretch ");
                            print_glue(MEM[p + 6].b32.s1, MEM[p + 5].b16.s0 as i32, b"");
                        }
                        if MEM[p + 4].b32.s1 != 0 {
                            print_cstr(b", shrink ");
                            print_glue(MEM[p + 4].b32.s1, MEM[p + 5].b16.s1 as i32, b"");
                        }
                    } else {
                        g = *BOX_glue_set(p);
                        if g != 0.0f64 && MEM[p + 5].b16.s1 != NORMAL {
                            print_cstr(b", glue set ");
                            if MEM[p + 5].b16.s1 == SHRINKING {
                                print_cstr(b"- ");
                            }
                            if g.abs() > 20000. {
                                if g > 0. {
                                    print_char('>' as i32);
                                } else {
                                    print_cstr(b"< -");
                                }
                                print_glue(
                                    (20000_i64 * 65536) as scaled_t,
                                    MEM[p + 5].b16.s0 as i32,
                                    b"",
                                );
                            } else {
                                print_glue(tex_round(65536_f64 * g), MEM[p + 5].b16.s0 as i32, b"");
                            }
                        }
                        if MEM[p + 4].b32.s1 != 0 {
                            print_cstr(b", shifted ");
                            print_scaled(MEM[p + 4].b32.s1);
                        }
                        /*1491:*/
                        if *NODE_type(p) == HLIST_NODE && MEM[p].b16.s0 == DLIST {
                            print_cstr(b", display");
                        }
                    }
                    str_pool[pool_ptr as usize] = '.' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 5].b32.s1);
                    pool_ptr -= 1
                }
                RULE_NODE => {
                    print_esc_cstr(b"rule(");
                    print_rule_dimen(MEM[p + 3].b32.s1);
                    print_char('+' as i32);
                    print_rule_dimen(MEM[p + 2].b32.s1);
                    print_cstr(b")x");
                    print_rule_dimen(MEM[p + 1].b32.s1);
                }
                INS_NODE => {
                    print_esc_cstr(b"insert");
                    print_int(MEM[p].b16.s0 as i32);
                    print_cstr(b", natural size ");
                    print_scaled(MEM[p + 3].b32.s1);
                    print_cstr(b"; split(");
                    print_spec(MEM[p + 4].b32.s1, b"");
                    print_char(',' as i32);
                    print_scaled(MEM[p + 2].b32.s1);
                    print_cstr(b"); float cost ");
                    print_int(MEM[p + 1].b32.s1);
                    str_pool[pool_ptr as usize] = '.' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 4].b32.s0);
                    pool_ptr -= 1
                }
                WHATSIT_NODE => match MEM[p].b16.s0 {
                    OPEN_NODE => {
                        print_write_whatsit(b"openout", p);
                        print_char('=' as i32);
                        print_file_name(MEM[p + 1].b32.s1, MEM[p + 2].b32.s0, MEM[p + 2].b32.s1);
                    }
                    WRITE_NODE => {
                        print_write_whatsit(b"write", p);
                        print_mark(MEM[p + 1].b32.s1);
                    }
                    CLOSE_NODE => print_write_whatsit(b"closeout", p),
                    SPECIAL_NODE => {
                        print_esc_cstr(b"special");
                        print_mark(MEM[p + 1].b32.s1);
                    }
                    LANGUAGE_NODE => {
                        print_esc_cstr(b"setlanguage");
                        print_int(MEM[p + 1].b32.s1);
                        print_cstr(b" (hyphenmin ");
                        print_int(MEM[p + 1].b16.s1 as i32);
                        print_char(',' as i32);
                        print_int(MEM[p + 1].b16.s0 as i32);
                        print_char(')' as i32);
                    }
                    NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                        print_esc(
                            (*hash
                                .offset((FONT_ID_BASE as i32 + MEM[p + 4].b16.s2 as i32) as isize))
                            .s1,
                        );
                        print_char(' ' as i32);
                        print_native_word(p);
                    }
                    GLYPH_NODE => {
                        print_esc(
                            (*hash
                                .offset((FONT_ID_BASE as i32 + MEM[p + 4].b16.s2 as i32) as isize))
                            .s1,
                        );
                        print_cstr(b" glyph#");
                        print_int(MEM[p + 4].b16.s1 as i32);
                    }
                    PIC_NODE | PDF_NODE => {
                        if MEM[p as usize].b16.s0 == PIC_NODE {
                            print_esc_cstr(b"XeTeXpicfile");
                        } else {
                            print_esc_cstr(b"XeTeXpdffile");
                        }
                        print_cstr(b"( ");
                        for i in 0..MEM[p + 4].b16.s1 {
                            print_raw_char(
                                *(&mut MEM[p + 9] as *mut memory_word as *mut u8).offset(i as isize)
                                    as UTF16_code,
                                true,
                            );
                        }
                        print('\"' as i32);
                    }
                    PDF_SAVE_POS_NODE => print_esc_cstr(b"pdfsavepos"),
                    _ => print_cstr(b"whatsit?"),
                },
                GLUE_NODE => {
                    if MEM[p].b16.s0 >= A_LEADERS {
                        /*198: */
                        print_esc_cstr(b""); /*:244 */
                        if MEM[p].b16.s0 == C_LEADERS {
                            print_char('c' as i32); /*214:*/
                        } else if MEM[p].b16.s0 == X_LEADERS {
                            print_char('x' as i32);
                        }
                        print_cstr(b"leaders ");
                        print_spec(MEM[p + 1].b32.s0, b"");
                        str_pool[pool_ptr as usize] = '.' as i32 as packed_UTF16_code;
                        pool_ptr += 1;
                        show_node_list(MEM[p + 1].b32.s1);
                        pool_ptr -= 1
                    } else {
                        print_esc_cstr(b"glue");
                        if MEM[p].b16.s0 != NORMAL {
                            print_char('(' as i32);
                            if MEM[p].b16.s0 < COND_MATH_GLUE {
                                match GluePar::try_from(MEM[p].b16.s0 - 1) {
                                    Ok(dimen) => print_skip_param(dimen),
                                    Err(e) => print_cstr(e),
                                }
                            } else if MEM[p].b16.s0 == COND_MATH_GLUE {
                                print_esc_cstr(b"nonscript");
                            } else {
                                print_esc_cstr(b"mskip");
                            }
                            print_char(')' as i32);
                        }
                        if MEM[p].b16.s0 != COND_MATH_GLUE {
                            print_char(' ' as i32);
                            if MEM[p].b16.s0 < COND_MATH_GLUE {
                                print_spec(MEM[p + 1].b32.s0, b"");
                            } else {
                                print_spec(MEM[p + 1].b32.s0, b"mu");
                            }
                        }
                    }
                }
                KERN_NODE => {
                    if MEM[p].b16.s0 != MU_GLUE {
                        print_esc_cstr(b"kern");
                        if MEM[p].b16.s0 != NORMAL {
                            print_char(' ' as i32);
                        }
                        print_scaled(MEM[p + 1].b32.s1);
                        if MEM[p].b16.s0 == ACC_KERN {
                            print_cstr(b" (for accent)");
                        } else if MEM[p].b16.s0 == SPACE_ADJUSTMENT {
                            print_cstr(b" (space adjustment)");
                        }
                    } else {
                        print_esc_cstr(b"mkern");
                        print_scaled(MEM[p + 1].b32.s1);
                        print_cstr(b"mu");
                    }
                }
                MARGIN_KERN_NODE => {
                    print_esc_cstr(b"kern");
                    print_scaled(MEM[p + 1].b32.s1);
                    if MEM[p].b16.s0 as i32 == 0 {
                        print_cstr(b" (left margin)");
                    } else {
                        print_cstr(b" (right margin)");
                    }
                }
                MATH_NODE => {
                    if MEM[p].b16.s0 > AFTER {
                        if MEM[p].b16.s0 as i32 & 1 != 0 {
                            print_esc_cstr(b"end");
                        } else {
                            print_esc_cstr(b"begin");
                        }
                        if MEM[p].b16.s0 > R_CODE {
                            print_char('R' as i32);
                        } else if MEM[p].b16.s0 > L_CODE {
                            print_char('L' as i32);
                        } else {
                            print_char('M' as i32);
                        }
                    } else {
                        print_esc_cstr(b"math");
                        if MEM[p].b16.s0 == BEFORE {
                            print_cstr(b"on");
                        } else {
                            print_cstr(b"off");
                        }
                        if MEM[p + 1].b32.s1 != 0 {
                            print_cstr(b", surrounded ");
                            print_scaled(MEM[p + 1].b32.s1);
                        }
                    }
                }
                LIGATURE_NODE => {
                    print_font_and_char(p as i32 + 1);
                    print_cstr(b" (ligature ");
                    if MEM[p].b16.s0 > 1 {
                        print_char('|' as i32);
                    }
                    font_in_short_display = MEM[p + 1].b16.s1 as i32;
                    short_display(MEM[p + 1].b32.s1);
                    if MEM[p].b16.s0 as i32 & 1 != 0 {
                        print_char('|' as i32);
                    }
                    print_char(')' as i32);
                }
                PENALTY_NODE => {
                    print_esc_cstr(b"penalty ");
                    print_int(MEM[p + 1].b32.s1);
                }
                DISC_NODE => {
                    print_esc_cstr(b"discretionary");
                    if MEM[p].b16.s0 > 0 {
                        print_cstr(b" replacing ");
                        print_int(MEM[p].b16.s0 as i32);
                    }
                    str_pool[pool_ptr as usize] = '.' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 1].b32.s0);
                    pool_ptr -= 1;
                    str_pool[pool_ptr as usize] = '|' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 1].b32.s1);
                    pool_ptr -= 1
                }
                MARK_NODE => {
                    print_esc_cstr(b"mark");
                    if MEM[p + 1].b32.s0 != 0 {
                        print_char('s' as i32);
                        print_int(MEM[p + 1].b32.s0);
                    }
                    print_mark(MEM[p + 1].b32.s1);
                }
                ADJUST_NODE => {
                    print_esc_cstr(b"vadjust");
                    if MEM[p].b16.s0 as i32 != 0 {
                        print_cstr(b" pre ");
                    }
                    str_pool[pool_ptr as usize] = '.' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 1].b32.s1);
                    pool_ptr -= 1
                }
                STYLE_NODE => print_style(MEM[p].b16.s0 as i32),
                CHOICE_NODE => {
                    print_esc_cstr(b"mathchoice");
                    str_pool[pool_ptr as usize] = 'D' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 1].b32.s0);
                    pool_ptr -= 1;
                    str_pool[pool_ptr as usize] = 'T' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 1].b32.s1);
                    pool_ptr -= 1;
                    str_pool[pool_ptr as usize] = 'S' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 2].b32.s0);
                    pool_ptr -= 1;
                    str_pool[pool_ptr as usize] = 's' as i32 as packed_UTF16_code;
                    pool_ptr += 1;
                    show_node_list(MEM[p + 2].b32.s1);
                    pool_ptr -= 1
                }
                ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD | PUNCT_NOAD
                | INNER_NOAD | RADICAL_NOAD | OVER_NOAD | UNDER_NOAD | VCENTER_NOAD
                | ACCENT_NOAD | LEFT_NOAD | RIGHT_NOAD => {
                    match MEM[p as usize].b16.s1 {
                        ORD_NOAD => print_esc_cstr(b"mathord"),
                        OP_NOAD => print_esc_cstr(b"mathop"),
                        BIN_NOAD => print_esc_cstr(b"mathbin"),
                        REL_NOAD => print_esc_cstr(b"mathrel"),
                        OPEN_NOAD => print_esc_cstr(b"mathopen"),
                        CLOSE_NOAD => print_esc_cstr(b"mathclose"),
                        PUNCT_NOAD => print_esc_cstr(b"mathpunct"),
                        INNER_NOAD => print_esc_cstr(b"mathinner"),
                        OVER_NOAD => print_esc_cstr(b"overline"),
                        UNDER_NOAD => print_esc_cstr(b"underline"),
                        VCENTER_NOAD => print_esc_cstr(b"vcenter"),
                        RADICAL_NOAD => {
                            print_esc_cstr(b"radical");
                            print_delimiter(p + 4);
                        }
                        ACCENT_NOAD => {
                            print_esc_cstr(b"accent");
                            print_fam_and_char(p + 4);
                        }
                        LEFT_NOAD => {
                            print_esc_cstr(b"left");
                            print_delimiter(p + 1);
                        }
                        RIGHT_NOAD => {
                            if MEM[p].b16.s0 == NORMAL {
                                print_esc_cstr(b"right");
                            } else {
                                print_esc_cstr(b"middle");
                            }
                            print_delimiter(p + 1);
                        }
                        _ => {}
                    }
                    if MEM[p].b16.s1 < LEFT_NOAD {
                        if MEM[p].b16.s0 != NORMAL {
                            if MEM[p].b16.s0 == LIMITS {
                                print_esc_cstr(b"limits");
                            } else {
                                print_esc_cstr(b"nolimits");
                            }
                        }
                        print_subsidiary_data(p + 1, '.' as i32 as UTF16_code);
                    }
                    print_subsidiary_data(p + 2, '^' as i32 as UTF16_code);
                    print_subsidiary_data(p + 3, '_' as i32 as UTF16_code);
                }
                FRACTION_NOAD => {
                    print_esc_cstr(b"fraction, thickness ");
                    if MEM[p + 1].b32.s1 == DEFAULT_CODE {
                        print_cstr(b"= default");
                    } else {
                        print_scaled(MEM[p + 1].b32.s1);
                    }
                    if MEM[p + 4].b16.s3 as i32 % 256 != 0
                        || MEM[p + 4].b16.s2 as i64
                            + (MEM[p + 4].b16.s3 as i32 / 256) as i64 * 65536
                            != 0
                        || MEM[p + 4].b16.s1 as i32 % 256 != 0
                        || MEM[p + 4].b16.s0 as i64
                            + (MEM[p + 4].b16.s1 as i32 / 256) as i64 * 65536
                            != 0
                    {
                        print_cstr(b", left-delimiter ");
                        print_delimiter(p + 4);
                    }
                    if MEM[p + 5].b16.s3 as i32 % 256 != 0
                        || MEM[p + 5].b16.s2 as i64
                            + (MEM[p + 5].b16.s3 as i32 / 256) as i64 * 65536
                            != 0
                        || MEM[p + 5].b16.s1 as i32 % 256 != 0
                        || MEM[p + 5].b16.s0 as i64
                            + (MEM[p + 5].b16.s1 as i32 / 256) as i64 * 65536
                            != 0
                    {
                        print_cstr(b", right-delimiter ");
                        print_delimiter(p + 5);
                    }
                    print_subsidiary_data(p + 2, '\\' as i32 as UTF16_code);
                    print_subsidiary_data(p + 3, '/' as i32 as UTF16_code);
                }
                _ => print_cstr(b"Unknown node type!"),
            }
        }
        p = *LLIST_link(p as usize)
    }
}
pub(crate) unsafe fn show_box(mut p: i32) {
    depth_threshold = *INTPAR(IntPar::show_box_depth);
    breadth_max = *INTPAR(IntPar::show_box_breadth);
    if breadth_max <= 0 {
        breadth_max = 5;
    }
    if pool_ptr + depth_threshold >= pool_size {
        depth_threshold = pool_size - pool_ptr - 1;
    }
    show_node_list(p);
    print_ln();
}
pub(crate) unsafe fn short_display_n(mut p: i32, mut m: i32) {
    breadth_max = m;
    depth_threshold = pool_size - pool_ptr - 1;
    show_node_list(p);
}
pub(crate) unsafe fn delete_token_ref(p: usize) {
    if MEM[p].b32.s0.is_texnull() {
        flush_list(Some(p));
    } else {
        MEM[p].b32.s0 -= 1;
    };
}
pub(crate) unsafe fn delete_glue_ref(p: usize) {
    if MEM[p].b32.s1.is_texnull() {
        free_node(p, GLUE_SPEC_SIZE);
    } else {
        MEM[p].b32.s1 -= 1;
    };
}
pub(crate) unsafe fn flush_node_list(mut p: i32) {
    let mut current_block: u64;
    let mut q: i32 = 0;
    while !p.is_texnull() {
        q = MEM[p as usize].b32.s1;
        if is_char_node(p) {
            MEM[p as usize].b32.s1 = avail;
            avail = p
        } else {
            let p = p as usize;
            match MEM[p].b16.s1 {
                HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                    flush_node_list(MEM[p + 5].b32.s1);
                    free_node(p, BOX_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                RULE_NODE => {
                    free_node(p, RULE_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                INS_NODE => {
                    flush_node_list(MEM[p + 4].b32.s0);
                    delete_glue_ref(*INSERTION_NODE_split_top_ptr(p) as usize);
                    free_node(p, INS_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                WHATSIT_NODE => {
                    match MEM[p].b16.s0 {
                        OPEN_NODE => free_node(p, OPEN_NODE_SIZE),
                        WRITE_NODE | SPECIAL_NODE => {
                            delete_token_ref(MEM[p + 1].b32.s1 as usize);
                            free_node(p, WRITE_NODE_SIZE);
                        }
                        CLOSE_NODE | LANGUAGE_NODE => free_node(p, SMALL_NODE_SIZE),
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                            if !MEM[p + 5].ptr.is_null() {
                                MEM[p + 5].ptr = mfree(*NATIVE_NODE_glyph_info_ptr(p));
                                MEM[p + 4].b16.s0 = 0_u16
                            }
                            free_node(p, *NATIVE_NODE_size(p) as i32);
                        }
                        GLYPH_NODE => free_node(p, GLYPH_NODE_SIZE),
                        PIC_NODE | PDF_NODE => {
                            free_node(
                                p,
                                (PIC_NODE_SIZE as u64).wrapping_add(
                                    (MEM[p + 4].b16.s1 as u64)
                                        .wrapping_add(::std::mem::size_of::<memory_word>() as u64)
                                        .wrapping_sub(1i32 as u64)
                                        .wrapping_div(::std::mem::size_of::<memory_word>() as u64),
                                ) as i32,
                            );
                        }
                        PDF_SAVE_POS_NODE => free_node(p, SMALL_NODE_SIZE),
                        _ => confusion(b"ext3"),
                    }
                    current_block = 16791665189521845338;
                }
                GLUE_NODE => {
                    if MEM[MEM[p + 1].b32.s0 as usize].b32.s1.is_texnull() {
                        free_node(MEM[p + 1].b32.s0 as usize, 4);
                    } else {
                        MEM[MEM[p + 1].b32.s0 as usize].b32.s1 -= 1
                    }
                    if !MEM[p + 1].b32.s1.is_texnull() {
                        flush_node_list(MEM[p + 1].b32.s1);
                    }
                    free_node(p, MEDIUM_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                KERN_NODE | MATH_NODE | PENALTY_NODE => {
                    free_node(p, MEDIUM_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                MARGIN_KERN_NODE => {
                    free_node(p, MARGIN_KERN_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                LIGATURE_NODE => {
                    flush_node_list(MEM[p + 1].b32.s1);
                    current_block = 8062065914618164218;
                }
                MARK_NODE => {
                    delete_token_ref(MEM[p + 1].b32.s1 as usize);
                    current_block = 8062065914618164218;
                }
                DISC_NODE => {
                    flush_node_list(MEM[p + 1].b32.s0);
                    flush_node_list(MEM[p + 1].b32.s1);
                    current_block = 8062065914618164218;
                }
                ADJUST_NODE => {
                    flush_node_list(MEM[p + 1].b32.s1);
                    current_block = 8062065914618164218;
                }
                STYLE_NODE => {
                    free_node(p, STYLE_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                CHOICE_NODE => {
                    flush_node_list(MEM[p + 1].b32.s0);
                    flush_node_list(MEM[p + 1].b32.s1);
                    flush_node_list(MEM[p + 2].b32.s0);
                    flush_node_list(MEM[p + 2].b32.s1);
                    free_node(p, STYLE_NODE_SIZE);
                    current_block = 16791665189521845338;
                }
                ORD_NOAD | OP_NOAD | BIN_NOAD | REL_NOAD | OPEN_NOAD | CLOSE_NOAD | PUNCT_NOAD
                | INNER_NOAD | RADICAL_NOAD | OVER_NOAD | UNDER_NOAD | VCENTER_NOAD
                | ACCENT_NOAD => {
                    if MEM[p + 1].b32.s1 >= SUB_BOX {
                        flush_node_list(MEM[p + 1].b32.s0);
                    }
                    if MEM[p + 2].b32.s1 >= SUB_BOX {
                        flush_node_list(MEM[p + 2].b32.s0);
                    }
                    if MEM[p + 3].b32.s1 >= SUB_BOX {
                        flush_node_list(MEM[p + 3].b32.s0);
                    }
                    if MEM[p].b16.s1 == RADICAL_NOAD {
                        free_node(p, RADICAL_NOAD_SIZE);
                    } else if MEM[p].b16.s1 == ACCENT_NOAD {
                        free_node(p, ACCENT_NOAD_SIZE);
                    } else {
                        free_node(p, NOAD_SIZE);
                    }
                    current_block = 16791665189521845338;
                }
                LEFT_NOAD | RIGHT_NOAD => {
                    free_node(p, NOAD_SIZE);
                    current_block = 16791665189521845338;
                }
                FRACTION_NOAD => {
                    flush_node_list(MEM[p + 2].b32.s0);
                    flush_node_list(MEM[p + 3].b32.s0);
                    free_node(p, FRACTION_NOAD_SIZE);
                    current_block = 16791665189521845338;
                }
                _ => confusion(b"flushing"),
            }
            match current_block {
                16791665189521845338 => {}
                _ => free_node(p, SMALL_NODE_SIZE),
            }
        }
        p = q
    }
}
pub(crate) unsafe fn copy_node_list(mut p: i32) -> i32 {
    let mut r: i32 = 0;
    let mut words: u8 = 0;
    let h = get_avail() as i32;
    let mut q = h;
    while !p.is_texnull() {
        words = 1;
        if is_char_node(p) {
            r = get_avail() as i32;
        } else {
            let p = p as usize;
            match MEM[p].b16.s1 {
                HLIST_NODE | VLIST_NODE | UNSET_NODE => {
                    r = get_node(BOX_NODE_SIZE) as i32;
                    *SYNCTEX_tag(r as usize, BOX_NODE_SIZE) = *SYNCTEX_tag(p, BOX_NODE_SIZE);
                    *SYNCTEX_line(r as usize, BOX_NODE_SIZE) = *SYNCTEX_line(p, BOX_NODE_SIZE);
                    MEM[(r + 6) as usize] = MEM[p + 6];
                    MEM[(r + 5) as usize] = MEM[p + 5];
                    MEM[(r + 5) as usize].b32.s1 = copy_node_list(MEM[p + 5].b32.s1);
                    words = 5_u8
                }
                RULE_NODE => {
                    r = get_node(RULE_NODE_SIZE) as i32;
                    words = (RULE_NODE_SIZE - 1) as u8
                }
                INS_NODE => {
                    r = get_node(INS_NODE_SIZE) as i32;
                    MEM[(r + 4) as usize] = MEM[p + 4];
                    *GLUE_SPEC_ref_count(MEM[p + 4].b32.s1 as usize) += 1;
                    MEM[(r + 4) as usize].b32.s0 = copy_node_list(MEM[p + 4].b32.s0);
                    words = (INS_NODE_SIZE - 1) as u8
                }
                WHATSIT_NODE => match MEM[p].b16.s0 {
                    OPEN_NODE => {
                        r = get_node(OPEN_NODE_SIZE) as i32;
                        words = OPEN_NODE_SIZE as u8
                    }
                    WRITE_NODE | SPECIAL_NODE => {
                        r = get_node(WRITE_NODE_SIZE) as i32;
                        MEM[MEM[p + 1].b32.s1 as usize].b32.s0 += 1;
                        words = WRITE_NODE_SIZE as u8
                    }
                    CLOSE_NODE | LANGUAGE_NODE => {
                        r = get_node(SMALL_NODE_SIZE) as i32;
                        words = SMALL_NODE_SIZE as u8
                    }
                    NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                        words = *NATIVE_NODE_size(p) as u8;
                        r = get_node(words as i32) as i32;
                        while words > 0 {
                            words -= 1;
                            MEM[(r + words as i32) as usize] = MEM[p + (words as usize)]
                        }
                        *NATIVE_NODE_glyph_info_ptr(r as usize) = ptr::null_mut();
                        *NATIVE_NODE_glyph_count(r as usize) = 0;
                        copy_native_glyph_info(p, r as usize);
                    }
                    GLYPH_NODE => {
                        r = get_node(GLYPH_NODE_SIZE) as i32;
                        words = GLYPH_NODE_SIZE as u8
                    }
                    PIC_NODE | PDF_NODE => {
                        words = (PIC_NODE_SIZE as u64).wrapping_add(
                            (MEM[p + 4].b16.s1 as u64)
                                .wrapping_add(::std::mem::size_of::<memory_word>() as u64)
                                .wrapping_sub(1)
                                .wrapping_div(::std::mem::size_of::<memory_word>() as u64),
                        ) as u8;
                        r = get_node(words as i32) as i32;
                    }
                    PDF_SAVE_POS_NODE => r = get_node(SMALL_NODE_SIZE) as i32,
                    _ => confusion(b"ext2"),
                },
                GLUE_NODE => {
                    r = get_node(MEDIUM_NODE_SIZE) as i32;
                    *GLUE_SPEC_ref_count(MEM[p + 1].b32.s0 as usize) += 1;
                    MEM[(r + 2) as usize].b32.s0 = MEM[p + 2].b32.s0;
                    MEM[(r + 2) as usize].b32.s1 = MEM[p + 2].b32.s1;
                    MEM[(r + 1) as usize].b32.s0 = MEM[p + 1].b32.s0;
                    MEM[(r + 1) as usize].b32.s1 = copy_node_list(MEM[p + 1].b32.s1)
                }
                KERN_NODE | MATH_NODE | PENALTY_NODE => {
                    r = get_node(MEDIUM_NODE_SIZE) as i32;
                    words = MEDIUM_NODE_SIZE as u8
                }
                MARGIN_KERN_NODE => {
                    r = get_node(MARGIN_KERN_NODE_SIZE) as i32;
                    words = MARGIN_KERN_NODE_SIZE as u8
                }
                LIGATURE_NODE => {
                    r = get_node(SMALL_NODE_SIZE) as i32;
                    MEM[(r + 1) as usize] = MEM[p + 1];
                    MEM[(r + 1) as usize].b32.s1 = copy_node_list(MEM[p + 1].b32.s1)
                }
                DISC_NODE => {
                    r = get_node(SMALL_NODE_SIZE) as i32;
                    MEM[(r + 1) as usize].b32.s0 = copy_node_list(MEM[p + 1].b32.s0);
                    MEM[(r + 1) as usize].b32.s1 = copy_node_list(MEM[p + 1].b32.s1)
                }
                MARK_NODE => {
                    r = get_node(SMALL_NODE_SIZE) as i32;
                    MEM[MEM[p + 1].b32.s1 as usize].b32.s0 += 1;
                    words = SMALL_NODE_SIZE as u8
                }
                ADJUST_NODE => {
                    r = get_node(SMALL_NODE_SIZE) as i32;
                    MEM[(r + 1) as usize].b32.s1 = copy_node_list(MEM[p + 1].b32.s1)
                }
                _ => confusion(b"copying"),
            }
        }
        while words as i32 > 0i32 {
            words = words.wrapping_sub(1);
            MEM[(r + words as i32) as usize] = MEM[(p + words as i32) as usize]
        }
        MEM[q as usize].b32.s1 = r;
        q = r;
        p = *LLIST_link(p as usize)
    }
    MEM[q as usize].b32.s1 = TEX_NULL;
    q = MEM[h as usize].b32.s1;
    MEM[h as usize].b32.s1 = avail;
    avail = h;
    q
}
pub(crate) unsafe fn print_mode(mut m: i32) {
    if m > 0 {
        match m / (MAX_COMMAND as i32 + 1) {
            0 => print_cstr(b"vertical mode"),
            1 => print_cstr(b"horizontal mode"),
            2 => print_cstr(b"display math mode"),
            _ => {}
        }
    } else if m == 0 {
        print_cstr(b"no mode");
    } else {
        match -m / (MAX_COMMAND as i32 + 1) {
            0 => print_cstr(b"internal vertical mode"),
            1 => print_cstr(b"restricted horizontal mode"),
            2 => print_cstr(b"math mode"),
            _ => {}
        }
    };
}
pub(crate) unsafe fn print_in_mode(mut m: i32) {
    if m > 0 {
        match m / (MAX_COMMAND as i32 + 1) {
            0 => print_cstr(b"\' in vertical mode"),
            1 => print_cstr(b"\' in horizontal mode"),
            2 => print_cstr(b"\' in display math mode"),
            _ => {}
        }
    } else if m == 0 {
        print_cstr(b"\' in no mode");
    } else {
        match -m / (MAX_COMMAND as i32 + 1) {
            0 => print_cstr(b"\' in internal vertical mode"),
            1 => print_cstr(b"\' in restricted horizontal mode"),
            2 => print_cstr(b"\' in math mode"),
            _ => {}
        }
    };
}
pub(crate) unsafe fn push_nest() {
    if NEST_PTR > MAX_NEST_STACK {
        MAX_NEST_STACK = NEST_PTR;
        if NEST_PTR == NEST_SIZE {
            overflow(b"semantic nest size", NEST_SIZE);
        }
    }
    NEST[NEST_PTR] = cur_list;
    NEST_PTR += 1;
    cur_list.head = get_avail();
    cur_list.tail = cur_list.head;
    cur_list.prev_graf = 0i32;
    cur_list.mode_line = line;
    cur_list.eTeX_aux = TEX_NULL;
}
pub(crate) unsafe fn pop_nest() {
    MEM[cur_list.head].b32.s1 = avail;
    avail = cur_list.head as i32;
    NEST_PTR -= 1;
    cur_list = NEST[NEST_PTR];
}
pub(crate) unsafe fn show_activities() {
    NEST[NEST_PTR] = cur_list;
    print_nl_cstr(b"");
    print_ln();
    let mut p = NEST_PTR as usize;
    let for_end = 0;
    if p >= for_end {
        loop {
            let mut m = NEST[p].mode;
            let a = NEST[p].aux;
            print_nl_cstr(b"### ");
            print_mode(m as i32);
            print_cstr(b" entered at line ");
            print_int(NEST[p].mode_line.abs());
            if m == HMODE {
                if NEST[p].prev_graf != 0x830000 {
                    print_cstr(b" (language");
                    print_int((NEST[p].prev_graf as i64 % 65536) as i32);
                    print_cstr(b":hyphenmin");
                    print_int(NEST[p].prev_graf / 0x400000);
                    print_char(',' as i32);
                    print_int((NEST[p].prev_graf as i64 / 65536 % 64) as i32);
                    print_char(')' as i32);
                }
            }
            if NEST[p].mode_line < 0 {
                print_cstr(b" (\\output routine)");
            }
            if p == 0 {
                if PAGE_HEAD != page_tail as usize {
                    print_nl_cstr(b"### current page:");
                    if output_active {
                        print_cstr(b" (held over for next output)");
                    }
                    show_box(MEM[PAGE_HEAD].b32.s1);
                    if page_contents as i32 > EMPTY {
                        print_nl_cstr(b"total height ");
                        print_totals();
                        print_nl_cstr(b" goal height ");
                        print_scaled(page_so_far[0]);
                        let mut r = MEM[PAGE_INS_HEAD].b32.s1 as usize;
                        while r != PAGE_INS_HEAD {
                            print_ln();
                            print_esc_cstr(b"insert");
                            let mut t = MEM[r].b16.s0 as i32;
                            print_int(t);
                            print_cstr(b" adds ");
                            if *COUNT_REG(t as usize) == 1000 {
                                t = MEM[r + 3].b32.s1
                            } else {
                                t = x_over_n(MEM[r + 3].b32.s1, 1000) * *COUNT_REG(t as usize)
                            }
                            print_scaled(t);
                            if MEM[r].b16.s1 as i32 == SPLIT_UP {
                                let mut q = PAGE_HEAD;
                                t = 0;
                                loop {
                                    q = *LLIST_link(q as usize) as usize;
                                    if MEM[q].b16.s1 as i32 == 3
                                        && MEM[q].b16.s0 as i32 == MEM[r].b16.s0 as i32
                                    {
                                        t += 1
                                    }
                                    if q == MEM[r + 1].b32.s0 as usize {
                                        break;
                                    }
                                }
                                print_cstr(b", #");
                                print_int(t);
                                print_cstr(b" might split");
                            }
                            r = *LLIST_link(r as usize) as usize;
                        }
                    }
                }
                if !MEM[CONTRIB_HEAD].b32.s1.is_texnull() {
                    print_nl_cstr(b"### recent contributions:");
                }
            }
            show_box(MEM[NEST[p].head as usize].b32.s1);
            match (m as i32).abs() / (MAX_COMMAND as i32 + 1) {
                0 => {
                    print_nl_cstr(b"prevdepth ");
                    if a.b32.s1 <= IGNORE_DEPTH {
                        print_cstr(b"ignored");
                    } else {
                        print_scaled(a.b32.s1);
                    }
                    if NEST[p].prev_graf != 0 {
                        print_cstr(b", prevgraf ");
                        print_int(NEST[p].prev_graf);
                        if NEST[p].prev_graf != 1 {
                            print_cstr(b" lines");
                        } else {
                            print_cstr(b" line");
                        }
                    }
                }
                1 => {
                    print_nl_cstr(b"spacefactor ");
                    print_int(a.b32.s0);
                    if m as i32 > 0 {
                        if a.b32.s1 > 0 {
                            print_cstr(b", current language ");
                            print_int(a.b32.s1);
                        }
                    }
                }
                2 => {
                    if !a.b32.s1.is_texnull() {
                        print_cstr(b"this will be denominator of:");
                        show_box(a.b32.s1);
                    }
                }
                _ => {}
            }
            if !(p > for_end) {
                break;
            }
            p -= 1;
        }
    };
}
pub(crate) unsafe fn print_param(n: IntPar) {
    use IntPar::*;
    match n {
        pretolerance => print_esc_cstr(b"pretolerance"),
        tolerance => print_esc_cstr(b"tolerance"),
        line_penalty => print_esc_cstr(b"linepenalty"),
        hyphen_penalty => print_esc_cstr(b"hyphenpenalty"),
        ex_hyphen_penalty => print_esc_cstr(b"exhyphenpenalty"),
        club_penalty => print_esc_cstr(b"clubpenalty"),
        widow_penalty => print_esc_cstr(b"widowpenalty"),
        display_widow_penalty => print_esc_cstr(b"displaywidowpenalty"),
        broken_penalty => print_esc_cstr(b"brokenpenalty"),
        bin_op_penalty => print_esc_cstr(b"binoppenalty"),
        rel_penalty => print_esc_cstr(b"relpenalty"),
        pre_display_penalty => print_esc_cstr(b"predisplaypenalty"),
        post_display_penalty => print_esc_cstr(b"postdisplaypenalty"),
        inter_line_penalty => print_esc_cstr(b"interlinepenalty"),
        double_hyphen_demerits => print_esc_cstr(b"doublehyphendemerits"),
        final_hyphen_demerits => print_esc_cstr(b"finalhyphendemerits"),
        adj_demerits => print_esc_cstr(b"adjdemerits"),
        mag => print_esc_cstr(b"mag"),
        delimiter_factor => print_esc_cstr(b"delimiterfactor"),
        looseness => print_esc_cstr(b"looseness"),
        time => print_esc_cstr(b"time"),
        day => print_esc_cstr(b"day"),
        month => print_esc_cstr(b"month"),
        year => print_esc_cstr(b"year"),
        show_box_breadth => print_esc_cstr(b"showboxbreadth"),
        show_box_depth => print_esc_cstr(b"showboxdepth"),
        hbadness => print_esc_cstr(b"hbadness"),
        vbadness => print_esc_cstr(b"vbadness"),
        pausing => print_esc_cstr(b"pausing"),
        tracing_online => print_esc_cstr(b"tracingonline"),
        tracing_macros => print_esc_cstr(b"tracingmacros"),
        tracing_stats => print_esc_cstr(b"tracingstats"),
        tracing_paragraphs => print_esc_cstr(b"tracingparagraphs"),
        tracing_pages => print_esc_cstr(b"tracingpages"),
        tracing_output => print_esc_cstr(b"tracingoutput"),
        tracing_lost_chars => print_esc_cstr(b"tracinglostchars"),
        tracing_commands => print_esc_cstr(b"tracingcommands"),
        tracing_restores => print_esc_cstr(b"tracingrestores"),
        uc_hyph => print_esc_cstr(b"uchyph"),
        output_penalty => print_esc_cstr(b"outputpenalty"),
        max_dead_cycles => print_esc_cstr(b"maxdeadcycles"),
        hang_after => print_esc_cstr(b"hangafter"),
        floating_penalty => print_esc_cstr(b"floatingpenalty"),
        global_defs => print_esc_cstr(b"globaldefs"),
        cur_fam => print_esc_cstr(b"fam"),
        escape_char => print_esc_cstr(b"escapechar"),
        default_hyphen_char => print_esc_cstr(b"defaulthyphenchar"),
        default_skew_char => print_esc_cstr(b"defaultskewchar"),
        end_line_char => print_esc_cstr(b"endlinechar"),
        new_line_char => print_esc_cstr(b"newlinechar"),
        language => print_esc_cstr(b"language"),
        left_hyphen_min => print_esc_cstr(b"lefthyphenmin"),
        right_hyphen_min => print_esc_cstr(b"righthyphenmin"),
        holding_inserts => print_esc_cstr(b"holdinginserts"),
        error_context_lines => print_esc_cstr(b"errorcontextlines"),
        char_sub_def_min => print_esc_cstr(b"charsubdefmin"),
        char_sub_def_max => print_esc_cstr(b"charsubdefmax"),
        tracing_char_sub_def => print_esc_cstr(b"tracingcharsubdef"),
        xetex_linebreak_penalty => print_esc_cstr(b"XeTeXlinebreakpenalty"),
        xetex_protrude_chars => print_esc_cstr(b"XeTeXprotrudechars"),
        synctex => print_esc_cstr(b"synctex"),
        tracing_assigns => print_esc_cstr(b"tracingassigns"),
        tracing_groups => print_esc_cstr(b"tracinggroups"),
        tracing_ifs => print_esc_cstr(b"tracingifs"),
        tracing_scan_tokens => print_esc_cstr(b"tracingscantokens"),
        tracing_nesting => print_esc_cstr(b"tracingnesting"),
        pre_display_correction => print_esc_cstr(b"predisplaydirection"),
        last_line_fit => print_esc_cstr(b"lastlinefit"),
        saving_vdiscards => print_esc_cstr(b"savingvdiscards"),
        saving_hyphs => print_esc_cstr(b"savinghyphcodes"),
        suppress_fontnotfound_error => print_esc_cstr(b"suppressfontnotfounderror"),
        texxet => print_esc_cstr(b"TeXXeTstate"),
        xetex_upwards => print_esc_cstr(b"XeTeXupwardsmode"),
        xetex_use_glyph_metrics => print_esc_cstr(b"XeTeXuseglyphmetrics"),
        xetex_inter_char_tokens => print_esc_cstr(b"XeTeXinterchartokenstate"),
        xetex_dash_break => print_esc_cstr(b"XeTeXdashbreakstate"),
        xetex_input_normalization => print_esc_cstr(b"XeTeXinputnormalization"),
        xetex_tracing_fonts => print_esc_cstr(b"XeTeXtracingfonts"),
        xetex_interword_space_shaping => print_esc_cstr(b"XeTeXinterwordspaceshaping"),
        xetex_generate_actual_text => print_esc_cstr(b"XeTeXgenerateactualtext"),
        xetex_hyphenatable_length => print_esc_cstr(b"XeTeXhyphenatablelength"),
        pdfoutput => print_esc_cstr(b"pdfoutput"),
        _ => print_cstr(b"[unknown i32 parameter!]"), // NOTE: several parameters not covered
    };
}
pub(crate) unsafe fn begin_diagnostic() {
    old_setting = selector;
    if *INTPAR(IntPar::tracing_online) <= 0 && selector == Selector::TERM_AND_LOG {
        selector = (u8::from(selector) - 1).into();
        if history == TTHistory::SPOTLESS {
            history = TTHistory::WARNING_ISSUED
        }
    };
}
pub(crate) unsafe fn end_diagnostic(mut blank_line: bool) {
    print_nl_cstr(b"");
    if blank_line {
        print_ln();
    }
    selector = old_setting;
}
pub(crate) unsafe fn print_length_param(mut n: DimenPar) {
    use DimenPar::*;
    match n {
        par_indent => print_esc_cstr(b"parindent"),
        math_surround => print_esc_cstr(b"mathsurround"),
        line_skip_limit => print_esc_cstr(b"lineskiplimit"),
        hsize => print_esc_cstr(b"hsize"),
        vsize => print_esc_cstr(b"vsize"),
        max_depth => print_esc_cstr(b"maxdepth"),
        split_max_depth => print_esc_cstr(b"splitmaxdepth"),
        box_max_depth => print_esc_cstr(b"boxmaxdepth"),
        hfuzz => print_esc_cstr(b"hfuzz"),
        vfuzz => print_esc_cstr(b"vfuzz"),
        delimiter_shortfall => print_esc_cstr(b"delimitershortfall"),
        null_delimiter_space => print_esc_cstr(b"nulldelimiterspace"),
        script_space => print_esc_cstr(b"scriptspace"),
        pre_display_size => print_esc_cstr(b"predisplaysize"),
        display_width => print_esc_cstr(b"displaywidth"),
        display_indent => print_esc_cstr(b"displayindent"),
        overfull_rule => print_esc_cstr(b"overfullrule"),
        hang_indent => print_esc_cstr(b"hangindent"),
        h_offset => print_esc_cstr(b"hoffset"),
        v_offset => print_esc_cstr(b"voffset"),
        emergency_stretch => print_esc_cstr(b"emergencystretch"),
        pdf_page_width => print_esc_cstr(b"pdfpagewidth"),
        pdf_page_height => print_esc_cstr(b"pdfpageheight"),
    }
}
pub(crate) unsafe fn print_cmd_chr(mut cmd: u16, mut chr_code: i32) {
    let mut font_name_str: str_number = 0;
    let mut quote_char: UTF16_code = 0;
    match cmd {
        LEFT_BRACE => {
            print_cstr(b"begin-group character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        RIGHT_BRACE => {
            print_cstr(b"end-group character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        MATH_SHIFT => {
            print_cstr(b"math shift character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        MAC_PARAM => {
            print_cstr(b"macro parameter character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        SUP_MARK => {
            print_cstr(b"superscript character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        SUB_MARK => {
            print_cstr(b"subscript character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        ENDV => print_cstr(b"end of alignment template"),
        SPACER => {
            print_cstr(b"blank space ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        LETTER => {
            print_cstr(b"the letter ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        OTHER_CHAR => {
            print_cstr(b"the character ");
            if (chr_code as i64) < 65536 {
                print(chr_code);
            } else {
                print_char(chr_code);
            }
        }
        ASSIGN_GLUE | ASSIGN_MU_GLUE => {
            if chr_code < SKIP_BASE as i32 {
                match GluePar::try_from((chr_code - GLUE_BASE as i32) as u16) {
                    Ok(dimen) => print_skip_param(dimen),
                    Err(e) => print_cstr(e),
                }
            } else if chr_code < MU_SKIP_BASE as i32 {
                print_esc_cstr(b"skip");
                print_int(chr_code - SKIP_BASE as i32);
            } else {
                print_esc_cstr(b"muskip");
                print_int(chr_code - MU_SKIP_BASE as i32);
            }
        }
        ASSIGN_TOKS => {
            if chr_code >= TOKS_BASE as i32 {
                print_esc_cstr(b"toks");
                print_int(chr_code - TOKS_BASE as i32);
            } else {
                match Local::try_from(chr_code - LOCAL_BASE as i32) {
                    Ok(Local::output_routine) => print_esc_cstr(b"output"),
                    Ok(Local::every_par) => print_esc_cstr(b"everypar"),
                    Ok(Local::every_math) => print_esc_cstr(b"everymath"),
                    Ok(Local::every_display) => print_esc_cstr(b"everydisplay"),
                    Ok(Local::every_hbox) => print_esc_cstr(b"everyhbox"),
                    Ok(Local::every_vbox) => print_esc_cstr(b"everyvbox"),
                    Ok(Local::every_job) => print_esc_cstr(b"everyjob"),
                    Ok(Local::every_cr) => print_esc_cstr(b"everycr"),
                    Ok(Local::every_eof) => print_esc_cstr(b"everyeof"),
                    Ok(Local::xetex_inter_char) => print_esc_cstr(b"XeTeXinterchartoks"),
                    Ok(Local::TectonicCodaTokens) => print_esc_cstr(b"TectonicCodaTokens"),
                    _ => print_esc_cstr(b"errhelp"),
                }
            }
        }
        ASSIGN_INT => {
            if chr_code < COUNT_BASE as i32 {
                match IntPar::try_from(chr_code - INT_BASE as i32) {
                    Ok(dimen) => print_param(dimen),
                    Err(e) => print_cstr(e),
                }
            } else {
                print_esc_cstr(b"count");
                print_int(chr_code - COUNT_BASE as i32);
            }
        }
        ASSIGN_DIMEN => {
            if chr_code < SCALED_BASE as i32 {
                match DimenPar::try_from(chr_code - DIMEN_BASE as i32) {
                    Ok(dimen) => print_length_param(dimen),
                    Err(e) => print_cstr(e),
                }
            } else {
                print_esc_cstr(b"dimen");
                print_int(chr_code - SCALED_BASE as i32);
            }
        }
        ACCENT => print_esc_cstr(b"accent"),
        ADVANCE => print_esc_cstr(b"advance"),
        AFTER_ASSIGNMENT => print_esc_cstr(b"afterassignment"),
        AFTER_GROUP => print_esc_cstr(b"aftergroup"),
        ASSIGN_FONT_DIMEN => print_esc_cstr(b"fontdimen"),
        BEGIN_GROUP => print_esc_cstr(b"begingroup"),
        BREAK_PENALTY => print_esc_cstr(b"penalty"),
        CHAR_NUM => print_esc_cstr(b"char"),
        CS_NAME => print_esc_cstr(b"csname"),
        DEF_FONT => print_esc_cstr(b"font"),
        DELIM_NUM => {
            if chr_code == 1 {
                print_esc_cstr(b"Udelimiter");
            } else {
                print_esc_cstr(b"delimiter");
            }
        }
        DIVIDE => print_esc_cstr(b"divide"),
        END_CS_NAME => print_esc_cstr(b"endcsname"),
        END_GROUP => print_esc_cstr(b"endgroup"),
        EX_SPACE => print_esc(' ' as i32),
        EXPAND_AFTER => {
            if chr_code == 0 {
                print_esc_cstr(b"expandafter");
            } else {
                print_esc_cstr(b"unless");
            }
        }
        HALIGN => print_esc_cstr(b"halign"),
        HRULE => print_esc_cstr(b"hrule"),
        IGNORE_SPACES => {
            if chr_code == 0 {
                print_esc_cstr(b"ignorespaces");
            } else {
                print_esc_cstr(b"primitive");
            }
        }
        INSERT => print_esc_cstr(b"insert"),
        ITAL_CORR => print_esc('/' as i32),
        MARK => {
            print_esc_cstr(b"mark");
            if chr_code > 0 {
                print_char('s' as i32);
            }
        }
        MATH_ACCENT => {
            if chr_code == 1 {
                print_esc_cstr(b"Umathaccent");
            } else {
                print_esc_cstr(b"mathaccent");
            }
        }
        MATH_CHAR_NUM => {
            if chr_code == 2 {
                print_esc_cstr(b"Umathchar");
            } else if chr_code == 1 {
                print_esc_cstr(b"Umathcharnum");
            } else {
                print_esc_cstr(b"mathchar");
            }
        }
        MATH_CHOICE => print_esc_cstr(b"mathchoice"),
        MULTIPLY => print_esc_cstr(b"multiply"),
        NO_ALIGN => print_esc_cstr(b"noalign"),
        NO_BOUNDARY => print_esc_cstr(b"noboundary"),
        NO_EXPAND => {
            if chr_code == 0 {
                print_esc_cstr(b"noexpand");
            } else {
                print_esc_cstr(b"primitive");
            }
        }
        NON_SCRIPT => print_esc_cstr(b"nonscript"),
        OMIT => print_esc_cstr(b"omit"),
        RADICAL => {
            if chr_code == 1 {
                print_esc_cstr(b"Uradical");
            } else {
                print_esc_cstr(b"radical");
            }
        }
        READ_TO_CS => {
            if chr_code == 0 {
                print_esc_cstr(b"read");
            } else {
                print_esc_cstr(b"readline");
            }
        }
        RELAX => print_esc_cstr(b"relax"),
        SET_BOX => print_esc_cstr(b"setbox"),
        SET_PREV_GRAF => print_esc_cstr(b"prevgraf"),
        SET_SHAPE => match chr_code as usize {
            c if c == LOCAL_BASE as usize + Local::par_shape as usize => {
                print_esc_cstr(b"parshape")
            }
            INTER_LINE_PENALTIES_LOC => print_esc_cstr(b"interlinepenalties"),
            CLUB_PENALTIES_LOC => print_esc_cstr(b"clubpenalties"),
            WIDOW_PENALTIES_LOC => print_esc_cstr(b"widowpenalties"),
            DISPLAY_WIDOW_PENALTIES_LOC => print_esc_cstr(b"displaywidowpenalties"),
            _ => {}
        },
        THE => {
            if chr_code == 0 {
                print_esc_cstr(b"the");
            } else if chr_code == 1 {
                print_esc_cstr(b"unexpanded");
            } else {
                print_esc_cstr(b"detokenize");
            }
        }
        TOKS_REGISTER => {
            print_esc_cstr(b"toks");
            if chr_code != 0 {
                print_sa_num(chr_code);
            }
        }
        VADJUST => print_esc_cstr(b"vadjust"),
        VALIGN => {
            if chr_code == 0 {
                print_esc_cstr(b"valign");
            } else {
                match chr_code {
                    BEGIN_L_CODE => print_esc_cstr(b"beginL"),
                    END_L_CODE => print_esc_cstr(b"endL"),
                    BEGIN_R_CODE => print_esc_cstr(b"beginR"),
                    _ => print_esc_cstr(b"endR"),
                }
            }
        }
        VCENTER => print_esc_cstr(b"vcenter"),
        VRULE => print_esc_cstr(b"vrule"),
        PAR_END => print_esc_cstr(b"par"),
        INPUT => {
            if chr_code == 0 {
                print_esc_cstr(b"input");
            } else if chr_code == 2i32 {
                print_esc_cstr(b"scantokens");
            } else {
                print_esc_cstr(b"endinput");
            }
        }
        TOP_BOT_MARK => {
            match chr_code % MARKS_CODE {
                FIRST_MARK_CODE => print_esc_cstr(b"firstmark"),
                BOT_MARK_CODE => print_esc_cstr(b"botmark"),
                SPLIT_FIRST_MARK_CODE => print_esc_cstr(b"splitfirstmark"),
                SPLIT_BOT_MARK_CODE => print_esc_cstr(b"splitbotmark"),
                _ => print_esc_cstr(b"topmark"),
            }
            if chr_code >= MARKS_CODE {
                print_char('s' as i32);
            }
        }
        REGISTER => {
            if chr_code < 0 || chr_code > 19 {
                /*lo_mem_stat_max*/
                cmd = (MEM[chr_code as usize].b16.s1 as i32 / 64) as u16
            } else {
                cmd = chr_code as u16;
                chr_code = TEX_NULL
            }
            if cmd == INT_VAL as u16 {
                print_esc_cstr(b"count");
            } else if cmd == DIMEN_VAL as u16 {
                print_esc_cstr(b"dimen");
            } else if cmd == GLUE_VAL as u16 {
                print_esc_cstr(b"skip");
            } else {
                print_esc_cstr(b"muskip");
            }
            if !chr_code.is_texnull() {
                print_sa_num(chr_code);
            }
        }
        SET_AUX => {
            if chr_code == VMODE as i32 {
                print_esc_cstr(b"prevdepth");
            } else {
                print_esc_cstr(b"spacefactor");
            }
        }
        SET_PAGE_INT => {
            if chr_code == 0 {
                print_esc_cstr(b"deadcycles");
            } else if chr_code == 2 {
                print_esc_cstr(b"interactionmode");
            } else {
                print_esc_cstr(b"insertpenalties");
            }
        }
        SET_BOX_DIMEN => {
            if chr_code == WIDTH_OFFSET {
                print_esc_cstr(b"wd");
            } else if chr_code == HEIGHT_OFFSET {
                print_esc_cstr(b"ht");
            } else {
                print_esc_cstr(b"dp");
            }
        }
        LAST_ITEM => match chr_code {
            0 => print_esc_cstr(b"lastpenalty"), // INT_VAL
            1 => print_esc_cstr(b"lastkern"),    // DIMEN_VAL
            2 => print_esc_cstr(b"lastskip"),    // GLUE_VAL
            INPUT_LINE_NO_CODE => print_esc_cstr(b"inputlineno"),
            PDF_SHELL_ESCAPE_CODE => print_esc_cstr(b"shellescape"),
            3 => print_esc_cstr(b"lastnodetype"), // LAST_NODE_TYPE_CODE
            ETEX_VERSION_CODE => print_esc_cstr(b"eTeXversion"),
            XETEX_VERSION_CODE => print_esc_cstr(b"XeTeXversion"),
            XETEX_COUNT_GLYPHS_CODE => print_esc_cstr(b"XeTeXcountglyphs"),
            XETEX_COUNT_VARIATIONS_CODE => print_esc_cstr(b"XeTeXcountvariations"),
            XETEX_VARIATION_CODE => print_esc_cstr(b"XeTeXvariation"),
            XETEX_FIND_VARIATION_BY_NAME_CODE => print_esc_cstr(b"XeTeXfindvariationbyname"),
            XETEX_VARIATION_MIN_CODE => print_esc_cstr(b"XeTeXvariationmin"),
            XETEX_VARIATION_MAX_CODE => print_esc_cstr(b"XeTeXvariationmax"),
            XETEX_VARIATION_DEFAULT_CODE => print_esc_cstr(b"XeTeXvariationdefault"),
            XETEX_COUNT_FEATURES_CODE => print_esc_cstr(b"XeTeXcountfeatures"),
            XETEX_FEATURE_CODE_CODE => print_esc_cstr(b"XeTeXfeaturecode"),
            XETEX_FIND_FEATURE_BY_NAME_CODE => print_esc_cstr(b"XeTeXfindfeaturebyname"),
            XETEX_IS_EXCLUSIVE_FEATURE_CODE => print_esc_cstr(b"XeTeXisexclusivefeature"),
            XETEX_COUNT_SELECTORS_CODE => print_esc_cstr(b"XeTeXcountselectors"),
            XETEX_SELECTOR_CODE_CODE => print_esc_cstr(b"XeTeXselectorcode"),
            XETEX_FIND_SELECTOR_BY_NAME_CODE => print_esc_cstr(b"XeTeXfindselectorbyname"),
            XETEX_IS_DEFAULT_SELECTOR_CODE => print_esc_cstr(b"XeTeXisdefaultselector"),
            XETEX_OT_COUNT_SCRIPTS_CODE => print_esc_cstr(b"XeTeXOTcountscripts"),
            XETEX_OT_COUNT_LANGUAGES_CODE => print_esc_cstr(b"XeTeXOTcountlanguages"),
            XETEX_OT_COUNT_FEATURES_CODE => print_esc_cstr(b"XeTeXOTcountfeatures"),
            XETEX_OT_SCRIPT_CODE => print_esc_cstr(b"XeTeXOTscripttag"),
            XETEX_OT_LANGUAGE_CODE => print_esc_cstr(b"XeTeXOTlanguagetag"),
            XETEX_OT_FEATURE_CODE => print_esc_cstr(b"XeTeXOTfeaturetag"),
            XETEX_MAP_CHAR_TO_GLYPH_CODE => print_esc_cstr(b"XeTeXcharglyph"),
            XETEX_GLYPH_INDEX_CODE => print_esc_cstr(b"XeTeXglyphindex"),
            XETEX_GLYPH_BOUNDS_CODE => print_esc_cstr(b"XeTeXglyphbounds"),
            XETEX_FONT_TYPE_CODE => print_esc_cstr(b"XeTeXfonttype"),
            XETEX_FIRST_CHAR_CODE => print_esc_cstr(b"XeTeXfirstfontchar"),
            XETEX_LAST_CHAR_CODE => print_esc_cstr(b"XeTeXlastfontchar"),
            PDF_LAST_X_POS_CODE => print_esc_cstr(b"pdflastxpos"),
            PDF_LAST_Y_POS_CODE => print_esc_cstr(b"pdflastypos"),
            XETEX_PDF_PAGE_COUNT_CODE => print_esc_cstr(b"XeTeXpdfpagecount"),
            CURRENT_GROUP_LEVEL_CODE => print_esc_cstr(b"currentgrouplevel"),
            CURRENT_GROUP_TYPE_CODE => print_esc_cstr(b"currentgrouptype"),
            CURRENT_IF_LEVEL_CODE => print_esc_cstr(b"currentiflevel"),
            CURRENT_IF_TYPE_CODE => print_esc_cstr(b"currentiftype"),
            CURRENT_IF_BRANCH_CODE => print_esc_cstr(b"currentifbranch"),
            FONT_CHAR_WD_CODE => print_esc_cstr(b"fontcharwd"),
            FONT_CHAR_HT_CODE => print_esc_cstr(b"fontcharht"),
            FONT_CHAR_DP_CODE => print_esc_cstr(b"fontchardp"),
            FONT_CHAR_IC_CODE => print_esc_cstr(b"fontcharic"),
            PAR_SHAPE_LENGTH_CODE => print_esc_cstr(b"parshapelength"),
            PAR_SHAPE_INDENT_CODE => print_esc_cstr(b"parshapeindent"),
            PAR_SHAPE_DIMEN_CODE => print_esc_cstr(b"parshapedimen"),
            // (ETEX_EXPR - INT_VAL + INT_VAL)
            59 => print_esc_cstr(b"numexpr"),
            // (ETEX_EXPR - INT_VAL + DIMEN_VAL)
            60 => print_esc_cstr(b"dimexpr"),
            // (ETEX_EXPR - INT_VAL + GLUE_VAL)
            61 => print_esc_cstr(b"glueexpr"),
            // (ETEX_EXPR - INT_VAL + MU_VAL)
            62 => print_esc_cstr(b"muexpr"),
            GLUE_STRETCH_ORDER_CODE => print_esc_cstr(b"gluestretchorder"),
            GLUE_SHRINK_ORDER_CODE => print_esc_cstr(b"glueshrinkorder"),
            GLUE_STRETCH_CODE => print_esc_cstr(b"gluestretch"),
            GLUE_SHRINK_CODE => print_esc_cstr(b"glueshrink"),
            MU_TO_GLUE_CODE => print_esc_cstr(b"mutoglue"),
            GLUE_TO_MU_CODE => print_esc_cstr(b"gluetomu"),
            _ => print_esc_cstr(b"badness"),
        },
        CONVERT => match chr_code {
            NUMBER_CODE => print_esc_cstr(b"number"),
            ROMAN_NUMERAL_CODE => print_esc_cstr(b"romannumeral"),
            STRING_CODE => print_esc_cstr(b"string"),
            MEANING_CODE => print_esc_cstr(b"meaning"),
            FONT_NAME_CODE => print_esc_cstr(b"fontname"),
            PDF_STRCMP_CODE => print_esc_cstr(b"strcmp"),
            PDF_MDFIVE_SUM_CODE => print_esc_cstr(b"mdfivesum"),
            LEFT_MARGIN_KERN_CODE => print_esc_cstr(b"leftmarginkern"),
            RIGHT_MARGIN_KERN_CODE => print_esc_cstr(b"rightmarginkern"),
            ETEX_REVISION_CODE => print_esc_cstr(b"eTeXrevision"),
            XETEX_REVISION_CODE => print_esc_cstr(b"XeTeXrevision"),
            XETEX_VARIATION_NAME_CODE => print_esc_cstr(b"XeTeXvariationname"),
            XETEX_FEATURE_NAME_CODE => print_esc_cstr(b"XeTeXfeaturename"),
            XETEX_SELECTOR_NAME_CODE => print_esc_cstr(b"XeTeXselectorname"),
            XETEX_GLYPH_NAME_CODE => print_esc_cstr(b"XeTeXglyphname"),
            XETEX_UCHAR_CODE => print_esc_cstr(b"Uchar"),
            XETEX_UCHARCAT_CODE => print_esc_cstr(b"Ucharcat"),
            _ => print_esc_cstr(b"jobname"),
        },
        IF_TEST => {
            if chr_code >= UNLESS_CODE {
                print_esc_cstr(b"unless");
            }
            match (chr_code % UNLESS_CODE) as i16 {
                IF_CAT_CODE => print_esc_cstr(b"ifcat"),
                IF_INT_CODE => print_esc_cstr(b"ifnum"),
                IF_DIM_CODE => print_esc_cstr(b"ifdim"),
                IF_ODD_CODE => print_esc_cstr(b"ifodd"),
                IF_VMODE_CODE => print_esc_cstr(b"ifvmode"),
                IF_HMODE_CODE => print_esc_cstr(b"ifhmode"),
                IF_MMODE_CODE => print_esc_cstr(b"ifmmode"),
                IF_INNER_CODE => print_esc_cstr(b"ifinner"),
                IF_VOID_CODE => print_esc_cstr(b"ifvoid"),
                IF_HBOX_CODE => print_esc_cstr(b"ifhbox"),
                IF_VBOX_CODE => print_esc_cstr(b"ifvbox"),
                IFX_CODE => print_esc_cstr(b"ifx"),
                IF_EOF_CODE => print_esc_cstr(b"ifeof"),
                IF_TRUE_CODE => print_esc_cstr(b"iftrue"),
                IF_FALSE_CODE => print_esc_cstr(b"iffalse"),
                IF_CASE_CODE => print_esc_cstr(b"ifcase"),
                IF_PRIMITIVE_CODE => print_esc_cstr(b"ifprimitive"),
                IF_DEF_CODE => print_esc_cstr(b"ifdefined"),
                IF_CS_CODE => print_esc_cstr(b"ifcsname"),
                IF_FONT_CHAR_CODE => print_esc_cstr(b"iffontchar"),
                IF_IN_CSNAME_CODE => print_esc_cstr(b"ifincsname"),
                _ => print_esc_cstr(b"if"),
            }
        }
        FI_OR_ELSE => {
            if chr_code == FI_CODE as i32 {
                print_esc_cstr(b"fi");
            } else if chr_code == OR_CODE as i32 {
                print_esc_cstr(b"or");
            } else {
                print_esc_cstr(b"else");
            }
        }
        TAB_MARK => {
            if chr_code == SPAN_CODE {
                print_esc_cstr(b"span");
            } else {
                print_cstr(b"alignment tab character ");
                if (chr_code as i64) < 65536 {
                    print(chr_code);
                } else {
                    print_char(chr_code);
                }
            }
        }
        CAR_RET => {
            if chr_code == CR_CODE {
                print_esc_cstr(b"cr");
            } else {
                print_esc_cstr(b"crcr");
            }
        }
        SET_PAGE_DIMEN => {
            match chr_code {
                0 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagegoal");
                }
                1 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagetotal");
                }
                2 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagestretch");
                }
                3 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagefilstretch");
                }
                4 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagefillstretch");
                }
                5 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pagefilllstretch");
                }
                6 => {
                    /* genuine literal in WEB */
                    print_esc_cstr(b"pageshrink");
                }
                _ => print_esc_cstr(b"pagedepth"),
            }
        }
        STOP => {
            if chr_code == 1 {
                print_esc_cstr(b"dump");
            } else {
                print_esc_cstr(b"end");
            }
        }
        HSKIP => match chr_code {
            SKIP_CODE => print_esc_cstr(b"hskip"),
            FIL_CODE => print_esc_cstr(b"hfil"),
            FILL_CODE => print_esc_cstr(b"hfill"),
            SS_CODE => print_esc_cstr(b"hss"),
            _ => print_esc_cstr(b"hfilneg"),
        },
        VSKIP => match chr_code {
            SKIP_CODE => print_esc_cstr(b"vskip"),
            FIL_CODE => print_esc_cstr(b"vfil"),
            FILL_CODE => print_esc_cstr(b"vfill"),
            SS_CODE => print_esc_cstr(b"vss"),
            _ => print_esc_cstr(b"vfilneg"),
        },
        MSKIP => print_esc_cstr(b"mskip"),
        KERN => print_esc_cstr(b"kern"),
        MKERN => print_esc_cstr(b"mkern"),
        HMOVE => {
            if chr_code == 1i32 {
                print_esc_cstr(b"moveleft");
            } else {
                print_esc_cstr(b"moveright");
            }
        }
        VMOVE => {
            if chr_code == 1 {
                print_esc_cstr(b"raise");
            } else {
                print_esc_cstr(b"lower");
            }
        }
        MAKE_BOX => match chr_code {
            BOX_CODE => print_esc_cstr(b"box"),
            COPY_CODE => print_esc_cstr(b"copy"),
            LAST_BOX_CODE => print_esc_cstr(b"lastbox"),
            VSPLIT_CODE => print_esc_cstr(b"vsplit"),
            VTOP_CODE => print_esc_cstr(b"vtop"),
            // (VTOP_CODE + VMODE)
            5 => print_esc_cstr(b"vbox"),
            _ => print_esc_cstr(b"hbox"),
        },
        LEADER_SHIP => {
            if chr_code as u16 == A_LEADERS {
                print_esc_cstr(b"leaders");
            } else if chr_code as u16 == C_LEADERS {
                print_esc_cstr(b"cleaders");
            } else if chr_code as u16 == X_LEADERS {
                print_esc_cstr(b"xleaders");
            } else {
                print_esc_cstr(b"shipout");
            }
        }
        START_PAR => {
            if chr_code == 0 {
                print_esc_cstr(b"noindent");
            } else {
                print_esc_cstr(b"indent");
            }
        }
        REMOVE_ITEM => {
            if chr_code as u16 == GLUE_NODE {
                print_esc_cstr(b"unskip");
            } else if chr_code as u16 == KERN_NODE {
                print_esc_cstr(b"unkern");
            } else {
                print_esc_cstr(b"unpenalty");
            }
        }
        UN_HBOX => {
            if chr_code == COPY_CODE {
                print_esc_cstr(b"unhcopy");
            } else {
                print_esc_cstr(b"unhbox");
            }
        }
        UN_VBOX => {
            if chr_code == COPY_CODE {
                print_esc_cstr(b"unvcopy");
            } else if chr_code == LAST_BOX_CODE {
                print_esc_cstr(b"pagediscards");
            } else if chr_code == VSPLIT_CODE {
                print_esc_cstr(b"splitdiscards");
            } else {
                print_esc_cstr(b"unvbox");
            }
        }
        DISCRETIONARY => {
            if chr_code == 1 {
                print_esc('-' as i32);
            } else {
                print_esc_cstr(b"discretionary");
            }
        }
        EQ_NO => {
            if chr_code == 1 {
                print_esc_cstr(b"leqno");
            } else {
                print_esc_cstr(b"eqno");
            }
        }
        MATH_COMP => match chr_code as u16 {
            ORD_NOAD => print_esc_cstr(b"mathord"),
            OP_NOAD => print_esc_cstr(b"mathop"),
            BIN_NOAD => print_esc_cstr(b"mathbin"),
            REL_NOAD => print_esc_cstr(b"mathrel"),
            OPEN_NOAD => print_esc_cstr(b"mathopen"),
            CLOSE_NOAD => print_esc_cstr(b"mathclose"),
            PUNCT_NOAD => print_esc_cstr(b"mathpunct"),
            INNER_NOAD => print_esc_cstr(b"mathinner"),
            UNDER_NOAD => print_esc_cstr(b"underline"),
            _ => print_esc_cstr(b"overline"),
        },
        LIMIT_SWITCH => {
            if chr_code as u16 == LIMITS {
                print_esc_cstr(b"limits");
            } else if chr_code as u16 == NO_LIMITS {
                print_esc_cstr(b"nolimits");
            } else {
                print_esc_cstr(b"displaylimits");
            }
        }
        MATH_STYLE => print_style(chr_code),
        ABOVE => match chr_code {
            OVER_CODE => print_esc_cstr(b"over"),
            ATOP_CODE => print_esc_cstr(b"atop"),
            // DELIMITED_CODE + ABOVE_CODE
            3 => print_esc_cstr(b"abovewithdelims"),
            // DELIMITED_CODE + OVER_CODE
            4 => print_esc_cstr(b"overwithdelims"),
            // DELIMITED_CODE + ATOP_CODE
            5 => print_esc_cstr(b"atopwithdelims"),
            _ => print_esc_cstr(b"above"),
        },
        LEFT_RIGHT => {
            if chr_code as u16 == LEFT_NOAD {
                print_esc_cstr(b"left");
            } else if chr_code as u16 == MIDDLE_NOAD {
                print_esc_cstr(b"middle");
            } else {
                print_esc_cstr(b"right");
            }
        }
        PREFIX => {
            if chr_code == 1 {
                print_esc_cstr(b"long");
            } else if chr_code == 2 {
                print_esc_cstr(b"outer");
            } else if chr_code == 8 {
                print_esc_cstr(b"protected");
            } else {
                print_esc_cstr(b"global");
            }
        }
        DEF => {
            if chr_code == 0 {
                print_esc_cstr(b"def");
            } else if chr_code == 1 {
                print_esc_cstr(b"gdef");
            } else if chr_code == 2 {
                print_esc_cstr(b"edef");
            } else {
                print_esc_cstr(b"xdef");
            }
        }
        LET => {
            if chr_code as u16 != NORMAL {
                print_esc_cstr(b"futurelet");
            } else {
                print_esc_cstr(b"let");
            }
        }
        SHORTHAND_DEF => match chr_code {
            CHAR_DEF_CODE => print_esc_cstr(b"chardef"),
            MATH_CHAR_DEF_CODE => print_esc_cstr(b"mathchardef"),
            XETEX_MATH_CHAR_DEF_CODE => print_esc_cstr(b"Umathchardef"),
            XETEX_MATH_CHAR_NUM_DEF_CODE => print_esc_cstr(b"Umathcharnumdef"),
            COUNT_DEF_CODE => print_esc_cstr(b"countdef"),
            DIMEN_DEF_CODE => print_esc_cstr(b"dimendef"),
            SKIP_DEF_CODE => print_esc_cstr(b"skipdef"),
            MU_SKIP_DEF_CODE => print_esc_cstr(b"muskipdef"),
            CHAR_SUB_DEF_CODE => print_esc_cstr(b"charsubdef"),
            _ => print_esc_cstr(b"toksdef"),
        },
        CHAR_GIVEN => {
            print_esc_cstr(b"char");
            print_hex(chr_code);
        }
        MATH_GIVEN => {
            print_esc_cstr(b"mathchar");
            print_hex(chr_code);
        }
        XETEX_MATH_GIVEN => {
            print_esc_cstr(b"Umathchar");
            print_hex(math_class(chr_code) as i32);
            print_hex(math_fam(chr_code) as i32);
            print_hex(math_char(chr_code) as i32);
        }
        DEF_CODE => {
            if chr_code == CAT_CODE_BASE as i32 {
                print_esc_cstr(b"catcode");
            } else if chr_code == MATH_CODE_BASE as i32 {
                print_esc_cstr(b"mathcode");
            } else if chr_code == LC_CODE_BASE as i32 {
                print_esc_cstr(b"lccode");
            } else if chr_code == UC_CODE_BASE as i32 {
                print_esc_cstr(b"uccode");
            } else if chr_code == SF_CODE_BASE as i32 {
                print_esc_cstr(b"sfcode");
            } else {
                print_esc_cstr(b"delcode");
            }
        }
        XETEX_DEF_CODE => {
            if chr_code == SF_CODE_BASE as i32 {
                print_esc_cstr(b"XeTeXcharclass");
            } else if chr_code == MATH_CODE_BASE as i32 {
                print_esc_cstr(b"Umathcodenum");
            } else if chr_code == MATH_CODE_BASE as i32 + 1 {
                print_esc_cstr(b"Umathcode");
            } else if chr_code == DEL_CODE_BASE as i32 {
                print_esc_cstr(b"Udelcodenum");
            } else {
                print_esc_cstr(b"Udelcode");
            }
        }
        DEF_FAMILY => print_size(chr_code - (MATH_FONT_BASE as i32)),
        HYPH_DATA => {
            if chr_code == 1 {
                print_esc_cstr(b"patterns");
            } else {
                print_esc_cstr(b"hyphenation");
            }
        }
        ASSIGN_FONT_INT => match chr_code {
            0 => print_esc_cstr(b"hyphenchar"),
            1 => print_esc_cstr(b"skewchar"),
            LP_CODE_BASE => print_esc_cstr(b"lpcode"),
            RP_CODE_BASE => print_esc_cstr(b"rpcode"),
            _ => {}
        },
        SET_FONT => {
            print_cstr(b"select font ");
            font_name_str = FONT_NAME[chr_code as usize];
            if FONT_AREA[chr_code as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[chr_code as usize] as u32 == OTGR_FONT_FLAG
            {
                let mut for_end = length(font_name_str) - 1;
                quote_char = '\"' as i32 as UTF16_code;
                for n in 0..=for_end {
                    if str_pool[(str_start[(font_name_str as i64 - 65536) as usize] + n) as usize]
                        as i32
                        == '\"' as i32
                    {
                        quote_char = '\'' as i32 as UTF16_code
                    }
                }
                print_char(quote_char as i32);
                print(font_name_str);
                print_char(quote_char as i32);
            } else {
                print(font_name_str);
            }
            if FONT_SIZE[chr_code as usize] != FONT_DSIZE[chr_code as usize] {
                print_cstr(b" at ");
                print_scaled(FONT_SIZE[chr_code as usize]);
                print_cstr(b"pt");
            }
        }
        SET_INTERACTION => match chr_code {
            BATCH_MODE => print_esc_cstr(b"batchmode"),
            NONSTOP_MODE => print_esc_cstr(b"nonstopmode"),
            SCROLL_MODE => print_esc_cstr(b"scrollmode"),
            _ => print_esc_cstr(b"errorstopmode"),
        },
        IN_STREAM => {
            if chr_code == 0 {
                print_esc_cstr(b"closein");
            } else {
                print_esc_cstr(b"openin");
            }
        }
        MESSAGE => {
            if chr_code == 0 {
                print_esc_cstr(b"message");
            } else {
                print_esc_cstr(b"errmessage");
            }
        }
        CASE_SHIFT => {
            if chr_code == LC_CODE_BASE as i32 {
                print_esc_cstr(b"lowercase");
            } else {
                print_esc_cstr(b"uppercase");
            }
        }
        XRAY => match chr_code {
            SHOW_BOX_CODE => print_esc_cstr(b"showbox"),
            SHOW_THE_CODE => print_esc_cstr(b"showthe"),
            SHOW_LISTS => print_esc_cstr(b"showlists"),
            SHOW_GROUPS => print_esc_cstr(b"showgroups"),
            SHOW_TOKENS => print_esc_cstr(b"showtokens"),
            SHOW_IFS => print_esc_cstr(b"showifs"),
            _ => print_esc_cstr(b"show"),
        },
        UNDEFINED_CS => print_cstr(b"undefined"),
        CALL | LONG_CALL | OUTER_CALL | LONG_OUTER_CALL => {
            let mut n = cmd - CALL;
            if MEM[MEM[chr_code as usize].b32.s1 as usize].b32.s0 == PROTECTED_TOKEN {
                n = n + 4
            }
            if n / 4 & 1 != 0 {
                print_esc_cstr(b"protected");
            }
            if n & 1 != 0 {
                print_esc_cstr(b"long");
            }
            if n / 2 & 1 != 0 {
                print_esc_cstr(b"outer");
            }
            if n > 0 {
                print_char(' ' as i32);
            }
            print_cstr(b"macro");
        }
        END_TEMPLATE => print_esc_cstr(b"outer endtemplate"),
        EXTENSION => match chr_code as u16 {
            OPEN_NODE => print_esc_cstr(b"openout"),
            WRITE_NODE => print_esc_cstr(b"write"),
            CLOSE_NODE => print_esc_cstr(b"closeout"),
            SPECIAL_NODE => print_esc_cstr(b"special"),
            IMMEDIATE_CODE => print_esc_cstr(b"immediate"),
            SET_LANGUAGE_CODE => print_esc_cstr(b"setlanguage"),
            PIC_FILE_CODE => print_esc_cstr(b"XeTeXpicfile"),
            PDF_FILE_CODE => print_esc_cstr(b"XeTeXpdffile"),
            GLYPH_CODE => print_esc_cstr(b"XeTeXglyph"),
            XETEX_LINEBREAK_LOCALE_EXTENSION_CODE => print_esc_cstr(b"XeTeXlinebreaklocale"),
            XETEX_INPUT_ENCODING_EXTENSION_CODE => print_esc_cstr(b"XeTeXinputencoding"),
            XETEX_DEFAULT_ENCODING_EXTENSION_CODE => print_esc_cstr(b"XeTeXdefaultencoding"),
            PDF_SAVE_POS_NODE => print_esc_cstr(b"pdfsavepos"),
            _ => print_cstr(b"[unknown extension!]"),
        },
        _ => print_cstr(b"[unknown command code!]"),
    };
}
pub(crate) unsafe fn not_aat_font_error(cmd: u16, mut c: i32, f: usize) {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Cannot use ");
    print_cmd_chr(cmd, c);
    print_cstr(b" with ");
    print(FONT_NAME[f]);
    print_cstr(b"; not an AAT font");
    error();
}
pub(crate) unsafe fn not_aat_gr_font_error(cmd: u16, mut c: i32, f: usize) {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Cannot use ");
    print_cmd_chr(cmd, c);
    print_cstr(b" with ");
    print(FONT_NAME[f]);
    print_cstr(b"; not an AAT or Graphite font");
    error();
}
pub(crate) unsafe fn not_ot_font_error(cmd: u16, mut c: i32, f: usize) {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Cannot use ");
    print_cmd_chr(cmd, c);
    print_cstr(b" with ");
    print(FONT_NAME[f]);
    print_cstr(b"; not an OpenType Layout font");
    error();
}
pub(crate) unsafe fn not_native_font_error(cmd: u16, c: i32, f: usize) {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Cannot use ");
    print_cmd_chr(cmd, c);
    print_cstr(b" with ");
    print(FONT_NAME[f]);
    print_cstr(b"; not a native platform font");
    error();
}
/*:1434*/
pub(crate) unsafe fn id_lookup(mut j: i32, mut l: i32) -> i32 {
    let mut h: i32 = 0; /*269:*/
    let mut p: i32 = 0;
    let mut ll: i32 = 0;
    h = 0i32;
    for k in j..=j + l - 1 {
        h = h + h + BUFFER[k as usize];
        while h >= HASH_PRIME as i32 {
            h = h - 8501;
        }
    }
    p = h + HASH_BASE as i32;
    ll = l;
    for d in 0..=l - 1 {
        if BUFFER[(j + d) as usize] as i64 >= 65536 {
            ll += 1
        }
    }
    loop {
        if (*hash.offset(p as isize)).s1 > 0 {
            if length((*hash.offset(p as isize)).s1) == ll {
                if str_eq_buf((*hash.offset(p as isize)).s1, j) {
                    break;
                }
            }
        }
        if (*hash.offset(p as isize)).s0 == 0 {
            if no_new_control_sequence {
                p = UNDEFINED_CONTROL_SEQUENCE as i32;
            } else {
                if (*hash.offset(p as isize)).s1 > 0 {
                    if hash_high < hash_extra {
                        hash_high += 1;
                        (*hash.offset(p as isize)).s0 = hash_high + EQTB_SIZE as i32;
                        p = hash_high + EQTB_SIZE as i32;
                    } else {
                        loop {
                            if hash_used == HASH_BASE as i32 {
                                overflow(b"hash size", HASH_SIZE + hash_extra as usize);
                            }
                            hash_used -= 1;
                            if !((*hash.offset(hash_used as isize)).s1 != 0) {
                                break;
                            }
                        }
                        (*hash.offset(p as isize)).s0 = hash_used;
                        p = hash_used
                    }
                }
                if pool_ptr + ll > pool_size {
                    overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
                }
                let d = cur_length();
                while pool_ptr > str_start[(str_ptr - TOO_BIG_CHAR) as usize] {
                    pool_ptr -= 1;
                    str_pool[(pool_ptr + l) as usize] = str_pool[pool_ptr as usize]
                }
                for k in j..=j + l - 1 {
                    if (BUFFER[k as usize] as i64) < 65536 {
                        str_pool[pool_ptr as usize] = BUFFER[k as usize] as packed_UTF16_code;
                        pool_ptr += 1
                    } else {
                        str_pool[pool_ptr as usize] = (0xd800_i64
                            + (BUFFER[k as usize] as i64 - 65536) / 1024)
                            as packed_UTF16_code;
                        pool_ptr += 1;
                        str_pool[pool_ptr as usize] = (0xdc00_i64
                            + (BUFFER[k as usize] as i64 - 65536) % 1024)
                            as packed_UTF16_code;
                        pool_ptr += 1
                    }
                }
                (*hash.offset(p as isize)).s1 = make_string();
                pool_ptr += d
            }
            break;
        } else {
            p = (*hash.offset(p as isize)).s0
        }
    }
    p
}
pub(crate) unsafe fn prim_lookup(mut s: str_number) -> i32 {
    let mut current_block: u64;
    let mut h: i32 = 0;
    let mut p: i32 = 0;
    let mut k: i32 = 0;
    let mut j: i32 = 0;
    let mut l: i32 = 0;
    if s <= BIGGEST_CHAR {
        if s < 0 {
            p = UNDEFINED_PRIMITIVE;
            current_block = 12583739755984661121;
        } else {
            p = (s % PRIM_PRIME) + 1;
            current_block = 11307063007268554308;
        }
    } else {
        j = str_start[(s as i64 - 65536) as usize];
        if s == str_ptr {
            l = cur_length()
        } else {
            l = length(s)
        }
        h = str_pool[j as usize] as i32;
        let mut for_end: i32 = 0;
        k = j + 1;
        for_end = j + l - 1;
        if k <= for_end {
            loop {
                h = h + h + str_pool[k as usize] as i32;
                while h >= PRIM_PRIME {
                    h = h - 431
                }
                if !(k < for_end) {
                    break;
                }
                k = k + 1;
            }
        }
        p = h + 1;
        current_block = 11307063007268554308;
    }
    loop {
        match current_block {
            12583739755984661121 => return p,
            _ => {
                if prim[p as usize].s1 as i64 > 65536 {
                    if length(prim[p as usize].s1) - 1 == l {
                        if str_eq_str(prim[p as usize].s1 - 1, s) {
                            current_block = 12583739755984661121;
                            continue;
                        }
                    }
                } else if prim[p as usize].s1 == 1 + s {
                    current_block = 12583739755984661121;
                    continue;
                }
                if prim[p as usize].s0 == 0i32 {
                    if no_new_control_sequence {
                        p = UNDEFINED_PRIMITIVE
                    } else {
                        /*272:*/
                        if prim[p as usize].s1 > 0 {
                            loop {
                                if prim_used == PRIM_BASE {
                                    overflow(b"primitive size", PRIM_SIZE as usize);
                                }
                                prim_used -= 1;
                                if prim[prim_used as usize].s1 == 0 {
                                    break;
                                }
                            }
                            prim[p as usize].s0 = prim_used;
                            p = prim_used
                        }
                        prim[p as usize].s1 = s + 1i32
                    }
                    current_block = 12583739755984661121;
                } else {
                    p = prim[p as usize].s0;
                    current_block = 11307063007268554308;
                }
            }
        }
    }
}
/*:276*/
/*280: *//*296: */
pub(crate) unsafe fn print_group(mut e: bool) {
    match cur_group {
        GroupCode::BOTTOM_LEVEL => {
            print_cstr(b"bottom level");
            return;
        }
        GroupCode::SIMPLE | GroupCode::SEMI_SIMPLE => {
            if cur_group == GroupCode::SEMI_SIMPLE {
                print_cstr(b"semi ");
            }
            print_cstr(b"simple");
        }
        GroupCode::HBOX | GroupCode::ADJUSTED_HBOX => {
            if cur_group == GroupCode::ADJUSTED_HBOX {
                print_cstr(b"adjusted ");
            }
            print_cstr(b"hbox");
        }
        GroupCode::VBOX => print_cstr(b"vbox"),
        GroupCode::VTOP => print_cstr(b"vtop"),
        GroupCode::ALIGN | GroupCode::NO_ALIGN => {
            if cur_group == GroupCode::NO_ALIGN {
                print_cstr(b"no ");
            }
            print_cstr(b"align");
        }
        GroupCode::OUTPUT => print_cstr(b"output"),
        GroupCode::DISC => print_cstr(b"disc"),
        GroupCode::INSERT => print_cstr(b"insert"),
        GroupCode::VCENTER => print_cstr(b"vcenter"),
        GroupCode::MATH | GroupCode::MATH_CHOICE | GroupCode::MATH_SHIFT | GroupCode::MATH_LEFT => {
            print_cstr(b"math");
            if cur_group == GroupCode::MATH_CHOICE {
                print_cstr(b" choice");
            } else if cur_group == GroupCode::MATH_SHIFT {
                print_cstr(b" shift");
            } else if cur_group == GroupCode::MATH_LEFT {
                print_cstr(b" left");
            }
        }
    }
    print_cstr(b" group (level ");
    print_int(cur_level as i32);
    print_char(')' as i32);
    if SAVE_STACK[SAVE_PTR - 1].b32.s1 != 0 {
        if e {
            print_cstr(b" entered at line ");
        } else {
            print_cstr(b" at line ");
        }
        print_int(SAVE_STACK[SAVE_PTR - 1].b32.s1);
    };
}
/*:1448*/
/*1449: */
pub(crate) unsafe fn pseudo_input() -> bool {
    let mut p: i32 = 0;
    let mut sz: i32 = 0;
    let mut r: i32 = 0;
    last = first;
    p = MEM[pseudo_files as usize].b32.s0;
    if p.is_texnull() {
        false
    } else {
        MEM[pseudo_files as usize].b32.s0 = MEM[p as usize].b32.s1;
        sz = MEM[p as usize].b32.s0;
        if 4 * sz - 3 >= BUF_SIZE as i32 - last {
            /*35: */
            cur_input.loc = first;
            cur_input.limit = last - 1;
            overflow(b"buffer size", BUF_SIZE);
        }
        last = first;
        let mut for_end: i32 = 0;
        r = p + 1;
        for_end = p + sz - 1;
        if r <= for_end {
            loop {
                let w = MEM[r as usize].b16;
                BUFFER[last as usize] = w.s3 as UnicodeScalar;
                BUFFER[(last + 1) as usize] = w.s2 as UnicodeScalar;
                BUFFER[(last + 2) as usize] = w.s1 as UnicodeScalar;
                BUFFER[(last + 3) as usize] = w.s0 as UnicodeScalar;
                last = last + 4;
                let fresh15 = r;
                r = r + 1;
                if !(fresh15 < for_end) {
                    break;
                }
            }
        }
        if last >= max_buf_stack {
            max_buf_stack = last + 1
        }
        while last > first && BUFFER[(last - 1) as usize] == ' ' as i32 {
            last -= 1
        }
        free_node(p as usize, sz);
        true
    }
}
pub(crate) unsafe fn pseudo_close() {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    p = MEM[pseudo_files as usize].b32.s1;
    q = MEM[pseudo_files as usize].b32.s0;
    MEM[pseudo_files as usize].b32.s1 = avail;
    avail = pseudo_files;
    pseudo_files = p;
    while !q.is_texnull() {
        p = q;
        q = MEM[p as usize].b32.s1;
        free_node(p as usize, MEM[p as usize].b32.s0);
    }
}
pub(crate) unsafe fn group_warning() {
    let mut w: bool = false;
    BASE_PTR = INPUT_PTR;
    INPUT_STACK[BASE_PTR] = cur_input;
    let mut i = IN_OPEN;
    w = false;
    while GRP_STACK[i] == cur_boundary && i > 0 {
        if *INTPAR(IntPar::tracing_nesting) > 0i32 {
            while INPUT_STACK[BASE_PTR].state == TOKEN_LIST
                || INPUT_STACK[BASE_PTR].index as usize > i
            {
                BASE_PTR -= 1;
            }
            if INPUT_STACK[BASE_PTR].name > 17 {
                w = true
            }
        }
        GRP_STACK[i] = SAVE_STACK[SAVE_PTR].b32.s1;
        i -= 1;
    }
    if w {
        print_nl_cstr(b"Warning: end of ");
        print_group(true);
        print_cstr(b" of a different file");
        print_ln();
        if *INTPAR(IntPar::tracing_nesting) > 1 {
            show_context();
        }
        if history == TTHistory::SPOTLESS {
            history = TTHistory::WARNING_ISSUED
        }
    };
}
pub(crate) unsafe fn if_warning() {
    let mut w: bool = false;
    BASE_PTR = INPUT_PTR;
    INPUT_STACK[BASE_PTR] = cur_input;
    let mut i = IN_OPEN;
    w = false;
    while IF_STACK[i] == cond_ptr {
        if *INTPAR(IntPar::tracing_nesting) > 0 {
            while INPUT_STACK[BASE_PTR].state == TOKEN_LIST
                || INPUT_STACK[BASE_PTR].index as usize > i
            {
                BASE_PTR -= 1
            }
            if INPUT_STACK[BASE_PTR].name > 17 {
                w = true
            }
        }
        IF_STACK[i] = MEM[cond_ptr as usize].b32.s1;
        i -= 1;
    }
    if w {
        print_nl_cstr(b"Warning: end of ");
        print_cmd_chr(107_u16, cur_if as i32);
        if if_line != 0i32 {
            print_cstr(b" entered on line ");
            print_int(if_line);
        }
        print_cstr(b" of a different file");
        print_ln();
        if *INTPAR(IntPar::tracing_nesting) > 1i32 {
            show_context();
        }
        if history == TTHistory::SPOTLESS {
            history = TTHistory::WARNING_ISSUED
        }
    };
}
pub(crate) unsafe fn file_warning() {
    let mut p: i32 = 0;
    let mut l: u16 = 0;
    let mut i: i32 = 0;
    p = SAVE_PTR as i32;
    l = cur_level;
    let c = cur_group;
    SAVE_PTR = cur_boundary as usize;
    while GRP_STACK[IN_OPEN] != SAVE_PTR as i32 {
        cur_level -= 1;
        print_nl_cstr(b"Warning: end of file when ");
        print_group(true);
        print_cstr(b" is incomplete");
        cur_group = GroupCode::from(SAVE_STACK[SAVE_PTR].b16.s0);
        SAVE_PTR = SAVE_STACK[SAVE_PTR].b32.s1 as usize
    }
    SAVE_PTR = p as usize;
    cur_level = l;
    cur_group = c;
    p = cond_ptr;
    l = if_limit as u16;
    let c = cur_if as u16;
    i = if_line;
    while IF_STACK[IN_OPEN] != cond_ptr {
        print_nl_cstr(b"Warning: end of file when ");
        print_cmd_chr(IF_TEST, cur_if as i32);
        if if_limit == FI_CODE {
            print_esc_cstr(b"else");
        }
        if if_line != 0 {
            print_cstr(b" entered on line ");
            print_int(if_line);
        }
        print_cstr(b" is incomplete");
        if_line = MEM[(cond_ptr + 1) as usize].b32.s1;
        cur_if = MEM[cond_ptr as usize].b16.s0 as small_number;
        if_limit = MEM[cond_ptr as usize].b16.s1 as u8;
        cond_ptr = *LLIST_link(cond_ptr as usize);
    }
    cond_ptr = p;
    if_limit = l as u8;
    cur_if = c as small_number;
    if_line = i;
    print_ln();
    if *INTPAR(IntPar::tracing_nesting) > 1 {
        show_context();
    }
    if history == TTHistory::SPOTLESS {
        history = TTHistory::WARNING_ISSUED
    };
}
pub(crate) unsafe fn delete_sa_ref(mut q: usize) {
    let mut i: small_number = 0;
    let mut s: small_number = 0;
    MEM[q + 1].b32.s0 -= 1;
    if !MEM[q + 1].b32.s0.is_texnull() {
        return;
    }
    if MEM[q].b16.s1 < DIMEN_VAL_LIMIT {
        if MEM[q + 2].b32.s1 == 0 {
            s = WORD_NODE_SIZE as small_number
        } else {
            return;
        }
    } else {
        if MEM[q].b16.s1 < MU_VAL_LIMIT {
            if MEM[q + 1].b32.s1 == 0 {
                delete_glue_ref(0);
            } else {
                return;
            }
        } else if !MEM[q + 1].b32.s1.is_texnull() {
            return;
        }
        s = POINTER_NODE_SIZE as small_number
    }
    loop {
        i = (MEM[q].b16.s1 as i32 % 64) as small_number;
        let p = q;
        let qi = MEM[p].b32.s1;
        q = qi as usize;
        free_node(p, s as i32);
        if qi.is_texnull() {
            sa_root[i as usize] = TEX_NULL;
            return;
        }
        if i as i32 & 1 != 0 {
            MEM[q + i as usize / 2 + 1].b32.s1 = TEX_NULL
        } else {
            MEM[q + i as usize / 2 + 1].b32.s0 = TEX_NULL
        }
        MEM[q].b16.s0 -= 1;
        s = INDEX_NODE_SIZE as small_number;
        if MEM[q].b16.s0 as i32 > 0 {
            break;
        }
    }
}
/*:1609*/
/*1611: */
pub(crate) unsafe fn sa_save(p: usize) {
    let mut q: usize;
    let mut i: u16 = 0;
    if cur_level as i32 != sa_level as i32 {
        if SAVE_PTR > MAX_SAVE_STACK {
            MAX_SAVE_STACK = SAVE_PTR;
            if MAX_SAVE_STACK > SAVE_SIZE - 7 {
                overflow(b"save size", SAVE_SIZE);
            }
        }
        SAVE_STACK[SAVE_PTR].b16.s1 = RESTORE_SA;
        SAVE_STACK[SAVE_PTR].b16.s0 = sa_level;
        SAVE_STACK[SAVE_PTR].b32.s1 = sa_chain;
        SAVE_PTR += 1;
        sa_chain = TEX_NULL;
        sa_level = cur_level
    }
    i = MEM[p].b16.s1;
    if i < DIMEN_VAL_LIMIT {
        if MEM[p + 2].b32.s1 == 0 {
            q = get_node(POINTER_NODE_SIZE) as usize;
            i = TOK_VAL_LIMIT
        } else {
            q = get_node(WORD_NODE_SIZE) as usize;
            MEM[q + 2].b32.s1 = MEM[p + 2].b32.s1
        }
        MEM[q + 1].b32.s1 = TEX_NULL
    } else {
        q = get_node(POINTER_NODE_SIZE) as usize;
        MEM[q + 1].b32.s1 = MEM[p + 1].b32.s1
    }
    MEM[q + 1].b32.s0 = p as i32;
    MEM[q].b16.s1 = i;
    MEM[q].b16.s0 = MEM[p].b16.s0;
    MEM[q].b32.s1 = sa_chain;
    sa_chain = q as i32;
    MEM[p + 1].b32.s0 += 1;
}
pub(crate) unsafe fn sa_destroy(p: usize) {
    if MEM[p].b16.s1 < MU_VAL_LIMIT {
        delete_glue_ref(MEM[p + 1].b32.s1 as usize);
    } else if !MEM[p + 1].b32.s1.is_texnull() {
        if MEM[p].b16.s1 < BOX_VAL_LIMIT {
            flush_node_list(MEM[p + 1].b32.s1);
        } else {
            delete_token_ref(MEM[p + 1].b32.s1 as usize);
        }
    };
}
pub(crate) unsafe fn sa_def(p: usize, e: i32) {
    MEM[p + 1].b32.s0 += 1;
    if MEM[p + 1].b32.s1 == e {
        sa_destroy(p);
    } else {
        if MEM[p].b16.s0 as i32 == cur_level as i32 {
            sa_destroy(p);
        } else {
            sa_save(p);
        }
        MEM[p].b16.s0 = cur_level;
        MEM[p + 1].b32.s1 = e
    }
    delete_sa_ref(p);
}
pub(crate) unsafe fn sa_w_def(p: usize, mut w: i32) {
    MEM[p + 1].b32.s0 += 1;
    if !(MEM[p + 2].b32.s1 == w) {
        if MEM[p].b16.s0 as i32 != cur_level as i32 {
            sa_save(p);
        }
        MEM[p].b16.s0 = cur_level;
        MEM[p + 2].b32.s1 = w
    }
    delete_sa_ref(p);
}
pub(crate) unsafe fn gsa_def(p: usize, e: i32) {
    MEM[p + 1].b32.s0 += 1;
    sa_destroy(p);
    MEM[p].b16.s0 = LEVEL_ONE;
    MEM[p + 1].b32.s1 = e;
    delete_sa_ref(p);
}
pub(crate) unsafe fn gsa_w_def(p: usize, w: i32) {
    MEM[p + 1].b32.s0 += 1;
    MEM[p].b16.s0 = LEVEL_ONE;
    MEM[p + 2].b32.s1 = w;
    delete_sa_ref(p);
}
pub(crate) unsafe fn sa_restore() {
    let mut p: i32 = 0;
    loop {
        p = MEM[(sa_chain + 1) as usize].b32.s0;
        if MEM[p as usize].b16.s0 == LEVEL_ONE {
            if MEM[p as usize].b16.s1 >= DIMEN_VAL_LIMIT {
                sa_destroy(sa_chain as usize);
            }
        } else {
            if MEM[p as usize].b16.s1 < DIMEN_VAL_LIMIT {
                if MEM[sa_chain as usize].b16.s1 < DIMEN_VAL_LIMIT {
                    MEM[(p + 2) as usize].b32.s1 = MEM[(sa_chain + 2) as usize].b32.s1
                } else {
                    MEM[(p + 2) as usize].b32.s1 = 0
                }
            } else {
                sa_destroy(p as usize);
                MEM[(p + 1) as usize].b32.s1 = MEM[(sa_chain + 1) as usize].b32.s1
            }
            MEM[p as usize].b16.s0 = MEM[sa_chain as usize].b16.s0
        }
        delete_sa_ref(p as usize);
        p = sa_chain;
        sa_chain = MEM[p as usize].b32.s1;
        if MEM[p as usize].b16.s1 < DIMEN_VAL_LIMIT {
            free_node(p as usize, WORD_NODE_SIZE);
        } else {
            free_node(p as usize, POINTER_NODE_SIZE);
        }
        if sa_chain.is_texnull() {
            break;
        }
    }
}
pub(crate) unsafe fn new_save_level(c: GroupCode) {
    if SAVE_PTR > MAX_SAVE_STACK {
        MAX_SAVE_STACK = SAVE_PTR;
        if MAX_SAVE_STACK > SAVE_SIZE - 7 {
            overflow(b"save size", SAVE_SIZE);
        }
    }
    SAVE_STACK[SAVE_PTR + 0].b32.s1 = line;
    SAVE_PTR += 1;
    SAVE_STACK[SAVE_PTR].b16.s1 = LEVEL_BOUNDARY;
    SAVE_STACK[SAVE_PTR].b16.s0 = cur_group as u16;
    SAVE_STACK[SAVE_PTR].b32.s1 = cur_boundary;
    if cur_level == u16::max_value() {
        overflow(b"grouping levels", u16::max_value() as usize);
    }
    cur_boundary = SAVE_PTR as i32;
    cur_group = c;
    cur_level += 1;
    SAVE_PTR += 1;
}
pub(crate) unsafe fn eq_destroy(w: memory_word) {
    let mut q: i32 = 0;
    match w.b16.s1 as i32 {
        113 | 114 | 115 | 116 => delete_token_ref(w.b32.s1 as usize),
        119 => delete_glue_ref(w.b32.s1 as usize),
        120 => {
            q = w.b32.s1;
            if !q.is_texnull() {
                free_node(
                    q as usize,
                    MEM[q as usize].b32.s0 + MEM[q as usize].b32.s0 + 1,
                );
            }
        }
        121 => flush_node_list(w.b32.s1),
        72 | 91 => {
            if w.b32.s1 < 0 || w.b32.s1 > 19 {
                delete_sa_ref(w.b32.s1 as usize);
            }
        }
        _ => {}
    };
}
pub(crate) unsafe fn eq_save(p: usize, l: u16) {
    if SAVE_PTR > MAX_SAVE_STACK {
        MAX_SAVE_STACK = SAVE_PTR;
        if MAX_SAVE_STACK > SAVE_SIZE - 7 {
            overflow(b"save size", SAVE_SIZE);
        }
    }
    if l == LEVEL_ZERO {
        SAVE_STACK[SAVE_PTR].b16.s1 = RESTORE_ZERO
    } else {
        SAVE_STACK[SAVE_PTR] = EQTB[p];
        SAVE_PTR += 1;
        SAVE_STACK[SAVE_PTR].b16.s1 = RESTORE_OLD_VALUE
    }
    SAVE_STACK[SAVE_PTR].b16.s0 = l;
    SAVE_STACK[SAVE_PTR].b32.s1 = p as i32;
    SAVE_PTR += 1;
}
pub(crate) unsafe fn eq_define(p: usize, t: u16, e: i32) {
    if EQTB[p].b16.s1 as i32 == t as i32 && EQTB[p].b32.s1 == e {
        eq_destroy(EQTB[p]);
        return;
    }
    if EQTB[p].b16.s0 as i32 == cur_level as i32 {
        eq_destroy(EQTB[p]);
    } else if cur_level > LEVEL_ONE {
        eq_save(p, EQTB[p].b16.s0);
    }
    EQTB[p].b16.s0 = cur_level;
    EQTB[p].b16.s1 = t;
    EQTB[p].b32.s1 = e;
}
pub(crate) unsafe fn eq_word_define(p: usize, w: i32) {
    if EQTB[p].b32.s1 == w {
        return;
    }
    if _xeq_level_array[p - (INT_BASE as usize)] as i32 != cur_level as i32 {
        eq_save(p, _xeq_level_array[p - (INT_BASE as usize)]);
        _xeq_level_array[p - (INT_BASE as usize)] = cur_level
    }
    EQTB[p].b32.s1 = w;
}
pub(crate) unsafe fn geq_define(p: usize, t: u16, e: i32) {
    eq_destroy(EQTB[p]);
    EQTB[p].b16.s0 = LEVEL_ONE;
    EQTB[p].b16.s1 = t;
    EQTB[p].b32.s1 = e;
}
pub(crate) unsafe fn geq_word_define(p: usize, w: i32) {
    EQTB[p].b32.s1 = w;
    _xeq_level_array[p - (INT_BASE as usize)] = LEVEL_ONE;
}
pub(crate) unsafe fn save_for_after(mut t: i32) {
    if cur_level > LEVEL_ONE {
        if SAVE_PTR > MAX_SAVE_STACK {
            MAX_SAVE_STACK = SAVE_PTR;
            if MAX_SAVE_STACK > SAVE_SIZE - 7 {
                overflow(b"save size", SAVE_SIZE);
            }
        }
        SAVE_STACK[SAVE_PTR].b16.s1 = INSERT_TOKEN;
        SAVE_STACK[SAVE_PTR].b16.s0 = LEVEL_ZERO;
        SAVE_STACK[SAVE_PTR].b32.s1 = t;
        SAVE_PTR += 1;
    };
}
pub(crate) unsafe fn unsave() {
    let mut p: i32 = 0;
    let mut l = 0_u16;
    let mut t: i32 = 0;
    let mut a: bool = false;
    a = false;
    if cur_level > LEVEL_ONE {
        cur_level -= 1;
        loop {
            SAVE_PTR -= 1;
            if SAVE_STACK[SAVE_PTR].b16.s1 == LEVEL_BOUNDARY {
                break;
            }
            p = SAVE_STACK[SAVE_PTR].b32.s1;
            if SAVE_STACK[SAVE_PTR].b16.s1 == INSERT_TOKEN {
                /*338: */
                t = cur_tok;
                cur_tok = p;
                if a {
                    p = get_avail() as i32;
                    MEM[p as usize].b32.s0 = cur_tok;
                    MEM[p as usize].b32.s1 = cur_input.loc;
                    cur_input.loc = p;
                    cur_input.start = p;
                    if cur_tok < RIGHT_BRACE_LIMIT {
                        if cur_tok < LEFT_BRACE_LIMIT {
                            align_state -= 1
                        } else {
                            align_state += 1
                        }
                    }
                } else {
                    back_input();
                    a = true
                }
                cur_tok = t
            } else if SAVE_STACK[SAVE_PTR].b16.s1 == RESTORE_SA {
                sa_restore();
                sa_chain = p;
                sa_level = SAVE_STACK[SAVE_PTR].b16.s0
            } else {
                if SAVE_STACK[SAVE_PTR].b16.s1 == RESTORE_OLD_VALUE {
                    l = SAVE_STACK[SAVE_PTR].b16.s0;
                    SAVE_PTR -= 1;
                } else {
                    SAVE_STACK[SAVE_PTR] = EQTB[UNDEFINED_CONTROL_SEQUENCE as usize];
                }
                if p < INT_BASE as i32 || p > EQTB_SIZE as i32 {
                    if EQTB[p as usize].b16.s0 == LEVEL_ONE {
                        eq_destroy(SAVE_STACK[SAVE_PTR]);
                    } else {
                        eq_destroy(EQTB[p as usize]);
                        EQTB[p as usize] = SAVE_STACK[SAVE_PTR]
                    }
                } else if _xeq_level_array[p as usize - INT_BASE] != LEVEL_ONE {
                    EQTB[p as usize] = SAVE_STACK[SAVE_PTR];
                    _xeq_level_array[p as usize - INT_BASE] = l
                }
            }
        }
        if GRP_STACK[IN_OPEN] == cur_boundary {
            group_warning();
        }
        cur_group = GroupCode::from(SAVE_STACK[SAVE_PTR].b16.s0);
        cur_boundary = SAVE_STACK[SAVE_PTR].b32.s1;
        SAVE_PTR -= 1;
    } else {
        confusion(b"curlevel");
    };
}
pub(crate) unsafe fn prepare_mag() {
    if mag_set > 0 && *INTPAR(IntPar::mag) != mag_set {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Incompatible magnification (");
        print_int(*INTPAR(IntPar::mag));
        print_cstr(b");");
        print_nl_cstr(b" the previous value will be retained");

        help_ptr = 2;
        help_line[1] = b"I can handle only one magnification ratio per job. So I\'ve";
        help_line[0] = b"reverted to the magnification you used earlier on this run.";
        int_error(mag_set);
        geq_word_define(INT_BASE as usize + IntPar::mag as usize, mag_set);
    }
    if *INTPAR(IntPar::mag) <= 0 || *INTPAR(IntPar::mag) as i64 > 32768 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Illegal magnification has been changed to 1000");

        help_ptr = 1;
        help_line[0] = b"The magnification ratio must be between 1 and 32768.";
        int_error(*INTPAR(IntPar::mag));
        geq_word_define(INT_BASE as usize + IntPar::mag as usize, 1000);
    }
    mag_set = *INTPAR(IntPar::mag);
}
pub(crate) unsafe fn token_show(p: Option<usize>) {
    if let Some(p) = p {
        show_token_list(MEM[p].b32.s1, TEX_NULL, 10000000);
    };
}
pub(crate) unsafe fn print_meaning() {
    print_cmd_chr(cur_cmd as u16, cur_chr);
    if cur_cmd as u16 >= CALL {
        print_char(':' as i32);
        print_ln();
        token_show(cur_chr.opt());
    } else if cur_cmd as u16 == TOP_BOT_MARK && cur_chr < 5 {
        print_char(':' as i32);
        print_ln();
        token_show(cur_mark[cur_chr as usize].opt());
    };
}
pub(crate) unsafe fn show_cur_cmd_chr() {
    let mut n: i32 = 0;
    let mut l: i32 = 0;
    let mut p: i32 = 0;
    begin_diagnostic();
    print_nl('{' as i32);
    if cur_list.mode as i32 != shown_mode as i32 {
        print_mode(cur_list.mode as i32);
        print_cstr(b": ");
        shown_mode = cur_list.mode
    }
    print_cmd_chr(cur_cmd as u16, cur_chr);
    if *INTPAR(IntPar::tracing_ifs) > 0 {
        if cur_cmd >= IF_TEST as u8 {
            if cur_cmd <= FI_OR_ELSE as u8 {
                print_cstr(b": ");
                if cur_cmd == FI_OR_ELSE as u8 {
                    print_cmd_chr(IF_TEST, cur_if as i32);
                    print_char(' ' as i32);
                    n = 0;
                    l = if_line
                } else {
                    n = 1;
                    l = line
                }
                p = cond_ptr;
                while !p.is_texnull() {
                    n += 1;
                    p = *LLIST_link(p as usize)
                }
                print_cstr(b"(level ");
                print_int(n);
                print_char(')' as i32);
                if l != 0 {
                    print_cstr(b" entered on line ");
                    print_int(l);
                }
            }
        }
    }
    print_char('}' as i32);
    end_diagnostic(false);
}
pub(crate) unsafe fn show_context() {
    let mut nn: i32 = 0;
    let mut bottom_line: bool = false;
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut l: i32 = 0;
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    BASE_PTR = INPUT_PTR;
    INPUT_STACK[BASE_PTR] = cur_input;
    nn = -1;
    bottom_line = false;
    loop {
        cur_input = INPUT_STACK[BASE_PTR];
        if cur_input.state != TOKEN_LIST {
            if cur_input.name > 19 || BASE_PTR == 0 {
                bottom_line = true
            }
        }
        if BASE_PTR == INPUT_PTR
            || bottom_line as i32 != 0
            || nn < *INTPAR(IntPar::error_context_lines)
        {
            /*324: */
            if BASE_PTR == INPUT_PTR
                || cur_input.state != TOKEN_LIST
                || cur_input.index != BACKED_UP
                || !cur_input.loc.is_texnull()
            {
                tally = 0i32;
                let old_setting_0 = selector;
                if cur_input.state != TOKEN_LIST {
                    if cur_input.name <= 17 {
                        if cur_input.name == 0 {
                            if BASE_PTR == 0 {
                                print_nl_cstr(b"<*>");
                            } else {
                                print_nl_cstr(b"<insert> ");
                            }
                        } else {
                            print_nl_cstr(b"<read ");
                            if cur_input.name == 17 {
                                print_char('*' as i32);
                            } else {
                                print_int(cur_input.name - 1);
                            }
                            print_char('>' as i32);
                        }
                    } else {
                        print_nl_cstr(b"l.");
                        if cur_input.index as usize == IN_OPEN {
                            print_int(line);
                        } else {
                            print_int(LINE_STACK[(cur_input.index as i32 + 1) as usize]);
                        }
                    }
                    print_char(' ' as i32);
                    l = tally;
                    tally = 0;
                    selector = Selector::PSEUDO;
                    trick_count = 1000000;
                    if BUFFER[cur_input.limit as usize] == *INTPAR(IntPar::end_line_char) {
                        j = cur_input.limit
                    } else {
                        j = cur_input.limit + 1
                    }
                    if j > 0 {
                        let mut for_end: i32 = 0;
                        i = cur_input.start;
                        for_end = j - 1;
                        if i <= for_end {
                            loop {
                                if i == cur_input.loc {
                                    first_count = tally;
                                    trick_count = tally + 1 + error_line - half_error_line;
                                    if trick_count < error_line {
                                        trick_count = error_line
                                    }
                                }
                                print_char(BUFFER[i as usize]);
                                let fresh23 = i;
                                i = i + 1;
                                if !(fresh23 < for_end) {
                                    break;
                                }
                            }
                        }
                    }
                } else {
                    match cur_input.index {
                        PARAMETER => print_nl_cstr(b"<argument> "),
                        U_TEMPLATE | V_TEMPLATE => print_nl_cstr(b"<template> "),
                        BACKED_UP | BACKED_UP_CHAR => {
                            if cur_input.loc.is_texnull() {
                                print_nl_cstr(b"<recently read> ");
                            } else {
                                print_nl_cstr(b"<to be read again> ");
                            }
                        }
                        INSERTED => print_nl_cstr(b"<inserted text> "),
                        MACRO => {
                            print_ln();
                            print_cs(cur_input.name);
                        }
                        OUTPUT_TEXT => print_nl_cstr(b"<output> "),
                        EVERY_PAR_TEXT => print_nl_cstr(b"<everypar> "),
                        EVERY_MATH_TEXT => print_nl_cstr(b"<everymath> "),
                        EVERY_DISPLAY_TEXT => print_nl_cstr(b"<everydisplay> "),
                        EVERY_HBOX_TEXT => print_nl_cstr(b"<everyhbox> "),
                        EVERY_VBOX_TEXT => print_nl_cstr(b"<everyvbox> "),
                        EVERY_JOB_TEXT => print_nl_cstr(b"<everyjob> "),
                        EVERY_CR_TEXT => print_nl_cstr(b"<everycr> "),
                        MARK_TEXT => print_nl_cstr(b"<mark> "),
                        EVERY_EOF_TEXT => print_nl_cstr(b"<everyeof> "),
                        INTER_CHAR_TEXT => print_nl_cstr(b"<XeTeXinterchartoks> "),
                        WRITE_TEXT => print_nl_cstr(b"<write> "),
                        TECTONIC_CODA_TEXT => print_nl_cstr(b"<TectonicCodaTokens> "),
                        _ => print_nl('?' as i32),
                    }
                    l = tally;
                    tally = 0i32;
                    selector = Selector::PSEUDO;
                    trick_count = 1_000_000;
                    if cur_input.index < MACRO {
                        show_token_list(cur_input.start, cur_input.loc, 100000);
                    } else {
                        show_token_list(
                            MEM[cur_input.start as usize].b32.s1,
                            cur_input.loc,
                            100_000,
                        );
                    }
                }
                selector = old_setting_0;
                if trick_count == 1_000_000 {
                    first_count = tally;
                    trick_count = tally + 1 + error_line - half_error_line;
                    if trick_count < error_line {
                        trick_count = error_line
                    }
                }
                if tally < trick_count {
                    m = tally - first_count
                } else {
                    m = trick_count - first_count
                }
                if l + first_count <= half_error_line {
                    p = 0;
                    n = l + first_count
                } else {
                    print_cstr(b"...");
                    p = l + first_count - half_error_line + 3;
                    n = half_error_line
                }
                let mut for_end_0: i32 = 0;
                q = p;
                for_end_0 = first_count - 1;
                if q <= for_end_0 {
                    loop {
                        print_raw_char(trick_buf[(q % error_line) as usize], true);
                        let fresh24 = q;
                        q = q + 1;
                        if !(fresh24 < for_end_0) {
                            break;
                        }
                    }
                }
                print_ln();
                let mut for_end_1: i32 = 0;
                q = 1;
                for_end_1 = n;
                if q <= for_end_1 {
                    loop {
                        print_raw_char(' ' as i32 as UTF16_code, true);
                        let fresh25 = q;
                        q = q + 1;
                        if !(fresh25 < for_end_1) {
                            break;
                        }
                    }
                }
                if m + n <= error_line {
                    p = first_count + m
                } else {
                    p = first_count + (error_line - n - 3i32)
                }
                let mut for_end_2: i32 = 0;
                q = first_count;
                for_end_2 = p - 1i32;
                if q <= for_end_2 {
                    loop {
                        print_raw_char(trick_buf[(q % error_line) as usize], true);
                        let fresh26 = q;
                        q = q + 1;
                        if !(fresh26 < for_end_2) {
                            break;
                        }
                    }
                }
                if m + n > error_line {
                    print_cstr(b"...");
                }
                nn += 1
            }
        } else if nn == *INTPAR(IntPar::error_context_lines) {
            print_nl_cstr(b"...");
            nn += 1
        }
        if bottom_line {
            break;
        }
        BASE_PTR -= 1
    }
    cur_input = INPUT_STACK[INPUT_PTR];
}
pub(crate) unsafe fn begin_token_list(p: usize, t: u16) {
    if INPUT_PTR > MAX_IN_STACK {
        MAX_IN_STACK = INPUT_PTR;
        if INPUT_PTR == STACK_SIZE {
            overflow(b"input stack size", STACK_SIZE);
        }
    }
    INPUT_STACK[INPUT_PTR] = cur_input;
    INPUT_PTR += 1;
    cur_input.state = 0_u16;
    cur_input.start = p as i32;
    cur_input.index = t;
    if t >= MACRO {
        MEM[p].b32.s0 += 1;
        if t == MACRO {
            cur_input.limit = PARAM_PTR as i32
        } else {
            cur_input.loc = MEM[p].b32.s1;
            if *INTPAR(IntPar::tracing_macros) > 1i32 {
                begin_diagnostic();
                print_nl_cstr(b"");
                match t {
                    MARK_TEXT => print_esc_cstr(b"mark"),
                    WRITE_TEXT => print_esc_cstr(b"write"),
                    _ => {
                        print_cmd_chr(
                            ASSIGN_TOKS,
                            (t as i32) + LOCAL_BASE as i32 + (Local::output_routine as i32)
                                - (OUTPUT_TEXT) as i32,
                        );
                    }
                }
                print_cstr(b"->");
                token_show(Some(p));
                end_diagnostic(false);
            }
        }
    } else {
        cur_input.loc = p as i32;
    };
}
pub(crate) unsafe fn end_token_list() {
    if cur_input.index >= BACKED_UP {
        if cur_input.index <= INSERTED {
            flush_list(cur_input.start.opt());
        } else {
            delete_token_ref(cur_input.start as usize);
            if cur_input.index == MACRO {
                while PARAM_PTR as i32 > cur_input.limit {
                    PARAM_PTR -= 1;
                    flush_list(PARAM_STACK[PARAM_PTR].opt());
                }
            }
        }
    } else if cur_input.index == U_TEMPLATE {
        if align_state > 500000 {
            align_state = 0
        } else {
            fatal_error(b"(interwoven alignment preambles are not allowed)");
        }
    }
    INPUT_PTR -= 1;
    cur_input = INPUT_STACK[INPUT_PTR];
}
pub(crate) unsafe fn back_input() {
    let mut p: i32 = 0;
    while cur_input.state == TOKEN_LIST
        && cur_input.loc.is_texnull()
        && cur_input.index != V_TEMPLATE
    {
        end_token_list();
    }
    p = get_avail() as i32;
    MEM[p as usize].b32.s0 = cur_tok;
    if cur_tok < RIGHT_BRACE_LIMIT {
        if cur_tok < LEFT_BRACE_LIMIT {
            align_state -= 1
        } else {
            align_state += 1
        }
    }
    if INPUT_PTR > MAX_IN_STACK {
        MAX_IN_STACK = INPUT_PTR;
        if INPUT_PTR == STACK_SIZE {
            overflow(b"input stack size", STACK_SIZE);
        }
    }
    INPUT_STACK[INPUT_PTR] = cur_input;
    INPUT_PTR += 1;
    cur_input.state = TOKEN_LIST;
    cur_input.start = p;
    cur_input.index = BACKED_UP;
    cur_input.loc = p;
}
pub(crate) unsafe fn back_error() {
    back_input();
    error();
}
pub(crate) unsafe fn ins_error() {
    back_input();
    cur_input.index = INSERTED;
    error();
}
pub(crate) unsafe fn begin_file_reading() {
    if IN_OPEN == MAX_IN_OPEN {
        overflow(b"text input levels", MAX_IN_OPEN);
    }
    if first as usize == BUF_SIZE {
        overflow(b"buffer size", BUF_SIZE);
    }
    IN_OPEN += 1;
    if INPUT_PTR > MAX_IN_STACK {
        MAX_IN_STACK = INPUT_PTR;
        if INPUT_PTR == STACK_SIZE {
            overflow(b"input stack size", STACK_SIZE);
        }
    }
    INPUT_STACK[INPUT_PTR] = cur_input;
    INPUT_PTR += 1;
    cur_input.index = IN_OPEN as u16;
    SOURCE_FILENAME_STACK[cur_input.index as usize] = 0;
    FULL_SOURCE_FILENAME_STACK[cur_input.index as usize] = 0;
    EOF_SEEN[cur_input.index as usize] = false;
    GRP_STACK[cur_input.index as usize] = cur_boundary;
    IF_STACK[cur_input.index as usize] = cond_ptr;
    LINE_STACK[cur_input.index as usize] = line;
    cur_input.start = first;
    cur_input.state = MID_LINE;
    cur_input.name = 0;
    cur_input.synctex_tag = 0;
}
pub(crate) unsafe fn end_file_reading() {
    first = cur_input.start;
    line = LINE_STACK[cur_input.index as usize];
    if cur_input.name == 18 || cur_input.name == 19 {
        pseudo_close();
    } else if cur_input.name > 17 {
        u_close(INPUT_FILE[cur_input.index as usize]);
    }
    INPUT_PTR -= 1;
    cur_input = INPUT_STACK[INPUT_PTR];
    IN_OPEN -= 1;
}
pub(crate) unsafe fn check_outer_validity() {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    if scanner_status as u16 != NORMAL {
        deletions_allowed = false;
        if cur_cs != 0 {
            if cur_input.state == TOKEN_LIST || cur_input.name < 1 || cur_input.name > 17 {
                p = get_avail() as i32;
                MEM[p as usize].b32.s0 = CS_TOKEN_FLAG + cur_cs;
                begin_token_list(p as usize, BACKED_UP);
            }
            cur_cmd = SPACER as u8;
            cur_chr = ' ' as i32
        }
        if scanner_status as u16 > SKIPPING {
            /*350:*/
            runaway();
            if cur_cs == 0 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"File ended");
            } else {
                cur_cs = 0;
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Forbidden control sequence found");
            }
            p = get_avail() as i32;
            match scanner_status as u16 {
                DEFINING => {
                    print_cstr(b" while scanning definition");
                    MEM[p as usize].b32.s0 = RIGHT_BRACE_TOKEN + '}' as i32
                }
                MATCHING => {
                    print_cstr(b" while scanning use");
                    MEM[p as usize].b32.s0 = par_token;
                    long_state = OUTER_CALL as u8
                }
                ALIGNING => {
                    print_cstr(b" while scanning preamble");
                    MEM[p as usize].b32.s0 = RIGHT_BRACE_TOKEN + '}' as i32;
                    q = p;
                    p = get_avail() as i32;
                    MEM[p as usize].b32.s1 = q;
                    MEM[p as usize].b32.s0 = CS_TOKEN_FLAG + FROZEN_CR as i32;
                    align_state = -1_000_000;
                }
                ABSORBING => {
                    print_cstr(b" while scanning text");
                    MEM[p as usize].b32.s0 = RIGHT_BRACE_TOKEN + '}' as i32
                }
                _ => {}
            }
            begin_token_list(p as usize, INSERTED);
            print_cstr(b" of ");
            sprint_cs(warning_index);
            help_ptr = 4;
            help_line[3] = b"I suspect you have forgotten a `}\', causing me";
            help_line[2] = b"to read past where you wanted me to stop.";
            help_line[1] = b"I\'ll try to recover; but if the error is serious,";
            help_line[0] = b"you\'d better type `E\' or `X\' now and fix your file.";
            error();
        } else {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Incomplete ");
            print_cmd_chr(IF_TEST, cur_if as i32);
            print_cstr(b"; all text was ignored after line ");
            print_int(skip_line);
            help_ptr = 3;
            help_line[2] = b"A forbidden control sequence occurred in skipped text.";
            help_line[1] = b"This kind of error happens when you say `\\if...\' and forget";
            help_line[0] = b"the matching `\\fi\'. I\'ve inserted a `\\fi\'; this might work.";
            if cur_cs != 0 {
                cur_cs = 0
            } else {
                help_line[2] = b"The file ended while I was skipping conditional text."
            }
            cur_tok = CS_TOKEN_FLAG + FROZEN_FI as i32;
            ins_error();
        }
        deletions_allowed = true
    };
}
/* These macros are kinda scary, but convenient */
pub(crate) unsafe fn get_next() {
    let mut current_block: u64;
    let mut k: i32 = 0;
    let mut t: i32 = 0;
    let mut cat: u8 = 0;
    let mut c: UnicodeScalar = 0;
    let mut lower: UTF16_code = 0;
    let mut d: small_number = 0;
    let mut sup_count: small_number = 0;
    'c_63502: loop {
        cur_cs = 0i32;
        if cur_input.state != TOKEN_LIST {
            /*355:*/
            'c_63807: loop
            /*357:*/
            {
                if cur_input.loc <= cur_input.limit {
                    cur_chr = BUFFER[cur_input.loc as usize];
                    cur_input.loc += 1;
                    if cur_chr >= 0xd800
                        && cur_chr < 0xdc00
                        && cur_input.loc <= cur_input.limit
                        && BUFFER[cur_input.loc as usize] >= 0xdc00
                        && BUFFER[cur_input.loc as usize] < 0xe000
                    {
                        lower = (BUFFER[cur_input.loc as usize] - 0xdc00) as UTF16_code;
                        cur_input.loc += 1;
                        cur_chr = (65536 + ((cur_chr - 0xd800) * 1024) as i64 + lower as i64) as i32
                    }
                    'c_65186: loop {
                        cur_cmd = *CAT_CODE(cur_chr as usize) as eight_bits;
                        match cur_input.state as i32 + cur_cmd as i32 {
                            10 | 26 | 42 | 27 | 43 => break, // ANY_STATE_PLUS(IGNORE) | SKIP_BLANKS + SPACER | NEW_LINE + SPACER
                            1 | 17 | 33 => {
                                // ANY_STATE_PLUS(ESCAPE)
                                if cur_input.loc > cur_input.limit {
                                    current_block = 17833034027772472439;
                                    break 'c_63807;
                                } else {
                                    current_block = 7720778817628725688;
                                    break 'c_63807;
                                }
                            }
                            14 | 30 | 46 => {
                                //  ANY_STATE_PLUS(ACTIVE_CHAR)
                                cur_cs = cur_chr + 1;
                                cur_cmd = EQTB[cur_cs as usize].b16.s1 as eight_bits;
                                cur_chr = EQTB[cur_cs as usize].b32.s1;
                                cur_input.state = MID_LINE;
                                if cur_cmd >= OUTER_CALL as u8 {
                                    check_outer_validity();
                                }
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            8 | 24 | 40 => {
                                // ANY_STATE_PLUS(SUP_MARK)
                                if !(cur_chr == BUFFER[cur_input.loc as usize]) {
                                    current_block = 8567661057257693057;
                                    break 'c_63807;
                                }
                                if !(cur_input.loc < cur_input.limit) {
                                    current_block = 8567661057257693057;
                                    break 'c_63807;
                                }
                                sup_count = 2i32 as small_number;
                                while (sup_count as i32) < 6i32
                                    && cur_input.loc + 2i32 * sup_count as i32 - 2i32
                                        <= cur_input.limit
                                    && cur_chr
                                        == BUFFER[(cur_input.loc + sup_count as i32 - 1) as usize]
                                {
                                    sup_count += 1
                                }
                                d = 1i32 as small_number;
                                while d as i32 <= sup_count as i32 {
                                    if !(BUFFER[(cur_input.loc + sup_count as i32 - 2 + d as i32)
                                        as usize]
                                        >= '0' as i32
                                        && BUFFER[(cur_input.loc + sup_count as i32 - 2 + d as i32)
                                            as usize]
                                            <= '9' as i32
                                        || BUFFER[(cur_input.loc + sup_count as i32 - 2 + d as i32)
                                            as usize]
                                            >= 'a' as i32
                                            && BUFFER[(cur_input.loc + sup_count as i32 - 2
                                                + d as i32)
                                                as usize]
                                                <= 'f' as i32)
                                    {
                                        c = BUFFER[(cur_input.loc + 1i32) as usize];
                                        if !(c < 128i32) {
                                            current_block = 8567661057257693057;
                                            break 'c_63807;
                                        }
                                        cur_input.loc = cur_input.loc + 2i32;
                                        if c < 64i32 {
                                            cur_chr = c + 64i32
                                        } else {
                                            cur_chr = c - 64i32
                                        }
                                        continue 'c_65186;
                                    } else {
                                        d += 1
                                    }
                                }
                                cur_chr = 0i32;
                                d = 1i32 as small_number;
                                while d as i32 <= sup_count as i32 {
                                    c = BUFFER[(cur_input.loc + sup_count as i32 - 2 + d as i32)
                                        as usize];
                                    if c <= '9' as i32 {
                                        cur_chr = 16i32 * cur_chr + c - '0' as i32
                                    } else {
                                        cur_chr = 16i32 * cur_chr + c - 'a' as i32 + 10i32
                                    }
                                    d += 1
                                }
                                if cur_chr > 0x10ffffi32 {
                                    cur_chr = BUFFER[cur_input.loc as usize];
                                    current_block = 8567661057257693057;
                                    break 'c_63807;
                                } else {
                                    cur_input.loc = cur_input.loc + 2i32 * sup_count as i32 - 1i32
                                }
                            }
                            16 | 32 | 48 => {
                                // ANY_STATE_PLUS(INVALID_CHAR)
                                if file_line_error_style_p != 0 {
                                    print_file_line();
                                } else {
                                    print_nl_cstr(b"! ");
                                }
                                print_cstr(b"Text line contains an invalid character");
                                help_ptr = 2_u8;
                                help_line[1] =
                                    b"A funny symbol that I can\'t read has just been input.";
                                help_line[0] = b"Continue, and I\'ll forget that it ever happened.";
                                deletions_allowed = false;
                                error();
                                deletions_allowed = true;
                                continue 'c_63502;
                            }
                            11 => {
                                // MID_LINE + SPACER
                                cur_input.state = SKIP_BLANKS as u16;
                                cur_chr = ' ' as i32;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            6 => {
                                // MID_LINE + CAR_RET
                                cur_input.loc = cur_input.limit + 1;
                                cur_cmd = SPACER as u8;
                                cur_chr = ' ' as i32;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            15 | 31 | 47 | 22 => {
                                // ANY_STATE_PLUS(COMMENT) | SKIP_BLANKS + CAR_RET
                                cur_input.loc = cur_input.limit + 1;
                                break;
                            }
                            38 => {
                                // NEW_LINE + CAR_RET
                                cur_input.loc = cur_input.limit + 1;
                                cur_cs = par_loc;
                                cur_cmd = EQTB[cur_cs as usize].b16.s1 as eight_bits;
                                cur_chr = EQTB[cur_cs as usize].b32.s1;
                                if cur_cmd >= OUTER_CALL as u8 {
                                    check_outer_validity();
                                }
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            2 => {
                                // MID_LINE + LEFT_BRACE
                                align_state += 1;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            18 | 34 => {
                                // SKIP_BLANKS + LEFT_BRACE | NEW_LINE + LEFT_BRACE
                                cur_input.state = MID_LINE;
                                align_state += 1;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            3 => {
                                // MID_LINE + RIGHT_BRACE
                                align_state -= 1;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            19 | 35 => {
                                // SKIP_BLANKS + RIGHT_BRACE | NEW_LINE + RIGHT_BRACE
                                cur_input.state = MID_LINE;
                                align_state -= 1;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            20 | 21 | 23 | 25 | 28 | 29 | 36 | 37 | 39 | 41 | 44 | 45 => {
                                // ADD_DELIMS_TO(SKIP_BLANKS) | ADD_DELIMS_TO(NEW_LINE)
                                cur_input.state = MID_LINE;
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                            _ => {
                                current_block = 14956172121224201915;
                                break 'c_63807;
                            }
                        }
                    }
                } else {
                    cur_input.state = NEW_LINE;
                    if cur_input.name > 17 {
                        /*374:*/
                        line += 1; /*367:*/
                        first = cur_input.start;
                        if !force_eof {
                            if cur_input.name <= 19 {
                                if pseudo_input() {
                                    cur_input.limit = last
                                } else if !LOCAL(Local::every_eof).is_texnull()
                                    && !EOF_SEEN[cur_input.index as usize]
                                {
                                    cur_input.limit = first - 1i32;
                                    EOF_SEEN[cur_input.index as usize] = true;
                                    begin_token_list(
                                        *LOCAL(Local::every_eof) as usize,
                                        EVERY_EOF_TEXT,
                                    );
                                    continue 'c_63502;
                                } else {
                                    force_eof = true
                                }
                            } else if input_line(INPUT_FILE[cur_input.index as usize]) != 0 {
                                cur_input.limit = last
                            } else if !LOCAL(Local::every_eof).is_texnull()
                                && !EOF_SEEN[cur_input.index as usize]
                            {
                                cur_input.limit = first - 1;
                                EOF_SEEN[cur_input.index as usize] = true;
                                begin_token_list(*LOCAL(Local::every_eof) as usize, EVERY_EOF_TEXT);
                                continue 'c_63502;
                            } else {
                                force_eof = true
                            }
                        }
                        if force_eof {
                            if *INTPAR(IntPar::tracing_nesting) > 0 {
                                if GRP_STACK[IN_OPEN] != cur_boundary
                                    || IF_STACK[IN_OPEN] != cond_ptr
                                {
                                    file_warning();
                                }
                            }
                            if cur_input.name >= 19 {
                                print_char(')' as i32);
                                open_parens -= 1;
                                rust_stdout.as_mut().unwrap().flush().unwrap();
                            }
                            force_eof = false;
                            end_file_reading();
                            check_outer_validity();
                            continue 'c_63502;
                        } else {
                            if *INTPAR(IntPar::end_line_char) < 0
                                || *INTPAR(IntPar::end_line_char) > 255
                            {
                                cur_input.limit -= 1
                            } else {
                                BUFFER[cur_input.limit as usize] = *INTPAR(IntPar::end_line_char)
                            }
                            first = cur_input.limit + 1;
                            cur_input.loc = cur_input.start
                        }
                    } else {
                        if cur_input.name != 0 {
                            cur_cmd = 0;
                            cur_chr = 0;
                            return;
                        }
                        if INPUT_PTR > 0 {
                            current_block = 4001239642700071046;
                            break;
                        } else {
                            current_block = 15055213890147597004;
                            break;
                        }
                    }
                }
            }
            match current_block {
                14956172121224201915 => {}
                _ => {
                    match current_block {
                        8567661057257693057 => {
                            cur_input.state = 1_u16;
                            current_block = 14956172121224201915;
                        }
                        7720778817628725688 => {
                            'c_65963: loop {
                                k = cur_input.loc;
                                cur_chr = BUFFER[k as usize];
                                cat = *CAT_CODE(cur_chr as usize) as u8;
                                k += 1;
                                if cat == LETTER as u8 {
                                    cur_input.state = SKIP_BLANKS as u16
                                } else if cat == SPACER as u8 {
                                    cur_input.state = SKIP_BLANKS as u16;
                                } else {
                                    cur_input.state = MID_LINE;
                                }
                                if cat == LETTER as u8 && k <= cur_input.limit {
                                    loop
                                    /*368:*/
                                    {
                                        cur_chr = BUFFER[k as usize];
                                        cat = *CAT_CODE(cur_chr as usize) as u8;
                                        k += 1;
                                        if !(cat == LETTER as u8 && k <= cur_input.limit) {
                                            break;
                                        }
                                    }
                                    if !(cat == SUP_MARK as u8
                                        && BUFFER[k as usize] == cur_chr
                                        && k < cur_input.limit)
                                    {
                                        current_block = 5873035170358615968;
                                        break;
                                    }
                                    /* Special characters: either ^^X, or up to six
                                     * ^'s followed by one hex character for each
                                     * ^. */
                                    let mut sup_count_save: i32 = 0;
                                    /* How many ^'s are there? */
                                    sup_count = 2;
                                    while sup_count < 6
                                        && k + 2 * sup_count as i32 - 2 <= cur_input.limit
                                        && BUFFER[(k + sup_count as i32 - 1) as usize] == cur_chr
                                    {
                                        sup_count += 1
                                    }
                                    /* If they are followed by a sufficient number of
                                     * hex characters, treat it as an extended ^^^
                                     * sequence. If not, treat it as original-style
                                     * ^^X. */
                                    sup_count_save = sup_count as i32;
                                    d = 1;
                                    while d as i32 <= sup_count_save {
                                        if !(BUFFER[(k + sup_count as i32 - 2 + d as i32) as usize]
                                            >= '0' as i32
                                            && BUFFER
                                                [(k + sup_count as i32 - 2 + d as i32) as usize]
                                                <= '9' as i32
                                            || BUFFER
                                                [(k + sup_count as i32 - 2 + d as i32) as usize]
                                                >= 'a' as i32
                                                && BUFFER[(k + sup_count as i32 - 2 + d as i32)
                                                    as usize]
                                                    <= 'f' as i32)
                                        {
                                            /* Non-hex: do it old style */
                                            c = BUFFER[(k + 1i32) as usize];
                                            if c < 128i32 {
                                                if c < 64i32 {
                                                    BUFFER[(k - 1i32) as usize] = c + 64i32
                                                } else {
                                                    BUFFER[(k - 1i32) as usize] = c - 64i32
                                                }
                                                d = 2i32 as small_number;
                                                cur_input.limit = cur_input.limit - d as i32;
                                                while k <= cur_input.limit {
                                                    BUFFER[k as usize] =
                                                        BUFFER[(k + d as i32) as usize];
                                                    k += 1
                                                }
                                                continue 'c_65963;
                                            } else {
                                                sup_count = 0i32 as small_number
                                            }
                                        }
                                        d += 1
                                    }
                                    if !(sup_count as i32 > 0i32) {
                                        current_block = 5873035170358615968;
                                        break;
                                    }
                                    cur_chr = 0i32;
                                    d = 1i32 as small_number;
                                    while d as i32 <= sup_count as i32 {
                                        c = BUFFER[(k + sup_count as i32 - 2 + d as i32) as usize];
                                        if c <= '9' as i32 {
                                            cur_chr = 16i32 * cur_chr + c - '0' as i32
                                        } else {
                                            cur_chr = 16i32 * cur_chr + c - 'a' as i32 + 10i32
                                        }
                                        d += 1
                                    }
                                    if cur_chr > 0x10ffffi32 {
                                        cur_chr = BUFFER[k as usize];
                                        current_block = 5873035170358615968;
                                        break;
                                    } else {
                                        BUFFER[(k - 1i32) as usize] = cur_chr;
                                        d = (2i32 * sup_count as i32 - 1i32) as small_number;
                                        cur_input.limit = cur_input.limit - d as i32;
                                        while k <= cur_input.limit {
                                            BUFFER[k as usize] = BUFFER[(k + d as i32) as usize];
                                            k += 1
                                        }
                                    }
                                } else {
                                    if !(cat as i32 == 7i32
                                        && BUFFER[k as usize] == cur_chr
                                        && k < cur_input.limit)
                                    {
                                        current_block = 1604201581803946138;
                                        break;
                                    }
                                    let mut sup_count_save_0: i32 = 0;
                                    sup_count = 2i32 as small_number;
                                    while (sup_count as i32) < 6i32
                                        && k + 2i32 * sup_count as i32 - 2i32 <= cur_input.limit
                                        && BUFFER[(k + sup_count as i32 - 1i32) as usize] == cur_chr
                                    {
                                        sup_count += 1
                                    }
                                    sup_count_save_0 = sup_count as i32;
                                    d = 1 as small_number;
                                    while d as i32 <= sup_count_save_0 {
                                        if !IS_LC_HEX(
                                            BUFFER[(k + sup_count as i32 - 2 + d as i32) as usize],
                                        ) {
                                            c = BUFFER[(k + 1) as usize];
                                            if c < 128 {
                                                if c < 64 {
                                                    BUFFER[(k - 1) as usize] = c + 64
                                                } else {
                                                    BUFFER[(k - 1) as usize] = c - 64
                                                }
                                                d = 2;
                                                cur_input.limit = cur_input.limit - d as i32;
                                                while k <= cur_input.limit {
                                                    BUFFER[k as usize] =
                                                        BUFFER[(k + d as i32) as usize];
                                                    k += 1
                                                }
                                                continue 'c_65963;
                                            } else {
                                                sup_count = 0;
                                            }
                                        }
                                        d += 1
                                    }
                                    if !(sup_count > 0) {
                                        current_block = 1604201581803946138;
                                        break;
                                    }
                                    cur_chr = 0;
                                    d = 1 as small_number;
                                    while d as i32 <= sup_count as i32 {
                                        c = BUFFER[(k + sup_count as i32 - 2 + d as i32) as usize];
                                        if c <= '9' as i32 {
                                            cur_chr = 16 * cur_chr + c - '0' as i32
                                        } else {
                                            cur_chr = 16 * cur_chr + c - 'a' as i32 + 10
                                        }
                                        d += 1
                                    }
                                    if cur_chr > BIGGEST_USV as i32 {
                                        cur_chr = BUFFER[k as usize];
                                        current_block = 1604201581803946138;
                                        break;
                                    } else {
                                        BUFFER[(k - 1) as usize] = cur_chr;
                                        d = (2 * sup_count as i32 - 1) as small_number;
                                        cur_input.limit = cur_input.limit - d as i32;
                                        while k <= cur_input.limit {
                                            BUFFER[k as usize] = BUFFER[(k + d as i32) as usize];
                                            k += 1
                                        }
                                    }
                                }
                            }
                            match current_block {
                                5873035170358615968 => {
                                    if cat != LETTER as u8 {
                                        k -= 1
                                    }
                                    if k > cur_input.loc + 1 {
                                        cur_cs = id_lookup(cur_input.loc, k - cur_input.loc);
                                        cur_input.loc = k;
                                        current_block = 10802200937357087535;
                                    } else {
                                        current_block = 1604201581803946138;
                                    }
                                }
                                _ => {}
                            }
                            match current_block {
                                10802200937357087535 => {}
                                _ => {
                                    if BUFFER[cur_input.loc as usize] as i64 > 65535 {
                                        cur_cs = id_lookup(cur_input.loc, 1i32);
                                        cur_input.loc += 1
                                    } else {
                                        cur_cs = 1i32
                                            + (0x10ffffi32 + 1i32)
                                            + BUFFER[cur_input.loc as usize];
                                        cur_input.loc += 1
                                    }
                                    current_block = 10802200937357087535;
                                }
                            }
                        }
                        17833034027772472439 => {
                            cur_cs = 1i32 + (0x10ffffi32 + 1i32) + (0x10ffffi32 + 1i32);
                            current_block = 10802200937357087535;
                        }
                        4001239642700071046 => {
                            end_file_reading();
                            continue;
                        }
                        _ =>
                        /* Tectonic extension: we add a \TectonicCodaTokens toklist
                         * that gets inserted at the very very end of processing if no
                         * \end or \dump has been seen. We just use a global state
                         * variable to make sure it only gets inserted once. */
                        {
                            if !used_tectonic_coda_tokens
                                && !LOCAL(Local::TectonicCodaTokens).is_texnull()
                            {
                                used_tectonic_coda_tokens = true; /* token list but no tokens left */
                                begin_token_list(
                                    *LOCAL(Local::TectonicCodaTokens) as usize,
                                    19_u16,
                                );
                                continue;
                            } else {
                                if u8::from(selector) < u8::from(Selector::LOG_ONLY) {
                                    open_log_file();
                                }
                                fatal_error(b"*** (job aborted, no legal \\end found)");
                            }
                        }
                    }
                    match current_block {
                        14956172121224201915 => {}
                        _ => {
                            cur_cmd = EQTB[cur_cs as usize].b16.s1 as eight_bits;
                            cur_chr = EQTB[cur_cs as usize].b32.s1;
                            if cur_cmd as i32 >= 115i32 {
                                check_outer_validity();
                            }
                        }
                    }
                }
            }
        } else if !cur_input.loc.is_texnull() {
            /* if we're inputting from a non-null token list: */
            t = MEM[cur_input.loc as usize].b32.s0;
            cur_input.loc = MEM[cur_input.loc as usize].b32.s1;
            if t >= CS_TOKEN_FLAG {
                cur_cs = t - CS_TOKEN_FLAG;
                cur_cmd = EQTB[cur_cs as usize].b16.s1 as u8;
                cur_chr = EQTB[cur_cs as usize].b32.s1;
                if cur_cmd >= OUTER_CALL as u8 {
                    if cur_cmd == DONT_EXPAND as u8 {
                        /*370:*/
                        cur_cs = MEM[cur_input.loc as usize].b32.s0 - CS_TOKEN_FLAG;
                        cur_input.loc = TEX_NULL;
                        cur_cmd = EQTB[cur_cs as usize].b16.s1 as u8;
                        cur_chr = EQTB[cur_cs as usize].b32.s1;
                        if cur_cmd > MAX_COMMAND as u8 {
                            cur_cmd = RELAX as u8;
                            cur_chr = NO_EXPAND_FLAG;
                        }
                    } else {
                        check_outer_validity();
                    }
                }
            } else {
                cur_cmd = (t / MAX_CHAR_VAL) as eight_bits;
                cur_chr = t % MAX_CHAR_VAL;
                match cur_cmd as i32 {
                    1 => {
                        current_block = 17818108259648334471;
                        align_state += 1;
                    }
                    2 => {
                        current_block = 1336783539463924428;
                        align_state -= 1;
                    }
                    5 => {
                        current_block = 1132450443677887731;
                        begin_token_list(
                            PARAM_STACK[(cur_input.limit + cur_chr - 1) as usize] as usize,
                            0_u16,
                        );
                        continue;
                    }
                    _ => {}
                }
            }
        } else {
            end_token_list();
            continue;
        }
        if cur_cmd <= CAR_RET as u8 && cur_cmd >= TAB_MARK as u8 && align_state == 0 {
            /*818:*/
            if scanner_status == ALIGNING as u8 || cur_align.is_texnull() {
                fatal_error(b"(interwoven alignment preambles are not allowed)");
            }
            cur_cmd = MEM[(cur_align + 5) as usize].b32.s0 as eight_bits;
            MEM[(cur_align + 5) as usize].b32.s0 = cur_chr;
            if cur_cmd == OMIT as u8 {
                begin_token_list(OMIT_TEMPLATE, V_TEMPLATE);
            } else {
                begin_token_list(MEM[(cur_align + 2) as usize].b32.s1 as usize, V_TEMPLATE);
            }
            align_state = 1000000;
        } else {
            return;
        }
    }
}
pub(crate) unsafe fn get_token() {
    no_new_control_sequence = false;
    get_next();
    no_new_control_sequence = true;
    if cur_cs == 0i32 {
        cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
    } else {
        cur_tok = CS_TOKEN_FLAG + cur_cs
    };
}
pub(crate) unsafe fn macro_call() {
    let mut current_block: u64;
    let mut r: i32 = 0;
    let mut p: i32 = TEX_NULL;
    let mut q: i32 = 0;
    let mut s: i32 = 0;
    let mut t: i32 = 0;
    let mut u: i32 = 0;
    let mut v: i32 = 0;
    let mut rbrace_ptr: i32 = TEX_NULL;
    let mut n: small_number = 0;
    let mut unbalance: i32 = 0;
    let mut m: i32 = 0i32;
    let mut save_scanner_status: small_number = 0;
    let mut save_warning_index: i32 = 0;
    let mut match_chr: UTF16_code = 0;
    save_scanner_status = scanner_status as small_number;
    save_warning_index = warning_index;
    warning_index = cur_cs;
    let ref_count = cur_chr as usize;
    r = MEM[ref_count].b32.s1;
    n = 0 as small_number;
    if *INTPAR(IntPar::tracing_macros) > 0 {
        /*419:*/
        begin_diagnostic();
        print_ln();
        print_cs(warning_index);
        token_show(Some(ref_count));
        end_diagnostic(false);
    }
    if MEM[r as usize].b32.s0 == PROTECTED_TOKEN {
        r = MEM[r as usize].b32.s1
    }
    if MEM[r as usize].b32.s0 != END_MATCH_TOKEN {
        /*409:*/
        scanner_status = MATCHING as u8;
        unbalance = 0;
        long_state = EQTB[cur_cs as usize].b16.s1 as u8;
        if long_state as u16 >= OUTER_CALL {
            long_state = (long_state as i32 - 2) as u8
        }
        's_135: loop {
            MEM[TEMP_HEAD].b32.s1 = TEX_NULL;
            if MEM[r as usize].b32.s0 >= END_MATCH_TOKEN || MEM[r as usize].b32.s0 < MATCH_TOKEN {
                s = TEX_NULL
            } else {
                match_chr = (MEM[r as usize].b32.s0 - MATCH_TOKEN) as UTF16_code;
                s = MEM[r as usize].b32.s1;
                r = s;
                p = TEMP_HEAD as i32;
                m = 0;
            }
            'c_67378: loop {
                get_token();
                if cur_tok == MEM[r as usize].b32.s0 {
                    /*412:*/
                    r = MEM[r as usize].b32.s1;
                    if !(MEM[r as usize].b32.s0 >= MATCH_TOKEN
                        && MEM[r as usize].b32.s0 <= END_MATCH_TOKEN)
                    {
                        continue;
                    }
                    if cur_tok < LEFT_BRACE_LIMIT {
                        align_state -= 1
                    }
                    break;
                } else {
                    if s != r {
                        if s.is_texnull() {
                            /*416:*/
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! ");
                            }
                            print_cstr(b"Use of ");
                            sprint_cs(warning_index);
                            print_cstr(b" doesn\'t match its definition");
                            help_ptr = 4;
                            help_line[3] =
                                b"If you say, e.g., `\\def\\a1{...}\', then you must always";
                            help_line[2] =
                                b"put `1\' after `\\a\', since control sequence names are";
                            help_line[1] = b"made up of letters only. The macro here has not been";
                            help_line[0] = b"followed by the required stuff, so I\'m ignoring it.";
                            error();
                            current_block = 16670727159935121194;
                            break 's_135;
                        } else {
                            t = s;
                            loop {
                                q = get_avail() as i32;
                                MEM[p as usize].b32.s1 = q;
                                MEM[q as usize].b32.s0 = MEM[t as usize].b32.s0;
                                p = q;
                                m += 1;
                                u = MEM[t as usize].b32.s1;
                                v = s;
                                loop {
                                    if u == r {
                                        if cur_tok != MEM[v as usize].b32.s0 {
                                            break;
                                        }
                                        r = MEM[v as usize].b32.s1;
                                        continue 'c_67378;
                                    } else {
                                        if MEM[u as usize].b32.s0 != MEM[v as usize].b32.s0 {
                                            break;
                                        }
                                        u = *LLIST_link(u as usize);
                                        v = *LLIST_link(v as usize);
                                    }
                                }
                                t = *LLIST_link(t as usize);
                                if !(t != r) {
                                    break;
                                }
                            }
                            r = s
                        }
                    }
                    if cur_tok == par_token {
                        if long_state as u16 != LONG_CALL {
                            /*414:*/
                            if long_state as u16 == CALL {
                                runaway(); /*411:*/
                                if file_line_error_style_p != 0 {
                                    print_file_line(); /*413:*/
                                } else {
                                    print_nl_cstr(b"! ");
                                }
                                print_cstr(b"Paragraph ended before ");
                                sprint_cs(warning_index);
                                print_cstr(b" was complete");
                                help_ptr = 3;
                                help_line[2] =
                                    b"I suspect you\'ve forgotten a `}\', causing me to apply this";
                                help_line[1] =
                                    b"control sequence to too much text. How can we recover?";
                                help_line[0] =
                                    b"My plan is to forget the whole thing and hope for the best.";
                                back_error();
                            }
                            pstack[n as usize] = MEM[TEMP_HEAD].b32.s1;
                            align_state = align_state - unbalance;
                            m = 0i32;
                            while m <= n as i32 {
                                flush_list(pstack[m as usize].opt());
                                m += 1
                            }
                            current_block = 16670727159935121194;
                            break 's_135;
                        }
                    }
                    if cur_tok < RIGHT_BRACE_LIMIT {
                        if cur_tok < LEFT_BRACE_LIMIT {
                            /*417:*/
                            unbalance = 1;
                            loop {
                                q = avail;
                                if q.is_texnull() {
                                    q = get_avail() as i32;
                                } else {
                                    avail = MEM[q as usize].b32.s1;
                                    MEM[q as usize].b32.s1 = TEX_NULL
                                }
                                MEM[p as usize].b32.s1 = q;
                                MEM[q as usize].b32.s0 = cur_tok;
                                p = q;
                                get_token();
                                if cur_tok == par_token {
                                    if long_state as u16 != LONG_CALL {
                                        /*414:*/
                                        if long_state as u16 == CALL {
                                            runaway();
                                            if file_line_error_style_p != 0 {
                                                print_file_line();
                                            } else {
                                                print_nl_cstr(b"! ");
                                            }
                                            print_cstr(b"Paragraph ended before ");
                                            sprint_cs(warning_index);
                                            print_cstr(b" was complete");
                                            help_ptr = 3_u8;
                                            help_line[2] =
                                                        b"I suspect you\'ve forgotten a `}\', causing me to apply this";
                                            help_line[1] =
                                                        b"control sequence to too much text. How can we recover?";
                                            help_line[0] =
                                                        b"My plan is to forget the whole thing and hope for the best.";
                                            back_error();
                                        }
                                        pstack[n as usize] = MEM[TEMP_HEAD].b32.s1;
                                        align_state = align_state - unbalance;
                                        m = 0i32;
                                        while m <= n as i32 {
                                            flush_list(pstack[m as usize].opt());
                                            m += 1
                                        }
                                        current_block = 16670727159935121194;
                                        break 's_135;
                                    }
                                }
                                if !(cur_tok < 0x600000i32) {
                                    continue;
                                }
                                if cur_tok < 0x400000i32 {
                                    unbalance += 1
                                } else {
                                    unbalance -= 1;
                                    if unbalance == 0i32 {
                                        break;
                                    }
                                }
                            }
                            rbrace_ptr = p;
                            q = get_avail() as i32;
                            MEM[p as usize].b32.s1 = q;
                            MEM[q as usize].b32.s0 = cur_tok;
                            p = q
                        } else {
                            back_input();
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! ");
                            }
                            print_cstr(b"Argument of ");
                            sprint_cs(warning_index);
                            print_cstr(b" has an extra }");
                            help_ptr = 6_u8;
                            help_line[5] =
                                b"I\'ve run across a `}\' that doesn\'t seem to match anything.";
                            help_line[4] =
                                b"For example, `\\def\\a#1{...}\' and `\\a}\' would produce";
                            help_line[3] =
                                b"this error. If you simply proceed now, the `\\par\' that";
                            help_line[2] = b"I\'ve just inserted will cause me to report a runaway";
                            help_line[1] =
                                b"argument that might be the root of the problem. But if";
                            help_line[0] =
                                b"your `}\' was spurious, just type `2\' and it will go away.";
                            align_state += 1;
                            long_state = CALL as u8;
                            cur_tok = par_token;
                            ins_error();
                            continue;
                        }
                    } else {
                        if cur_tok == SPACE_TOKEN {
                            if MEM[r as usize].b32.s0 <= END_MATCH_TOKEN {
                                if MEM[r as usize].b32.s0 >= MATCH_TOKEN {
                                    continue;
                                }
                            }
                        }
                        q = get_avail() as i32;
                        MEM[p as usize].b32.s1 = q;
                        MEM[q as usize].b32.s0 = cur_tok;
                        p = q
                    }
                    m += 1;
                    if MEM[r as usize].b32.s0 > END_MATCH_TOKEN {
                        continue;
                    }
                    if !(MEM[r as usize].b32.s0 < MATCH_TOKEN) {
                        break;
                    }
                }
            }
            if !s.is_texnull() {
                /*418:*/
                if m == 1 && MEM[p as usize].b32.s0 < RIGHT_BRACE_LIMIT && p != TEMP_HEAD as i32 {
                    MEM[rbrace_ptr as usize].b32.s1 = TEX_NULL;
                    MEM[p as usize].b32.s1 = avail;
                    avail = p;
                    p = MEM[TEMP_HEAD].b32.s1;
                    pstack[n as usize] = MEM[p as usize].b32.s1;
                    MEM[p as usize].b32.s1 = avail;
                    avail = p
                } else {
                    pstack[n as usize] = MEM[TEMP_HEAD].b32.s1
                }
                n += 1;
                if *INTPAR(IntPar::tracing_macros) > 0 {
                    begin_diagnostic();
                    print_nl(match_chr as str_number);
                    print_int(n as i32);
                    print_cstr(b"<-");
                    show_token_list(pstack[(n as i32 - 1) as usize], TEX_NULL, 1000);
                    end_diagnostic(false);
                }
            }
            if !(MEM[r as usize].b32.s0 != END_MATCH_TOKEN) {
                current_block = 12717620301112128284;
                break;
            }
        }
    } else {
        current_block = 12717620301112128284;
    }
    match current_block {
        12717620301112128284 => {
            while cur_input.state == TOKEN_LIST
                && cur_input.loc.is_texnull()
                && cur_input.index != V_TEMPLATE
            {
                end_token_list();
            }
            begin_token_list(ref_count, 6_u16);
            cur_input.name = warning_index;
            cur_input.loc = MEM[r as usize].b32.s1;
            if n as i32 > 0 {
                if PARAM_PTR + n as usize > MAX_PARAM_STACK {
                    MAX_PARAM_STACK = PARAM_PTR + n as usize;
                    if MAX_PARAM_STACK > PARAM_SIZE {
                        overflow(b"parameter stack size", PARAM_SIZE);
                    }
                }
                m = 0i32;
                while m <= n as i32 - 1i32 {
                    PARAM_STACK[PARAM_PTR + m as usize] = pstack[m as usize];
                    m += 1
                }
                PARAM_PTR += n as usize;
            }
        }
        _ => {}
    }
    scanner_status = save_scanner_status as u8;
    warning_index = save_warning_index;
}
pub(crate) unsafe fn insert_relax() {
    cur_tok = CS_TOKEN_FLAG + cur_cs;
    back_input();
    cur_tok = CS_TOKEN_FLAG + FROZEN_RELAX as i32;
    back_input();
    cur_input.index = INSERTED;
}
pub(crate) unsafe fn new_index(mut i: u16, mut q: i32) {
    let mut k: small_number = 0;
    cur_ptr = get_node(INDEX_NODE_SIZE) as i32;
    MEM[cur_ptr as usize].b16.s1 = i;
    MEM[cur_ptr as usize].b16.s0 = 0_u16;
    MEM[cur_ptr as usize].b32.s1 = q;
    k = 1 as small_number;
    let for_end = INDEX_NODE_SIZE - 1;
    if k as i32 <= for_end {
        loop {
            MEM[(cur_ptr + k as i32) as usize] = sa_null;
            if !((k as i32) < for_end) {
                break;
            }
            k = k + 1;
        }
    };
}
pub(crate) unsafe fn find_sa_element(mut t: small_number, mut n: i32, mut w: bool) {
    let mut current_block: u64;
    let mut q: i32 = 0;
    let mut i: small_number = 0;
    cur_ptr = sa_root[t as usize];
    if cur_ptr.is_texnull() {
        if w {
            new_index(t as u16, TEX_NULL);
            sa_root[t as usize] = cur_ptr;
            q = cur_ptr;
            i = (n / 0x40000) as small_number
        } else {
            return;
        }
        current_block = 15806769474000922024;
    } else {
        q = cur_ptr;
        i = (n / 0x40000) as small_number;
        if i as i32 & 1i32 != 0 {
            cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s1
        } else {
            cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s0
        }
        if cur_ptr.is_texnull() {
            if w {
                current_block = 15806769474000922024;
            } else {
                return;
            }
        } else {
            q = cur_ptr;
            i = (n / 4096 % 64) as small_number;
            if i as i32 & 1i32 != 0 {
                cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s1
            } else {
                cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s0
            }
            if cur_ptr.is_texnull() {
                if w {
                    current_block = 14787586673191526541;
                } else {
                    return;
                }
            } else {
                q = cur_ptr;
                i = (n / 64 % 64) as small_number;
                if i as i32 & 1i32 != 0 {
                    cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s1
                } else {
                    cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s0
                }
                if cur_ptr.is_texnull() {
                    if w {
                        current_block = 9497429165911859091;
                    } else {
                        return;
                    }
                } else {
                    q = cur_ptr;
                    i = (n % 64) as small_number;
                    if i as i32 & 1i32 != 0 {
                        cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s1
                    } else {
                        cur_ptr = MEM[(q + i as i32 / 2 + 1) as usize].b32.s0
                    }
                    if cur_ptr.is_texnull() && w as i32 != 0 {
                        current_block = 10182473981606373355;
                    } else {
                        return;
                    }
                }
            }
        }
    }
    match current_block {
        15806769474000922024 => {
            /*not_found1 */
            new_index(i as u16, q);
            if i as i32 & 1i32 != 0 {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s1 = cur_ptr
            } else {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s0 = cur_ptr
            }
            MEM[q as usize].b16.s0 += 1;
            q = cur_ptr;
            i = (n / 4096 % 64) as small_number;
            current_block = 14787586673191526541;
        }
        _ => {}
    }
    match current_block {
        14787586673191526541 => {
            /*not_found2 */
            new_index(i as u16, q);
            if i as i32 & 1i32 != 0 {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s1 = cur_ptr
            } else {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s0 = cur_ptr
            }
            MEM[q as usize].b16.s0 += 1;
            q = cur_ptr;
            i = (n / 64 % 64) as small_number;
            current_block = 9497429165911859091;
        }
        _ => {}
    }
    match current_block {
        9497429165911859091 => {
            /*not_found3 */
            new_index(i as u16, q);
            if i as i32 & 1i32 != 0 {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s1 = cur_ptr
            } else {
                MEM[(q + i as i32 / 2 + 1) as usize].b32.s0 = cur_ptr
            }
            MEM[q as usize].b16.s0 += 1;
            q = cur_ptr;
            i = (n % 64i32) as small_number
        }
        _ => {}
    }
    /*not_found4 *//*1608: */
    if t as i32 == MARK_VAL {
        cur_ptr = get_node(MARK_CLASS_NODE_SIZE) as i32; /*level_one *//*:1608 */
        MEM[(cur_ptr + 1) as usize] = sa_null;
        MEM[(cur_ptr + 2) as usize] = sa_null;
        MEM[(cur_ptr + 3) as usize] = sa_null
    } else {
        if t as i32 <= 1i32 {
            cur_ptr = get_node(WORD_NODE_SIZE) as i32;
            MEM[(cur_ptr + 2) as usize].b32.s1 = 0;
            MEM[(cur_ptr + 1) as usize].b32.s1 = n
        } else {
            cur_ptr = get_node(POINTER_NODE_SIZE) as i32;
            if t <= MU_VAL as i16 {
                MEM[(cur_ptr + 1) as usize].b32.s1 = 0;
                MEM[0].b32.s1 += 1;
            } else {
                MEM[(cur_ptr + 1) as usize].b32.s1 = TEX_NULL
            }
        }
        MEM[(cur_ptr + 1) as usize].b32.s0 = TEX_NULL
    }
    MEM[cur_ptr as usize].b16.s1 = (64 * t as i32 + i as i32) as u16;
    MEM[cur_ptr as usize].b16.s0 = 1;
    MEM[cur_ptr as usize].b32.s1 = q;
    if i as i32 & 1i32 != 0 {
        MEM[(q + i as i32 / 2 + 1) as usize].b32.s1 = cur_ptr
    } else {
        MEM[(q + i as i32 / 2 + 1) as usize].b32.s0 = cur_ptr
    }
    MEM[q as usize].b16.s0 += 1;
}
pub(crate) unsafe fn expand() {
    let mut t: i32 = 0;
    let mut b: bool = false;
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut j: i32 = 0;
    let mut cv_backup: i32 = 0;
    let mut cvl_backup: small_number = 0;
    let mut radix_backup: small_number = 0;
    let mut co_backup: small_number = 0;
    let mut backup_backup: i32 = 0;
    let mut save_scanner_status: small_number = 0;
    expand_depth_count += 1;
    if expand_depth_count >= expand_depth {
        overflow(b"expansion depth", expand_depth as usize);
    }
    cv_backup = cur_val;
    cvl_backup = cur_val_level as small_number;
    radix_backup = radix;
    co_backup = cur_order as small_number;
    backup_backup = MEM[BACKUP_HEAD].b32.s1;
    loop {
        if (cur_cmd as u16) < CALL {
            /*384:*/
            if *INTPAR(IntPar::tracing_commands) > 1 {
                show_cur_cmd_chr(); /*1612:*/
            }
            match cur_cmd as u16 {
                TOP_BOT_MARK => {
                    t = cur_chr % 5;
                    if cur_chr >= 5 {
                        scan_register_num();
                    } else {
                        cur_val = 0;
                    }
                    if cur_val == 0 {
                        cur_ptr = cur_mark[t as usize]
                    } else {
                        find_sa_element(MARK_VAL as small_number, cur_val, false);
                        if !cur_ptr.is_texnull() {
                            if t & 1i32 != 0 {
                                cur_ptr = MEM[(cur_ptr + t / 2 + 1) as usize].b32.s1
                            } else {
                                cur_ptr = MEM[(cur_ptr + t / 2 + 1) as usize].b32.s0
                            }
                        }
                    }
                    if !cur_ptr.is_texnull() {
                        begin_token_list(cur_ptr as usize, MARK_TEXT);
                    }
                    break;
                }
                EXPAND_AFTER => {
                    /*385:*/
                    if cur_chr == 0 {
                        get_token(); /*1553: "\unless" implementation */
                        t = cur_tok;
                        get_token();
                        if cur_cmd > MAX_COMMAND {
                            expand();
                        } else {
                            back_input();
                        }
                        cur_tok = t;
                        back_input();
                        break;
                    } else {
                        get_token();
                        if cur_cmd as u16 == IF_TEST && cur_chr != IF_CASE_CODE as i32 {
                            cur_chr = cur_chr + UNLESS_CODE
                        } else {
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! ");
                            }
                            print_cstr(b"You can\'t use `");
                            print_esc_cstr(b"unless");
                            print_cstr(b"\' before `");
                            print_cmd_chr(cur_cmd as u16, cur_chr);
                            print_char('\'' as i32);
                            help_ptr = 1;
                            help_line[0] = b"Continue, and I\'ll forget that it ever happened.";
                            back_error();
                            break;
                        }
                    }
                }
                NO_EXPAND => {
                    /*386:*/
                    if cur_chr == 0 {
                        save_scanner_status = scanner_status as small_number; /*387: \primitive implementation */
                        scanner_status = NORMAL as u8;
                        get_token();
                        scanner_status = save_scanner_status as u8;
                        t = cur_tok;
                        back_input();
                        if t >= CS_TOKEN_FLAG {
                            p = get_avail() as i32;
                            MEM[p as usize].b32.s0 = CS_TOKEN_FLAG + FROZEN_DONT_EXPAND as i32;
                            MEM[p as usize].b32.s1 = cur_input.loc;
                            cur_input.start = p;
                            cur_input.loc = p
                        }
                        break;
                    } else {
                        save_scanner_status = scanner_status as small_number;
                        scanner_status = NORMAL as u8;
                        get_token();
                        scanner_status = save_scanner_status as u8;
                        if cur_cs < HASH_BASE as i32 {
                            cur_cs = prim_lookup(cur_cs - SINGLE_BASE as i32)
                        } else {
                            cur_cs = prim_lookup((*hash.offset(cur_cs as isize)).s1)
                        }
                        if !(cur_cs != UNDEFINED_PRIMITIVE) {
                            break;
                        }
                        t = prim_eqtb[cur_cs as usize].b16.s1 as i32;
                        if t > MAX_COMMAND as i32 {
                            cur_cmd = t as eight_bits;
                            cur_chr = prim_eqtb[cur_cs as usize].b32.s1;
                            cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                            cur_cs = 0;
                        } else {
                            back_input();
                            p = get_avail() as i32;
                            MEM[p as usize].b32.s0 = CS_TOKEN_FLAG + FROZEN_PRIMITIVE as i32;
                            MEM[p as usize].b32.s1 = cur_input.loc;
                            cur_input.loc = p;
                            cur_input.start = p;
                            break;
                        }
                    }
                }
                CS_NAME => {
                    let r = get_avail();
                    p = r as i32;
                    b = is_in_csname;
                    is_in_csname = true;
                    loop {
                        get_x_token();
                        if cur_cs == 0 {
                            q = get_avail() as i32;
                            MEM[p as usize].b32.s1 = q;
                            MEM[q as usize].b32.s0 = cur_tok;
                            p = q
                        }
                        if !(cur_cs == 0) {
                            break;
                        }
                    }
                    if cur_cmd as u16 != END_CS_NAME {
                        /*391:*/
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! ");
                        }
                        print_cstr(b"Missing ");
                        print_esc_cstr(b"endcsname");
                        print_cstr(b" inserted");
                        help_ptr = 2;
                        help_line[1] = b"The control sequence marked <to be read again> should";
                        help_line[0] = b"not appear between \\csname and \\endcsname.";
                        back_error();
                    }
                    is_in_csname = b;
                    j = first;
                    p = MEM[r].b32.s1;
                    while !p.is_texnull() {
                        if j >= max_buf_stack {
                            max_buf_stack = j + 1;
                            if max_buf_stack as usize == BUF_SIZE {
                                overflow(b"buffer size", BUF_SIZE);
                            }
                        }
                        BUFFER[j as usize] = MEM[p as usize].b32.s0 % MAX_CHAR_VAL;
                        j += 1;
                        p = *LLIST_link(p as usize)
                    }
                    if j > first + 1 || BUFFER[first as usize] as i64 > 65535 {
                        no_new_control_sequence = false;
                        cur_cs = id_lookup(first, j - first);
                        no_new_control_sequence = true
                    } else if j == first {
                        cur_cs = NULL_CS as i32;
                    } else {
                        cur_cs = SINGLE_BASE as i32 + BUFFER[first as usize];
                        /*:392*/
                    }
                    flush_list(Some(r));
                    if EQTB[cur_cs as usize].b16.s1 == UNDEFINED_CS {
                        eq_define(cur_cs as usize, RELAX, TOO_BIG_USV);
                    }
                    cur_tok = cur_cs + CS_TOKEN_FLAG;
                    back_input();
                    break;
                }
                CONVERT => {
                    conv_toks();
                    break;
                }
                THE => {
                    ins_the_toks();
                    break;
                }
                IF_TEST => {
                    conditional();
                    break;
                }
                FI_OR_ELSE => {
                    if *INTPAR(IntPar::tracing_ifs) > 0 {
                        if *INTPAR(IntPar::tracing_commands) <= 1i32 {
                            show_cur_cmd_chr();
                        }
                    }
                    if cur_chr > if_limit as i32 {
                        if if_limit == IF_CODE {
                            insert_relax();
                        } else {
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! ");
                            }
                            print_cstr(b"Extra ");
                            print_cmd_chr(FI_OR_ELSE, cur_chr);
                            help_ptr = 1;
                            help_line[0] = b"I\'m ignoring this; it doesn\'t match any \\if.";
                            error();
                        }
                    } else {
                        while cur_chr != FI_CODE as i32 {
                            pass_text();
                        }
                        if IF_STACK[IN_OPEN] == cond_ptr {
                            if_warning();
                        }
                        p = cond_ptr;
                        if_line = MEM[(p + 1) as usize].b32.s1;
                        cur_if = MEM[p as usize].b16.s0 as small_number;
                        if_limit = MEM[p as usize].b16.s1 as u8;
                        cond_ptr = MEM[p as usize].b32.s1;
                        free_node(p as usize, IF_NODE_SIZE);
                    }
                    break;
                }
                INPUT => {
                    if cur_chr == 1 {
                        /* \endinput */
                        force_eof = true
                    } else if cur_chr == 2 {
                        /*1537:*/
                        /* \scantokens */
                        pseudo_start();
                    } else if name_in_progress {
                        insert_relax();
                    } else {
                        /* \input */
                        start_input(ptr::null()); /*393:*/
                    }
                    break;
                }
                _ => {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Undefined control sequence");
                    help_ptr = 5;
                    help_line[4] = b"The control sequence at the end of the top line";
                    help_line[3] = b"of your error message was never \\def\'ed. If you have";
                    help_line[2] = b"misspelled it (e.g., `\\hobx\'), type `I\' and the correct";
                    help_line[1] = b"spelling (e.g., `I\\hbox\'). Otherwise just continue,";
                    help_line[0] = b"and I\'ll forget about whatever was undefined.";
                    error();
                    break;
                }
            }
        } else {
            if (cur_cmd as u16) < END_TEMPLATE {
                macro_call();
            } else {
                cur_tok = CS_TOKEN_FLAG + FROZEN_ENDV as i32;
                back_input();
            }
            break;
        }
    }
    cur_val = cv_backup;
    cur_val_level = cvl_backup as u8;
    radix = radix_backup;
    cur_order = co_backup as glue_ord;
    MEM[BACKUP_HEAD].b32.s1 = backup_backup;
    expand_depth_count -= 1;
}
pub(crate) unsafe fn get_x_token() {
    loop {
        get_next();
        if cur_cmd <= MAX_COMMAND {
            break;
        }
        if cur_cmd as u16 >= CALL {
            if (cur_cmd as u16) < END_TEMPLATE {
                macro_call();
            } else {
                cur_cs = FROZEN_ENDV as i32;
                cur_cmd = ENDV as eight_bits;
                break;
            }
        } else {
            expand();
        }
    }
    cur_tok = if cur_cs == 0 {
        cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
    } else {
        CS_TOKEN_FLAG + cur_cs
    };
}
pub(crate) unsafe fn x_token() {
    while cur_cmd > MAX_COMMAND {
        expand();
        get_next();
    }
    cur_tok = if cur_cs == 0 {
        cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
    } else {
        CS_TOKEN_FLAG + cur_cs
    };
}
pub(crate) unsafe fn scan_left_brace() {
    loop {
        get_x_token();
        if !(cur_cmd as u16 == SPACER || cur_cmd as u16 == RELAX) {
            break;
        }
    }
    if cur_cmd as u16 != LEFT_BRACE {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Missing { inserted");
        help_ptr = 4;
        help_line[3] = b"A left brace was mandatory here, so I\'ve put one in.";
        help_line[2] = b"You might want to delete and/or insert some corrections";
        help_line[1] = b"so that I will find a matching right brace soon.";
        help_line[0] = b"(If you\'re confused by all this, try typing `I}\' now.)";
        back_error();
        cur_tok = LEFT_BRACE_TOKEN + '{' as i32;
        cur_cmd = LEFT_BRACE as eight_bits;
        cur_chr = '{' as i32;
        align_state += 1
    };
}
pub(crate) unsafe fn scan_optional_equals() {
    loop {
        get_x_token();
        if !(cur_cmd as u16 == SPACER) {
            break;
        }
    }
    if cur_tok != OTHER_TOKEN + 61 {
        /*"="*/
        back_input();
    };
}

pub(crate) unsafe fn scan_keyword(s: &[u8]) -> bool {
    let mut p = BACKUP_HEAD;
    MEM[p].b32.s1 = TEX_NULL;
    if s.len() == 1 {
        let mut c: i8 = s[0] as i8;
        loop {
            get_x_token();
            if cur_cs == 0 && (cur_chr == c as i32 || cur_chr == c as i32 - 32) {
                let q = get_avail();
                MEM[p].b32.s1 = q as i32;
                MEM[q].b32.s0 = cur_tok;
                p = q;
                flush_list(MEM[BACKUP_HEAD].b32.s1.opt());
                return true;
            } else {
                if cur_cmd as i32 != SPACER as _ || p != BACKUP_HEAD {
                    back_input();
                    if p != BACKUP_HEAD {
                        begin_token_list(MEM[BACKUP_HEAD].b32.s1 as usize, BACKED_UP);
                    }
                    return false;
                }
            }
        }
    }
    let slen = s.len();
    let mut i = 0;
    while i < slen {
        get_x_token();
        if cur_cs == 0
            && (cur_chr == s[i as usize] as i8 as i32 || cur_chr == s[i as usize] as i8 as i32 - 32)
        {
            let q = get_avail();
            MEM[p].b32.s1 = q as i32;
            MEM[q].b32.s0 = cur_tok;
            p = q;
            i = i.wrapping_add(1)
        } else if cur_cmd as i32 != SPACER as _ || p != BACKUP_HEAD {
            back_input();
            if p != BACKUP_HEAD {
                begin_token_list(MEM[BACKUP_HEAD].b32.s1 as usize, BACKED_UP);
            }
            return false;
        }
    }
    flush_list(MEM[BACKUP_HEAD].b32.s1.opt());
    true
}

pub(crate) unsafe fn mu_error() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Incompatible glue units");
    help_ptr = 1;
    help_line[0] = b"I\'m going to assume that 1mu=1pt when they\'re mixed.";
    error();
}
pub(crate) unsafe fn scan_glyph_number(mut f: internal_font_number) {
    if scan_keyword(b"/") {
        scan_and_pack_name();
        cur_val = map_glyph_to_index(f);
        cur_val_level = INT_VAL as u8;
    } else if scan_keyword(b"u") {
        scan_char_num();
        cur_val = map_char_to_glyph(f, cur_val);
        cur_val_level = INT_VAL as u8;
    } else {
        scan_int();
    };
}
pub(crate) unsafe fn scan_char_class() {
    scan_int();
    if cur_val < 0 || cur_val > CHAR_CLASS_LIMIT {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad character class");
        help_ptr = 2;
        help_line[1] = b"A character class must be between 0 and 4096.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_char_class_not_ignored() {
    scan_int();
    if cur_val < 0 || cur_val > CHAR_CLASS_LIMIT {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad character class");
        help_ptr = 2;
        help_line[1] = b"A class for inter-character transitions must be between 0 and 4095.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_eight_bit_int() {
    scan_int();
    if cur_val < 0 || cur_val > 255 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad register code");
        help_ptr = 2;
        help_line[1] = b"A register code or char class must be between 0 and 255.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_usv_num() {
    scan_int();
    if cur_val < 0 || cur_val > BIGGEST_USV as i32 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad character code");
        help_ptr = 2;
        help_line[1] = b"A Unicode scalar value must be between 0 and \"10FFFF.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_char_num() {
    scan_int();
    if cur_val < 0 || cur_val > BIGGEST_CHAR {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad character code");
        help_ptr = 2;
        help_line[1] = b"A character number must be between 0 and 65535.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_xetex_math_char_int() {
    scan_int();
    if math_char(cur_val) == ACTIVE_MATH_CHAR as u32 {
        if cur_val != ACTIVE_MATH_CHAR {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Bad active XeTeX math code");

            help_ptr = 2;
            help_line[1] = b"Since I ignore class and family for active math chars,";
            help_line[0] = b"I changed this one to \"1FFFFF.";
            int_error(cur_val);
            cur_val = ACTIVE_MATH_CHAR;
        }
    } else if math_char(cur_val) as u32 > BIGGEST_USV as u32 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad XeTeX math character code");

        help_ptr = 2;
        help_line[1] = b"Since I expected a character number between 0 and \"10FFFF,";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_math(p: usize) {
    let mut c: i32 = 0;
    'c_118470: loop {
        loop
        /*422:*/
        {
            get_x_token();
            if !(cur_cmd as u16 == SPACER || cur_cmd as u16 == RELAX) {
                break;
            }
        }
        loop {
            match cur_cmd as i32 {
                11 | 12 | 68 => {
                    c = *MATH_CODE(cur_chr as usize);
                    if !(c as u32 & 0x1fffff_u32 == ACTIVE_MATH_CHAR as u32) {
                        break 'c_118470;
                    }
                    cur_cs = cur_chr + 1;
                    cur_cmd = EQTB[cur_cs as usize].b16.s1 as eight_bits;
                    cur_chr = EQTB[cur_cs as usize].b32.s1;
                    x_token();
                    back_input();
                    break;
                }
                16 => {
                    scan_char_num();
                    cur_chr = cur_val;
                    cur_cmd = CHAR_GIVEN as eight_bits
                }
                17 => {
                    if cur_chr == 2 {
                        scan_math_class_int();
                        c = ((cur_val as u32 & 0x7_u32) << 21i32) as i32;
                        scan_math_fam_int();
                        c = (c as u32).wrapping_add((cur_val as u32 & 0xff_u32) << 24i32) as i32;
                        scan_usv_num();
                        c = c + cur_val
                    } else if cur_chr == 1i32 {
                        scan_xetex_math_char_int();
                        c = cur_val
                    } else {
                        scan_fifteen_bit_int();
                        c = (((cur_val / 4096) as u32 & 0x7_u32) << 21)
                            .wrapping_add(((cur_val % 4096 / 256) as u32 & 0xff_u32) << 24)
                            .wrapping_add((cur_val % 256) as u32) as i32
                    }
                    break 'c_118470;
                }
                69 => {
                    c = (((cur_chr / 4096) as u32 & 0x7_u32) << 21)
                        .wrapping_add(((cur_chr % 4096 / 256) as u32 & 0xff_u32) << 24)
                        .wrapping_add((cur_chr % 256) as u32) as i32;
                    break 'c_118470;
                }
                70 => {
                    c = cur_chr;
                    break 'c_118470;
                }
                15 => {
                    if cur_chr == 1 {
                        scan_math_class_int();
                        c = ((cur_val as u32 & 0x7_u32) << 21) as i32;
                        scan_math_fam_int();
                        c = (c as u32).wrapping_add((cur_val as u32 & 0xff_u32) << 24) as i32;
                        scan_usv_num();
                        c = c + cur_val
                    } else {
                        scan_delimiter_int();
                        c = cur_val / 4096;
                        c = (((c / 4096) as u32 & 0x7_u32) << 21)
                            .wrapping_add(((c % 4096 / 256) as u32 & 0xff_u32) << 24)
                            .wrapping_add((c % 256) as u32) as i32
                    }
                    break 'c_118470;
                }
                _ => {
                    back_input();
                    scan_left_brace();
                    SAVE_STACK[SAVE_PTR + 0].b32.s1 = p as i32;
                    SAVE_PTR += 1;
                    push_math(GroupCode::MATH);
                    return;
                }
            }
        }
    }
    MEM[p].b32.s1 = MATH_CHAR;
    MEM[p].b16.s0 = (c as i64 % 65536) as u16;
    if (math_class(c) == 7)
        && (*INTPAR(IntPar::cur_fam) >= 0i32 && *INTPAR(IntPar::cur_fam) < 256i32)
    {
        MEM[p].b16.s1 = *INTPAR(IntPar::cur_fam) as u16
    } else {
        MEM[p].b16.s1 = (c as u32 >> 24 & 0xff_u32) as u16
    }
    MEM[p].b16.s1 =
        (MEM[p].b16.s1 as i64 + (c as u32 & 0x1fffff_u32) as i64 / 65536 * 256i32 as i64) as u16;
}
pub(crate) unsafe fn set_math_char(mut c: i32) {
    let mut ch: UnicodeScalar = 0;
    if c as u32 & 0x1fffff_u32 == ACTIVE_MATH_CHAR as u32 {
        /*1187: */
        cur_cs = cur_chr + 1; /* ... "between 0 and 15" */
        cur_cmd = EQTB[cur_cs as usize].b16.s1 as eight_bits; /* ... "between 0 and 15" */
        cur_chr = EQTB[cur_cs as usize].b32.s1;
        x_token();
        back_input();
    } else {
        let p = new_noad();
        MEM[p + 1].b32.s1 = MATH_CHAR;
        ch = (c as u32 & 0x1fffff_u32) as UnicodeScalar;
        MEM[p + 1].b16.s0 = (ch as i64 % 65536) as u16;
        MEM[p + 1].b16.s1 = (c as u32 >> 24 & 0xff_u32) as u16;
        if c as u32 >> 21i32 & 0x7_u32 == 7 {
            if *INTPAR(IntPar::cur_fam) >= 0
                && *INTPAR(IntPar::cur_fam) < NUMBER_MATH_FAMILIES as i32
            {
                MEM[p + 1].b16.s1 = *INTPAR(IntPar::cur_fam) as u16
            }
            MEM[p].b16.s1 = ORD_NOAD
        } else {
            MEM[p].b16.s1 = (ORD_NOAD as u32).wrapping_add(c as u32 >> 21 & 0x7_u32) as u16
        }
        MEM[p + 1].b16.s1 = (MEM[p + 1].b16.s1 as i64 + ch as i64 / 65536 * 256 as i64) as u16;
        MEM[cur_list.tail].b32.s1 = p as i32;
        cur_list.tail = p;
    };
}
pub(crate) unsafe fn scan_math_class_int() {
    scan_int();
    if cur_val < 0 || cur_val > 7 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad math class");
        help_ptr = 2;
        help_line[1] = b"Since I expected to read a number between 0 and 7,";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0
    };
}
pub(crate) unsafe fn scan_math_fam_int() {
    scan_int();
    if cur_val < 0 || cur_val > NUMBER_MATH_FAMILIES as i32 - 1 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad math family");
        help_ptr = 2;
        help_line[1] = b"Since I expected to read a number between 0 and 255,";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_four_bit_int() {
    scan_int();
    if cur_val < 0 || cur_val > 15 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad number");
        help_ptr = 2;
        help_line[1] = b"Since I expected to read a number between 0 and 15,";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_fifteen_bit_int() {
    scan_int();
    if cur_val < 0 || cur_val > 32767 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad mathchar");
        help_ptr = 2;
        help_line[1] = b"A mathchar number must be between 0 and 32767.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_delimiter_int() {
    scan_int();
    if cur_val < 0 || cur_val > 0x7ffffff {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad delimiter code");
        help_ptr = 2;
        help_line[1] = b"A numeric delimiter code must be between 0 and 2^{27}-1.";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_register_num() {
    scan_int();
    if cur_val < 0 || cur_val > max_reg_num {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad register code");
        help_ptr = 2_u8;
        help_line[1] = max_reg_help_line;
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn scan_four_bit_int_or_18() {
    scan_int();
    if cur_val < 0 || cur_val > 15 && cur_val != 18 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad number");
        help_ptr = 2;
        help_line[1] = b"Since I expected to read a number between 0 and 15,";
        help_line[0] = b"I changed this one to zero.";
        int_error(cur_val);
        cur_val = 0;
    };
}
pub(crate) unsafe fn get_x_or_protected() {
    loop {
        get_token();
        if cur_cmd <= MAX_COMMAND as u8 {
            return;
        }
        if cur_cmd as u16 >= CALL && (cur_cmd as u16) < END_TEMPLATE {
            if MEM[MEM[cur_chr as usize].b32.s1 as usize].b32.s0 == PROTECTED_TOKEN {
                return;
            }
        }
        expand();
    }
}
pub(crate) unsafe fn effective_char(
    mut _err_p: bool,
    mut f: internal_font_number,
    mut c: u16,
) -> i32 {
    if !xtx_ligature_present && !(FONT_MAPPING[f]).is_null() {
        c = apply_tfm_font_mapping(FONT_MAPPING[f], c as i32) as u16
    }
    xtx_ligature_present = false;
    c as i32
}
pub(crate) unsafe fn scan_font_ident() {
    let mut f: internal_font_number = 0;
    let mut m: i32 = 0;
    loop {
        get_x_token();
        if !(cur_cmd as u16 == SPACER) {
            break;
        }
    }
    if cur_cmd as u16 == DEF_FONT {
        f = EQTB[CUR_FONT_LOC].b32.s1 as usize;
    } else if cur_cmd as u16 == SET_FONT {
        f = cur_chr as usize;
    } else if cur_cmd as u16 == DEF_FAMILY {
        m = cur_chr;
        scan_math_fam_int();
        f = EQTB[(m + cur_val) as usize].b32.s1 as usize
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Missing font identifier");
        help_ptr = 2;
        help_line[1] = b"I was looking for a control sequence whose";
        help_line[0] = b"current meaning has been defined by \\font.";
        back_error();
        f = FONT_BASE as usize;
    }
    cur_val = f as i32;
}
pub(crate) unsafe fn find_font_dimen(mut writing: bool) {
    let mut n: i32 = 0;
    scan_int();
    n = cur_val;
    scan_font_ident();
    let mut f = cur_val as usize;
    if n <= 0 {
        cur_val = fmem_ptr
    } else {
        if writing as i32 != 0
            && n <= SPACE_SHRINK_CODE
            && n >= SPACE_CODE
            && !FONT_GLUE[f].is_texnull()
        {
            delete_glue_ref(FONT_GLUE[f] as usize);
            FONT_GLUE[f] = TEX_NULL
        }
        if n > FONT_PARAMS[f] {
            if f < FONT_PTR {
                cur_val = fmem_ptr
            } else {
                loop
                /*599: */
                {
                    if fmem_ptr == FONT_MEM_SIZE as i32 {
                        overflow(b"font memory", FONT_MEM_SIZE);
                    }
                    FONT_INFO[fmem_ptr as usize].b32.s1 = 0;
                    fmem_ptr += 1;
                    FONT_PARAMS[f] += 1;
                    if n == FONT_PARAMS[f] {
                        break;
                    }
                }
                cur_val = fmem_ptr - 1;
            }
        } else {
            cur_val = n + PARAM_BASE[f]
        }
    }
    if cur_val == fmem_ptr {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Font ");
        print_esc((*hash.offset((FROZEN_NULL_FONT + f) as isize)).s1);
        print_cstr(b" has only ");
        print_int(FONT_PARAMS[f]);
        print_cstr(b" fontdimen parameters");
        help_ptr = 2;
        help_line[1] = b"To increase the number of font parameters, you must";
        help_line[0] = b"use \\fontdimen immediately after the \\font is loaded.";
        error();
    };
}
pub(crate) unsafe fn scan_something_internal(mut level: small_number, mut negative: bool) {
    let mut m: i32 = 0;
    let mut n: i32 = 0;
    let mut k: i32 = 0;
    let mut kk: i32 = 0;
    let mut q: i32 = 0;
    let mut r: i32 = 0;
    let mut tx: i32 = 0;
    let mut i: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    m = cur_chr;
    match cur_cmd as u16 {
        DEF_CODE => {
            scan_usv_num();
            if m == MATH_CODE_BASE as i32 {
                cur_val1 = *MATH_CODE(cur_val as usize);
                if math_char(cur_val1) == ACTIVE_MATH_CHAR as u32 {
                    cur_val1 = 0x8000;
                } else if math_class(cur_val1) > 7
                    || math_fam(cur_val1) > 15
                    || math_char(cur_val1) > 255
                {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Extended mathchar used as mathchar");
                    help_ptr = 2;
                    help_line[1] = b"A mathchar number must be between 0 and \"7FFF.";
                    help_line[0] = b"I changed this one to zero.";
                    int_error(cur_val1);
                    cur_val1 = 0;
                }
                cur_val1 = (math_class(cur_val1) * 0x1000
                    + math_fam(cur_val1) * 0x100
                    + math_char(cur_val1)) as i32;
                cur_val = cur_val1;
                cur_val_level = INT_VAL
            } else if m == DEL_CODE_BASE as i32 {
                cur_val1 = EQTB[DEL_CODE_BASE + cur_val as usize].b32.s1;
                if cur_val1 >= 0x40000000 {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Extended delcode used as delcode");
                    help_ptr = 2;
                    help_line[1] = b"I can only go up to 2147483647=\'17777777777=\"7FFFFFFF,";
                    help_line[0] = b"I changed this one to zero.";
                    error();
                    cur_val = 0;
                    cur_val_level = INT_VAL;
                } else {
                    cur_val = cur_val1;
                    cur_val_level = INT_VAL;
                }
            } else if m < SF_CODE_BASE as i32 {
                cur_val = EQTB[(m + cur_val) as usize].b32.s1;
                cur_val_level = INT_VAL;
            } else if m < MATH_CODE_BASE as i32 {
                cur_val = (EQTB[(m + cur_val) as usize].b32.s1 as i64 % 65536) as i32;
                cur_val_level = INT_VAL;
            } else {
                cur_val = EQTB[(m + cur_val) as usize].b32.s1;
                cur_val_level = INT_VAL;
            }
        }
        XETEX_DEF_CODE => {
            scan_usv_num();
            if m == SF_CODE_BASE as i32 {
                cur_val = (*SF_CODE(cur_val as usize) as i64 / 65536) as i32;
                cur_val_level = INT_VAL
            } else if m == MATH_CODE_BASE as i32 {
                cur_val = *MATH_CODE(cur_val as usize);
                cur_val_level = INT_VAL
            } else if m == MATH_CODE_BASE as i32 + 1 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Can\'t use \\Umathcode as a number (try \\Umathcodenum)");
                help_ptr = 2;
                help_line[1] = b"\\Umathcode is for setting a mathcode from separate values;";
                help_line[0] = b"use \\Umathcodenum to access them as single values.";
                error();
                cur_val = 0;
                cur_val_level = INT_VAL;
            } else if m == DEL_CODE_BASE as i32 {
                cur_val = EQTB[DEL_CODE_BASE + cur_val as usize].b32.s1;
                cur_val_level = INT_VAL;
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Can\'t use \\Udelcode as a number (try \\Udelcodenum)");
                help_ptr = 2;
                help_line[1] = b"\\Udelcode is for setting a delcode from separate values;";
                help_line[0] = b"use \\Udelcodenum to access them as single values.";
                error();
                cur_val = 0;
                cur_val_level = INT_VAL;
            }
        }
        TOKS_REGISTER | ASSIGN_TOKS | DEF_FAMILY | SET_FONT | DEF_FONT => {
            if level != TOK_VAL as i16 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Missing number, treated as zero");
                help_ptr = 3;
                help_line[2] = b"A number should have been here; I inserted `0\'.";
                help_line[1] = b"(If you can\'t figure out why I needed to see a number,";
                help_line[0] = b"look up `weird error\' in the index to The TeXbook.)";
                back_error();
                cur_val = 0;
                cur_val_level = DIMEN_VAL;
            } else if cur_cmd <= ASSIGN_TOKS as u8 {
                if cur_cmd < ASSIGN_TOKS as u8 {
                    if m == 0i32 {
                        scan_register_num();
                        if cur_val < 256 {
                            cur_val = EQTB[TOKS_BASE + cur_val as usize].b32.s1
                        } else {
                            find_sa_element(TOK_VAL as small_number, cur_val, false);
                            if cur_ptr.is_texnull() {
                                cur_val = TEX_NULL
                            } else {
                                cur_val = MEM[(cur_ptr + 1) as usize].b32.s1
                            }
                        }
                    } else {
                        cur_val = MEM[(m + 1) as usize].b32.s1
                    }
                } else if cur_chr == LOCAL_BASE as i32 + Local::xetex_inter_char as i32 {
                    scan_char_class_not_ignored();
                    cur_ptr = cur_val;
                    scan_char_class_not_ignored();
                    find_sa_element(
                        INTER_CHAR_VAL as small_number,
                        cur_ptr * CHAR_CLASS_LIMIT + cur_val,
                        false,
                    );
                    if cur_ptr.is_texnull() {
                        cur_val = TEX_NULL;
                    } else {
                        cur_val = MEM[(cur_ptr + 1) as usize].b32.s1
                    }
                } else {
                    cur_val = EQTB[m as usize].b32.s1
                }
                cur_val_level = TOK_VAL;
            } else {
                back_input();
                scan_font_ident();
                cur_val = FONT_ID_BASE as i32 + cur_val;
                cur_val_level = IDENT_VAL;
            }
        }
        ASSIGN_INT => {
            cur_val = EQTB[m as usize].b32.s1;
            cur_val_level = INT_VAL;
        }
        ASSIGN_DIMEN => {
            cur_val = EQTB[m as usize].b32.s1;
            cur_val_level = DIMEN_VAL;
        }
        ASSIGN_GLUE => {
            cur_val = EQTB[m as usize].b32.s1;
            cur_val_level = GLUE_VAL;
        }
        ASSIGN_MU_GLUE => {
            cur_val = EQTB[m as usize].b32.s1;
            cur_val_level = MU_VAL;
        }
        SET_AUX => {
            if (cur_list.mode as i32).abs() != m {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Improper ");
                print_cmd_chr(SET_AUX, m);
                help_ptr = 4;
                help_line[3] = b"You can refer to \\spacefactor only in horizontal mode;";
                help_line[2] = b"you can refer to \\prevdepth only in vertical mode; and";
                help_line[1] = b"neither of these is meaningful inside \\write. So";
                help_line[0] = b"I\'m forgetting what you said and using zero instead.";
                error();
                if level != TOK_VAL as i16 {
                    cur_val = 0;
                    cur_val_level = DIMEN_VAL;
                } else {
                    cur_val = 0;
                    cur_val_level = INT_VAL;
                }
            } else if m == VMODE as i32 {
                cur_val = cur_list.aux.b32.s1;
                cur_val_level = DIMEN_VAL;
            } else {
                cur_val = cur_list.aux.b32.s0;
                cur_val_level = INT_VAL;
            }
        }
        SET_PREV_GRAF => {
            if cur_list.mode == 0 {
                cur_val = 0;
                cur_val_level = INT_VAL
            } else {
                NEST[NEST_PTR] = cur_list;
                let mut p = NEST_PTR;
                while NEST[p].mode.abs() != VMODE {
                    p -= 1
                }
                cur_val = NEST[p].prev_graf;
                cur_val_level = INT_VAL
            }
        }
        SET_PAGE_INT => {
            if m == 0 {
                cur_val = dead_cycles
            } else if m == 2 {
                cur_val = interaction as i32
            } else {
                cur_val = insert_penalties
            }
            cur_val_level = INT_VAL
        }
        SET_PAGE_DIMEN => {
            if page_contents as i32 == EMPTY && !output_active {
                if m == 0 {
                    cur_val = MAX_HALFWORD;
                } else {
                    cur_val = 0;
                }
            } else {
                cur_val = page_so_far[m as usize]
            }
            cur_val_level = DIMEN_VAL
        }
        SET_SHAPE => {
            if m > LOCAL_BASE as i32 + Local::par_shape as i32 {
                /*1654:*/
                scan_int();
                if EQTB[m as usize].b32.s1.is_texnull() || cur_val < 0 {
                    cur_val = 0;
                } else {
                    if cur_val > MEM[(EQTB[m as usize].b32.s1 + 1) as usize].b32.s1 {
                        cur_val = MEM[(EQTB[m as usize].b32.s1 + 1) as usize].b32.s1
                    }
                    cur_val = MEM[(EQTB[m as usize].b32.s1 + cur_val + 1) as usize].b32.s1
                }
            } else if LOCAL(Local::par_shape).is_texnull() {
                cur_val = 0;
            } else {
                cur_val = MEM[*LOCAL(Local::par_shape) as usize].b32.s0
            }
            cur_val_level = INT_VAL;
        }
        SET_BOX_DIMEN => {
            scan_register_num();
            if cur_val < 256 {
                q = *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    q = TEX_NULL
                } else {
                    q = MEM[(cur_ptr + 1) as usize].b32.s1
                }
            }
            if q.is_texnull() {
                cur_val = 0;
            } else {
                cur_val = MEM[(q + m) as usize].b32.s1
            }
            cur_val_level = DIMEN_VAL
        }
        CHAR_GIVEN | MATH_GIVEN => {
            cur_val = cur_chr;
            cur_val_level = INT_VAL
        }
        ASSIGN_FONT_DIMEN => {
            find_font_dimen(false);
            FONT_INFO[fmem_ptr as usize].b32.s1 = 0;
            cur_val = FONT_INFO[cur_val as usize].b32.s1;
            cur_val_level = DIMEN_VAL
        }
        ASSIGN_FONT_INT => {
            scan_font_ident();
            if m == 0 {
                cur_val = HYPHEN_CHAR[cur_val as usize];
                cur_val_level = INT_VAL
            } else if m == 1 {
                cur_val = SKEW_CHAR[cur_val as usize];
                cur_val_level = INT_VAL
            } else {
                n = cur_val;
                if FONT_AREA[n as usize] as u32 == AAT_FONT_FLAG
                    || FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                {
                    scan_glyph_number(n as usize);
                } else {
                    scan_char_num();
                }
                k = cur_val;
                match m {
                    LP_CODE_BASE => {
                        cur_val = get_cp_code(n as usize, k as u32, LEFT_SIDE);
                        cur_val_level = INT_VAL
                    }
                    RP_CODE_BASE => {
                        cur_val = get_cp_code(n as usize, k as u32, RIGHT_SIDE);
                        cur_val_level = INT_VAL
                    }
                    _ => {}
                }
            }
        }
        REGISTER => {
            if m < 0 || m > 19 {
                /* 19 = "lo_mem_stat_max" */
                cur_val_level = (MEM[m as usize].b16.s1 as i32 / 64) as u8;
                if cur_val_level < GLUE_VAL {
                    cur_val = MEM[(m + 2) as usize].b32.s1
                } else {
                    cur_val = MEM[(m + 1) as usize].b32.s1
                }
            } else {
                scan_register_num();
                cur_val_level = m as u8;
                if cur_val > 255 {
                    find_sa_element(cur_val_level as small_number, cur_val, false);
                    if cur_ptr.is_texnull() {
                        cur_val = 0;
                    } else if cur_val_level < GLUE_VAL {
                        cur_val = MEM[(cur_ptr + 2) as usize].b32.s1
                    } else {
                        cur_val = MEM[(cur_ptr + 1) as usize].b32.s1
                    }
                } else {
                    match cur_val_level {
                        INT_VAL => cur_val = *COUNT_REG(cur_val as usize),
                        DIMEN_VAL => cur_val = *SCALED_REG(cur_val as usize),
                        GLUE_VAL => cur_val = *SKIP_REG(cur_val as usize),
                        MU_VAL => cur_val = *MU_SKIP_REG(cur_val as usize),
                        _ => {}
                    }
                }
            }
        }
        LAST_ITEM => {
            if m >= INPUT_LINE_NO_CODE {
                if m >= ETEX_GLUE {
                    /*1568:*/
                    if m < ETEX_MU {
                        match m {
                            MU_TO_GLUE_CODE => {
                                /*1595:*/
                                scan_mu_glue();
                            }
                            _ => {}
                        }
                        cur_val_level = GLUE_VAL
                    } else if m < ETEX_EXPR {
                        match m {
                            GLUE_TO_MU_CODE => {
                                /*1596:*/
                                scan_normal_glue(); /* if(m >= XETEX_DIM) */
                            }
                            _ => {}
                        }
                        cur_val_level = MU_VAL;
                    } else {
                        cur_val_level = (m - ETEX_EXPR) as u8;
                        scan_expr();
                    }
                    while cur_val_level as i32 > level as i32 {
                        if cur_val_level == GLUE_VAL {
                            m = cur_val;
                            cur_val = MEM[(m + 1) as usize].b32.s1;
                            delete_glue_ref(m as usize);
                        } else if cur_val_level == MU_VAL {
                            mu_error();
                        }
                        cur_val_level -= 1;
                    }
                    if negative {
                        if cur_val_level >= GLUE_VAL {
                            m = cur_val;
                            let cur_val_ = new_spec(m as usize);
                            cur_val = cur_val_ as i32;
                            delete_glue_ref(m as usize);
                            MEM[cur_val_ + 1].b32.s1 = -MEM[cur_val_ + 1].b32.s1;
                            MEM[cur_val_ + 2].b32.s1 = -MEM[cur_val_ + 2].b32.s1;
                            MEM[cur_val_ + 3].b32.s1 = -MEM[cur_val_ + 3].b32.s1
                        } else {
                            cur_val = -cur_val
                        }
                    }
                    return;
                }
                if m >= XETEX_DIM {
                    match m {
                        XETEX_GLYPH_BOUNDS_CODE => {
                            /*1435:*/
                            if FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == AAT_FONT_FLAG
                                || FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32
                                    == OTGR_FONT_FLAG
                            {
                                scan_int(); /* shellenabledp */
                                n = cur_val;
                                if n < 1 || n > 4 {
                                    if file_line_error_style_p != 0 {
                                        print_file_line();
                                    } else {
                                        print_nl_cstr(b"! ");
                                    }
                                    print_cstr(
                                        b"\\\\XeTeXglyphbounds requires an edge index from 1 to 4;",
                                    );
                                    print_nl_cstr(b"I don\'t know anything about edge ");
                                    print_int(n);
                                    error();
                                    cur_val = 0;
                                } else {
                                    scan_int();
                                    cur_val = get_glyph_bounds(
                                        EQTB[CUR_FONT_LOC].b32.s1 as usize,
                                        n,
                                        cur_val,
                                    )
                                }
                            } else {
                                not_native_font_error(
                                    LAST_ITEM,
                                    m,
                                    EQTB[CUR_FONT_LOC].b32.s1 as usize,
                                );
                                cur_val = 0;
                            }
                        }
                        FONT_CHAR_WD_CODE | FONT_CHAR_HT_CODE | FONT_CHAR_DP_CODE
                        | FONT_CHAR_IC_CODE => {
                            scan_font_ident();
                            q = cur_val;
                            scan_usv_num();
                            if FONT_AREA[q as usize] as u32 == AAT_FONT_FLAG
                                || FONT_AREA[q as usize] as u32 == OTGR_FONT_FLAG
                            {
                                match m {
                                    FONT_CHAR_WD_CODE => {
                                        cur_val = getnativecharwd(q as usize, cur_val)
                                    }
                                    FONT_CHAR_HT_CODE => {
                                        cur_val = getnativecharht(q as usize, cur_val)
                                    }
                                    FONT_CHAR_DP_CODE => {
                                        cur_val = getnativechardp(q as usize, cur_val)
                                    }
                                    FONT_CHAR_IC_CODE => {
                                        cur_val = getnativecharic(q as usize, cur_val)
                                    }
                                    _ => {}
                                }
                            } else if FONT_BC[q as usize] as i32 <= cur_val
                                && FONT_EC[q as usize] as i32 >= cur_val
                            {
                                i = FONT_CHARACTER_INFO(
                                    q as usize,
                                    effective_char(true, q as usize, cur_val as u16) as usize,
                                );
                                match m {
                                    FONT_CHAR_WD_CODE => {
                                        cur_val = *FONT_CHARINFO_WIDTH(q as usize, i);
                                    }
                                    FONT_CHAR_HT_CODE => {
                                        cur_val = *FONT_CHARINFO_HEIGHT(q as usize, i);
                                    }
                                    FONT_CHAR_DP_CODE => {
                                        cur_val = *FONT_CHARINFO_DEPTH(q as usize, i);
                                    }
                                    FONT_CHAR_IC_CODE => {
                                        cur_val = *FONT_CHARINFO_ITALCORR(q as usize, i);
                                    }
                                    _ => {}
                                }
                            } else {
                                cur_val = 0;
                            }
                        }
                        PAR_SHAPE_LENGTH_CODE | PAR_SHAPE_INDENT_CODE | PAR_SHAPE_DIMEN_CODE => {
                            q = cur_chr - PAR_SHAPE_LENGTH_CODE;
                            scan_int();
                            if LOCAL(Local::par_shape).is_texnull() || cur_val <= 0 {
                                cur_val = 0;
                            } else {
                                if q == 2 {
                                    q = cur_val % 2;
                                    cur_val = (cur_val + q) / 2;
                                }
                                if cur_val > MEM[*LOCAL(Local::par_shape) as usize].b32.s0 {
                                    cur_val = MEM[*LOCAL(Local::par_shape) as usize].b32.s0
                                }
                                cur_val = MEM[(*LOCAL(Local::par_shape) + 2 * cur_val - q) as usize]
                                    .b32
                                    .s1
                            }
                            cur_val_level = DIMEN_VAL
                        }
                        GLUE_STRETCH_CODE | GLUE_SHRINK_CODE => {
                            scan_normal_glue();
                            q = cur_val;
                            if m == GLUE_STRETCH_CODE {
                                cur_val = MEM[(q + 2) as usize].b32.s1
                            } else {
                                cur_val = MEM[(q + 3) as usize].b32.s1
                            }
                            delete_glue_ref(q as usize);
                        }
                        _ => {}
                    }
                    cur_val_level = DIMEN_VAL
                } else {
                    match m {
                        INPUT_LINE_NO_CODE => cur_val = line,
                        BADNESS_CODE => cur_val = last_badness,
                        PDF_SHELL_ESCAPE_CODE => cur_val = 0,
                        ETEX_VERSION_CODE => cur_val = ETEX_VERSION,
                        XETEX_VERSION_CODE => cur_val = XETEX_VERSION,
                        XETEX_COUNT_GLYPHS_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    cur_val = aat::aat_font_get(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                    )
                                }
                                OTGR_FONT_FLAG => {
                                    cur_val = ot_font_get(m - 14, FONT_LAYOUT_ENGINE[n as usize])
                                }
                                _ => cur_val = 0,
                            }
                        }
                        XETEX_COUNT_FEATURES_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    cur_val = aat::aat_font_get(
                                        m - 14i32,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                    )
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => cur_val = -1,
                                OTGR_FONT_FLAG => {
                                    if usingGraphite(
                                        FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                    ) as i32
                                        != 0
                                    {
                                        cur_val =
                                            ot_font_get(m - 14i32, FONT_LAYOUT_ENGINE[n as usize]);
                                    } else {
                                        cur_val = 0;
                                    }
                                }
                                _ => cur_val = 0,
                            }
                        }
                        XETEX_VARIATION_CODE
                        | XETEX_VARIATION_MIN_CODE
                        | XETEX_VARIATION_MAX_CODE
                        | XETEX_VARIATION_DEFAULT_CODE
                        | XETEX_COUNT_VARIATIONS_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            cur_val = 0;
                        }
                        XETEX_FEATURE_CODE_CODE
                        | XETEX_IS_EXCLUSIVE_FEATURE_CODE
                        | XETEX_COUNT_SELECTORS_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    cur_val = aat::aat_font_get_1(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                        k,
                                    )
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    cur_val = -1;
                                }
                                OTGR_FONT_FLAG => {
                                    if usingGraphite(
                                        FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                    ) as i32
                                        != 0
                                    {
                                        scan_int();
                                        k = cur_val;
                                        cur_val =
                                            ot_font_get_1(m - 14, FONT_LAYOUT_ENGINE[n as usize], k)
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                        cur_val = -1;
                                    }
                                }
                                _ => {
                                    not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                    cur_val = -1;
                                }
                            }
                        }
                        XETEX_SELECTOR_CODE_CODE | XETEX_IS_DEFAULT_SELECTOR_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    scan_int();
                                    cur_val = aat::aat_font_get_2(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                        k,
                                        cur_val,
                                    )
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    scan_int();
                                    cur_val = -1;
                                }
                                OTGR_FONT_FLAG => {
                                    if usingGraphite(
                                        FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                    ) as i32
                                        != 0
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_int();
                                        cur_val = ot_font_get_2(
                                            m - 14,
                                            FONT_LAYOUT_ENGINE[n as usize],
                                            k,
                                            cur_val,
                                        )
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                        cur_val = -1;
                                    }
                                }
                                _ => {
                                    not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                    cur_val = -1;
                                }
                            }
                        }
                        XETEX_FIND_VARIATION_BY_NAME_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    scan_and_pack_name();
                                    cur_val = aat::aat_font_get_named(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                    );
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => {
                                    scan_and_pack_name();
                                    cur_val = -1;
                                }
                                _ => {
                                    not_aat_font_error(LAST_ITEM, m, n as usize);
                                    cur_val = -1;
                                }
                            }
                        }
                        XETEX_FIND_FEATURE_BY_NAME_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    scan_and_pack_name();
                                    cur_val = aat::aat_font_get_named(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                    );
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => {
                                    scan_and_pack_name();
                                    cur_val = -1;
                                }
                                OTGR_FONT_FLAG => {
                                    if usingGraphite(
                                        FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                    ) as i32
                                        != 0
                                    {
                                        scan_and_pack_name();
                                        cur_val = gr_font_get_named(
                                            m - 14,
                                            FONT_LAYOUT_ENGINE[n as usize],
                                        )
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                        cur_val = -1;
                                    }
                                }
                                _ => {
                                    not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                    cur_val = -1;
                                }
                            }
                        }
                        XETEX_FIND_SELECTOR_BY_NAME_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            match FONT_AREA[n as usize] as u32 {
                                #[cfg(target_os = "macos")]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    scan_and_pack_name();
                                    cur_val = aat::aat_font_get_named_1(
                                        m - 14,
                                        (FONT_LAYOUT_ENGINE[n as usize]) as _,
                                        k,
                                    );
                                }
                                #[cfg(not(target_os = "macos"))]
                                AAT_FONT_FLAG => {
                                    scan_int();
                                    k = cur_val;
                                    scan_and_pack_name();
                                    cur_val = -1;
                                }
                                OTGR_FONT_FLAG => {
                                    if usingGraphite(
                                        FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                    ) as i32
                                        != 0
                                    {
                                        scan_int();
                                        k = cur_val;
                                        scan_and_pack_name();
                                        cur_val = gr_font_get_named_1(
                                            m - 14,
                                            FONT_LAYOUT_ENGINE[n as usize],
                                            k,
                                        )
                                    } else {
                                        not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                        cur_val = -1;
                                    }
                                }
                                _ => {
                                    not_aat_gr_font_error(LAST_ITEM, m, n as usize);
                                    cur_val = -1;
                                }
                            }
                        }
                        XETEX_OT_COUNT_SCRIPTS_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingOpenType(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                cur_val = ot_font_get(m - 14, FONT_LAYOUT_ENGINE[n as usize])
                            } else {
                                cur_val = 0;
                            }
                        }
                        XETEX_OT_COUNT_LANGUAGES_CODE | XETEX_OT_SCRIPT_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingOpenType(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                scan_int();
                                cur_val =
                                    ot_font_get_1(m - 14, FONT_LAYOUT_ENGINE[n as usize], cur_val)
                            } else {
                                not_ot_font_error(LAST_ITEM, m, n as usize);
                                cur_val = -1;
                            }
                        }
                        XETEX_OT_COUNT_FEATURES_CODE | XETEX_OT_LANGUAGE_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingOpenType(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                scan_int();
                                k = cur_val;
                                scan_int();
                                cur_val = ot_font_get_2(
                                    m - 14,
                                    FONT_LAYOUT_ENGINE[n as usize],
                                    k,
                                    cur_val,
                                )
                            } else {
                                not_ot_font_error(LAST_ITEM, m, n as usize);
                                cur_val = -1;
                            }
                        }
                        XETEX_OT_FEATURE_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingOpenType(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                scan_int();
                                k = cur_val;
                                scan_int();
                                kk = cur_val;
                                scan_int();
                                cur_val = ot_font_get_3(
                                    m - 14,
                                    FONT_LAYOUT_ENGINE[n as usize],
                                    k,
                                    kk,
                                    cur_val,
                                )
                            } else {
                                not_ot_font_error(LAST_ITEM, m, n as usize);
                                cur_val = -1;
                            }
                        }
                        XETEX_MAP_CHAR_TO_GLYPH_CODE => {
                            if FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == AAT_FONT_FLAG
                                || FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32
                                    == OTGR_FONT_FLAG
                            {
                                scan_int();
                                n = cur_val;
                                cur_val = map_char_to_glyph(EQTB[CUR_FONT_LOC].b32.s1 as usize, n)
                            } else {
                                not_native_font_error(
                                    LAST_ITEM,
                                    m,
                                    EQTB[CUR_FONT_LOC].b32.s1 as usize,
                                );
                                cur_val = 0;
                            }
                        }
                        XETEX_GLYPH_INDEX_CODE => {
                            if FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == AAT_FONT_FLAG
                                || FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32
                                    == OTGR_FONT_FLAG
                            {
                                scan_and_pack_name();
                                cur_val = map_glyph_to_index(EQTB[CUR_FONT_LOC].b32.s1 as usize)
                            } else {
                                not_native_font_error(
                                    LAST_ITEM,
                                    m,
                                    EQTB[CUR_FONT_LOC].b32.s1 as usize,
                                );
                                cur_val = 0;
                            }
                        }
                        XETEX_FONT_TYPE_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == AAT_FONT_FLAG {
                                cur_val = 1;
                            } else if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingOpenType(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                cur_val = 2;
                            } else if FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                                && usingGraphite(
                                    FONT_LAYOUT_ENGINE[n as usize] as XeTeXLayoutEngine,
                                ) as i32
                                    != 0
                            {
                                cur_val = 3;
                            } else {
                                cur_val = 0;
                            }
                        }
                        XETEX_FIRST_CHAR_CODE | XETEX_LAST_CHAR_CODE => {
                            scan_font_ident();
                            n = cur_val;
                            if FONT_AREA[n as usize] as u32 == AAT_FONT_FLAG
                                || FONT_AREA[n as usize] as u32 == OTGR_FONT_FLAG
                            {
                                cur_val = get_font_char_range(
                                    n as usize,
                                    (m == XETEX_FIRST_CHAR_CODE) as i32,
                                )
                            } else if m == XETEX_FIRST_CHAR_CODE {
                                cur_val = FONT_BC[n as usize] as i32
                            } else {
                                cur_val = FONT_EC[n as usize] as i32
                            }
                        }
                        PDF_LAST_X_POS_CODE => cur_val = pdf_last_x_pos,
                        PDF_LAST_Y_POS_CODE => cur_val = pdf_last_y_pos,
                        XETEX_PDF_PAGE_COUNT_CODE => {
                            scan_and_pack_name();
                            cur_val = count_pdf_file_pages()
                        }
                        CURRENT_GROUP_LEVEL_CODE => cur_val = cur_level as i32 - 1,
                        CURRENT_GROUP_TYPE_CODE => cur_val = cur_group as i32,
                        CURRENT_IF_LEVEL_CODE => {
                            q = cond_ptr;
                            cur_val = 0;
                            while !q.is_texnull() {
                                cur_val += 1;
                                q = *LLIST_link(q as usize)
                            }
                        }
                        CURRENT_IF_TYPE_CODE => {
                            if cond_ptr.is_texnull() {
                                cur_val = 0;
                            } else if (cur_if as i32) < UNLESS_CODE {
                                cur_val = cur_if as i32 + 1;
                            } else {
                                cur_val = -(cur_if as i32 - 31)
                            }
                        }
                        CURRENT_IF_BRANCH_CODE => {
                            if if_limit == OR_CODE || if_limit == ELSE_CODE {
                                cur_val = 1;
                            } else if if_limit == FI_CODE {
                                cur_val = -1;
                            } else {
                                cur_val = 0;
                            }
                        }
                        GLUE_STRETCH_ORDER_CODE | GLUE_SHRINK_ORDER_CODE => {
                            scan_normal_glue();
                            q = cur_val;
                            if m == GLUE_STRETCH_ORDER_CODE {
                                cur_val = MEM[q as usize].b16.s1 as i32
                            } else {
                                cur_val = MEM[q as usize].b16.s0 as i32
                            }
                            delete_glue_ref(q as usize);
                        }
                        _ => {}
                    }
                    cur_val_level = INT_VAL;
                }
            } else {
                cur_val = 0;
                tx = cur_list.tail as i32;
                if tx < hi_mem_min {
                    if MEM[tx as usize].b16.s1 == MATH_NODE && MEM[tx as usize].b16.s0 == END_M_CODE
                    {
                        r = cur_list.head as i32;
                        loop {
                            q = r;
                            r = MEM[q as usize].b32.s1;
                            if !(r != tx) {
                                break;
                            }
                        }
                        tx = q
                    }
                }
                if cur_chr == LAST_NODE_TYPE_CODE as i32 {
                    cur_val_level = INT_VAL;
                    if tx == cur_list.head as i32 || cur_list.mode as i32 == 0 {
                        cur_val = -1;
                    }
                } else {
                    cur_val_level = cur_chr as u8
                }
                if tx < hi_mem_min && cur_list.mode as i32 != 0 {
                    match cur_chr as u8 {
                        INT_VAL => {
                            if *NODE_type(tx as usize) == PENALTY_NODE {
                                cur_val = MEM[(tx + 1) as usize].b32.s1;
                            }
                        }
                        DIMEN_VAL => {
                            if *NODE_type(tx as usize) == KERN_NODE {
                                cur_val = MEM[(tx + 1) as usize].b32.s1;
                            }
                        }
                        GLUE_VAL => {
                            if *NODE_type(tx as usize) == GLUE_NODE {
                                cur_val = MEM[(tx + 1) as usize].b32.s0;
                                if MEM[tx as usize].b16.s0 == MU_GLUE {
                                    cur_val_level = MU_VAL;
                                }
                            }
                        }
                        LAST_NODE_TYPE_CODE => {
                            if *NODE_type(tx as usize) <= UNSET_NODE {
                                cur_val = MEM[tx as usize].b16.s1 as i32 + 1;
                            } else {
                                cur_val = UNSET_NODE as i32 + 2;
                            }
                        }
                        _ => {}
                    }
                } else if cur_list.mode == VMODE && tx == cur_list.head as i32 {
                    match cur_chr as u8 {
                        INT_VAL => cur_val = last_penalty,
                        DIMEN_VAL => cur_val = last_kern,
                        GLUE_VAL => {
                            if last_glue != MAX_HALFWORD {
                                cur_val = last_glue
                            }
                        }
                        LAST_NODE_TYPE_CODE => cur_val = last_node_type,
                        _ => {}
                    }
                }
            }
        }
        _ => {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"You can\'t use `");
            print_cmd_chr(cur_cmd as u16, cur_chr);
            print_cstr(b"\' after ");
            print_esc_cstr(b"the");
            help_ptr = 1;
            help_line[0] = b"I\'m forgetting what you said and using zero instead.";
            error();
            cur_val = 0;
            if level != TOK_VAL as i16 {
                cur_val_level = DIMEN_VAL;
            } else {
                cur_val_level = INT_VAL;
            }
        }
    }
    while cur_val_level as i32 > level as i32 {
        /*447:*/
        if cur_val_level == GLUE_VAL {
            cur_val = MEM[(cur_val + 1) as usize].b32.s1
        } else if cur_val_level == MU_VAL {
            mu_error();
        }
        cur_val_level -= 1;
    }
    if negative {
        if cur_val_level >= GLUE_VAL {
            let cur_val_ = new_spec(cur_val as usize);
            cur_val = cur_val_ as i32;
            MEM[cur_val_ + 1].b32.s1 = -MEM[cur_val_ + 1].b32.s1;
            MEM[cur_val_ + 2].b32.s1 = -MEM[cur_val_ + 2].b32.s1;
            MEM[cur_val_ + 3].b32.s1 = -MEM[cur_val_ + 3].b32.s1
        } else {
            cur_val = -cur_val;
        }
    } else if cur_val_level >= GLUE_VAL && cur_val_level <= MU_VAL {
        MEM[cur_val as usize].b32.s1 += 1;
    };
}
pub(crate) unsafe fn scan_int() {
    let mut d: small_number = 0;
    radix = 0;
    let mut OK_so_far = true;
    let mut negative = false;
    loop {
        loop
        /*424:*/
        {
            get_x_token();
            if !(cur_cmd == SPACER as u8) {
                break;
            }
        }
        if cur_tok == OTHER_TOKEN + '-' as i32 {
            negative = !negative;
            cur_tok = OTHER_TOKEN + '+' as i32
        }
        if !(cur_tok == OTHER_TOKEN + '+' as i32) {
            break;
        }
    }
    if cur_tok == ALPHA_TOKEN as i32 {
        /*460:*/
        get_token(); /*461:*/
        if cur_tok < CS_TOKEN_FLAG {
            cur_val = cur_chr; /*462:*/
            if cur_cmd <= RIGHT_BRACE as u8 {
                if cur_cmd == RIGHT_BRACE as u8 {
                    align_state += 1;
                } else {
                    align_state -= 1;
                }
            }
        } else if cur_tok < CS_TOKEN_FLAG + SINGLE_BASE as i32 {
            cur_val = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE as i32)
        } else {
            cur_val = cur_tok - (CS_TOKEN_FLAG + SINGLE_BASE as i32)
        } /*:463*/
        if cur_val > BIGGEST_USV as i32 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Improper alphabetic constant");
            help_ptr = 2_u8;
            help_line[1] = b"A one-character control sequence belongs after a ` mark.";
            help_line[0] = b"So I\'m essentially inserting \\0 here.";
            cur_val = '0' as i32;
            back_error();
        } else {
            get_x_token();
            if cur_cmd != SPACER as u8 {
                back_input();
            }
        }
    } else if cur_cmd >= MIN_INTERNAL && cur_cmd <= MAX_INTERNAL {
        scan_something_internal(INT_VAL as small_number, false);
    } else {
        radix = 10;
        let mut m = 0xccccccc;
        if cur_tok == OCTAL_TOKEN {
            radix = 8;
            m = 0x10000000;
            get_x_token();
        } else if cur_tok == HEX_TOKEN {
            radix = 16;
            m = 0x8000000;
            get_x_token();
        }
        let mut vacuous = true;
        cur_val = 0;
        loop {
            if cur_tok < ZERO_TOKEN + radix as i32
                && cur_tok >= ZERO_TOKEN
                && cur_tok <= ZERO_TOKEN + 9
            {
                d = (cur_tok - ZERO_TOKEN) as small_number
            } else {
                if !(radix as i32 == 16) {
                    break;
                }
                if cur_tok <= A_TOKEN + 5 && cur_tok >= A_TOKEN {
                    d = (cur_tok - A_TOKEN + 10) as small_number
                } else {
                    if !(cur_tok <= OTHER_A_TOKEN + 5 && cur_tok >= OTHER_A_TOKEN) {
                        break;
                    }
                    d = (cur_tok - OTHER_A_TOKEN + 10) as small_number
                }
            }
            vacuous = false;
            if cur_val >= m && (cur_val > m || d as i32 > 7 || radix as i32 != 10) {
                if OK_so_far {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Number too big");
                    help_ptr = 2_u8;
                    help_line[1] = b"I can only go up to 2147483647=\'17777777777=\"7FFFFFFF,";
                    help_line[0] = b"so I\'m using that number instead of yours.";
                    error();
                    cur_val = TEX_INFINITY;
                    OK_so_far = false
                }
            } else {
                cur_val = cur_val * radix as i32 + d as i32
            }
            get_x_token();
        }
        if vacuous {
            /*464:*/
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Missing number, treated as zero");
            help_ptr = 3_u8;
            help_line[2] = b"A number should have been here; I inserted `0\'.";
            help_line[1] = b"(If you can\'t figure out why I needed to see a number,";
            help_line[0] = b"look up `weird error\' in the index to The TeXbook.)";
            back_error();
        } else if cur_cmd != SPACER as u8 {
            back_input();
        }
    }
    if negative {
        cur_val = -cur_val;
    };
}
unsafe fn round_decimals(mut k: small_number) -> scaled_t {
    let mut a: i32 = 0;
    while k as i32 > 0 {
        k -= 1;
        a = (a + dig[k as usize] as i32 * 0x20000) / 10
    }
    (a + 1) / 2
}
pub(crate) unsafe fn xetex_scan_dimen(
    mut mu: bool,
    mut inf: bool,
    mut shortcut: bool,
    mut requires_units: bool,
) {
    let mut current_block: u64;
    let mut negative: bool = false;
    let mut f: i32 = 0;
    let mut num: i32 = 0;
    let mut denom: i32 = 0;
    let mut k: small_number = 0;
    let mut kk: small_number = 0;
    let mut p: i32 = 0;
    let mut v: scaled_t = 0;
    let mut save_cur_val: i32 = 0;

    f = 0;
    arith_error = false;
    cur_order = NORMAL as glue_ord;
    negative = false;
    if !shortcut {
        negative = false;
        loop {
            loop {
                get_x_token();
                if !(cur_cmd == SPACER as u8) {
                    break;
                }
            }
            if cur_tok == OTHER_TOKEN + '-' as i32 {
                negative = !negative;
                cur_tok = OTHER_TOKEN + '+' as i32
            }
            if !(cur_tok == OTHER_TOKEN + '+' as i32) {
                break;
            }
        }
        if cur_cmd >= MIN_INTERNAL && cur_cmd <= MAX_INTERNAL {
            /*468:*/
            if mu {
                scan_something_internal(MU_VAL as small_number, false);
                if cur_val_level >= GLUE_VAL {
                    v = MEM[(cur_val + 1) as usize].b32.s1;
                    delete_glue_ref(cur_val as usize);
                    cur_val = v;
                }
                if cur_val_level == MU_VAL {
                    current_block = 16246449912548656671;
                } else {
                    if cur_val_level != INT_VAL {
                        mu_error();
                    }
                    current_block = 5028470053297453708;
                }
            } else {
                scan_something_internal(DIMEN_VAL as small_number, false);
                if cur_val_level == DIMEN_VAL {
                    current_block = 16246449912548656671;
                } else {
                    current_block = 5028470053297453708;
                }
            }
        } else {
            back_input();
            if cur_tok == CONTINENTAL_POINT_TOKEN {
                cur_tok = POINT_TOKEN;
            }
            if cur_tok != POINT_TOKEN {
                scan_int();
            } else {
                radix = 10;
                cur_val = 0;
            }
            if cur_tok == CONTINENTAL_POINT_TOKEN {
                cur_tok = POINT_TOKEN;
            }
            if radix as i32 == 10 && cur_tok == POINT_TOKEN {
                /*471:*/
                k = 0; /* if(requires_units) */
                p = TEX_NULL;
                get_token();
                loop {
                    get_x_token();
                    if cur_tok > ZERO_TOKEN + 9 || cur_tok < ZERO_TOKEN {
                        break;
                    }
                    if (k as i32) < 17 {
                        let q = get_avail();
                        MEM[q].b32.s1 = p;
                        MEM[q].b32.s0 = cur_tok - ZERO_TOKEN;
                        p = q as i32;
                        k += 1
                    }
                }
                kk = k;
                while kk as i32 >= 1 {
                    dig[(kk as i32 - 1) as usize] = MEM[p as usize].b32.s0 as u8;
                    let q = p as usize;
                    p = *LLIST_link(p as usize);
                    MEM[q].b32.s1 = avail;
                    avail = q as i32;
                    kk -= 1
                }
                f = round_decimals(k);
                if cur_cmd != SPACER as u8 {
                    back_input();
                }
            }
            current_block = 5028470053297453708;
        }
    } else {
        current_block = 5028470053297453708;
    }
    match current_block {
        5028470053297453708 => {
            if cur_val < 0 {
                negative = !negative;
                cur_val = -cur_val
            }
            if requires_units {
                if inf {
                    /*473:*/
                    if scan_keyword(b"fil") {
                        cur_order = FIL as glue_ord;
                        while scan_keyword(b"l") {
                            if cur_order as i32 == FILLL {
                                if file_line_error_style_p != 0 {
                                    print_file_line();
                                } else {
                                    print_nl_cstr(b"! ");
                                }
                                print_cstr(b"Illegal unit of measure (");
                                print_cstr(b"replaced by filll)");
                                help_ptr = 1;
                                help_line[0] = b"I dddon\'t go any higher than filll.";
                                error();
                            } else {
                                cur_order += 1;
                            }
                        }
                        current_block = 6063453238281986051;
                    } else {
                        current_block = 2750570471926810434;
                    }
                } else {
                    current_block = 2750570471926810434;
                }
                match current_block {
                    2750570471926810434 => {
                        save_cur_val = cur_val;
                        loop {
                            get_x_token();
                            if !(cur_cmd == SPACER as u8) {
                                break;
                            }
                        }
                        if cur_cmd < MIN_INTERNAL || cur_cmd > MAX_INTERNAL {
                            back_input();
                            if mu {
                                current_block = 17751730340908002208;
                            } else {
                                if scan_keyword(b"em") {
                                    v = FONT_INFO[(QUAD_CODE
                                        + PARAM_BASE[EQTB[CUR_FONT_LOC].b32.s1 as usize])
                                        as usize]
                                        .b32
                                        .s1;
                                    current_block = 5195798230510548452;
                                } else if scan_keyword(b"ex") {
                                    v = FONT_INFO[(X_HEIGHT_CODE
                                        + PARAM_BASE[EQTB[CUR_FONT_LOC].b32.s1 as usize])
                                        as usize]
                                        .b32
                                        .s1;
                                    current_block = 5195798230510548452;
                                } else {
                                    current_block = 17751730340908002208;
                                }
                                match current_block {
                                    17751730340908002208 => {}
                                    _ => {
                                        get_x_token();
                                        if cur_cmd != SPACER as u8 {
                                            back_input();
                                        }
                                        current_block = 7531702508219610202;
                                    }
                                }
                            }
                            match current_block {
                                7531702508219610202 => {}
                                _ => {
                                    if mu {
                                        /*475:*/
                                        if scan_keyword(b"mu") {
                                            current_block = 6063453238281986051;
                                        } else {
                                            if file_line_error_style_p != 0 {
                                                print_file_line();
                                            } else {
                                                print_nl_cstr(b"! ");
                                            }
                                            print_cstr(b"Illegal unit of measure (");
                                            print_cstr(b"mu inserted)");
                                            help_ptr = 4;
                                            help_line[3] =
                                                b"The unit of measurement in math glue must be mu.";
                                            help_line[2] =
                                                b"To recover gracefully from this error, it\'s best to";
                                            help_line[1] =
                                                b"delete the erroneous units; e.g., type `2\' to delete";
                                            help_line[0] =
                                                b"two letters. (See Chapter 27 of The TeXbook.)";
                                            error();
                                            current_block = 6063453238281986051;
                                        }
                                    } else {
                                        if scan_keyword(b"true") {
                                            /*476:*/
                                            prepare_mag(); /* magic ratio consant */
                                            if *INTPAR(IntPar::mag) != 1000 {
                                                cur_val =
                                                    xn_over_d(cur_val, 1000, *INTPAR(IntPar::mag)); /* magic ratio consant */
                                                f = (((1000 * f) as i64
                                                    + 65536 * tex_remainder as i64)
                                                    / *INTPAR(IntPar::mag) as i64)
                                                    as i32;
                                                cur_val =
                                                    (cur_val as i64 + f as i64 / 65536) as i32;
                                                f = (f as i64 % 65536) as i32
                                            }
                                        }
                                        if scan_keyword(b"pt") {
                                            current_block = 6063453238281986051;
                                        } else {
                                            if scan_keyword(b"in") {
                                                num = 7227;
                                                denom = 100;
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"pc") {
                                                num = 12;
                                                denom = 1;
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"cm") {
                                                num = 7227; // magic ratio consant
                                                denom = 254; // magic ratio consant
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"mm") {
                                                num = 7227; // magic ratio consant
                                                denom = 2540; // magic ratio consant
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"bp") {
                                                num = 7227; // magic ratio consant
                                                denom = 7200; // magic ratio consant
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"dd") {
                                                num = 1238; // magic ratio consant
                                                denom = 1157; // magic ratio consant
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"cc") {
                                                num = 14856; // magic ratio consant
                                                denom = 1157; // magic ratio consant
                                                current_block = 15908231092227701503;
                                            } else if scan_keyword(b"sp") {
                                                current_block = 8982780081639585757;
                                            /*478:*/
                                            } else {
                                                if file_line_error_style_p != 0 {
                                                    print_file_line();
                                                } else {
                                                    print_nl_cstr(b"! ");
                                                }
                                                print_cstr(b"Illegal unit of measure (");
                                                print_cstr(b"pt inserted)");
                                                help_ptr = 6;
                                                help_line[5] =
                                                    b"Dimensions can be in units of em, ex, in, pt, pc,";
                                                help_line[4] =
                                                    b"cm, mm, dd, cc, bp, or sp; but yours is a new one!";
                                                help_line[3] =
                                                    b"I\'ll assume that you meant to say pt, for printer\'s points.";
                                                help_line[2] =
                                                    b"To recover gracefully from this error, it\'s best to";
                                                help_line[1] =
                                                    b"delete the erroneous units; e.g., type `2\' to delete";
                                                help_line[0] =
                                                    b"two letters. (See Chapter 27 of The TeXbook.)";
                                                error();
                                                current_block = 6063453238281986051;
                                            }
                                            match current_block {
                                                6063453238281986051 => {}
                                                8982780081639585757 => {}
                                                _ => {
                                                    cur_val = xn_over_d(cur_val, num, denom);
                                                    f = (((num * f) as i64
                                                        + 65536 * tex_remainder as i64)
                                                        / denom as i64)
                                                        as i32;
                                                    cur_val =
                                                        (cur_val as i64 + f as i64 / 65536) as i32;
                                                    f = (f as i64 % 65536) as i32;
                                                    current_block = 6063453238281986051;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            if mu {
                                scan_something_internal(MU_VAL as i16, false);
                                if cur_val_level >= GLUE_VAL {
                                    v = MEM[(cur_val + 1) as usize].b32.s1;
                                    delete_glue_ref(cur_val as usize);
                                    cur_val = v
                                }
                                if cur_val_level != MU_VAL {
                                    mu_error();
                                }
                            } else {
                                scan_something_internal(DIMEN_VAL as small_number, false);
                            }
                            v = cur_val;
                            current_block = 7531702508219610202;
                        }
                        match current_block {
                            6063453238281986051 => {}
                            8982780081639585757 => {}
                            _ => {
                                cur_val = mult_and_add(
                                    save_cur_val,
                                    v,
                                    xn_over_d(v, f, 65536 as i32),
                                    0x3fffffff,
                                );
                                current_block = 16246449912548656671;
                            }
                        }
                    }
                    _ => {}
                }
                match current_block {
                    16246449912548656671 => {}
                    _ => {
                        match current_block {
                            6063453238281986051 => {
                                if cur_val >= 16384 {
                                    arith_error = true
                                } else {
                                    cur_val = (cur_val as i64 * 65536 + f as i64) as i32
                                }
                            }
                            _ => {}
                        }
                        get_x_token();
                        if cur_cmd != SPACER as u8 {
                            back_input();
                        }
                    }
                }
            } else if cur_val >= 16384 {
                arith_error = true
            } else {
                cur_val = (cur_val as i64 * 65536 + f as i64) as i32
            }
        }
        _ => {}
    }
    if arith_error as i32 != 0 || cur_val.wrapping_abs() >= 0x40000000 { // TODO: check
        /*479:*/
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Dimension too large");
        help_ptr = 2;
        help_line[1] = b"I can\'t work with sizes bigger than about 19 feet.";
        help_line[0] = b"Continue and I\'ll use the largest value I can.";
        error();
        cur_val = MAX_HALFWORD;
        arith_error = false
    }
    if negative {
        cur_val = -cur_val
    };
}
pub(crate) unsafe fn scan_dimen(mut mu: bool, mut inf: bool, mut shortcut: bool) {
    xetex_scan_dimen(mu, inf, shortcut, true);
}
pub(crate) unsafe fn scan_decimal() {
    xetex_scan_dimen(false, false, false, false);
}
pub(crate) unsafe fn scan_glue(mut level: small_number) {
    let mut negative: bool = false;
    let mut mu: bool = false;
    mu = level == MU_VAL as i16;
    negative = false;
    loop {
        loop {
            get_x_token();
            if !(cur_cmd == SPACER as u8) {
                break;
            }
        }
        if cur_tok == OTHER_TOKEN + 45 {
            /*"-"*/
            negative = !negative;
            cur_tok = OTHER_TOKEN + 43
            /*"+"*/
        }
        if !(cur_tok == OTHER_TOKEN + 43) {
            break;
        }
        /*"+"*/
    }
    if cur_cmd >= MIN_INTERNAL && cur_cmd <= MAX_INTERNAL {
        scan_something_internal(level, negative);
        if cur_val_level >= GLUE_VAL {
            if cur_val_level as i16 != level {
                mu_error();
            }
            return;
        }
        if cur_val_level == INT_VAL {
            scan_dimen(mu, false, true);
        } else if level == MU_VAL as i16 {
            mu_error();
        }
    } else {
        back_input();
        scan_dimen(mu, false, false);
        if negative {
            cur_val = -cur_val;
        }
    }
    let q = new_spec(0);
    MEM[(q + 1) as usize].b32.s1 = cur_val;
    if scan_keyword(b"plus") {
        scan_dimen(mu, true, false);
        MEM[(q + 2) as usize].b32.s1 = cur_val;
        MEM[q as usize].b16.s1 = cur_order as u16
    }
    if scan_keyword(b"minus") {
        scan_dimen(mu, true, false);
        MEM[(q + 3) as usize].b32.s1 = cur_val;
        MEM[q as usize].b16.s0 = cur_order as u16
    }
    cur_val = q as i32;
    /*:481*/
}
pub(crate) unsafe fn add_or_sub(
    mut x: i32,
    mut y: i32,
    mut max_answer: i32,
    mut negative: bool,
) -> i32 {
    let mut a: i32 = 0;
    if negative {
        y = -y
    }
    if x >= 0 {
        if y <= max_answer - x {
            a = x + y;
        } else {
            arith_error = true;
            a = 0;
        }
    } else if y >= -max_answer - x {
        a = x + y;
    } else {
        arith_error = true;
        a = 0;
    }
    a
}
pub(crate) unsafe fn quotient(mut n: i32, mut d: i32) -> i32 {
    let mut negative: bool = false;
    let mut a: i32 = 0;
    if d == 0 {
        arith_error = true;
        a = 0
    } else {
        if d > 0 {
            negative = false
        } else {
            d = -d;
            negative = true
        }
        if n < 0 {
            n = -n;
            negative = !negative
        }
        a = n / d;
        n = n - a * d;
        d = n - d;
        if d + n >= 0 {
            a += 1;
        }
        if negative {
            a = -a;
        }
    }
    a
}
pub(crate) unsafe fn fract(mut x: i32, mut n: i32, mut d: i32, mut max_answer: i32) -> i32 {
    fn too_big() -> i32 {
        unsafe {
            arith_error = true;
        }
        0
    }
    fn found(a: i32, negative: bool) -> i32 {
        if negative {
            -a
        } else {
            a
        }
    }

    let mut a: i32 = 0;
    let mut f: i32 = 0;
    let mut h: i32 = 0;
    let mut r: i32 = 0;
    let mut t: i32 = 0;
    if d == 0 {
        return too_big();
    }
    a = 0;
    let mut negative = if d > 0 {
        false
    } else {
        d = -d;
        true
    };
    if x < 0 {
        x = -x;
        negative = !negative;
    } else if x == 0 {
        return a;
    }
    if n < 0 {
        n = -n;
        negative = !negative
    }
    t = n / d;
    if t > max_answer / x {
        return too_big();
    }
    a = t * x;
    n = n - t * d;
    if n == 0 {
        return found(a, negative);
    }
    t = x / d;
    if t > (max_answer - a) / n {
        return too_big();
    }
    a = a + t * n;
    x = x - t * d;
    if x == 0 {
        return found(a, negative);
    }
    if x < n {
        t = x;
        x = n;
        n = t
    }
    f = 0;
    r = d / 2 - d;
    h = -r;
    loop {
        if n & 1 != 0 {
            r = r + x;
            if r >= 0 {
                r = r - d;
                f += 1
            }
        }
        n = n / 2;
        if n == 0 {
            break;
        }
        if x < h {
            x = x + x
        } else {
            t = x - d;
            x = t + x;
            f = f + n;
            if !(x < n) {
                continue;
            }
            if x == 0 {
                break;
            }
            t = x;
            x = n;
            n = t
        }
    }
    if f > max_answer - a {
        too_big()
    } else {
        a = a + f;
        found(a, negative)
    }
}
pub(crate) unsafe fn scan_expr() {
    let mut r: small_number = 0;
    let mut s: small_number = 0;
    let mut o: small_number = 0;
    let mut e: i32 = 0;
    let mut t: i32 = 0;
    let mut f: i32 = 0;
    let mut n: i32 = 0;
    let mut q: i32 = 0;
    let mut l = cur_val_level as small_number;
    let mut a = arith_error;
    let mut b = false;
    let mut p = TEX_NULL;
    'c_78022: loop {
        r = EXPR_NONE as small_number;
        e = 0;
        s = EXPR_NONE as small_number;
        t = 0;
        n = 0;
        loop {
            if s == EXPR_NONE {
                o = l
            } else {
                o = INT_VAL as small_number
            }
            loop {
                get_x_token();
                if !(cur_cmd == SPACER as u8) {
                    break;
                }
            }
            if cur_tok == OTHER_TOKEN + 40 {
                break;
            }
            back_input();
            if o == INT_VAL as i16 {
                scan_int();
            } else if o == DIMEN_VAL as i16 {
                scan_dimen(false, false, false);
            } else if o == GLUE_VAL as i16 {
                scan_normal_glue();
            } else {
                scan_mu_glue();
            }
            f = cur_val;
            loop {
                loop
                /*1572:*//*424:*/
                {
                    get_x_token();
                    if !(cur_cmd == SPACER as u8) {
                        break;
                    }
                }
                if cur_tok == OTHER_TOKEN + 43 {
                    o = EXPR_ADD as small_number
                } else if cur_tok == OTHER_TOKEN + 45 {
                    o = EXPR_SUB as small_number
                } else if cur_tok == OTHER_TOKEN + 42 {
                    o = EXPR_MULT as small_number
                } else if cur_tok == OTHER_TOKEN + 47 {
                    o = EXPR_DIV as small_number
                } else {
                    o = EXPR_NONE as small_number;
                    if p.is_texnull() {
                        if cur_cmd != RELAX as u8 {
                            back_input();
                        }
                    } else if cur_tok != OTHER_TOKEN + 41 {
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! ");
                        }
                        print_cstr(b"Missing ) inserted for expression");
                        help_ptr = 1;
                        help_line[0] =
                            b"I was expecting to see `+\', `-\', `*\', `/\', or `)\'. Didn\'t.";
                        back_error();
                    }
                }
                arith_error = b;
                if l == INT_VAL as i16 || s > EXPR_SUB {
                    if f > TEX_INFINITY || f < -TEX_INFINITY {
                        arith_error = true;
                        f = 0;
                    }
                } else if l == DIMEN_VAL as i16 {
                    if f.abs() > MAX_HALFWORD {
                        arith_error = true;
                        f = 0;
                    }
                } else if (MEM[(f + 1) as usize].b32.s1).abs() > MAX_HALFWORD
                    || (MEM[(f + 2) as usize].b32.s1).abs() > MAX_HALFWORD
                    || (MEM[(f + 3) as usize].b32.s1).abs() > MAX_HALFWORD
                {
                    arith_error = true;
                    delete_glue_ref(f as usize);
                    f = new_spec(0) as i32
                }
                match s as i32 {
                    0 => {
                        /*1579: */
                        t = if l >= GLUE_VAL as i16 && o != EXPR_NONE {
                            let t = new_spec(f as usize);
                            delete_glue_ref(f as usize);
                            if MEM[t + 2].b32.s1 == 0 {
                                MEM[t].b16.s1 = NORMAL;
                            }
                            if MEM[t + 3].b32.s1 == 0 {
                                MEM[t].b16.s0 = NORMAL;
                            }
                            t as i32
                        } else {
                            f
                        };
                    }
                    3 => {
                        if o == EXPR_DIV {
                            n = f;
                            o = EXPR_SCALE as small_number
                        } else if l == INT_VAL as i16 {
                            t = mult_and_add(t, f, 0, TEX_INFINITY)
                        } else if l == DIMEN_VAL as i16 {
                            t = mult_and_add(t, f, 0, MAX_HALFWORD)
                        } else {
                            MEM[(t + 1) as usize].b32.s1 =
                                mult_and_add(MEM[(t + 1) as usize].b32.s1, f, 0, 0x3fffffff);
                            MEM[(t + 2) as usize].b32.s1 =
                                mult_and_add(MEM[(t + 2) as usize].b32.s1, f, 0, 0x3fffffff);
                            MEM[(t + 3) as usize].b32.s1 =
                                mult_and_add(MEM[(t + 3) as usize].b32.s1, f, 0, 0x3fffffff)
                        }
                    }
                    4 => {
                        if l < GLUE_VAL as i16 {
                            t = quotient(t, f)
                        } else {
                            MEM[(t + 1) as usize].b32.s1 =
                                quotient(MEM[(t + 1) as usize].b32.s1, f);
                            MEM[(t + 2) as usize].b32.s1 =
                                quotient(MEM[(t + 2) as usize].b32.s1, f);
                            MEM[(t + 3) as usize].b32.s1 = quotient(MEM[(t + 3) as usize].b32.s1, f)
                        }
                    }
                    5 => {
                        if l == INT_VAL as i16 {
                            t = fract(t, n, f, TEX_INFINITY)
                        } else if l == DIMEN_VAL as i16 {
                            t = fract(t, n, f, MAX_HALFWORD)
                        } else {
                            MEM[(t + 1) as usize].b32.s1 =
                                fract(MEM[(t + 1) as usize].b32.s1, n, f, 0x3fffffff);
                            MEM[(t + 2) as usize].b32.s1 =
                                fract(MEM[(t + 2) as usize].b32.s1, n, f, 0x3fffffff);
                            MEM[(t + 3) as usize].b32.s1 =
                                fract(MEM[(t + 3) as usize].b32.s1, n, f, 0x3fffffff)
                        }
                    }
                    _ => {}
                }
                if o > EXPR_SUB {
                    s = o;
                } else {
                    /*1580: */
                    s = EXPR_NONE as small_number;
                    if r == EXPR_NONE {
                        e = t
                    } else if l == INT_VAL as i16 {
                        e = add_or_sub(e, t, TEX_INFINITY, r == EXPR_SUB)
                    } else if l == DIMEN_VAL as i16 {
                        e = add_or_sub(e, t, MAX_HALFWORD, r == EXPR_SUB)
                    } else {
                        /*1582: */
                        MEM[(e + 1) as usize].b32.s1 = add_or_sub(
                            MEM[(e + 1) as usize].b32.s1,
                            MEM[(t + 1) as usize].b32.s1,
                            MAX_HALFWORD,
                            r == EXPR_SUB,
                        );
                        if MEM[e as usize].b16.s1 as i32 == MEM[t as usize].b16.s1 as i32 {
                            MEM[(e + 2) as usize].b32.s1 = add_or_sub(
                                MEM[(e + 2) as usize].b32.s1,
                                MEM[(t + 2) as usize].b32.s1,
                                MAX_HALFWORD,
                                r == EXPR_SUB,
                            )
                        } else if (MEM[e as usize].b16.s1 as i32) < MEM[t as usize].b16.s1 as i32
                            && MEM[(t + 2) as usize].b32.s1 != 0
                        {
                            MEM[(e + 2) as usize].b32.s1 = MEM[(t + 2) as usize].b32.s1;
                            MEM[e as usize].b16.s1 = MEM[t as usize].b16.s1
                        }
                        if MEM[e as usize].b16.s0 as i32 == MEM[t as usize].b16.s0 as i32 {
                            MEM[(e + 3) as usize].b32.s1 = add_or_sub(
                                MEM[(e + 3) as usize].b32.s1,
                                MEM[(t + 3) as usize].b32.s1,
                                MAX_HALFWORD,
                                r == EXPR_SUB,
                            )
                        } else if (MEM[e as usize].b16.s0 as i32) < MEM[t as usize].b16.s0 as i32
                            && MEM[(t + 3) as usize].b32.s1 != 0
                        {
                            MEM[(e + 3) as usize].b32.s1 = MEM[(t + 3) as usize].b32.s1;
                            MEM[e as usize].b16.s0 = MEM[t as usize].b16.s0
                        }
                        delete_glue_ref(t as usize);
                        if MEM[(e + 2) as usize].b32.s1 == 0 {
                            MEM[e as usize].b16.s1 = NORMAL;
                        }
                        if MEM[(e + 3) as usize].b32.s1 == 0 {
                            MEM[e as usize].b16.s0 = NORMAL;
                        }
                    }
                    r = o;
                }
                b = arith_error;
                if o != EXPR_NONE as i16 {
                    break;
                }
                if p.is_texnull() {
                    break 'c_78022;
                }
                /*1577: */
                f = e;
                q = p;
                e = MEM[(q + 1) as usize].b32.s1;
                t = MEM[(q + 2) as usize].b32.s1;
                n = MEM[(q + 3) as usize].b32.s1;
                s = (MEM[q as usize].b16.s0 as i32 / 4) as small_number;
                r = (MEM[q as usize].b16.s0 as i32 % 4) as small_number;
                l = MEM[q as usize].b16.s1 as small_number;
                p = MEM[q as usize].b32.s1;
                free_node(q as usize, EXPR_NODE_SIZE);
            }
        }
        /*1576: */
        q = get_node(EXPR_NODE_SIZE) as i32;
        MEM[q as usize].b32.s1 = p;
        MEM[q as usize].b16.s1 = l as u16;
        MEM[q as usize].b16.s0 = (4 * s as i32 + r as i32) as u16;
        MEM[(q + 1) as usize].b32.s1 = e;
        MEM[(q + 2) as usize].b32.s1 = t;
        MEM[(q + 3) as usize].b32.s1 = n;
        p = q;
        l = o
    }
    if b {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Arithmetic overflow");
        help_ptr = 2;
        help_line[1] = b"I can\'t evaluate this expression,";
        help_line[0] = b"since the result is out of range.";
        error();
        if l >= GLUE_VAL as i16 {
            delete_glue_ref(e as usize);
            e = 0;
            MEM[e as usize].b32.s1 += 1;
        } else {
            e = 0;
        }
    }
    arith_error = a;
    cur_val = e;
    cur_val_level = l as u8;
}
pub(crate) unsafe fn scan_normal_glue() {
    scan_glue(GLUE_VAL as small_number);
}
pub(crate) unsafe fn scan_mu_glue() {
    scan_glue(MU_VAL as small_number);
}
pub(crate) unsafe fn scan_rule_spec() -> usize {
    let q = new_rule();
    if cur_cmd == VRULE as u8 {
        MEM[q + 1].b32.s1 = DEFAULT_RULE
    } else {
        MEM[q + 3].b32.s1 = DEFAULT_RULE;
        MEM[q + 2].b32.s1 = 0
    }
    loop {
        if scan_keyword(b"width") {
            scan_dimen(false, false, false);
            MEM[q + 1].b32.s1 = cur_val
        } else if scan_keyword(b"height") {
            scan_dimen(false, false, false);
            MEM[q + 3].b32.s1 = cur_val
        } else {
            if !scan_keyword(b"depth") {
                break;
            }
            scan_dimen(false, false, false);
            MEM[q + 2].b32.s1 = cur_val
        }
    }
    q
}
pub(crate) unsafe fn scan_general_text() {
    let mut unbalance: i32 = 0;
    let mut s = scanner_status;
    let mut w = warning_index;
    let mut d = def_ref;
    scanner_status = ABSORBING as u8;
    warning_index = cur_cs;
    def_ref = get_avail();
    MEM[def_ref].b32.s0 = TEX_NULL;
    let mut p = def_ref;
    scan_left_brace();
    unbalance = 1;
    loop {
        get_token();
        if cur_tok < RIGHT_BRACE_LIMIT {
            if cur_cmd < RIGHT_BRACE as u8 {
                unbalance += 1
            } else {
                unbalance -= 1;
                if unbalance == 0 {
                    break;
                }
            }
        }
        let q = get_avail();
        MEM[p].b32.s1 = q as i32;
        MEM[q].b32.s0 = cur_tok;
        p = q;
    }
    let q = MEM[def_ref].b32.s1;
    MEM[def_ref].b32.s1 = avail;
    avail = def_ref as i32;
    if q.is_texnull() {
        cur_val = TEMP_HEAD as i32;
    } else {
        cur_val = p as i32;
    }
    MEM[TEMP_HEAD].b32.s1 = q;
    scanner_status = s;
    warning_index = w;
    def_ref = d;
}
pub(crate) unsafe fn pseudo_start() {
    let mut w: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    scan_general_text();
    let old_setting_0 = selector;
    selector = Selector::NEW_STRING;
    token_show(Some(TEMP_HEAD));
    selector = old_setting_0;
    flush_list(MEM[TEMP_HEAD].b32.s1.opt());
    if pool_ptr + 1 > pool_size {
        overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
    }
    let mut s = make_string();
    str_pool[pool_ptr as usize] = ' ' as i32 as packed_UTF16_code;
    let mut l = str_start[(s as i64 - 65536) as usize];
    let mut nl = *INTPAR(IntPar::new_line_char);
    let mut p = get_avail();
    let mut q = p as i32;
    while l < pool_ptr {
        let mut m = l;
        while l < pool_ptr && str_pool[l as usize] as i32 != nl {
            l += 1
        }
        let mut sz = (l - m + 7) / 4;
        if sz == 1 {
            sz = 2
        }
        let mut r = get_node(sz) as i32;
        MEM[q as usize].b32.s1 = r;
        q = r;
        MEM[q as usize].b32.s0 = sz;
        while sz > 2 {
            sz -= 1;
            r += 1;
            w.s3 = str_pool[m as usize];
            w.s2 = str_pool[(m + 1) as usize];
            w.s1 = str_pool[(m + 2) as usize];
            w.s0 = str_pool[(m + 3) as usize];
            MEM[r as usize].b16 = w;
            m = m + 4
        }
        w.s3 = ' ' as i32 as u16;
        w.s2 = ' ' as i32 as u16;
        w.s1 = ' ' as i32 as u16;
        w.s0 = ' ' as i32 as u16;
        if l > m {
            w.s3 = str_pool[m as usize];
            if l > m + 1 {
                w.s2 = str_pool[(m + 1) as usize];
                if l > m + 2 {
                    w.s1 = str_pool[(m + 2) as usize];
                    if l > m + 3 {
                        w.s0 = str_pool[(m + 3) as usize]
                    }
                }
            }
        }
        MEM[(r + 1) as usize].b16 = w;
        if str_pool[l as usize] as i32 == nl {
            l += 1
        }
    }
    MEM[p].b32.s0 = MEM[p].b32.s1;
    MEM[p].b32.s1 = pseudo_files;
    pseudo_files = p as i32;
    str_ptr -= 1;
    pool_ptr = str_start[(str_ptr - 65536) as usize];
    begin_file_reading();
    line = 0;
    cur_input.limit = cur_input.start;
    cur_input.loc = cur_input.limit + 1;
    if *INTPAR(IntPar::tracing_scan_tokens) > 0 {
        if term_offset > max_print_line - 3 {
            print_ln();
        } else if term_offset > 0 || file_offset > 0 {
            print_char(' ' as i32);
        }
        cur_input.name = 19;
        print_cstr(b"( ");
        open_parens += 1;
        rust_stdout.as_mut().unwrap().flush().unwrap();
    } else {
        cur_input.name = 18;
        cur_input.synctex_tag = 0;
    };
}
pub(crate) unsafe fn str_toks_cat(mut b: pool_pointer, mut cat: small_number) -> usize {
    if pool_ptr + 1 > pool_size {
        overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
    }
    let mut p = TEMP_HEAD;
    MEM[p].b32.s1 = TEX_NULL;
    let mut k = b;
    while k < pool_ptr {
        let mut t = str_pool[k as usize] as i32;
        if t == ' ' as i32 && cat == 0 {
            t = SPACE_TOKEN
        } else {
            if t >= 0xd800
                && t < 0xdc00
                && k + 1 < pool_ptr
                && str_pool[(k + 1) as usize] as i32 >= 0xdc00
                && (str_pool[(k + 1) as usize] as i32) < 0xe000
            {
                k += 1;
                t = (65536
                    + ((t - 0xd800) * 1024) as i64
                    + (str_pool[k as usize] as i32 - 0xdc00) as i64) as i32
            }
            if cat as i32 == 0 {
                t = OTHER_TOKEN + t
            } else {
                t = MAX_CHAR_VAL * cat as i32 + t
            }
        }
        let q = if avail.is_texnull() {
            get_avail()
        } else {
            let q = avail as usize;
            avail = MEM[q].b32.s1;
            MEM[q].b32.s1 = TEX_NULL;
            q
        };
        MEM[p].b32.s1 = q as i32;
        MEM[q].b32.s0 = t;
        p = q;
        k += 1;
    }
    pool_ptr = b;
    p
}
pub(crate) unsafe fn str_toks(mut b: pool_pointer) -> usize {
    str_toks_cat(b, 0)
}
pub(crate) unsafe fn the_toks() -> usize {
    if cur_chr & 1 != 0 {
        let c = cur_chr as small_number;
        scan_general_text();
        if c == 1 {
            return cur_val as usize; // TODO: check TEX_NULL
        } else {
            let old_setting_0 = selector;
            selector = Selector::NEW_STRING;
            let b = pool_ptr;
            let p = get_avail();
            MEM[p].b32.s1 = MEM[TEMP_HEAD].b32.s1;
            token_show(Some(p));
            flush_list(Some(p));
            selector = old_setting_0;
            return str_toks(b);
        }
    }
    get_x_token();
    scan_something_internal(TOK_VAL as small_number, false);
    if cur_val_level >= IDENT_VAL {
        /*485: */
        let mut p = TEMP_HEAD;
        MEM[p].b32.s1 = TEX_NULL;
        if cur_val_level == IDENT_VAL {
            let q = get_avail();
            MEM[p].b32.s1 = q as i32;
            MEM[q].b32.s0 = CS_TOKEN_FLAG + cur_val;
            p = q;
        } else if !cur_val.is_texnull() {
            let mut r = MEM[cur_val as usize].b32.s1;
            while !r.is_texnull() {
                let q = if avail.is_texnull() {
                    get_avail()
                } else {
                    let q = avail as usize;
                    avail = MEM[q].b32.s1;
                    MEM[q].b32.s1 = TEX_NULL;
                    q
                };
                MEM[p].b32.s1 = q as i32;
                MEM[q].b32.s0 = MEM[r as usize].b32.s0;
                p = q;
                r = *LLIST_link(r as usize);
            }
        }
        return p;
    } else {
        let old_setting_0 = selector;
        selector = Selector::NEW_STRING;
        let b = pool_ptr;
        match cur_val_level as i32 {
            0 => print_int(cur_val),
            1 => {
                print_scaled(cur_val);
                print_cstr(b"pt");
            }
            2 => {
                print_spec(cur_val, b"pt");
                delete_glue_ref(cur_val as usize);
            }
            3 => {
                print_spec(cur_val, b"mu");
                delete_glue_ref(cur_val as usize);
            }
            _ => {}
        }
        selector = old_setting_0;
        return str_toks(b);
    };
}
pub(crate) unsafe fn ins_the_toks() {
    MEM[GARBAGE as usize].b32.s1 = the_toks() as i32;
    begin_token_list(MEM[TEMP_HEAD].b32.s1 as usize, INSERTED);
}
pub(crate) unsafe fn conv_toks() {
    let mut save_warning_index: i32 = 0;
    let mut boolvar: bool = false;
    let mut s: str_number = 0;
    let mut u: str_number = 0;
    let mut save_scanner_status: small_number = 0;
    let mut b: pool_pointer = 0;
    let mut fnt: usize = 0;
    let mut arg1: i32 = 0i32;
    let mut arg2: i32 = 0i32;
    let mut font_name_str: str_number = 0;
    let mut quote_char: UTF16_code = 0;
    let mut saved_chr: UnicodeScalar = 0;
    let mut p: i32 = TEX_NULL;
    let mut q: i32 = 0;
    let mut cat = 0i32 as small_number;
    let mut c = cur_chr as small_number;
    match c as i32 {
        NUMBER_CODE | ROMAN_NUMERAL_CODE => scan_int(),
        STRING_CODE | MEANING_CODE => {
            save_scanner_status = scanner_status as small_number;
            scanner_status = NORMAL as u8;
            get_token();
            scanner_status = save_scanner_status as u8
        }
        FONT_NAME_CODE => scan_font_ident(),
        XETEX_UCHAR_CODE => scan_usv_num(),
        XETEX_UCHARCAT_CODE => {
            scan_usv_num();
            saved_chr = cur_val;
            scan_int();
            if cur_val < LEFT_BRACE as i32
                || cur_val > OTHER_CHAR as i32
                || cur_val == OUT_PARAM as i32
                || cur_val == IGNORE as i32
            {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Invalid code (");
                print_int(cur_val);
                print_cstr(b"), should be in the ranges 1..4, 6..8, 10..12");
                help_ptr = 1_u8;
                help_line[0] = b"I\'m going to use 12 instead of that illegal code value.";
                error();
                cat = 12;
            } else {
                cat = cur_val as small_number
            }
            cur_val = saved_chr
        }
        PDF_STRCMP_CODE => {
            save_scanner_status = scanner_status as small_number;
            save_warning_index = warning_index;
            let save_def_ref = def_ref;
            if str_start[(str_ptr - TOO_BIG_CHAR) as usize] < pool_ptr {
                u = make_string()
            } else {
                u = 0;
            }
            compare_strings();
            def_ref = save_def_ref;
            warning_index = save_warning_index;
            scanner_status = save_scanner_status as u8;
            if u != 0 {
                str_ptr -= 1;
            }
        }
        PDF_MDFIVE_SUM_CODE => {
            save_scanner_status = scanner_status as small_number;
            save_warning_index = warning_index;
            let save_def_ref = def_ref;
            if str_start[(str_ptr - TOO_BIG_CHAR) as usize] < pool_ptr {
                u = make_string()
            } else {
                u = 0;
            }
            boolvar = scan_keyword(b"file");
            scan_pdf_ext_toks();
            if selector == Selector::NEW_STRING {
                pdf_error(
                    b"tokens",
                    b"tokens_to_string() called while selector = new_string",
                );
            }
            let old_setting_0 = selector;
            selector = Selector::NEW_STRING;
            show_token_list(MEM[def_ref].b32.s1, TEX_NULL, pool_size - pool_ptr);
            selector = old_setting_0;
            s = make_string();
            delete_token_ref(def_ref);
            def_ref = save_def_ref;
            warning_index = save_warning_index;
            scanner_status = save_scanner_status as u8;
            b = pool_ptr;
            getmd5sum(s, boolvar);
            MEM[GARBAGE as usize].b32.s1 = str_toks(b) as i32;
            if s == str_ptr - 1 {
                str_ptr -= 1;
                pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize]
            }
            begin_token_list(MEM[TEMP_HEAD].b32.s1 as usize, INSERTED);
            if u != 0 {
                str_ptr -= 1;
            }
            return;
        }
        XETEX_VARIATION_NAME_CODE => {
            scan_font_ident();
            fnt = cur_val as usize;
            if FONT_AREA[fnt as usize] as u32 == AAT_FONT_FLAG {
                scan_int();
                arg1 = cur_val;
                arg2 = 0;
            } else {
                not_aat_font_error(CONVERT, c as i32, fnt);
            }
        }
        XETEX_FEATURE_NAME_CODE => {
            scan_font_ident();
            fnt = cur_val as usize;
            if FONT_AREA[fnt as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[fnt as usize] as u32 == OTGR_FONT_FLAG
                    && usingGraphite(FONT_LAYOUT_ENGINE[fnt as usize] as XeTeXLayoutEngine) as i32
                        != 0
            {
                scan_int();
                arg1 = cur_val;
                arg2 = 0;
            } else {
                not_aat_gr_font_error(CONVERT, c as i32, fnt);
            }
        }
        XETEX_SELECTOR_NAME_CODE => {
            scan_font_ident();
            fnt = cur_val as usize;
            if FONT_AREA[fnt as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[fnt as usize] as u32 == OTGR_FONT_FLAG
                    && usingGraphite(FONT_LAYOUT_ENGINE[fnt as usize] as XeTeXLayoutEngine) as i32
                        != 0
            {
                scan_int();
                arg1 = cur_val;
                scan_int();
                arg2 = cur_val
            } else {
                not_aat_gr_font_error(CONVERT, c as i32, fnt);
            }
        }
        XETEX_GLYPH_NAME_CODE => {
            scan_font_ident();
            fnt = cur_val as usize;
            if FONT_AREA[fnt as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[fnt as usize] as u32 == OTGR_FONT_FLAG
            {
                scan_int();
                arg1 = cur_val
            } else {
                not_native_font_error(CONVERT, c as i32, fnt);
            }
        }
        LEFT_MARGIN_KERN_CODE | RIGHT_MARGIN_KERN_CODE => {
            scan_register_num();
            if cur_val < 256 {
                p = *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    p = TEX_NULL
                } else {
                    p = MEM[(cur_ptr + 1) as usize].b32.s1
                }
            }
            if p.is_texnull() || *NODE_type(p as usize) != HLIST_NODE {
                pdf_error(b"marginkern", b"a non-empty hbox expected");
            }
        }
        JOB_NAME_CODE => {
            if job_name == 0 {
                open_log_file();
            }
        }
        ETEX_REVISION_CODE | XETEX_REVISION_CODE | _ => {}
    }
    let old_setting_0 = selector;
    selector = Selector::NEW_STRING;
    b = pool_ptr;
    match c as i32 {
        NUMBER_CODE => print_int(cur_val),
        ROMAN_NUMERAL_CODE => print_roman_int(cur_val),
        STRING_CODE => {
            if cur_cs != 0 {
                sprint_cs(cur_cs);
            } else {
                print_char(cur_chr);
            }
        }
        MEANING_CODE => print_meaning(),
        FONT_NAME_CODE => {
            font_name_str = FONT_NAME[cur_val as usize];
            match FONT_AREA[cur_val as usize] as u32 {
                AAT_FONT_FLAG | OTGR_FONT_FLAG => {
                    quote_char = '\"' as i32 as UTF16_code;
                    for i in 0..=(length(font_name_str) - 1) {
                        if str_pool[(str_start[(font_name_str as i64 - 65536) as usize] + i as i32)
                            as usize] as i32
                            == '\"' as i32
                        {
                            quote_char = '\'' as i32 as UTF16_code
                        }
                    }
                    print_char(quote_char as i32);
                    print(font_name_str);
                    print_char(quote_char as i32);
                }
                _ => print(font_name_str),
            }
            if FONT_SIZE[cur_val as usize] != FONT_DSIZE[cur_val as usize] {
                print_cstr(b" at ");
                print_scaled(FONT_SIZE[cur_val as usize]);
                print_cstr(b"pt");
            }
        }
        XETEX_UCHAR_CODE | XETEX_UCHARCAT_CODE => print_char(cur_val),
        ETEX_REVISION_CODE => print_cstr(b".6"),
        PDF_STRCMP_CODE => print_int(cur_val),
        XETEX_REVISION_CODE => print_cstr(b".99998"),
        XETEX_VARIATION_NAME_CODE => {
            match FONT_AREA[fnt as usize] as u32 {
                #[cfg(target_os = "macos")]
                AAT_FONT_FLAG => {
                    aat::aat_print_font_name(
                        c as i32,
                        (FONT_LAYOUT_ENGINE[fnt as usize]) as _,
                        arg1,
                        arg2,
                    );
                }
                #[cfg(not(target_os = "macos"))]
                AAT_FONT_FLAG => {
                    // do nothing
                }
                _ => {
                    // do nothing
                }
            }
        }
        XETEX_FEATURE_NAME_CODE | XETEX_SELECTOR_NAME_CODE => {
            match FONT_AREA[fnt as usize] as u32 {
                #[cfg(target_os = "macos")]
                AAT_FONT_FLAG => {
                    aat::aat_print_font_name(
                        c as i32,
                        (FONT_LAYOUT_ENGINE[fnt as usize]) as _,
                        arg1,
                        arg2,
                    );
                }
                #[cfg(not(target_os = "macos"))]
                AAT_FONT_FLAG => {
                    // do nothing
                }
                OTGR_FONT_FLAG => {
                    if usingGraphite(FONT_LAYOUT_ENGINE[fnt as usize] as XeTeXLayoutEngine) as i32
                        != 0
                    {
                        gr_print_font_name(c as i32, FONT_LAYOUT_ENGINE[fnt as usize], arg1, arg2);
                    }
                }
                _ => {}
            }
        }
        XETEX_GLYPH_NAME_CODE => match FONT_AREA[fnt as usize] as u32 {
            AAT_FONT_FLAG | OTGR_FONT_FLAG => print_glyph_name(fnt, arg1),
            _ => {}
        },
        LEFT_MARGIN_KERN_CODE => {
            p = MEM[(p + 5) as usize].b32.s1;
            while !p.is_texnull()
                && (p < hi_mem_min
                    && (*NODE_type(p as usize) == INS_NODE
                        || *NODE_type(p as usize) == MARK_NODE
                        || *NODE_type(p as usize) == ADJUST_NODE
                        || *NODE_type(p as usize) == PENALTY_NODE
                        || *NODE_type(p as usize) == DISC_NODE
                            && MEM[(p + 1) as usize].b32.s0.is_texnull()
                            && MEM[(p + 1) as usize].b32.s1.is_texnull()
                            && MEM[p as usize].b16.s0 == 0
                        || *NODE_type(p as usize) == MATH_NODE
                            && MEM[(p + 1) as usize].b32.s1 == 0
                        || *NODE_type(p as usize) == KERN_NODE
                            && (MEM[(p + 1) as usize].b32.s1 == 0
                                || MEM[p as usize].b16.s0 == NORMAL)
                        || *NODE_type(p as usize) == GLUE_NODE
                            && MEM[(p + 1) as usize].b32.s0 == 0
                        || *NODE_type(p as usize) == HLIST_NODE
                            && MEM[(p + 1) as usize].b32.s1 == 0
                            && MEM[(p + 3) as usize].b32.s1 == 0
                            && MEM[(p + 2) as usize].b32.s1 == 0
                            && MEM[(p + 5) as usize].b32.s1.is_texnull())
                    || p < hi_mem_min
                        && *NODE_type(p as usize) == GLUE_NODE
                        && MEM[p as usize].b16.s0 == (GluePar::left_skip as u16) + 1)
            {
                p = *LLIST_link(p as usize)
            }
            if !p.is_texnull()
                && p < hi_mem_min
                && *NODE_type(p as usize) == MARGIN_KERN_NODE
                && MEM[p as usize].b16.s0 as i32 == 0
            {
                print_scaled(MEM[(p + 1) as usize].b32.s1);
            } else {
                print('0' as i32);
            }
            print_cstr(b"pt");
        }
        RIGHT_MARGIN_KERN_CODE => {
            q = MEM[(p + 5) as usize].b32.s1;
            p = prev_rightmost(q, TEX_NULL);
            while !p.is_texnull()
                && (p < hi_mem_min
                    && (*NODE_type(p as usize) == INS_NODE
                        || *NODE_type(p as usize) == MARK_NODE
                        || *NODE_type(p as usize) == ADJUST_NODE
                        || *NODE_type(p as usize) == PENALTY_NODE
                        || *NODE_type(p as usize) == DISC_NODE
                            && MEM[(p + 1) as usize].b32.s0.is_texnull()
                            && MEM[(p + 1) as usize].b32.s1.is_texnull()
                            && MEM[p as usize].b16.s0 as i32 == 0
                        || *NODE_type(p as usize) == MATH_NODE
                            && MEM[(p + 1) as usize].b32.s1 == 0
                        || *NODE_type(p as usize) == KERN_NODE
                            && (MEM[(p + 1) as usize].b32.s1 == 0
                                || MEM[p as usize].b16.s0 == NORMAL)
                        || *NODE_type(p as usize) == GLUE_NODE
                            && MEM[(p + 1) as usize].b32.s0 == 0
                        || *NODE_type(p as usize) == HLIST_NODE
                            && MEM[(p + 1) as usize].b32.s1 == 0
                            && MEM[(p + 3) as usize].b32.s1 == 0
                            && MEM[(p + 2) as usize].b32.s1 == 0
                            && MEM[(p + 5) as usize].b32.s1.is_texnull())
                    || p < hi_mem_min
                        && *NODE_type(p as usize) == GLUE_NODE
                        && MEM[p as usize].b16.s0 == (GluePar::right_skip as u16) + 1)
            {
                p = prev_rightmost(q, p)
            }
            if !p.is_texnull()
                && p < hi_mem_min
                && *NODE_type(p as usize) == MARGIN_KERN_NODE
                && MEM[p as usize].b16.s0 == 1
            {
                print_scaled(MEM[(p + 1) as usize].b32.s1);
            } else {
                print('0' as i32);
            }
            print_cstr(b"pt");
        }
        JOB_NAME_CODE => print_file_name(job_name, 0, 0),
        _ => {}
    }
    selector = old_setting_0;
    MEM[GARBAGE].b32.s1 = str_toks_cat(b, cat) as i32;
    begin_token_list(MEM[TEMP_HEAD].b32.s1 as usize, INSERTED);
}
pub(crate) unsafe fn scan_toks(mut macro_def: bool, mut xpand: bool) -> usize {
    unsafe fn found(p: usize, hash_brace: i32) -> usize {
        scanner_status = NORMAL as u8;
        if hash_brace != 0 {
            let q = get_avail();
            MEM[p].b32.s1 = q as i32;
            MEM[q].b32.s0 = hash_brace;
            q
        } else {
            p
        }
    }

    let mut current_block: u64;
    let mut s: i32 = 0;
    let mut unbalance: i32 = 0;
    scanner_status = if macro_def {
        DEFINING as u8
    } else {
        ABSORBING as u8
    };
    warning_index = cur_cs;
    def_ref = get_avail();
    MEM[def_ref].b32.s0 = TEX_NULL;
    let mut p = def_ref;
    let mut hash_brace = 0;
    let mut t = ZERO_TOKEN;
    if macro_def {
        loop
        /*493: */
        {
            get_token();
            if cur_tok < RIGHT_BRACE_LIMIT {
                current_block = 7086859973843054082;
                break;
            }
            if cur_cmd == MAC_PARAM as u8 {
                /*495: */
                s = MATCH_TOKEN + cur_chr;
                get_token();
                if cur_cmd == LEFT_BRACE as u8 {
                    hash_brace = cur_tok;
                    let q = get_avail();
                    MEM[p].b32.s1 = q as i32;
                    MEM[q].b32.s0 = cur_tok;
                    p = q;
                    let q = get_avail();
                    MEM[p].b32.s1 = q as i32;
                    MEM[q].b32.s0 = END_MATCH_TOKEN;
                    p = q;
                    current_block = 2723324002591448311;
                    break;
                } else if t == ZERO_TOKEN + 9 {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"You already have nine parameters");
                    help_ptr = 1;
                    help_line[0] = b"I\'m going to ignore the # sign you just used.";
                    error();
                } else {
                    t += 1;
                    if cur_tok != t {
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! ");
                        }
                        print_cstr(b"Parameters must be numbered consecutively");
                        help_ptr = 2;
                        help_line[1] =
                            b"I\'ve inserted the digit you should have used after the #.";
                        help_line[0] = b"Type `1\' to delete what you did use.";
                        back_error();
                    }
                    cur_tok = s
                }
            }
            let q = get_avail();
            MEM[p].b32.s1 = q as i32;
            MEM[q].b32.s0 = cur_tok;
            p = q;
        }
        match current_block {
            2723324002591448311 => {}
            _ => {
                let q = get_avail();
                MEM[p].b32.s1 = q as i32;
                MEM[q].b32.s0 = END_MATCH_TOKEN;
                p = q;
                if cur_cmd == RIGHT_BRACE as u8 {
                    /*494: */
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Missing { inserted");
                    align_state += 1;
                    help_ptr = 2;
                    help_line[1] =
                        b"Where was the left brace? You said something like `\\def\\a}\',";
                    help_line[0] = b"which I\'m going to interpret as `\\def\\a{}\'.";
                    error();
                    return found(p, hash_brace);
                } else {
                    current_block = 2723324002591448311;
                }
            }
        }
    } else {
        scan_left_brace();
        current_block = 2723324002591448311;
    }
    match current_block {
        2723324002591448311 => {
            unbalance = 1;
            loop {
                if xpand {
                    loop
                    /*497: */
                    {
                        get_next();
                        if cur_cmd >= CALL as u8 {
                            if MEM[MEM[cur_chr as usize].b32.s1 as usize].b32.s0 == PROTECTED_TOKEN
                            {
                                cur_cmd = RELAX as u8;
                                cur_chr = NO_EXPAND_FLAG;
                            }
                        }
                        if cur_cmd <= MAX_COMMAND {
                            break;
                        }
                        if cur_cmd != THE as u8 {
                            expand();
                        } else {
                            let q = the_toks();
                            if !MEM[TEMP_HEAD].b32.s1.is_texnull() {
                                MEM[p].b32.s1 = MEM[TEMP_HEAD].b32.s1;
                                p = q
                            }
                        }
                    }
                    x_token();
                } else {
                    get_token();
                }
                if cur_tok < RIGHT_BRACE_LIMIT {
                    if cur_cmd < RIGHT_BRACE as u8 {
                        unbalance += 1;
                    } else {
                        unbalance -= 1;
                        if unbalance == 0 {
                            return found(p, hash_brace);
                        }
                    }
                } else if cur_cmd == MAC_PARAM as u8 {
                    if macro_def {
                        /*498: */
                        s = cur_tok;
                        if xpand {
                            get_x_token();
                        } else {
                            get_token();
                        }
                        if cur_cmd != MAC_PARAM as u8 {
                            if cur_tok <= ZERO_TOKEN || cur_tok > t {
                                if file_line_error_style_p != 0 {
                                    print_file_line();
                                } else {
                                    print_nl_cstr(b"! ");
                                }
                                print_cstr(b"Illegal parameter number in definition of ");
                                sprint_cs(warning_index);
                                help_ptr = 3;
                                help_line[2] = b"You meant to type ## instead of #, right?";
                                help_line[1] =
                                    b"Or maybe a } was forgotten somewhere earlier, and things";
                                help_line[0] =
                                    b"are all screwed up? I\'m going to assume that you meant ##.";
                                back_error();
                                cur_tok = s
                            } else {
                                cur_tok = OUT_PARAM_TOKEN - 48 + cur_chr
                            }
                        }
                    }
                }
                let q = get_avail();
                MEM[p].b32.s1 = q as i32;
                MEM[q].b32.s0 = cur_tok;
                p = q;
            }
        }
        _ => {}
    }
    found(p, hash_brace)
}
pub(crate) unsafe fn read_toks(mut n: i32, mut r: i32, mut j: i32) {
    let mut s: i32 = 0;
    let mut m: small_number = 0;
    scanner_status = DEFINING as u8;
    warning_index = r;
    def_ref = get_avail();
    MEM[def_ref].b32.s0 = TEX_NULL;
    let p = def_ref;
    let q = get_avail();
    MEM[p].b32.s1 = q as i32;
    MEM[q].b32.s0 = END_MATCH_TOKEN;
    let mut p = q;
    if n < 0 || n > 15 {
        m = 16;
    } else {
        m = n as small_number
    }
    s = align_state;
    align_state = 1000000;
    loop {
        /*502:*/
        begin_file_reading();
        cur_input.name = m as i32 + 1;
        assert!(
            read_open[m as usize] as i32 != 2,
            /*503:*/
            "terminal input forbidden"
        );
        /*505:*/
        if read_open[m as usize] == CLOSED {
            /*504:*/
            if input_line(read_file[m as usize]) != JUST_OPEN {
                read_open[m as usize] = NORMAL as u8;
            } else {
                u_close(read_file[m as usize]);
                read_open[m as usize] = CLOSED;
            }
        } else if input_line(read_file[m as usize]) == 0 {
            u_close(read_file[m as usize]);
            read_open[m as usize] = CLOSED;
            if align_state as i64 != 1000000 {
                runaway();
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"File ended within ");
                print_esc_cstr(b"read");
                help_ptr = 1_u8;
                help_line[0] = b"This \\read has unbalanced braces.";
                align_state = 1000000;
                error();
            }
        }
        cur_input.limit = last;
        if *INTPAR(IntPar::end_line_char) < 0 || *INTPAR(IntPar::end_line_char) > 255 {
            cur_input.limit -= 1
        } else {
            BUFFER[cur_input.limit as usize] = *INTPAR(IntPar::end_line_char)
        }
        first = cur_input.limit + 1;
        cur_input.loc = cur_input.start;
        cur_input.state = NEW_LINE;
        if j == 1 {
            while cur_input.loc <= cur_input.limit {
                cur_chr = BUFFER[cur_input.loc as usize];
                cur_input.loc += 1;
                if cur_chr == ' ' as i32 {
                    cur_tok = SPACE_TOKEN
                } else {
                    cur_tok = cur_chr + OTHER_TOKEN
                }
                let q = get_avail();
                MEM[p].b32.s1 = q as i32;
                MEM[q].b32.s0 = cur_tok;
                p = q;
            }
        } else {
            loop {
                get_token();
                if cur_tok == 0 {
                    break;
                }
                if (align_state as i64) < 1000000 {
                    loop {
                        get_token();
                        if !(cur_tok != 0) {
                            break;
                        }
                    }
                    align_state = 1000000;
                    break;
                } else {
                    let q = get_avail();
                    MEM[p].b32.s1 = q as i32;
                    MEM[q].b32.s0 = cur_tok;
                    p = q;
                }
            }
        }
        end_file_reading();
        if !(align_state as i64 != 1000000) {
            break;
        }
    }
    cur_val = def_ref as i32;
    scanner_status = NORMAL as u8;
    align_state = s;
}
pub(crate) unsafe fn pass_text() {
    let mut l: i32 = 0;
    let mut save_scanner_status: small_number = 0;
    save_scanner_status = scanner_status as small_number;
    scanner_status = SKIPPING as u8;
    l = 0;
    skip_line = line;
    loop {
        get_next();
        if cur_cmd == FI_OR_ELSE as u8 {
            if l == 0 {
                break;
            }
            if cur_chr == FI_CODE as i32 {
                l -= 1;
            }
        } else if cur_cmd == IF_TEST as u8 {
            l += 1;
        }
    }
    scanner_status = save_scanner_status as u8;
    if *INTPAR(IntPar::tracing_ifs) > 0 {
        show_cur_cmd_chr();
    };
}
pub(crate) unsafe fn change_if_limit(l: u8, mut p: i32) {
    let mut q: i32 = 0;
    if p == cond_ptr {
        if_limit = l;
    } else {
        q = cond_ptr;
        loop {
            if q.is_texnull() {
                confusion(b"if");
            }
            if MEM[q as usize].b32.s1 == p {
                MEM[q as usize].b16.s1 = l as u16;
                return;
            }
            q = *LLIST_link(q as usize);
        }
    };
}
pub(crate) unsafe fn conditional() {
    let mut current_block: u64;
    let mut b: bool = false;
    let mut e: bool = false;
    let mut r: u8 = 0;
    let mut q: i32 = 0;
    let mut save_scanner_status: small_number = 0;
    if *INTPAR(IntPar::tracing_ifs) > 0 {
        if *INTPAR(IntPar::tracing_commands) <= 1 {
            show_cur_cmd_chr();
        }
    }
    let mut p = get_node(IF_NODE_SIZE) as i32;
    MEM[p as usize].b32.s1 = cond_ptr;
    MEM[p as usize].b16.s1 = if_limit as u16;
    MEM[p as usize].b16.s0 = cur_if as u16;
    MEM[(p + 1) as usize].b32.s1 = if_line;
    cond_ptr = p;
    cur_if = cur_chr as small_number;
    if_limit = IF_CODE;
    if_line = line;
    let mut save_cond_ptr = cond_ptr;
    let mut is_unless = cur_chr >= UNLESS_CODE;
    let mut this_if = (cur_chr % UNLESS_CODE) as small_number;
    match this_if {
        IF_CHAR_CODE | IF_CAT_CODE => {
            get_x_token();
            if cur_cmd == RELAX as u8 {
                if cur_chr == NO_EXPAND_FLAG {
                    cur_cmd = ACTIVE_CHAR as u8;
                    cur_chr = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE as i32)
                }
            }
            let (m, n) = if cur_cmd > ACTIVE_CHAR as u8 || cur_chr > BIGGEST_USV as i32 {
                (RELAX as i32, TOO_BIG_USV)
            } else {
                (cur_cmd as i32, cur_chr)
            };
            get_x_token();
            if cur_cmd == RELAX as u8 {
                if cur_chr == NO_EXPAND_FLAG {
                    cur_cmd = ACTIVE_CHAR as u8;
                    cur_chr = cur_tok - (CS_TOKEN_FLAG + ACTIVE_BASE as i32)
                }
            }
            if cur_cmd > ACTIVE_CHAR as u8 || cur_chr > BIGGEST_USV as i32 {
                cur_cmd = RELAX as u8;
                cur_chr = TOO_BIG_USV;
            }
            if this_if as i32 == 0i32 {
                b = n == cur_chr
            } else {
                b = m == cur_cmd as i32
            }
            current_block = 16915215315900843183;
        }
        IF_INT_CODE | IF_DIM_CODE => {
            if this_if as i32 == 2i32 {
                scan_int();
            } else {
                scan_dimen(false, false, false);
            }
            let n = cur_val;
            loop {
                get_x_token();
                if !(cur_cmd == SPACER as u8) {
                    break;
                }
            }
            if cur_tok >= OTHER_TOKEN + 60 && cur_tok <= OTHER_TOKEN + 62 {
                r = (cur_tok - OTHER_TOKEN) as u8
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Missing = inserted for ");
                print_cmd_chr(IF_TEST, this_if as i32);
                help_ptr = 1;
                help_line[0] = b"I was expecting to see `<\', `=\', or `>\'. Didn\'t.";
                back_error();
                r = b'=';
            }
            if this_if == IF_INT_CODE as i16 {
                scan_int();
            } else {
                scan_dimen(false, false, false);
            }
            match r {
                60 => {
                    /*"<"*/
                    b = n < cur_val
                }
                61 => {
                    /*"="*/
                    b = n == cur_val
                }
                62 => {
                    /*">"*/
                    b = n > cur_val
                }
                _ => {}
            } /*527:*/
            current_block = 16915215315900843183; /* !shellenabledp */
        }
        IF_ODD_CODE => {
            scan_int();
            b = cur_val & 1i32 != 0;
            current_block = 16915215315900843183;
        }
        IF_VMODE_CODE => {
            b = cur_list.mode.abs() == VMODE;
            current_block = 16915215315900843183;
        }
        IF_HMODE_CODE => {
            b = cur_list.mode.abs() == HMODE;
            current_block = 16915215315900843183;
        }
        IF_MMODE_CODE => {
            b = cur_list.mode.abs() == MMODE;
            current_block = 16915215315900843183;
        }
        IF_INNER_CODE => {
            b = cur_list.mode < 0;
            current_block = 16915215315900843183;
        }
        IF_VOID_CODE | IF_HBOX_CODE | IF_VBOX_CODE => {
            scan_register_num();
            if cur_val < 256 {
                p = *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    p = TEX_NULL
                } else {
                    p = MEM[(cur_ptr + 1) as usize].b32.s1
                }
            }
            if this_if == IF_VOID_CODE {
                b = p.is_texnull()
            } else if p.is_texnull() {
                b = false
            } else if this_if == IF_HBOX_CODE {
                b = *NODE_type(p as usize) == HLIST_NODE;
            } else {
                b = *NODE_type(p as usize) == VLIST_NODE;
            }
            current_block = 16915215315900843183;
        }
        IFX_CODE => {
            save_scanner_status = scanner_status as small_number;
            scanner_status = NORMAL as u8;
            get_next();
            let n = cur_cs;
            p = cur_cmd as i32;
            q = cur_chr;
            get_next();
            if cur_cmd as i32 != p {
                b = false
            } else if cur_cmd < CALL as u8 {
                b = cur_chr == q
            } else {
                p = MEM[cur_chr as usize].b32.s1;
                q = MEM[EQTB[n as usize].b32.s1 as usize].b32.s1;
                if p == q {
                    b = true
                } else {
                    while !p.is_texnull() && !q.is_texnull() {
                        if MEM[p as usize].b32.s0 != MEM[q as usize].b32.s0 {
                            p = TEX_NULL
                        } else {
                            p = *LLIST_link(p as usize);
                            q = *LLIST_link(q as usize);
                        }
                    }
                    b = p.is_texnull() && q.is_texnull()
                }
            }
            scanner_status = save_scanner_status as u8;
            current_block = 16915215315900843183;
        }
        IF_EOF_CODE => {
            scan_four_bit_int_or_18();
            b = if cur_val == 18 {
                true
            } else {
                read_open[cur_val as usize] == CLOSED
            };
            current_block = 16915215315900843183;
        }
        IF_TRUE_CODE => {
            b = true;
            current_block = 16915215315900843183;
        }
        IF_FALSE_CODE => {
            b = false;
            current_block = 16915215315900843183;
        }
        IF_DEF_CODE => {
            save_scanner_status = scanner_status as small_number;
            scanner_status = NORMAL as u8;
            get_next();
            b = cur_cmd != UNDEFINED_CS as u8;
            scanner_status = save_scanner_status as u8;
            current_block = 16915215315900843183;
        }
        IF_CS_CODE => {
            let n = get_avail();
            p = n as i32;
            e = is_in_csname;
            is_in_csname = true;
            loop {
                get_x_token();
                if cur_cs == 0 {
                    q = get_avail() as i32;
                    MEM[p as usize].b32.s1 = q;
                    MEM[q as usize].b32.s0 = cur_tok;
                    p = q
                }
                if !(cur_cs == 0) {
                    break;
                }
            }
            if cur_cmd != END_CS_NAME as u8 {
                /*391:*/
                if file_line_error_style_p != 0 {
                    print_file_line(); /*:1556*/
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Missing ");
                print_esc_cstr(b"endcsname");
                print_cstr(b" inserted");
                help_ptr = 2;
                help_line[1] = b"The control sequence marked <to be read again> should";
                help_line[0] = b"not appear between \\csname and \\endcsname.";
                back_error();
            }
            let mut m = first;
            p = MEM[n].b32.s1;
            while !p.is_texnull() {
                if m >= max_buf_stack {
                    max_buf_stack = m + 1i32;
                    if max_buf_stack as usize == BUF_SIZE {
                        overflow(b"buffer size", BUF_SIZE);
                    }
                }
                BUFFER[m as usize] = MEM[p as usize].b32.s0 % MAX_CHAR_VAL;
                m += 1;
                p = *LLIST_link(p as usize)
            }
            cur_cs = if m == first {
                NULL_CS as i32
            } else if m == first + 1i32 {
                SINGLE_BASE as i32 + BUFFER[first as usize]
            } else {
                id_lookup(first, m - first)
            };

            flush_list(Some(n));
            b = EQTB[cur_cs as usize].b16.s1 != UNDEFINED_CS;
            is_in_csname = e;
            current_block = 16915215315900843183;
        }
        IF_IN_CSNAME_CODE => {
            b = is_in_csname;
            current_block = 16915215315900843183;
        }
        IF_FONT_CHAR_CODE => {
            scan_font_ident();
            let n = cur_val as usize;
            scan_usv_num();
            b = if FONT_AREA[n] as u32 == AAT_FONT_FLAG || FONT_AREA[n] as u32 == OTGR_FONT_FLAG {
                map_char_to_glyph(n, cur_val) > 0
            } else if FONT_BC[n] as i32 <= cur_val && FONT_EC[n] as i32 >= cur_val {
                FONT_CHARACTER_INFO(n, effective_char(true, n, cur_val as u16) as usize).s3 > 0
            } else {
                false
            };
            current_block = 16915215315900843183;
        }
        IF_CASE_CODE => {
            scan_int();
            let mut n = cur_val;
            if *INTPAR(IntPar::tracing_commands) > 1 {
                begin_diagnostic();
                print_cstr(b"{case ");
                print_int(n);
                print_char('}' as i32);
                end_diagnostic(false);
            }
            loop {
                if !(n != 0) {
                    current_block = 8672804474533504599;
                    break;
                }
                pass_text();
                if cond_ptr == save_cond_ptr {
                    if !(cur_chr == OR_CODE as i32) {
                        current_block = 17018179191097466409;
                        break;
                    }
                    n -= 1
                } else if cur_chr == FI_CODE as i32 {
                    /*515:*/
                    if IF_STACK[IN_OPEN] == cond_ptr {
                        if_warning();
                    }
                    p = cond_ptr;
                    if_line = MEM[(p + 1) as usize].b32.s1;
                    cur_if = MEM[p as usize].b16.s0 as small_number;
                    if_limit = MEM[p as usize].b16.s1 as u8;
                    cond_ptr = MEM[p as usize].b32.s1;
                    free_node(p as usize, IF_NODE_SIZE);
                }
            }
            match current_block {
                17018179191097466409 => {}
                _ => {
                    change_if_limit(OR_CODE, save_cond_ptr);
                    return;
                }
            }
        }
        IF_PRIMITIVE_CODE => {
            save_scanner_status = scanner_status as small_number;
            scanner_status = NORMAL as u8;
            get_next();
            scanner_status = save_scanner_status as u8;
            let m = if cur_cs < HASH_BASE as i32 {
                prim_lookup(cur_cs - SINGLE_BASE as i32)
            } else {
                prim_lookup((*hash.offset(cur_cs as isize)).s1)
            };
            b = cur_cmd != UNDEFINED_CS as u8
                && m != UNDEFINED_PRIMITIVE
                && cur_cmd as i32 == prim_eqtb[m as usize].b16.s1 as i32
                && cur_chr == prim_eqtb[m as usize].b32.s1;
            current_block = 16915215315900843183;
        }
        _ => current_block = 16915215315900843183,
    }
    match current_block {
        16915215315900843183 => {
            if is_unless {
                b = !b
            }
            if *INTPAR(IntPar::tracing_commands) > 1 {
                /*521:*/
                begin_diagnostic();
                if b {
                    print_cstr(b"{true}");
                } else {
                    print_cstr(b"{false}");
                }
                end_diagnostic(false);
            }
            if b {
                change_if_limit(ELSE_CODE, save_cond_ptr);
                return;
            }
            loop {
                pass_text();
                if cond_ptr == save_cond_ptr {
                    if cur_chr != OR_CODE as i32 {
                        break;
                    }
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Extra ");
                    print_esc_cstr(b"or");
                    help_ptr = 1;
                    help_line[0] = b"I\'m ignoring this; it doesn\'t match any \\if.";
                    error();
                } else if cur_chr == FI_CODE as i32 {
                    /*515:*/
                    if IF_STACK[IN_OPEN] == cond_ptr {
                        if_warning();
                    }
                    p = cond_ptr;
                    if_line = MEM[(p + 1) as usize].b32.s1;
                    cur_if = MEM[p as usize].b16.s0 as small_number;
                    if_limit = MEM[p as usize].b16.s1 as u8;
                    cond_ptr = MEM[p as usize].b32.s1;
                    free_node(p as usize, IF_NODE_SIZE);
                }
            }
        }
        _ => {}
    }
    if cur_chr == FI_CODE as i32 {
        /*515:*/
        if IF_STACK[IN_OPEN] == cond_ptr {
            if_warning();
        }
        p = cond_ptr;
        if_line = MEM[(p + 1) as usize].b32.s1;
        cur_if = MEM[p as usize].b16.s0 as small_number;
        if_limit = MEM[p as usize].b16.s1 as u8;
        cond_ptr = MEM[p as usize].b32.s1;
        free_node(p as usize, IF_NODE_SIZE);
    } else {
        if_limit = FI_CODE;
    };
}
pub(crate) unsafe fn begin_name() {
    area_delimiter = 0;
    ext_delimiter = 0;
    quoted_filename = false;
    file_name_quote_char = 0 as UTF16_code;
}
pub(crate) unsafe fn more_name(mut c: UTF16_code) -> bool {
    if stop_at_space && file_name_quote_char == 0 && c as i32 == ' ' as i32 {
        return false;
    }
    if stop_at_space && file_name_quote_char != 0 && c as i32 == file_name_quote_char as i32 {
        file_name_quote_char = 0 as UTF16_code;
        return true;
    }
    if stop_at_space
        && file_name_quote_char == 0
        && (c as i32 == '\"' as i32 || c as i32 == '\'' as i32)
    {
        file_name_quote_char = c;
        quoted_filename = true;
        return true;
    }
    if pool_ptr + 1 > pool_size {
        overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
    }
    let fresh37 = pool_ptr;
    pool_ptr = pool_ptr + 1;
    str_pool[fresh37 as usize] = c;
    if c as i32 == '/' as i32 {
        // IS_DIR_SEP
        area_delimiter = cur_length();
        ext_delimiter = 0;
    } else if c as i32 == '.' as i32 {
        ext_delimiter = cur_length()
    }
    true
}
pub(crate) unsafe fn end_name() {
    let mut temp_str: str_number = 0;
    let mut j: pool_pointer = 0;
    if str_ptr + 3 > max_strings as i32 {
        overflow(b"number of strings", max_strings - init_str_ptr as usize);
    }
    /* area_delimiter is the length from the start of the filename to the
     * directory seperator "/", which we use to construct the stringpool
     * string `cur_area`. If there was already a string in the stringpool for
     * the area, reuse it. */
    if area_delimiter == 0 {
        cur_area = EMPTY_STRING as str_number
    } else {
        cur_area = str_ptr;
        str_start[((str_ptr + 1) as i64 - 65536) as usize] =
            str_start[(str_ptr - 65536) as usize] + area_delimiter;
        str_ptr += 1;
        temp_str = search_string(cur_area);
        if temp_str > 0 {
            cur_area = temp_str;
            str_ptr -= 1;
            j = str_start[((str_ptr + 1) as i64 - 65536) as usize];
            while j <= pool_ptr - 1 {
                str_pool[(j - area_delimiter) as usize] = str_pool[j as usize];
                j += 1
            }
            pool_ptr = pool_ptr - area_delimiter
        }
    }
    /* ext_delimiter is the length from the start of the filename to the
     * extension '.' delimiter, which we use to construct the stringpool
     * strings `cur_ext` and `cur_name`. */
    if ext_delimiter == 0 {
        cur_ext = (65536 + 1 as i64) as str_number;
        cur_name = slow_make_string()
    } else {
        cur_name = str_ptr;
        str_start[((str_ptr + 1) as i64 - 65536) as usize] =
            str_start[(str_ptr - 65536) as usize] + ext_delimiter - area_delimiter - 1;
        str_ptr += 1;
        cur_ext = make_string();
        str_ptr -= 1;
        temp_str = search_string(cur_name);
        if temp_str > 0 {
            cur_name = temp_str;
            str_ptr -= 1;
            j = str_start[((str_ptr + 1) as i64 - 65536) as usize];
            while j <= pool_ptr - 1 {
                str_pool[(j - ext_delimiter + area_delimiter + 1) as usize] = str_pool[j as usize];
                j += 1
            }
            pool_ptr = pool_ptr - ext_delimiter + area_delimiter + 1i32
        }
        cur_ext = slow_make_string()
    };
}
pub(crate) unsafe fn pack_file_name(mut n: str_number, mut a: str_number, mut e: str_number) {
    // Note that we populate the buffer in an order different than how the
    // arguments are passed to this function!
    let mut work_buffer: *mut i8 =
        xmalloc_array((length(a) + length(n) + length(e)) as usize * 3 + 1);
    *work_buffer.offset(0) = '\u{0}' as i32 as i8;
    let mut a_utf8: *mut i8 = gettexstring(a);
    strcat(work_buffer, a_utf8);
    free(a_utf8 as *mut libc::c_void);
    let mut n_utf8: *mut i8 = gettexstring(n);
    strcat(work_buffer, n_utf8);
    free(n_utf8 as *mut libc::c_void);
    let mut e_utf8: *mut i8 = gettexstring(e);
    strcat(work_buffer, e_utf8);
    free(e_utf8 as *mut libc::c_void);
    name_length = strlen(work_buffer) as i32;
    free(name_of_file as *mut libc::c_void);
    name_of_file = xmalloc_array(name_length as usize + 1);
    strcpy(name_of_file, work_buffer);
    free(work_buffer as *mut libc::c_void);
}
pub(crate) unsafe fn make_name_string() -> str_number {
    let mut k: i32 = 0;
    let mut save_area_delimiter: pool_pointer = 0;
    let mut save_ext_delimiter: pool_pointer = 0;
    let mut save_name_in_progress: bool = false;
    let mut save_stop_at_space: bool = false;
    if pool_ptr + name_length > pool_size || str_ptr == max_strings as i32 || cur_length() > 0 {
        return '?' as i32;
    }
    make_utf16_name();
    k = 0;
    while k < name_length16 {
        let fresh38 = pool_ptr;
        pool_ptr = pool_ptr + 1;
        str_pool[fresh38 as usize] = *name_of_file16.offset(k as isize);
        k += 1
    }
    let mut Result: str_number = make_string();
    save_area_delimiter = area_delimiter;
    save_ext_delimiter = ext_delimiter;
    save_name_in_progress = name_in_progress;
    save_stop_at_space = stop_at_space;
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    k = 0;
    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) as i32 != 0 {
        k += 1
    }
    stop_at_space = save_stop_at_space;
    end_name();
    name_in_progress = save_name_in_progress;
    area_delimiter = save_area_delimiter;
    ext_delimiter = save_ext_delimiter;
    Result
}
pub(crate) unsafe fn scan_file_name() {
    name_in_progress = true;
    begin_name();
    loop {
        get_x_token();
        if !(cur_cmd == SPACER as u8) {
            break;
        }
    }
    loop {
        if cur_cmd > OTHER_CHAR as u8 || cur_chr > BIGGEST_CHAR {
            back_input();
            break;
        } else {
            if !more_name(cur_chr as UTF16_code) {
                break;
            }
            get_x_token();
        }
    }
    end_name();
    name_in_progress = false;
}
pub(crate) unsafe fn pack_job_name(s: &[u8]) {
    cur_area = EMPTY_STRING as str_number;
    cur_ext = maketexstring(s);
    cur_name = job_name;
    pack_file_name(cur_name, cur_area, cur_ext);
}
pub(crate) unsafe fn open_log_file() {
    let mut k: i32 = 0;
    let mut l: i32 = 0;
    let old_setting_0 = selector;
    if job_name == 0 {
        job_name = maketexstring(b"texput")
    }
    pack_job_name(b".log");
    log_file = ttstub_output_open(name_of_file, 0);
    if log_file.is_none() {
        abort!(
            "cannot open log file output \"{}\"",
            CStr::from_ptr(name_of_file).display()
        );
    }
    texmf_log_name = make_name_string();
    selector = Selector::LOG_ONLY;
    log_opened = true;
    INPUT_STACK[INPUT_PTR] = cur_input;
    /* Here we catch the log file up with anything that has already been
     * printed. The eqtb reference is end_line_char. */
    print_nl_cstr(b"**");
    l = INPUT_STACK[0].limit;
    if BUFFER[l as usize] == *INTPAR(IntPar::end_line_char) {
        l -= 1
    }
    k = 1;
    while k <= l {
        print(BUFFER[k as usize]);
        k += 1;
    }
    print_ln();
    selector = (u8::from(old_setting_0) + 2).into();
}
pub(crate) unsafe fn start_input(mut primary_input_name: *const i8) {
    let mut format = TTInputFormat::TEX;
    let mut temp_str: str_number = 0;
    if !primary_input_name.is_null() {
        /* If this is the case, we're opening the primary input file, and the
         * name that we should use to refer to it has been handed directly to
         * us. We emulate the hacks used below to fill in cur_name, etc., from
         * a UTF-8 C string. It looks like the `cur_{name,area,ext}` strings
         * are hardly used so it'd be nice to get rid of them someday. */
        format = TTInputFormat::TECTONIC_PRIMARY;
        name_in_progress = true;
        begin_name();
        stop_at_space = false;
        let mut cp: *const u8 = primary_input_name as *const u8;
        assert!(
            !((pool_ptr as usize).wrapping_add(strlen(primary_input_name).wrapping_mul(2))
                >= pool_size as usize),
            "string pool overflow [{} bytes]",
            pool_size,
        );
        let mut rval: u32 = 0;
        loop {
            let fresh39 = cp;
            cp = cp.offset(1);
            rval = *fresh39 as u32;
            if !(rval != 0_u32) {
                break;
            }
            let mut extraBytes: u16 = bytesFromUTF8[rval as usize] as u16;
            let mut current_block_21: u64;
            match extraBytes as i32 {
                5 => {
                    /* note: code falls through cases! */
                    rval <<= 6i32;
                    if *cp != 0 {
                        let fresh40 = cp;
                        cp = cp.offset(1);
                        rval = (rval as u32).wrapping_add(*fresh40 as u32) as u32 as u32
                    }
                    current_block_21 = 7676382540965064243;
                }
                4 => current_block_21 = 7676382540965064243,
                3 => current_block_21 = 13258898395114305131,
                2 => current_block_21 = 10625751394499422232,
                1 => current_block_21 = 4051951890355284227,
                0 | _ => current_block_21 = 14818589718467733107,
            }
            match current_block_21 {
                7676382540965064243 => {
                    rval <<= 6i32;
                    if *cp != 0 {
                        let fresh41 = cp;
                        cp = cp.offset(1);
                        rval = (rval as u32).wrapping_add(*fresh41 as u32) as u32 as u32
                    }
                    current_block_21 = 13258898395114305131;
                }
                _ => {}
            }
            match current_block_21 {
                13258898395114305131 => {
                    rval <<= 6i32;
                    if *cp != 0 {
                        let fresh42 = cp;
                        cp = cp.offset(1);
                        rval = (rval as u32).wrapping_add(*fresh42 as u32) as u32 as u32
                    }
                    current_block_21 = 10625751394499422232;
                }
                _ => {}
            }
            match current_block_21 {
                10625751394499422232 => {
                    rval <<= 6i32;
                    if *cp != 0 {
                        let fresh43 = cp;
                        cp = cp.offset(1);
                        rval = (rval as u32).wrapping_add(*fresh43 as u32) as u32 as u32
                    }
                    current_block_21 = 4051951890355284227;
                }
                _ => {}
            }
            match current_block_21 {
                4051951890355284227 => {
                    rval <<= 6i32;
                    if *cp != 0 {
                        let fresh44 = cp;
                        cp = cp.offset(1);
                        rval = (rval as u32).wrapping_add(*fresh44 as u32) as u32 as u32
                    }
                }
                _ => {}
            }
            rval = (rval as u32).wrapping_sub(offsetsFromUTF8[extraBytes as usize]) as u32 as u32;
            if rval > 0xffff_u32 {
                rval = (rval as u32).wrapping_sub(0x10000_u32) as u32 as u32;
                let fresh45 = pool_ptr;
                pool_ptr = pool_ptr + 1;
                str_pool[fresh45 as usize] =
                    (0xd800_u32).wrapping_add(rval.wrapping_div(0x400_u32)) as packed_UTF16_code;
                let fresh46 = pool_ptr;
                pool_ptr = pool_ptr + 1;
                str_pool[fresh46 as usize] =
                    (0xdc00_u32).wrapping_add(rval.wrapping_rem(0x400_u32)) as packed_UTF16_code
            } else {
                let fresh47 = pool_ptr;
                pool_ptr = pool_ptr + 1;
                str_pool[fresh47 as usize] = rval as packed_UTF16_code
            }
            if rval == '/' as i32 as u32 {
                area_delimiter = cur_length();
                ext_delimiter = 0;
            } else if rval == '.' as i32 as u32 {
                ext_delimiter = cur_length()
            }
        }
        stop_at_space = true;
        end_name();
        name_in_progress = false
    } else {
        /* Scan in the file name from the current token stream. The file name to
         * input is saved as the stringpool strings `cur_{name,area,ext}` and the
         * UTF-8 string `name_of_file`. */
        scan_file_name();
    }
    pack_file_name(cur_name, cur_area, cur_ext);
    /* Open up the new file to be read. The name of the file to be read comes
     * from `name_of_file`. */
    begin_file_reading();
    if u_open_in(
        &mut INPUT_FILE[cur_input.index as usize],
        format,
        b"rb",
        *INTPAR(IntPar::xetex_default_input_mode),
        *INTPAR(IntPar::xetex_default_input_encoding),
    ) == 0
    {
        abort!(
            "failed to open input file \"{}\"",
            CStr::from_ptr(name_of_file).display()
        );
    }
    /* Now re-encode `name_of_file` into the UTF-16 variable `name_of_file16`,
     * and use that to recompute `cur_{name,area,ext}`. */
    make_utf16_name();
    name_in_progress = true;
    begin_name();
    stop_at_space = false;
    let mut k: i32 = 0i32;
    while k < name_length16 && more_name(*name_of_file16.offset(k as isize)) as i32 != 0 {
        k += 1
    }
    stop_at_space = true;
    end_name();
    name_in_progress = false;
    /* Now generate a stringpool string corresponding to the full path of the
     * input file. This calls make_utf16_name() again and reruns through the
     * {begin,more,end}_name() trifecta to re-re-compute
     * `cur_{name,area,ext}`. */
    cur_input.name = make_name_string();
    SOURCE_FILENAME_STACK[IN_OPEN] = cur_input.name;
    /* *This* variant is a TeX string made out of `name_of_input_file`. */
    FULL_SOURCE_FILENAME_STACK[IN_OPEN] =
        maketexstring(CStr::from_ptr(name_of_input_file).to_bytes());
    if cur_input.name == str_ptr - 1 {
        temp_str = search_string(cur_input.name);
        if temp_str > 0 {
            cur_input.name = temp_str;
            str_ptr -= 1;
            pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize]
        }
    }
    /* Finally we start really doing stuff with the newly-opened file. */
    if job_name == 0 {
        job_name = cur_name; /* this is the "flush_string" macro which discards the most recent string */
        open_log_file(); /* "really a CFDictionaryRef or XeTeXLayoutEngine" */
    } /* = first_math_fontdimen (=10) + lastMathConstant (= radicalDegreeBottomRaisePercent = 55) */
    if term_offset + length(FULL_SOURCE_FILENAME_STACK[IN_OPEN]) > max_print_line - 2 {
        print_ln();
    } else if term_offset > 0 || file_offset > 0 {
        print_char(' ' as i32);
    }
    print_char('(' as i32);
    open_parens += 1;
    print(FULL_SOURCE_FILENAME_STACK[IN_OPEN]);
    rust_stdout.as_mut().unwrap().flush().unwrap();
    cur_input.state = NEW_LINE;
    synctex_start_input();
    line = 1;
    input_line(INPUT_FILE[cur_input.index as usize]);
    cur_input.limit = last;
    if *INTPAR(IntPar::end_line_char) < 0 || *INTPAR(IntPar::end_line_char) > 255 {
        cur_input.limit -= 1
    } else {
        BUFFER[cur_input.limit as usize] = *INTPAR(IntPar::end_line_char)
    }
    first = cur_input.limit + 1;
    cur_input.loc = cur_input.start;
}
pub(crate) unsafe fn effective_char_info(mut f: internal_font_number, mut c: u16) -> b16x4 {
    if !xtx_ligature_present && !(FONT_MAPPING[f]).is_null() {
        c = apply_tfm_font_mapping(FONT_MAPPING[f], c as i32) as u16
    }
    xtx_ligature_present = false;
    FONT_CHARACTER_INFO(f, c as usize)
}
pub(crate) unsafe fn char_warning(mut f: internal_font_number, mut c: i32) {
    let mut old_setting_0: i32 = 0;
    if *INTPAR(IntPar::tracing_lost_chars) > 0 {
        old_setting_0 = *INTPAR(IntPar::tracing_online);
        if *INTPAR(IntPar::tracing_lost_chars) > 1 {
            *INTPAR(IntPar::tracing_online) = 1
        }
        begin_diagnostic();
        print_nl_cstr(b"Missing character: There is no ");
        if (c as i64) < 65536 {
            print(c);
        } else {
            print_char(c);
        }
        print_cstr(b" in font ");
        print(FONT_NAME[f]);
        print_char('!' as i32);
        end_diagnostic(false);
        *INTPAR(IntPar::tracing_online) = old_setting_0
    }
    let mut fn_0: *mut i8 = gettexstring(FONT_NAME[f]);
    let mut chr: *mut i8 = 0 as *mut i8;
    let prev_selector = selector;
    let mut s: i32 = 0;
    selector = Selector::NEW_STRING;
    if c < 0x10000 {
        print(c);
    } else {
        print_char(c);
    }
    selector = prev_selector;
    s = make_string();
    chr = gettexstring(s);
    str_ptr -= 1;
    pool_ptr = str_start[(str_ptr - 0x10000) as usize];
    ttstub_issue_warning(
        b"could not represent character \"%s\" in font \"%s\"\x00" as *const u8 as *const i8,
        chr,
        fn_0,
    );
    free(fn_0 as *mut libc::c_void);
    free(chr as *mut libc::c_void);
    if !gave_char_warning_help {
        ttstub_issue_warning(
            b"  you may need to load the `fontspec` package and use (e.g.) \\setmainfont to\x00"
                as *const u8 as *const i8,
        );
        ttstub_issue_warning(
            b"  choose a different font that covers the unrepresentable character(s)\x00"
                as *const u8 as *const i8,
        );
        gave_char_warning_help = true
    };
}
pub(crate) unsafe fn new_native_word_node(mut f: internal_font_number, mut n: i32) -> usize {
    let mut l: i32 = 0;
    l = (NATIVE_NODE_SIZE as u64).wrapping_add(
        (n as u64)
            .wrapping_mul(::std::mem::size_of::<UTF16_code>() as u64)
            .wrapping_add(::std::mem::size_of::<memory_word>() as u64)
            .wrapping_sub(1i32 as u64)
            .wrapping_div(::std::mem::size_of::<memory_word>() as u64),
    ) as i32;
    let q = get_node(l) as usize;
    *NODE_type(q) = WHATSIT_NODE;
    if *INTPAR(IntPar::xetex_generate_actual_text) > 0i32 {
        *NODE_subtype(q) = NATIVE_WORD_NODE_AT;
    } else {
        *NODE_subtype(q) = NATIVE_WORD_NODE;
    }
    *NATIVE_NODE_size(q) = l as u16;
    *NATIVE_NODE_font(q) = f as u16;
    *NATIVE_NODE_length(q) = n as u16;
    *NATIVE_NODE_glyph_count(q) = 0_u16;
    *NATIVE_NODE_glyph_info_ptr(q) = ptr::null_mut();
    q
}
pub(crate) unsafe fn new_native_character(
    mut f: internal_font_number,
    mut c: UnicodeScalar,
) -> usize {
    let p: usize;
    let mut i: i32 = 0;
    let mut len: i32 = 0;
    if !(FONT_MAPPING[f]).is_null() {
        if c as i64 > 65535 {
            if pool_ptr + 2 > pool_size {
                overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
            }
            str_pool[pool_ptr as usize] =
                ((c as i64 - 65536) / 1024 as i64 + 0xd800) as packed_UTF16_code;
            pool_ptr += 1;
            str_pool[pool_ptr as usize] =
                ((c as i64 - 65536) % 1024 as i64 + 0xdc00) as packed_UTF16_code;
            pool_ptr += 1
        } else {
            if pool_ptr + 1 > pool_size {
                overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
            }
            str_pool[pool_ptr as usize] = c as packed_UTF16_code;
            pool_ptr += 1
        }

        len = apply_mapping(
            FONT_MAPPING[f],
            &mut str_pool[str_start[(str_ptr - TOO_BIG_CHAR) as usize] as usize],
            cur_length(),
        );
        pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize];

        i = 0;

        while i < len {
            if *mapped_text.offset(i as isize) as i32 >= 0xd800
                && (*mapped_text.offset(i as isize) as i32) < 0xdc00
            {
                c = (*mapped_text.offset(i as isize) as i32 - 0xd800) * 1024
                    + *mapped_text.offset((i + 1) as isize) as i32
                    + 9216;
                if map_char_to_glyph(f, c) == 0 {
                    char_warning(f, c);
                }
                i += 2;
            } else {
                if map_char_to_glyph(f, *mapped_text.offset(i as isize) as i32) == 0 {
                    char_warning(f, *mapped_text.offset(i as isize) as i32);
                }
                i += 1;
            }
        }

        p = new_native_word_node(f, len);

        i = 0;
        while i <= len - 1 {
            *(&mut MEM[p + 6] as *mut memory_word as *mut u16).offset(i as isize) =
                *mapped_text.offset(i as isize);
            i += 1
        }
    } else {
        if *INTPAR(IntPar::tracing_lost_chars) > 0 {
            if map_char_to_glyph(f, c) == 0 {
                char_warning(f, c);
            }
        }
        p = get_node(NATIVE_NODE_SIZE + 1);
        MEM[p].b16.s1 = WHATSIT_NODE;
        MEM[p].b16.s0 = NATIVE_WORD_NODE;
        MEM[p + 4].b16.s3 = (NATIVE_NODE_SIZE + 1) as u16;
        MEM[p + 4].b16.s0 = 0;
        MEM[p + 5].ptr = ptr::null_mut();
        MEM[p + 4].b16.s2 = f as u16;
        if c as i64 > 65535 {
            MEM[p + 4].b16.s1 = 2;
            *(&mut MEM[p + 6] as *mut memory_word as *mut u16).offset(0) =
                ((c as i64 - 65536) / 1024 as i64 + 0xd800) as u16;
            *(&mut MEM[p + 6] as *mut memory_word as *mut u16).offset(1) =
                ((c as i64 - 65536) % 1024 as i64 + 0xdc00) as u16
        } else {
            MEM[p + 4].b16.s1 = 1;
            *(&mut MEM[p + 6] as *mut memory_word as *mut u16).offset(0) = c as u16
        }
    }
    measure_native_node(
        &mut MEM[p] as *mut memory_word as *mut libc::c_void,
        (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0) as i32,
    );
    p
}
pub(crate) unsafe fn font_feature_warning(feature_name: &[u8], setting_name: &[u8]) {
    begin_diagnostic();
    print_nl_cstr(b"Unknown ");
    if !setting_name.is_empty() {
        print_cstr(b"selector `");
        print_utf8_str(setting_name);
        print_cstr(b"\' for ");
    }
    print_cstr(b"feature `");
    print_utf8_str(feature_name);
    print_cstr(b"\' in font `");
    let mut i: i32 = 0i32;
    while *name_of_file.offset(i as isize) as i32 != 0 {
        print_raw_char(*name_of_file.offset(i as isize) as UTF16_code, true);
        i += 1
    }
    print_cstr(b"\'.");
    end_diagnostic(false);
}
pub(crate) unsafe fn font_mapping_warning(mut mapping_name: &[u8], mut warningType: i32) {
    begin_diagnostic();
    if warningType == 0i32 {
        print_nl_cstr(b"Loaded mapping `");
    } else {
        print_nl_cstr(b"Font mapping `");
    }
    print_utf8_str(mapping_name);
    print_cstr(b"\' for font `");
    let mut i = 0i32;
    while *name_of_file.offset(i as isize) != 0 {
        print_raw_char(*name_of_file.offset(i as isize) as UTF16_code, true);
        i += 1;
    }
    match warningType {
        1 => print_cstr(b"\' not found."),
        2 => {
            print_cstr(b"\' not usable;");
            print_nl_cstr(b"bad mapping file or incorrect mapping type.");
        }
        _ => print_cstr(b"\'."),
    }
    end_diagnostic(false);
}
pub(crate) unsafe fn graphite_warning() {
    begin_diagnostic();
    print_nl_cstr(b"Font `");
    let mut i = 0i32;
    while *name_of_file.offset(i as isize) != 0 {
        print_raw_char(*name_of_file.offset(i as isize) as UTF16_code, true);
        i += 1
    }
    print_cstr(b"\' does not support Graphite. Trying OpenType layout instead.");
    end_diagnostic(false);
}
pub(crate) unsafe fn load_native_font(
    mut u: i32,
    mut nom: str_number,
    mut aire: str_number,
    mut s: scaled_t,
) -> usize {
    let mut num_font_dimens: i32 = 0;
    let mut actual_size: scaled_t = 0;
    let mut ascent: scaled_t = 0;
    let mut descent: scaled_t = 0;
    let mut font_slant: scaled_t = 0;
    let mut x_ht: scaled_t = 0;
    let mut cap_ht: scaled_t = 0;
    let mut font_engine = find_native_font(name_of_file, s);
    if font_engine.is_null() {
        return FONT_BASE as usize;
    }
    if s >= 0 {
        actual_size = s
    } else if s != -1000 {
        actual_size = xn_over_d(loaded_font_design_size, -s, 1000)
    } else {
        actual_size = loaded_font_design_size
    }
    if pool_ptr + name_length > pool_size {
        overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
    }
    for k in 0..name_length {
        str_pool[pool_ptr as usize] = *name_of_file.offset(k as isize) as packed_UTF16_code;
        pool_ptr = pool_ptr + 1;
    }

    let full_name = make_string();

    for f in (0 + 1)..=FONT_PTR {
        if FONT_AREA[f] == native_font_type_flag
            && str_eq_str(FONT_NAME[f], full_name)
            && FONT_SIZE[f] == actual_size
        {
            release_font_engine(font_engine, native_font_type_flag);
            str_ptr -= 1;
            pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize];
            return f;
        }
    }

    num_font_dimens = if native_font_type_flag as u32 == OTGR_FONT_FLAG
        && isOpenTypeMathFont(font_engine as XeTeXLayoutEngine)
    {
        65 // = first_math_fontdimen (=10) + lastMathConstant (= radicalDegreeBottomRaisePercent = 55)
    } else {
        8
    };
    if FONT_PTR == FONT_MAX || fmem_ptr + num_font_dimens > FONT_MEM_SIZE as i32 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Font ");
        sprint_cs(u);
        print_char('=' as i32);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        print_file_name(nom, aire, cur_ext);
        if file_name_quote_char != 0 {
            print_char(file_name_quote_char as i32);
        }
        if s >= 0 {
            print_cstr(b" at ");
            print_scaled(s);
            print_cstr(b"pt");
        } else if s != -1000 {
            print_cstr(b" scaled ");
            print_int(-s);
        }
        print_cstr(b" not loaded: Not enough room left");
        help_ptr = 4;
        help_line[3] = b"I\'m afraid I won\'t be able to make use of this font,";
        help_line[2] = b"because my memory for character-size data is too small.";
        help_line[1] = b"If you\'re really stuck, ask a wizard to enlarge me.";
        help_line[0] = b"Or maybe try `I\\font<same font id>=<name of loaded font>\'.";
        error();
        return FONT_BASE as usize;
    }
    FONT_PTR += 1;
    FONT_AREA[FONT_PTR] = native_font_type_flag;
    FONT_NAME[FONT_PTR] = full_name;
    FONT_CHECK[FONT_PTR].s3 = 0;
    FONT_CHECK[FONT_PTR].s2 = 0;
    FONT_CHECK[FONT_PTR].s1 = 0;
    FONT_CHECK[FONT_PTR].s0 = 0;
    FONT_GLUE[FONT_PTR] = TEX_NULL;
    FONT_DSIZE[FONT_PTR] = loaded_font_design_size;
    FONT_SIZE[FONT_PTR] = actual_size;
    match native_font_type_flag as u32 {
        #[cfg(target_os = "macos")]
        AAT_FONT_FLAG => {
            aat::aat_get_font_metrics(
                font_engine as _,
                &mut ascent,
                &mut descent,
                &mut x_ht,
                &mut cap_ht,
                &mut font_slant,
            );
        }
        #[cfg(not(target_os = "macos"))]
        AAT_FONT_FLAG => {
            // do nothing
        }
        _ => {
            ot_get_font_metrics(
                font_engine,
                &mut ascent,
                &mut descent,
                &mut x_ht,
                &mut cap_ht,
                &mut font_slant,
            );
        }
    }
    HEIGHT_BASE[FONT_PTR] = ascent;
    DEPTH_BASE[FONT_PTR] = -descent;
    FONT_PARAMS[FONT_PTR] = num_font_dimens;
    FONT_BC[FONT_PTR] = 0 as UTF16_code;
    FONT_EC[FONT_PTR] = 65535 as UTF16_code;
    *font_used.offset(FONT_PTR as isize) = false;
    HYPHEN_CHAR[FONT_PTR] = *INTPAR(IntPar::default_hyphen_char);
    SKEW_CHAR[FONT_PTR] = *INTPAR(IntPar::default_skew_char);
    PARAM_BASE[FONT_PTR] = fmem_ptr - 1;
    FONT_LAYOUT_ENGINE[FONT_PTR] = font_engine;
    FONT_MAPPING[FONT_PTR] = 0 as *mut libc::c_void;
    FONT_LETTER_SPACE[FONT_PTR] = loaded_font_letter_space;
    /* "measure the width of the space character and set up font parameters" */
    let p = new_native_character(FONT_PTR, ' ' as i32);
    s = MEM[p + 1].b32.s1 + loaded_font_letter_space;
    free_node(p, *NATIVE_NODE_size(p) as i32);

    FONT_INFO[fmem_ptr as usize].b32.s1 = font_slant;
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = s;
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = s / 2; // space_stretch
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = s / 3; // space_shrink
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = x_ht;
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = FONT_SIZE[FONT_PTR]; // quad
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = s / 3; // extra_space
    fmem_ptr += 1;
    FONT_INFO[fmem_ptr as usize].b32.s1 = cap_ht;
    fmem_ptr += 1;
    if num_font_dimens == 65 {
        FONT_INFO[fmem_ptr as usize].b32.s1 = num_font_dimens;
        fmem_ptr += 1;
        for k in 0..=55 {
            /* 55 = lastMathConstant */
            /*:582*/
            FONT_INFO[fmem_ptr as usize].b32.s1 = get_ot_math_constant(FONT_PTR, k);
            fmem_ptr += 1;
        }
    }
    FONT_MAPPING[FONT_PTR] = loaded_font_mapping;
    FONT_FLAGS[FONT_PTR] = loaded_font_flags;
    FONT_PTR
}
pub(crate) unsafe fn do_locale_linebreaks(mut s: i32, mut len: i32) {
    let mut offs: i32 = 0;
    let mut prevOffs: i32 = 0;
    let mut i: i32 = 0;
    let mut use_penalty: bool = false;
    let mut use_skip: bool = false;
    if *INTPAR(IntPar::xetex_linebreak_locale) == 0 || len == 1 {
        MEM[cur_list.tail].b32.s1 = new_native_word_node(main_f, len) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        let mut for_end: i32 = 0;
        i = 0;
        for_end = len - 1;
        if i <= for_end {
            loop {
                *(&mut MEM[cur_list.tail + 6] as *mut memory_word as *mut u16).offset(i as isize) =
                    *native_text.offset((s + i) as isize);
                let fresh64 = i;
                i = i + 1;
                if !(fresh64 < for_end) {
                    break;
                }
            }
        }
        measure_native_node(
            &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
            (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0) as i32,
        );
    } else {
        use_skip = *GLUEPAR(GluePar::xetex_linebreak_skip) != 0;
        use_penalty = *INTPAR(IntPar::xetex_linebreak_penalty) != 0 || !use_skip;
        linebreak_start(
            main_f,
            *INTPAR(IntPar::xetex_linebreak_locale),
            native_text.offset(s as isize),
            len,
        );
        offs = 0;
        loop {
            prevOffs = offs;
            offs = linebreak_next();
            if offs > 0 {
                if prevOffs != 0 {
                    if use_penalty {
                        MEM[cur_list.tail].b32.s1 =
                            new_penalty(*INTPAR(IntPar::xetex_linebreak_penalty)) as i32;
                        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                    }
                    if use_skip {
                        MEM[cur_list.tail].b32.s1 = new_param_glue(15 as small_number) as i32;
                        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                    }
                }
                MEM[cur_list.tail].b32.s1 = new_native_word_node(main_f, offs - prevOffs) as i32;
                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                let mut for_end_0: i32 = 0;
                i = prevOffs;
                for_end_0 = offs - 1;
                if i <= for_end_0 {
                    loop {
                        *(&mut MEM[cur_list.tail + 6] as *mut memory_word as *mut u16)
                            .offset((i - prevOffs) as isize) =
                            *native_text.offset((s + i) as isize);
                        let fresh65 = i;
                        i = i + 1;
                        if !(fresh65 < for_end_0) {
                            break;
                        }
                    }
                }
                measure_native_node(
                    &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                    (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0) as i32,
                );
            }
            if offs < 0 {
                break;
            }
        }
    };
}
pub(crate) unsafe fn bad_utf8_warning() {
    begin_diagnostic();
    print_nl_cstr(b"Invalid UTF-8 byte or sequence");
    if cur_input.name == 0 {
        print_cstr(b" in terminal input");
    } else {
        print_cstr(b" at line ");
        print_int(line);
    }
    print_cstr(b" replaced by U+FFFD.");
    end_diagnostic(false);
}
pub(crate) unsafe fn get_input_normalization_state() -> i32 {
    if EQTB.is_empty() {
        0
    } else {
        *INTPAR(IntPar::xetex_input_normalization)
    }
}
pub(crate) unsafe fn get_tracing_fonts_state() -> i32 {
    *INTPAR(IntPar::xetex_tracing_fonts)
}
pub(crate) unsafe fn read_font_info(
    mut u: i32,
    mut nom: str_number,
    mut aire: str_number,
    mut s: scaled_t,
) -> internal_font_number {
    let mut k: font_index = 0;
    let mut name_too_long: bool = false;
    let mut lf: i32 = 0;
    let mut lh: i32 = 0;
    let mut bc: i32 = 0;
    let mut ec: i32 = 0;
    let mut nw: i32 = 0;
    let mut nh: i32 = 0;
    let mut nd: i32 = 0;
    let mut ni: i32 = 0;
    let mut nl: i32 = 0;
    let mut nk: i32 = 0;
    let mut ne: i32 = 0;
    let mut np: i32 = 0;
    let mut f: internal_font_number = 0;
    let mut a: i32 = 0;
    let mut b: i32 = 0;
    let mut c: i32 = 0;
    let mut d: i32 = 0;
    let mut qw: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut sw: scaled_t = 0;
    let mut bch_label: i32 = 0;
    let mut bchar_0: i16 = 0;
    let mut z: scaled_t = 0;
    let mut alpha: i32 = 0;
    let mut beta: u8 = 0;

    let mut g = FONT_BASE as usize;

    pack_file_name(nom, aire, cur_ext);

    if *INTPAR(IntPar::xetex_tracing_fonts) > 0 {
        begin_diagnostic();
        print_nl_cstr(b"Requested font \"");
        print_c_string(name_of_file);
        print('\"' as i32);
        if s < 0 {
            print_cstr(b" scaled ");
            print_int(-s);
        } else {
            print_cstr(b" at ");
            print_scaled(s);
            print_cstr(b"pt");
        }
        end_diagnostic(false);
    }

    if quoted_filename {
        g = load_native_font(u, nom, aire, s);
        if g != FONT_BASE as usize {
            return done(None, g);
        }
    }

    name_too_long = length(nom) > 255 || length(aire) > 255;
    if name_too_long {
        return bad_tfm(None, g, u, nom, aire, s, name_too_long);
    }
    pack_file_name(nom, aire, EMPTY_STRING as str_number);
    check_for_tfm_font_mapping();

    let mut tfm_file_owner = tt_xetex_open_input(TTInputFormat::TFM);
    if tfm_file_owner.is_none() {
        if !quoted_filename {
            g = load_native_font(u, nom, aire, s);
            if g != FONT_BASE as usize {
                return done(None, g);
            }
        }
        return bad_tfm(None, g, u, nom, aire, s, name_too_long);
    }

    let tfm_file = tfm_file_owner.as_mut().unwrap();

    /* We are a bit cavalier about EOF-checking since we can't very
     * conveniently implement feof() in the Rust layer, and it only ever is
     * used in this one place. */

    macro_rules! READFIFTEEN (
        ($x:expr) => {
            $x = ttstub_input_getc(tfm_file);
            if $x > 127 || $x == libc::EOF {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            $x *= 256;
            $x += ttstub_input_getc(tfm_file);

        };
    );

    READFIFTEEN!(lf);
    READFIFTEEN!(lh);
    READFIFTEEN!(bc);
    READFIFTEEN!(ec);

    if bc > ec + 1 || ec > 255 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    if bc > 255 {
        bc = 1;
        ec = 0
    }

    READFIFTEEN!(nw);
    READFIFTEEN!(nh);
    READFIFTEEN!(nd);
    READFIFTEEN!(ni);
    READFIFTEEN!(nl);
    READFIFTEEN!(nk);
    READFIFTEEN!(ne);
    READFIFTEEN!(np);

    if lf != 6 + lh + (ec - bc + 1) + nw + nh + nd + ni + nl + nk + ne + np {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    } else if nw == 0 || nh == 0 || nd == 0 || ni == 0 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }

    lf = lf - 6 - lh;
    if np < 7 {
        lf = lf + 7 - np
    }
    assert!(
        !(FONT_PTR == FONT_MAX || fmem_ptr + lf > FONT_MEM_SIZE as i32),
        "not enough memory to load another font"
    );

    f = FONT_PTR + 1;
    CHAR_BASE[f] = fmem_ptr - bc;
    WIDTH_BASE[f] = CHAR_BASE[f] + ec + 1;
    HEIGHT_BASE[f] = WIDTH_BASE[f] + nw;
    DEPTH_BASE[f] = HEIGHT_BASE[f] + nh;
    ITALIC_BASE[f] = DEPTH_BASE[f] + nd;
    LIG_KERN_BASE[f] = ITALIC_BASE[f] + ni;
    KERN_BASE[f] = LIG_KERN_BASE[f] + nl - 256 * 128;
    EXTEN_BASE[f] = KERN_BASE[f] + 256 * 128 + nk;
    PARAM_BASE[f] = EXTEN_BASE[f] + ne;
    if lh < 2 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    a = ttstub_input_getc(tfm_file);
    qw.s3 = a as u16;
    b = ttstub_input_getc(tfm_file);
    qw.s2 = b as u16;
    c = ttstub_input_getc(tfm_file);
    qw.s1 = c as u16;
    d = ttstub_input_getc(tfm_file);
    qw.s0 = d as u16;
    if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    FONT_CHECK[f as usize] = qw;

    READFIFTEEN!(z);
    z = z * 256 + ttstub_input_getc(tfm_file);
    z = z * 16 + ttstub_input_getc(tfm_file) / 16;
    if z < 65536 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    while lh > 2 {
        ttstub_input_getc(tfm_file);
        ttstub_input_getc(tfm_file);
        ttstub_input_getc(tfm_file);
        ttstub_input_getc(tfm_file);
        lh -= 1
    }
    FONT_DSIZE[f] = z;
    if s != -1000 {
        if s >= 0 {
            z = s
        } else {
            z = xn_over_d(z, -s, 1000)
        }
    }
    FONT_SIZE[f] = z;

    k = fmem_ptr;
    loop {
        if !(k <= WIDTH_BASE[f] - 1) {
            break;
        }
        a = ttstub_input_getc(tfm_file);
        qw.s3 = a as u16;
        b = ttstub_input_getc(tfm_file);
        qw.s2 = b as u16;
        c = ttstub_input_getc(tfm_file);
        qw.s1 = c as u16;
        d = ttstub_input_getc(tfm_file);
        qw.s0 = d as u16;
        if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
        FONT_INFO[k as usize].b16 = qw;

        if a >= nw || b / 16 >= nh || b % 16 >= nd || c / 4 >= ni {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }

        match c % 4 {
            1 => {
                if d >= nl {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
            }
            3 => {
                if d >= ne {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
            }
            2 => {
                if d < bc || d > ec {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
                loop {
                    if !(d < k + bc - fmem_ptr) {
                        break;
                    }
                    qw = FONT_INFO[(CHAR_BASE[f] + d) as usize].b16;
                    if qw.s1 as i32 % 4 != LIST_TAG {
                        break;
                    }
                    d = qw.s0 as i32
                }
                if d == k + bc - fmem_ptr {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
            }
            _ => {}
        }
        k += 1;
    }

    alpha = 16;
    while z >= 0x800000 {
        z = z / 2;
        alpha = alpha + alpha
    }
    beta = (256 / alpha) as u8;
    alpha = alpha * z;

    for k in WIDTH_BASE[f]..=LIG_KERN_BASE[f] - 1 {
        a = ttstub_input_getc(tfm_file);
        b = ttstub_input_getc(tfm_file);
        c = ttstub_input_getc(tfm_file);
        d = ttstub_input_getc(tfm_file);
        if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
        sw = ((d * z / 256 + c * z) / 256 + b * z) / beta as i32;

        if a == 0 {
            FONT_INFO[k as usize].b32.s1 = sw
        } else if a == 255 {
            FONT_INFO[k as usize].b32.s1 = sw - alpha
        } else {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
    }

    if FONT_INFO[WIDTH_BASE[f] as usize].b32.s1 != 0 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    if FONT_INFO[HEIGHT_BASE[f] as usize].b32.s1 != 0 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    if FONT_INFO[DEPTH_BASE[f] as usize].b32.s1 != 0 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }
    if FONT_INFO[ITALIC_BASE[f] as usize].b32.s1 != 0 {
        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
    }

    bch_label = 32767;
    bchar_0 = 256;
    if nl > 0 {
        for k in LIG_KERN_BASE[f]..=KERN_BASE[f] + 256 * 128 - 1 {
            a = ttstub_input_getc(tfm_file);
            qw.s3 = a as u16;
            b = ttstub_input_getc(tfm_file);
            qw.s2 = b as u16;
            c = ttstub_input_getc(tfm_file);
            qw.s1 = c as u16;
            d = ttstub_input_getc(tfm_file);
            qw.s0 = d as u16;
            if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            FONT_INFO[k as usize].b16 = qw;

            if a > 128 {
                if 256 * c + d >= nl {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
                if a == 255 && k == LIG_KERN_BASE[f] {
                    bchar_0 = b as i16
                }
            } else {
                if b != bchar_0 as i32 {
                    if b < bc || b > ec {
                        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                    }

                    qw = FONT_INFO[(CHAR_BASE[f] + b) as usize].b16;
                    if !(qw.s3 > 0) {
                        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                    }
                }

                if c < 128 {
                    if d < bc || d > ec {
                        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                    }
                    qw = FONT_INFO[(CHAR_BASE[f] + d) as usize].b16;
                    if !(qw.s3 > 0) {
                        return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                    }
                } else if 256 * (c - 128) + d >= nk {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
                if a < 128 && k - LIG_KERN_BASE[f] + a + 1i32 >= nl {
                    return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
                }
            }
        }
        if a == 255 {
            bch_label = 256 * c + d
        }
    }

    for k in KERN_BASE[f] + 256 * 128..=EXTEN_BASE[f] - 1 {
        a = ttstub_input_getc(tfm_file);
        b = ttstub_input_getc(tfm_file);
        c = ttstub_input_getc(tfm_file);
        d = ttstub_input_getc(tfm_file);
        if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
        sw = ((d * z / 256i32 + c * z) / 256i32 + b * z) / beta as i32;
        if a == 0 {
            FONT_INFO[k as usize].b32.s1 = sw
        } else if a == 255 {
            FONT_INFO[k as usize].b32.s1 = sw - alpha
        } else {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
    }

    for k in EXTEN_BASE[f]..=PARAM_BASE[f] - 1 {
        a = ttstub_input_getc(tfm_file);
        qw.s3 = a as u16;
        b = ttstub_input_getc(tfm_file);
        qw.s2 = b as u16;
        c = ttstub_input_getc(tfm_file);
        qw.s1 = c as u16;
        d = ttstub_input_getc(tfm_file);
        qw.s0 = d as u16;
        if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
        FONT_INFO[k as usize].b16 = qw;

        if a != 0 {
            if a < bc || a > ec {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            qw = FONT_INFO[(CHAR_BASE[f] + a) as usize].b16;
            if !(qw.s3 as i32 > 0i32) {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
        }

        if b != 0 {
            if b < bc || b > ec {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            qw = FONT_INFO[(CHAR_BASE[f] + b) as usize].b16;
            if !(qw.s3 > 0) {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
        }

        if c != 0 {
            if c < bc || c > ec {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            qw = FONT_INFO[(CHAR_BASE[f] + c) as usize].b16;
            if !(qw.s3 > 0) {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
        }

        if d < bc || d > ec {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
        qw = FONT_INFO[(CHAR_BASE[f] + d) as usize].b16;
        if !(qw.s3 > 0) {
            return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
        }
    }

    for k in 1..=np {
        if k == 1 {
            sw = ttstub_input_getc(tfm_file);
            if sw == libc::EOF {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            if sw > 127 {
                sw = sw - 256
            }

            sw = sw * 256 + ttstub_input_getc(tfm_file);
            sw = sw * 256 + ttstub_input_getc(tfm_file);
            FONT_INFO[PARAM_BASE[f] as usize].b32.s1 = sw * 16 + ttstub_input_getc(tfm_file) / 16
        } else {
            a = ttstub_input_getc(tfm_file);
            b = ttstub_input_getc(tfm_file);
            c = ttstub_input_getc(tfm_file);
            d = ttstub_input_getc(tfm_file);
            if a == libc::EOF || b == libc::EOF || c == libc::EOF || d == libc::EOF {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
            sw = ((d * z / 256 + c * z) / 256 + b * z) / beta as i32;
            if a == 0 {
                FONT_INFO[(PARAM_BASE[f] + k - 1) as usize].b32.s1 = sw
            } else if a == 255 {
                FONT_INFO[(PARAM_BASE[f] + k - 1) as usize].b32.s1 = sw - alpha
            } else {
                return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);
            }
        }
    }

    for k in np + 1..=7 {
        FONT_INFO[(PARAM_BASE[f] + k - 1) as usize].b32.s1 = 0;
    }

    if np >= 7 {
        FONT_PARAMS[f] = np
    } else {
        FONT_PARAMS[f] = 7
    }

    HYPHEN_CHAR[f] = *INTPAR(IntPar::default_hyphen_char);
    SKEW_CHAR[f] = *INTPAR(IntPar::default_skew_char);
    if bch_label < nl {
        BCHAR_LABEL[f] = bch_label + LIG_KERN_BASE[f]
    } else {
        BCHAR_LABEL[f] = NON_ADDRESS;
    }
    FONT_BCHAR[f] = bchar_0 as _;
    FONT_FALSE_BCHAR[f] = bchar_0 as nine_bits;

    if bchar_0 as i32 <= ec {
        if bchar_0 as i32 >= bc {
            qw = FONT_INFO[(CHAR_BASE[f] + bchar_0 as i32) as usize].b16;
            if qw.s3 as i32 > 0 {
                FONT_FALSE_BCHAR[f] = TOO_BIG_CHAR;
            }
        }
    }

    FONT_NAME[f] = nom;
    FONT_AREA[f] = aire;
    FONT_BC[f] = bc as UTF16_code;
    FONT_EC[f] = ec as UTF16_code;
    FONT_GLUE[f] = TEX_NULL;
    PARAM_BASE[f] -= 1;
    fmem_ptr = fmem_ptr + lf;
    FONT_PTR = f;
    g = f;
    FONT_MAPPING[f] = load_tfm_font_mapping();

    return done(tfm_file_owner, g);

    /// Called on error
    unsafe fn bad_tfm(
        tfm_file: Option<InputHandleWrapper>,
        g: usize,
        u: i32,
        nom: i32,
        aire: i32,
        s: i32,
        name_too_long: bool,
    ) -> usize {
        if *INTPAR(IntPar::suppress_fontnotfound_error) == 0 {
            /* NOTE: must preserve this path to keep passing the TRIP tests */
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Font ");
            sprint_cs(u);
            print_char('=' as i32);
            if file_name_quote_char as i32 != 0i32 {
                print_char(file_name_quote_char as i32);
            }
            print_file_name(nom, aire, cur_ext);
            if file_name_quote_char as i32 != 0i32 {
                print_char(file_name_quote_char as i32);
            }
            if s >= 0 {
                print_cstr(b" at ");
                print_scaled(s);
                print_cstr(b"pt");
            } else if s != -1000 {
                print_cstr(b" scaled ");
                print_int(-s);
            }
            if tfm_file.is_some() {
                print_cstr(b" not loadable: Bad metric (TFM) file");
            } else if name_too_long {
                print_cstr(b" not loadable: Metric (TFM) file name too long");
            } else {
                print_cstr(b" not loadable: Metric (TFM) file or installed font not found");
            }
            help_ptr = 5_u8;
            help_line[4] = b"I wasn\'t able to read the size data for this font,";
            help_line[3] = b"so I will ignore the font specification.";
            help_line[2] = b"[Wizards can fix TFM files using TFtoPL/PLtoTF.]";
            help_line[1] = b"You might try inserting a different font spec;";
            help_line[0] = b"e.g., type `I\\font<same font id>=<substitute font name>\'.";
            error();
        }
        return done(tfm_file, g);
    }
    // unreachable
    // return bad_tfm(tfm_file_owner, g, u, nom, aire, s, name_too_long);

    unsafe fn done(tfm_file: Option<InputHandleWrapper>, g: usize) -> usize {
        let file_opened = tfm_file.is_some();
        if let Some(handle) = tfm_file {
            ttstub_input_close(handle);
        }

        if *INTPAR(IntPar::xetex_tracing_fonts) > 0 {
            if g == FONT_BASE as usize {
                begin_diagnostic();
                print_nl_cstr(b" -> font not found, using \"nullfont\"");
                end_diagnostic(false);
            } else if file_opened {
                begin_diagnostic();
                print_nl_cstr(b" -> ");
                print_c_string(name_of_file);
                end_diagnostic(false);
            }
        }
        g
    }
    // unreachable
    // return done(tfm_file_owner, g);
}
pub(crate) unsafe fn new_character(
    mut f: internal_font_number,
    mut c: UTF16_code,
) -> Option<usize> {
    let mut ec: u16 = 0;
    if FONT_AREA[f] as u32 == AAT_FONT_FLAG || FONT_AREA[f] as u32 == OTGR_FONT_FLAG {
        return Some(new_native_character(f, c as UnicodeScalar));
    }
    ec = effective_char(false, f, c) as u16;
    if FONT_BC[f] as i32 <= ec as i32 {
        if FONT_EC[f] as i32 >= ec as i32 {
            if FONT_INFO[(CHAR_BASE[f] + ec as i32) as usize].b16.s3 > 0 {
                let p = get_avail();
                MEM[p].b16.s1 = f as u16;
                MEM[p].b16.s0 = c;
                return Some(p);
            }
        }
    }
    char_warning(f, c as i32);
    None
}
pub(crate) unsafe fn scan_spec(c: GroupCode, mut three_codes: bool) {
    let mut current_block: u64;
    let mut s: i32 = 0;
    let mut spec_code: u8 = 0;
    if three_codes {
        s = SAVE_STACK[SAVE_PTR + 0].b32.s1
    }
    if scan_keyword(b"to") {
        spec_code = EXACTLY;
        current_block = 8515828400728868193;
    } else if scan_keyword(b"spread") {
        spec_code = ADDITIONAL;
        current_block = 8515828400728868193;
    } else {
        spec_code = ADDITIONAL;
        cur_val = 0;
        current_block = 4427475217998452135;
    }
    match current_block {
        8515828400728868193 => scan_dimen(false, false, false),
        _ => {}
    }
    if three_codes {
        SAVE_STACK[SAVE_PTR + 0].b32.s1 = s;
        SAVE_PTR += 1;
    }
    SAVE_STACK[SAVE_PTR + 0].b32.s1 = spec_code as i32;
    SAVE_STACK[SAVE_PTR + 1].b32.s1 = cur_val;
    SAVE_PTR += 2;
    new_save_level(c);
    scan_left_brace();
}
pub(crate) unsafe fn char_pw(mut p: i32, mut side: small_number) -> scaled_t {
    if side == 0 {
        last_leftmost_char = TEX_NULL
    } else {
        last_rightmost_char = TEX_NULL
    }
    if p.is_texnull() {
        return 0;
    }
    if !p.is_texnull()
        && !is_char_node(p)
        && *NODE_type(p as usize) == WHATSIT_NODE
        && (*NODE_subtype(p as usize) == NATIVE_WORD_NODE
            || *NODE_subtype(p as usize) == NATIVE_WORD_NODE_AT)
    {
        if !(*NATIVE_NODE_glyph_info_ptr(p as usize)).is_null() {
            let f = *NATIVE_NODE_font(p as usize) as internal_font_number;
            return round_xn_over_d(
                FONT_INFO[(QUAD_CODE + PARAM_BASE[f]) as usize].b32.s1,
                real_get_native_word_cp(
                    &mut MEM[p as usize] as *mut memory_word as *mut libc::c_void,
                    side as i32,
                ),
                1000,
            );
        } else {
            return 0;
        }
    }
    if !p.is_texnull()
        && !is_char_node(p)
        && *NODE_type(p as usize) == WHATSIT_NODE
        && MEM[p as usize].b16.s0 == GLYPH_NODE
    {
        let f = MEM[(p + 4) as usize].b16.s2 as internal_font_number;
        return round_xn_over_d(
            FONT_INFO[(QUAD_CODE + PARAM_BASE[f]) as usize].b32.s1,
            get_cp_code(f, MEM[(p + 4) as usize].b16.s1 as u32, side as i32),
            1000,
        );
    }
    if !is_char_node(p) {
        if *NODE_type(p as usize) == LIGATURE_NODE {
            p = p + 1;
        } else {
            return 0;
        }
    }
    let f = *CHAR_NODE_font(p as usize) as internal_font_number;
    let c = get_cp_code(f, MEM[p as usize].b16.s0 as u32, side as i32);
    match side as i32 {
        0 => last_leftmost_char = p,
        1 => last_rightmost_char = p,
        _ => {}
    }
    if c == 0 {
        return 0;
    }
    round_xn_over_d(
        FONT_INFO[(QUAD_CODE + PARAM_BASE[f]) as usize].b32.s1,
        c,
        1000i32,
    )
}
pub(crate) unsafe fn new_margin_kern(w: scaled_t, _p: i32, side: small_number) -> usize {
    let k = get_node(MARGIN_KERN_NODE_SIZE);
    MEM[k].b16.s1 = MARGIN_KERN_NODE;
    MEM[k].b16.s0 = side as u16;
    MEM[k + 1].b32.s1 = w;
    k
}
pub(crate) unsafe fn hpack(mut p: i32, mut w: scaled_t, mut m: small_number) -> i32 {
    let mut current_block: u64;
    let mut s: scaled_t = 0;
    let mut g: i32 = 0;
    let mut o: glue_ord = 0;
    let mut f: internal_font_number = 0;
    let mut i: b16x4 = b16x4 {
        s0: 0,
        s1: 0,
        s2: 0,
        s3: 0,
    };
    let mut pp: i32 = 0;
    let mut ppp: i32 = TEX_NULL;
    let mut total_chars: i32 = 0;
    last_badness = 0;
    let r = get_node(BOX_NODE_SIZE) as i32;
    MEM[r as usize].b16.s1 = 0_u16;
    MEM[r as usize].b16.s0 = 0_u16;
    MEM[(r + 4) as usize].b32.s1 = 0;
    let mut q = r + 5;
    MEM[q as usize].b32.s1 = p;
    let mut h = 0;
    let mut d = 0;
    let mut x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;
    if *INTPAR(IntPar::texxet) > 0 {
        /*1497: */
        temp_ptr = get_avail() as i32;
        MEM[temp_ptr as usize].b32.s0 = BEFORE as i32;
        MEM[temp_ptr as usize].b32.s1 = LR_ptr;
        LR_ptr = temp_ptr
    }
    's_130: while !p.is_texnull() {
        loop
        /*674: */
        {
            while is_char_node(p) {
                /*677: */
                f = *CHAR_NODE_font(p as usize) as internal_font_number;
                i = FONT_CHARACTER_INFO(
                    f,
                    effective_char(true, f, *CHAR_NODE_character(p as usize)) as usize,
                );
                x = x + *FONT_CHARINFO_WIDTH(f, i);
                s = *FONT_CHARINFO_HEIGHT(f, i);
                if s > h {
                    h = s;
                }
                s = *FONT_CHARINFO_DEPTH(f, i);
                if s > d {
                    d = s;
                }
                p = *LLIST_link(p as usize)
            }
            if p.is_texnull() {
                continue 's_130;
            }
            match MEM[p as usize].b16.s1 as i32 {
                0 | 1 | 2 | 13 => {
                    x = x + MEM[(p + 1) as usize].b32.s1;
                    if MEM[p as usize].b16.s1 >= RULE_NODE {
                        s = 0;
                    } else {
                        s = MEM[(p + 4) as usize].b32.s1
                    }
                    if MEM[(p + 3) as usize].b32.s1 - s > h {
                        h = MEM[(p + 3) as usize].b32.s1 - s
                    }
                    if MEM[(p + 2) as usize].b32.s1 + s > d {
                        d = MEM[(p + 2) as usize].b32.s1 + s
                    }
                    current_block = 1176253869785344635;
                    break;
                }
                3 | 4 | 5 => {
                    if !adjust_tail.is_texnull() || !pre_adjust_tail.is_texnull() {
                        /*680: */
                        while MEM[q as usize].b32.s1 != p {
                            q = MEM[q as usize].b32.s1
                        }
                        if MEM[p as usize].b16.s1 == ADJUST_NODE {
                            if MEM[p as usize].b16.s0 as i32 != 0 {
                                if pre_adjust_tail.is_texnull() {
                                    confusion(b"pre vadjust");
                                }
                                MEM[pre_adjust_tail as usize].b32.s1 = MEM[(p + 1) as usize].b32.s1;
                                while !MEM[pre_adjust_tail as usize].b32.s1.is_texnull() {
                                    pre_adjust_tail = MEM[pre_adjust_tail as usize].b32.s1
                                }
                            } else {
                                if adjust_tail.is_texnull() {
                                    confusion(b"pre vadjust");
                                }
                                MEM[adjust_tail as usize].b32.s1 = MEM[(p + 1) as usize].b32.s1;
                                while !MEM[adjust_tail as usize].b32.s1.is_texnull() {
                                    adjust_tail = MEM[adjust_tail as usize].b32.s1
                                }
                            }
                            p = *LLIST_link(p as usize);
                            free_node(MEM[q as usize].b32.s1 as usize, SMALL_NODE_SIZE);
                        } else {
                            MEM[adjust_tail as usize].b32.s1 = p;
                            adjust_tail = p;
                            p = *LLIST_link(p as usize)
                        }
                        MEM[q as usize].b32.s1 = p;
                        p = q
                    }
                    current_block = 1176253869785344635;
                    break;
                }
                8 => match MEM[p as usize].b16.s0 as i32 {
                    40 | 41 => {
                        current_block = 10435735846551762309;
                        break;
                    }
                    42 | 43 | 44 => {
                        current_block = 9371553318591620115;
                        break;
                    }
                    _ => {
                        current_block = 1176253869785344635;
                        break;
                    }
                },
                10 => {
                    g = MEM[(p + 1) as usize].b32.s0;
                    x = x + MEM[(g + 1) as usize].b32.s1;
                    o = MEM[g as usize].b16.s1 as glue_ord;
                    total_stretch[o as usize] =
                        total_stretch[o as usize] + MEM[(g + 2) as usize].b32.s1;
                    o = MEM[g as usize].b16.s0 as glue_ord;
                    total_shrink[o as usize] =
                        total_shrink[o as usize] + MEM[(g + 3) as usize].b32.s1;
                    if MEM[p as usize].b16.s0 >= A_LEADERS {
                        g = MEM[(p + 1) as usize].b32.s1;
                        if MEM[(g + 3) as usize].b32.s1 > h {
                            h = MEM[(g + 3) as usize].b32.s1
                        }
                        if MEM[(g + 2) as usize].b32.s1 > d {
                            d = MEM[(g + 2) as usize].b32.s1
                        }
                    }
                    current_block = 1176253869785344635;
                    break;
                }
                11 => {
                    x = x + MEM[(p + 1) as usize].b32.s1;
                    current_block = 1176253869785344635;
                    break;
                }
                40 => {
                    x = x + MEM[(p + 1) as usize].b32.s1;
                    current_block = 1176253869785344635;
                    break;
                }
                9 => {
                    x = x + MEM[(p + 1) as usize].b32.s1;
                    if *INTPAR(IntPar::texxet) > 0 {
                        /*1498: */
                        if MEM[p as usize].b16.s0 as i32 & 1 != 0 {
                            if MEM[LR_ptr as usize].b32.s0
                                == (L_CODE as i32) * (MEM[p as usize].b16.s0 as i32 / 4) + 3
                            {
                                temp_ptr = LR_ptr; /*689: */
                                LR_ptr = MEM[temp_ptr as usize].b32.s1;
                                MEM[temp_ptr as usize].b32.s1 = avail;
                                avail = temp_ptr
                            } else {
                                LR_problems += 1;
                                MEM[p as usize].b16.s1 = KERN_NODE;
                                MEM[p as usize].b16.s0 = EXPLICIT;
                            }
                        } else {
                            temp_ptr = get_avail() as i32;
                            MEM[temp_ptr as usize].b32.s0 =
                                (L_CODE as i32) * (MEM[p as usize].b16.s0 as i32 / 4) + 3;
                            MEM[temp_ptr as usize].b32.s1 = LR_ptr;
                            LR_ptr = temp_ptr;
                        }
                    }
                    current_block = 1176253869785344635;
                    break;
                }
                6 => {
                    MEM[GARBAGE] = MEM[(p + 1) as usize];
                    MEM[GARBAGE].b32.s1 = MEM[p as usize].b32.s1;
                    p = GARBAGE as i32;
                    xtx_ligature_present = true;
                }
                _ => {
                    current_block = 1176253869785344635;
                    break;
                }
            }
        }
        match current_block {
            10435735846551762309 => {
                let mut k = if q != r + 5 && *NODE_type(q as usize) == DISC_NODE {
                    MEM[q as usize].b16.s0 as i32
                } else {
                    0
                };
                while MEM[q as usize].b32.s1 != p {
                    k -= 1;
                    q = MEM[q as usize].b32.s1;
                    if *NODE_type(q as usize) == DISC_NODE {
                        k = MEM[q as usize].b16.s0 as i32
                    }
                }
                pp = *LLIST_link(p as usize);
                while k <= 0 && !pp.is_texnull() && !is_char_node(pp) {
                    if *NODE_type(pp as usize) == WHATSIT_NODE
                        && (MEM[pp as usize].b16.s0 == NATIVE_WORD_NODE
                            || MEM[pp as usize].b16.s0 == NATIVE_WORD_NODE_AT)
                        && MEM[(pp + 4) as usize].b16.s2 as i32
                            == MEM[(p + 4) as usize].b16.s2 as i32
                    {
                        pp = MEM[pp as usize].b32.s1
                    } else {
                        if !(*NODE_type(pp as usize) == DISC_NODE) {
                            break;
                        }
                        ppp = MEM[pp as usize].b32.s1;
                        if !(!ppp.is_texnull()
                            && !is_char_node(ppp)
                            && *NODE_type(ppp as usize) == WHATSIT_NODE
                            && (MEM[ppp as usize].b16.s0 == NATIVE_WORD_NODE
                                || MEM[ppp as usize].b16.s0 == NATIVE_WORD_NODE_AT)
                            && MEM[(ppp + 4) as usize].b16.s2 as i32
                                == MEM[(p + 4) as usize].b16.s2 as i32)
                        {
                            break;
                        }
                        pp = MEM[ppp as usize].b32.s1
                    }
                }
                if pp != MEM[p as usize].b32.s1 {
                    total_chars = 0;
                    p = MEM[q as usize].b32.s1;
                    while p != pp {
                        if *NODE_type(p as usize) == WHATSIT_NODE {
                            total_chars = total_chars + MEM[(p + 4) as usize].b16.s1 as i32
                        }
                        ppp = p;
                        p = *LLIST_link(p as usize)
                    }
                    p = MEM[q as usize].b32.s1;
                    pp = new_native_word_node(
                        MEM[(p + 4) as usize].b16.s2 as internal_font_number,
                        total_chars,
                    ) as i32;
                    MEM[pp as usize].b16.s0 = MEM[p as usize].b16.s0;
                    MEM[q as usize].b32.s1 = pp;
                    MEM[pp as usize].b32.s1 = MEM[ppp as usize].b32.s1;
                    MEM[ppp as usize].b32.s1 = TEX_NULL;
                    total_chars = 0;
                    ppp = p;
                    loop {
                        if *NODE_type(ppp as usize) == WHATSIT_NODE {
                            let mut for_end: i32 = 0;
                            let mut k = 0;
                            for_end = MEM[(ppp + 4) as usize].b16.s1 as i32 - 1;
                            if k <= for_end {
                                loop {
                                    *(&mut MEM[(pp + 6) as usize] as *mut memory_word
                                        as *mut u16)
                                        .offset(total_chars as isize) =
                                        *(&mut MEM[(ppp + 6) as usize] as *mut memory_word
                                            as *mut u16)
                                            .offset(k as isize);
                                    total_chars += 1;
                                    if !(k < for_end) {
                                        break;
                                    }
                                    k = k + 1;
                                }
                            }
                        }
                        ppp = MEM[ppp as usize].b32.s1;
                        if ppp.is_texnull() {
                            break;
                        }
                    }
                    flush_node_list(p);
                    p = MEM[q as usize].b32.s1;
                    measure_native_node(
                        &mut MEM[p as usize] as *mut memory_word as *mut libc::c_void,
                        (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0i32) as i32,
                    );
                }
                if MEM[(p + 3) as usize].b32.s1 > h {
                    h = MEM[(p + 3) as usize].b32.s1
                }
                if MEM[(p + 2) as usize].b32.s1 > d {
                    d = MEM[(p + 2) as usize].b32.s1
                }
                x = x + MEM[(p + 1) as usize].b32.s1
            }
            9371553318591620115 => {
                if MEM[(p + 3) as usize].b32.s1 > h {
                    h = MEM[(p + 3) as usize].b32.s1
                }
                if MEM[(p + 2) as usize].b32.s1 > d {
                    d = MEM[(p + 2) as usize].b32.s1
                }
                x = x + MEM[(p + 1) as usize].b32.s1
            }
            _ => {}
        }
        p = *LLIST_link(p as usize)
    }
    if !adjust_tail.is_texnull() {
        MEM[adjust_tail as usize].b32.s1 = TEX_NULL;
    }
    if !pre_adjust_tail.is_texnull() {
        MEM[pre_adjust_tail as usize].b32.s1 = TEX_NULL;
    }
    MEM[(r + 3) as usize].b32.s1 = h;
    MEM[(r + 2) as usize].b32.s1 = d;
    if m == ADDITIONAL as i16 {
        w = x + w;
    }
    MEM[(r + 1) as usize].b32.s1 = w;
    x = w - x;
    if x == 0 {
        MEM[(r + 5) as usize].b16.s1 = NORMAL;
        MEM[(r + 5) as usize].b16.s0 = NORMAL;
        *BOX_glue_set(r as usize) = 0.;
        current_block = 2380354494544673732;
    } else if x > 0 {
        /*683: */
        if total_stretch[FILLL as usize] != 0 {
            o = FILLL as glue_ord
        } else if total_stretch[FILL as usize] != 0 {
            o = FILL as glue_ord
        } else if total_stretch[FIL as usize] != 0 {
            o = FIL as glue_ord
        } else {
            o = 0 as glue_ord
        } /*normal *//*:684 */
        MEM[(r + 5) as usize].b16.s0 = o as u16;
        MEM[(r + 5) as usize].b16.s1 = STRETCHING;
        if total_stretch[o as usize] != 0 {
            MEM[(r + 6) as usize].gr = x as f64 / total_stretch[o as usize] as f64
        } else {
            MEM[(r + 5) as usize].b16.s1 = NORMAL;
            MEM[(r + 6) as usize].gr = 0.;
        }
        if o == NORMAL as u8 {
            if !MEM[(r + 5) as usize].b32.s1.is_texnull() {
                /*685: */
                last_badness = badness(x, total_stretch[NORMAL as usize]); /*normal *//*:690 */
                if last_badness > *INTPAR(IntPar::hbadness) {
                    print_ln();
                    if last_badness > 100 {
                        print_nl_cstr(b"Underfull");
                    } else {
                        print_nl_cstr(b"Loose");
                    }
                    print_cstr(b" \\hbox (badness ");
                    print_int(last_badness);
                    current_block = 13814253595362444008;
                } else {
                    current_block = 2380354494544673732;
                }
            } else {
                current_block = 2380354494544673732;
            }
        } else {
            current_block = 2380354494544673732;
        }
    } else {
        if total_shrink[FILLL as usize] != 0 {
            o = FILLL as glue_ord
        } else if total_shrink[FILL as usize] != 0 {
            o = FILL as glue_ord
        } else if total_shrink[FIL as usize] != 0 {
            o = FIL as glue_ord
        } else {
            o = 0 as glue_ord
        }
        MEM[(r + 5) as usize].b16.s0 = o as u16;
        MEM[(r + 5) as usize].b16.s1 = SHRINKING;
        if total_shrink[o as usize] != 0 {
            *BOX_glue_set(r as usize) = -x as f64 / total_shrink[o as usize] as f64
        } else {
            MEM[(r + 5) as usize].b16.s1 = NORMAL;
            *BOX_glue_set(r as usize) = 0.;
        }
        if total_shrink[o as usize] < -x
            && o == NORMAL as u8
            && !MEM[(r + 5) as usize].b32.s1.is_texnull()
        {
            last_badness = 1000000;
            MEM[(r + 6) as usize].gr = 1.;
            if -x - total_shrink[0] > *DIMENPAR(DimenPar::hfuzz) || *INTPAR(IntPar::hbadness) < 100
            {
                if *DIMENPAR(DimenPar::overfull_rule) > 0
                    && -x - total_shrink[0] > *DIMENPAR(DimenPar::hfuzz)
                {
                    while !MEM[q as usize].b32.s1.is_texnull() {
                        q = MEM[q as usize].b32.s1
                    }
                    MEM[q as usize].b32.s1 = new_rule() as i32;
                    MEM[(MEM[q as usize].b32.s1 + 1) as usize].b32.s1 =
                        *DIMENPAR(DimenPar::overfull_rule)
                }
                print_ln();
                print_nl_cstr(b"Overfull \\hbox (");
                print_scaled(-x - total_shrink[NORMAL as usize]);
                print_cstr(b"pt too wide");
                current_block = 13814253595362444008;
            } else {
                current_block = 2380354494544673732;
            }
        } else if o == NORMAL as u8 {
            if !MEM[(r + 5) as usize].b32.s1.is_texnull() {
                /*692: */
                last_badness = badness(-x, total_shrink[NORMAL as usize]);
                if last_badness > *INTPAR(IntPar::hbadness) {
                    print_ln();
                    print_nl_cstr(b"Tight \\hbox (badness ");
                    print_int(last_badness);
                    current_block = 13814253595362444008;
                } else {
                    current_block = 2380354494544673732;
                }
            } else {
                current_block = 2380354494544673732;
            }
        } else {
            current_block = 2380354494544673732;
        }
    }
    loop {
        match current_block {
            13814253595362444008 => {
                if output_active {
                    print_cstr(b") has occurred while \\output is active");
                } else {
                    if pack_begin_line != 0i32 {
                        if pack_begin_line > 0i32 {
                            print_cstr(b") in paragraph at lines ");
                        } else {
                            print_cstr(b") in alignment at lines ");
                        }
                        print_int(pack_begin_line.abs());
                        print_cstr(b"--");
                    } else {
                        print_cstr(b") detected at line ");
                    }
                    print_int(line);
                }
                print_ln();
                font_in_short_display = 0i32;
                short_display(MEM[(r + 5) as usize].b32.s1);
                print_ln();
                begin_diagnostic();
                show_box(r);
                end_diagnostic(true);
                current_block = 2380354494544673732;
            }
            _ => {
                if !(*INTPAR(IntPar::texxet) > 0) {
                    break;
                }
                /*1499: */
                if MEM[LR_ptr as usize].b32.s0 != BEFORE as i32 {
                    while !MEM[q as usize].b32.s1.is_texnull() {
                        q = *LLIST_link(q as usize)
                    } /*:673 */
                    loop {
                        temp_ptr = q;
                        q = new_math(0, MEM[LR_ptr as usize].b32.s0 as small_number) as i32;
                        MEM[temp_ptr as usize].b32.s1 = q;
                        LR_problems = LR_problems + 10000;
                        temp_ptr = LR_ptr;
                        LR_ptr = MEM[temp_ptr as usize].b32.s1;
                        MEM[temp_ptr as usize].b32.s1 = avail;
                        avail = temp_ptr;
                        if MEM[LR_ptr as usize].b32.s0 == BEFORE as i32 {
                            break;
                        }
                    }
                }
                if LR_problems > 0 {
                    print_ln();
                    print_nl_cstr(b"\\endL or \\endR problem (");
                    print_int(LR_problems / 10000);
                    print_cstr(b" missing, ");
                    print_int(LR_problems % 10000);
                    print_cstr(b" extra");
                    LR_problems = 0;
                    current_block = 13814253595362444008;
                } else {
                    temp_ptr = LR_ptr;
                    LR_ptr = MEM[temp_ptr as usize].b32.s1;
                    MEM[temp_ptr as usize].b32.s1 = avail;
                    avail = temp_ptr;
                    if !LR_ptr.is_texnull() {
                        confusion(b"LR1");
                    }
                    break;
                }
            }
        }
    }
    r
}
pub(crate) unsafe fn vpackage(
    mut p: i32,
    mut h: scaled_t,
    mut m: small_number,
    mut l: scaled_t,
) -> usize {
    let mut current_block: u64;
    last_badness = 0;
    let r = get_node(BOX_NODE_SIZE) as usize;
    *NODE_type(r) = VLIST_NODE;
    if *INTPAR(IntPar::xetex_upwards) > 0i32 {
        MEM[r].b16.s0 = 1;
    } else {
        MEM[r].b16.s0 = 0;
    }
    MEM[r + 4].b32.s1 = 0;
    MEM[r + 5].b32.s1 = p;
    let mut w = 0;
    let mut d = 0;
    let mut x = 0;
    total_stretch[NORMAL as usize] = 0;
    total_shrink[NORMAL as usize] = 0;
    total_stretch[FIL as usize] = 0;
    total_shrink[FIL as usize] = 0;
    total_stretch[FILL as usize] = 0;
    total_shrink[FILL as usize] = 0;
    total_stretch[FILLL as usize] = 0;
    total_shrink[FILLL as usize] = 0;
    while !p.is_texnull() {
        /*694: */
        if is_char_node(p) {
            confusion(b"vpack"); /*701: */
        } else {
            let p = p as usize;
            match MEM[p].b16.s1 as i32 {
                0 | 1 | 2 | 13 => {
                    x = x + d + MEM[p + 3].b32.s1;
                    d = MEM[p + 2].b32.s1;
                    let s = if *NODE_type(p) >= RULE_NODE {
                        0
                    } else {
                        MEM[p + 4].b32.s1
                    };
                    if MEM[p + 1].b32.s1 + s > w {
                        w = MEM[p + 1].b32.s1 + s
                    }
                }
                8 => {
                    if MEM[p].b16.s0 == PIC_NODE || MEM[p].b16.s0 == PDF_NODE {
                        x = x + d + MEM[p + 3].b32.s1;
                        d = MEM[p + 2].b32.s1;
                        if MEM[p + 1].b32.s1 > w {
                            w = MEM[p + 1].b32.s1
                        }
                    }
                }
                10 => {
                    x = x + d;
                    d = 0;
                    let g = MEM[p + 1].b32.s0 as usize;
                    x = x + MEM[g + 1].b32.s1;
                    let o = MEM[g].b16.s1 as glue_ord as usize;
                    total_stretch[o as usize] = total_stretch[o as usize] + MEM[g + 2].b32.s1;
                    let o = MEM[g].b16.s0 as glue_ord as usize;
                    total_shrink[o as usize] = total_shrink[o as usize] + MEM[g + 3].b32.s1;
                    if MEM[p].b16.s0 >= A_LEADERS {
                        let g = MEM[p + 1].b32.s1 as usize;
                        if MEM[g + 1].b32.s1 > w {
                            w = MEM[g + 1].b32.s1
                        }
                    }
                }
                11 => {
                    x = x + d + MEM[p + 1].b32.s1;
                    d = 0;
                }
                _ => {}
            }
        }
        p = *LLIST_link(p as usize)
    }
    MEM[r + 1].b32.s1 = w;
    if d > l {
        x = x + d - l;
        MEM[r + 2].b32.s1 = l
    } else {
        MEM[r + 2].b32.s1 = d
    }
    if m == ADDITIONAL as i16 {
        h = x + h;
    }
    MEM[r + 3].b32.s1 = h;
    x = h - x;
    if x == 0 {
        MEM[r + 5].b16.s1 = NORMAL;
        MEM[r + 5].b16.s0 = NORMAL;
        MEM[r + 6].gr = 0.;
    } else {
        if x > 0 {
            /*698: */
            let o = if total_stretch[FILLL as usize] != 0 {
                FILLL as glue_ord
            } else if total_stretch[FILL as usize] != 0 {
                FILL as glue_ord
            } else if total_stretch[FIL as usize] != 0 {
                FIL as glue_ord
            } else {
                0 as glue_ord
            }; /*normal *//*:684 */
            MEM[r + 5].b16.s0 = o as u16;
            MEM[r + 5].b16.s1 = STRETCHING;
            if total_stretch[o as usize] != 0 {
                MEM[r + 6].gr = x as f64 / total_stretch[o as usize] as f64
            } else {
                MEM[r + 5].b16.s1 = NORMAL;
                *BOX_glue_set(r) = 0.;
            }
            if o == NORMAL as u8 {
                if !MEM[r + 5].b32.s1.is_texnull() {
                    /*699: */
                    last_badness = badness(x, total_stretch[NORMAL as usize]); /*normal *//*:690 */
                    if last_badness > *INTPAR(IntPar::vbadness) {
                        print_ln();
                        if last_badness > 100 {
                            print_nl_cstr(b"Underfull");
                        } else {
                            print_nl_cstr(b"Loose");
                        }
                        print_cstr(b" \\vbox (badness ");
                        print_int(last_badness);
                        current_block = 13130523023485106979;
                    } else {
                        current_block = 13281346226780081721;
                    }
                } else {
                    current_block = 13281346226780081721;
                }
            } else {
                current_block = 13281346226780081721;
            }
        } else {
            let o = if total_shrink[FILLL as usize] != 0 {
                FILLL as glue_ord
            } else if total_shrink[FILL as usize] != 0 {
                FILL as glue_ord
            } else if total_shrink[FIL as usize] != 0 {
                FIL as glue_ord
            } else {
                0 as glue_ord
            };
            MEM[r + 5].b16.s0 = o as u16;
            MEM[r + 5].b16.s1 = SHRINKING;
            if total_shrink[o as usize] != 0 {
                MEM[r + 6].gr = -x as f64 / total_shrink[o as usize] as f64
            } else {
                MEM[r + 5].b16.s1 = NORMAL;
                *BOX_glue_set(r) = 0.;
            }
            if total_shrink[o as usize] < -x && o == NORMAL as u8 && !MEM[r + 5].b32.s1.is_texnull()
            {
                last_badness = 1000000;
                *BOX_glue_set(r) = 1.;
                if -x - total_shrink[0] > *DIMENPAR(DimenPar::vfuzz)
                    || *INTPAR(IntPar::vbadness) < 100
                {
                    print_ln();
                    print_nl_cstr(b"Overfull \\vbox (");
                    print_scaled(-x - total_shrink[NORMAL as usize]);
                    print_cstr(b"pt too high");
                    current_block = 13130523023485106979;
                } else {
                    current_block = 13281346226780081721;
                }
            } else if o == NORMAL as u8 {
                if !MEM[r + 5].b32.s1.is_texnull() {
                    /*703: */
                    last_badness = badness(-x, total_shrink[NORMAL as usize]);
                    if last_badness > *INTPAR(IntPar::vbadness) {
                        print_ln();
                        print_nl_cstr(b"Tight \\vbox (badness ");
                        print_int(last_badness);
                        current_block = 13130523023485106979;
                    } else {
                        current_block = 13281346226780081721;
                    }
                } else {
                    current_block = 13281346226780081721;
                }
            } else {
                current_block = 13281346226780081721;
            }
        }
        match current_block {
            13281346226780081721 => {}
            _ => {
                if output_active {
                    print_cstr(b") has occurred while \\output is active");
                } else {
                    if pack_begin_line != 0 {
                        print_cstr(b") in alignment at lines ");
                        print_int(pack_begin_line.abs());
                        print_cstr(b"--");
                    } else {
                        print_cstr(b") detected at line ");
                    }
                    print_int(line);
                    print_ln();
                }
                begin_diagnostic();
                show_box(r as i32);
                end_diagnostic(true);
            }
        }
    }
    r
}
pub(crate) unsafe fn append_to_vlist(b: usize) {
    let mut d: scaled_t = 0;
    let mut p: i32 = 0;
    let mut upwards: bool = false;
    upwards = *INTPAR(IntPar::xetex_upwards) > 0;
    if cur_list.aux.b32.s1 > IGNORE_DEPTH {
        if upwards {
            d = MEM[(*GLUEPAR(GluePar::baseline_skip) + 1) as usize].b32.s1
                - cur_list.aux.b32.s1
                - MEM[b + 2].b32.s1
        } else {
            d = MEM[(*GLUEPAR(GluePar::baseline_skip) + 1) as usize].b32.s1
                - cur_list.aux.b32.s1
                - MEM[b + 3].b32.s1
        }
        if d < *DIMENPAR(DimenPar::line_skip_limit) {
            p = new_param_glue(GluePar::line_skip as small_number) as i32;
        } else {
            p = new_skip_param(GluePar::baseline_skip as small_number) as i32;
            MEM[(temp_ptr + 1) as usize].b32.s1 = d
        }
        MEM[cur_list.tail].b32.s1 = p;
        cur_list.tail = p as usize;
    }
    MEM[cur_list.tail].b32.s1 = b as i32;
    cur_list.tail = b;
    if upwards {
        cur_list.aux.b32.s1 = MEM[b + 3].b32.s1
    } else {
        cur_list.aux.b32.s1 = MEM[b + 2].b32.s1
    };
}
pub(crate) unsafe fn new_noad() -> usize {
    let p = get_node(NOAD_SIZE);
    MEM[p].b16.s1 = ORD_NOAD;
    MEM[p].b16.s0 = NORMAL;
    MEM[p + 1].b32 = empty;
    MEM[p + 3].b32 = empty;
    MEM[p + 2].b32 = empty;
    p
}
pub(crate) unsafe fn new_style(mut s: small_number) -> usize {
    let p = get_node(STYLE_NODE_SIZE);
    MEM[p].b16.s1 = STYLE_NODE;
    MEM[p].b16.s0 = s as u16;
    MEM[p + 1].b32.s1 = 0;
    MEM[p + 2].b32.s1 = 0;
    p
}
pub(crate) unsafe fn new_choice() -> usize {
    let p = get_node(STYLE_NODE_SIZE);
    MEM[p].b16.s1 = CHOICE_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s0 = TEX_NULL;
    MEM[p + 1].b32.s1 = TEX_NULL;
    MEM[p + 2].b32.s0 = TEX_NULL;
    MEM[p + 2].b32.s1 = TEX_NULL;
    p
}
pub(crate) unsafe fn show_info() {
    show_node_list(MEM[temp_ptr as usize].b32.s0);
}
pub(crate) unsafe fn push_alignment() {
    let p = get_node(ALIGN_STACK_NODE_SIZE);
    MEM[p].b32.s1 = align_ptr;
    MEM[p].b32.s0 = cur_align;
    MEM[p + 1].b32.s0 = MEM[ALIGN_HEAD].b32.s1;
    MEM[p + 1].b32.s1 = cur_span;
    MEM[p + 2].b32.s1 = cur_loop;
    MEM[p + 3].b32.s1 = align_state;
    MEM[p + 4].b32.s0 = cur_head;
    MEM[p + 4].b32.s1 = cur_tail;
    MEM[p + 5].b32.s0 = cur_pre_head;
    MEM[p + 5].b32.s1 = cur_pre_tail;
    align_ptr = p as i32;
    cur_head = get_avail() as i32;
    cur_pre_head = get_avail() as i32;
}
pub(crate) unsafe fn pop_alignment() {
    MEM[cur_head as usize].b32.s1 = avail;
    avail = cur_head;
    MEM[cur_pre_head as usize].b32.s1 = avail;
    avail = cur_pre_head;
    let p = align_ptr as usize;
    cur_tail = MEM[p + 4].b32.s1;
    cur_head = MEM[p + 4].b32.s0;
    cur_pre_tail = MEM[p + 5].b32.s1;
    cur_pre_head = MEM[p + 5].b32.s0;
    align_state = MEM[p + 3].b32.s1;
    cur_loop = MEM[p + 2].b32.s1;
    cur_span = MEM[p + 1].b32.s1;
    MEM[ALIGN_HEAD].b32.s1 = MEM[p + 1].b32.s0;
    cur_align = MEM[p].b32.s0;
    align_ptr = MEM[p].b32.s1;
    free_node(p, ALIGN_STACK_NODE_SIZE);
}
pub(crate) unsafe fn get_preamble_token() {
    loop {
        get_token();
        while cur_chr == SPAN_CODE && cur_cmd == TAB_MARK as u8 {
            get_token();
            if cur_cmd > MAX_COMMAND {
                expand();
                get_token();
            }
        }
        if cur_cmd == ENDV as u8 {
            fatal_error(b"(interwoven alignment preambles are not allowed)");
        }
        if !(cur_cmd == ASSIGN_GLUE as u8 && cur_chr == GLUE_BASE as i32 + GluePar::tab_skip as i32)
        {
            break;
        }
        scan_optional_equals();
        scan_glue(GLUE_VAL as small_number);
        if *INTPAR(IntPar::global_defs) > 0 {
            geq_define(
                (GLUE_BASE as usize) + (GluePar::tab_skip as usize),
                GLUE_REF,
                cur_val,
            );
        } else {
            eq_define(
                (GLUE_BASE as usize) + (GluePar::tab_skip as usize),
                GLUE_REF,
                cur_val,
            );
        }
    }
}
pub(crate) unsafe fn init_align() {
    let mut save_cs_ptr: i32 = 0;
    let mut p: i32 = 0;
    save_cs_ptr = cur_cs;
    push_alignment();
    align_state = -1000000;
    if cur_list.mode == MMODE
        && (cur_list.tail != cur_list.head || !cur_list.aux.b32.s1.is_texnull())
    {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Improper ");
        print_esc_cstr(b"halign");
        print_cstr(b" inside $$\'s");
        help_ptr = 3;
        help_line[2] = b"Displays can use special alignments (like \\eqalignno)";
        help_line[1] = b"only if nothing but the alignment itself is between $$\'s.";
        help_line[0] = b"So I\'ve deleted the formulas that preceded this alignment.";
        error();
        flush_math();
    }
    push_nest();
    if cur_list.mode == MMODE {
        cur_list.mode = -1;
        cur_list.aux.b32.s1 = NEST[NEST_PTR - 2].aux.b32.s1
    } else if cur_list.mode > 0 {
        cur_list.mode = -(cur_list.mode as i32) as i16
        /*:804*/
    }
    scan_spec(GroupCode::ALIGN, false);
    MEM[ALIGN_HEAD].b32.s1 = TEX_NULL;
    cur_align = ALIGN_HEAD as i32;
    cur_loop = TEX_NULL;
    scanner_status = ALIGNING as u8;
    warning_index = save_cs_ptr;
    align_state = -1000000;
    loop {
        MEM[cur_align as usize].b32.s1 = new_param_glue(GluePar::tab_skip as small_number) as i32;
        /*:808 */
        cur_align = *LLIST_link(cur_align as usize); /*:807*/
        if cur_cmd == CAR_RET as u8 {
            break; /*:813*/
        } /*:806 */
        p = HOLD_HEAD as i32;
        MEM[p as usize].b32.s1 = TEX_NULL;
        loop {
            get_preamble_token();
            if cur_cmd == MAC_PARAM as u8 {
                break;
            }
            if cur_cmd <= CAR_RET as u8
                && cur_cmd >= TAB_MARK as u8
                && align_state as i64 == -1000000
            {
                if p == HOLD_HEAD as i32 && cur_loop.is_texnull() && cur_cmd == TAB_MARK as u8 {
                    cur_loop = cur_align
                } else {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Missing # inserted in alignment preamble");
                    help_ptr = 3;
                    help_line[2] = b"There should be exactly one # between &\'s, when an";
                    help_line[1] = b"\\halign or \\valign is being set up. In this case you had";
                    help_line[0] = b"none, so I\'ve put one in; maybe that will work.";
                    back_error();
                    break;
                }
            } else if cur_cmd != SPACER as u8 || p != HOLD_HEAD as i32 {
                MEM[p as usize].b32.s1 = get_avail() as i32;
                p = *LLIST_link(p as usize);
                MEM[p as usize].b32.s0 = cur_tok;
            }
        }
        MEM[cur_align as usize].b32.s1 = new_null_box() as i32;
        cur_align = *LLIST_link(cur_align as usize);
        MEM[cur_align as usize].b32.s0 = END_SPAN as i32;
        MEM[(cur_align + 1) as usize].b32.s1 = NULL_FLAG;
        MEM[(cur_align + 3) as usize].b32.s1 = MEM[HOLD_HEAD].b32.s1;
        p = HOLD_HEAD as i32;
        MEM[p as usize].b32.s1 = TEX_NULL;
        loop {
            get_preamble_token();
            if cur_cmd <= CAR_RET as u8
                && cur_cmd >= TAB_MARK as u8
                && align_state as i64 == -1000000
            {
                break;
            }
            if cur_cmd == MAC_PARAM as u8 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Only one # is allowed per tab");
                help_ptr = 3;
                help_line[2] = b"There should be exactly one # between &\'s, when an";
                help_line[1] = b"\\halign or \\valign is being set up. In this case you had";
                help_line[0] = b"more than one, so I\'m ignoring all but the first.";
                error();
            } else {
                MEM[p as usize].b32.s1 = get_avail() as i32;
                p = *LLIST_link(p as usize);
                MEM[p as usize].b32.s0 = cur_tok;
            }
        }
        MEM[p as usize].b32.s1 = get_avail() as i32;
        p = *LLIST_link(p as usize);
        MEM[p as usize].b32.s0 = CS_TOKEN_FLAG + FROZEN_END_TEMPLATE as i32;
        MEM[(cur_align + 2) as usize].b32.s1 = MEM[HOLD_HEAD].b32.s1
    }
    scanner_status = NORMAL as u8;
    new_save_level(GroupCode::ALIGN);
    if !LOCAL(Local::every_cr).is_texnull() {
        begin_token_list(*LOCAL(Local::every_cr) as usize, EVERY_CR_TEXT);
    }
    align_peek();
}
pub(crate) unsafe fn init_span(p: i32) {
    push_nest();
    if cur_list.mode as i32 == -104 {
        cur_list.aux.b32.s0 = 1000;
    } else {
        cur_list.aux.b32.s1 = IGNORE_DEPTH;
        normal_paragraph();
    }
    cur_span = p;
}
pub(crate) unsafe fn init_row() {
    push_nest();
    cur_list.mode = (-105 - cur_list.mode as i32) as i16;
    if cur_list.mode as i32 == -104 {
        cur_list.aux.b32.s0 = 0;
    } else {
        cur_list.aux.b32.s1 = 0;
    }
    MEM[cur_list.tail].b32.s1 = new_glue(MEM[(MEM[ALIGN_HEAD].b32.s1 + 1) as usize].b32.s0) as i32;
    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    MEM[cur_list.tail].b16.s0 = GluePar::tab_skip as u16 + 1;
    cur_align = MEM[MEM[ALIGN_HEAD].b32.s1 as usize].b32.s1;
    cur_tail = cur_head;
    cur_pre_tail = cur_pre_head;
    init_span(cur_align);
}
pub(crate) unsafe fn init_col() {
    MEM[(cur_align + 5) as usize].b32.s0 = cur_cmd as i32;
    if cur_cmd == OMIT as u8 {
        align_state = 0;
    } else {
        back_input();
        begin_token_list(MEM[(cur_align + 3) as usize].b32.s1 as usize, U_TEMPLATE);
    };
}
pub(crate) unsafe fn fin_col() -> bool {
    let mut r: i32 = 0;
    let mut s: i32 = 0;
    let mut u: i32 = 0;
    let mut w: scaled_t = 0;
    let mut o: glue_ord = 0;
    let mut n: i32 = 0;
    if cur_align.is_texnull() {
        confusion(b"endv");
    }
    let mut q = MEM[cur_align as usize].b32.s1;
    if q.is_texnull() {
        confusion(b"endv");
    }
    if (align_state as i64) < 500000 {
        fatal_error(b"(interwoven alignment preambles are not allowed)");
    }
    let mut p = MEM[q as usize].b32.s1;
    if p.is_texnull() && MEM[(cur_align + 5) as usize].b32.s0 < CR_CODE {
        if !cur_loop.is_texnull() {
            /*822: */
            MEM[q as usize].b32.s1 = new_null_box() as i32; /*:823 */
            p = MEM[q as usize].b32.s1;
            MEM[p as usize].b32.s0 = END_SPAN as i32;
            MEM[(p + 1) as usize].b32.s1 = NULL_FLAG;
            cur_loop = MEM[cur_loop as usize].b32.s1;
            q = HOLD_HEAD as i32;
            r = MEM[(cur_loop + 3) as usize].b32.s1;
            while !r.is_texnull() {
                MEM[q as usize].b32.s1 = get_avail() as i32;
                q = *LLIST_link(q as usize);
                MEM[q as usize].b32.s0 = MEM[r as usize].b32.s0;
                r = *LLIST_link(r as usize);
            }
            MEM[q as usize].b32.s1 = TEX_NULL;
            MEM[(p + 3) as usize].b32.s1 = MEM[HOLD_HEAD].b32.s1;
            q = HOLD_HEAD as i32;
            r = MEM[(cur_loop + 2) as usize].b32.s1;
            while !r.is_texnull() {
                MEM[q as usize].b32.s1 = get_avail() as i32;
                q = *LLIST_link(q as usize);
                MEM[q as usize].b32.s0 = MEM[r as usize].b32.s0;
                r = *LLIST_link(r as usize);
            }
            MEM[q as usize].b32.s1 = TEX_NULL;
            MEM[(p + 2) as usize].b32.s1 = MEM[HOLD_HEAD].b32.s1;
            cur_loop = MEM[cur_loop as usize].b32.s1;
            MEM[p as usize].b32.s1 = new_glue(MEM[(cur_loop + 1) as usize].b32.s0) as i32;
        } else {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Extra alignment tab has been changed to ");
            print_esc_cstr(b"cr");
            help_ptr = 3;
            help_line[2] = b"You have given more \\span or & marks than there were";
            help_line[1] = b"in the preamble to the \\halign or \\valign now in progress.";
            help_line[0] = b"So I\'ll assume that you meant to type \\cr instead.";
            MEM[(cur_align + 5) as usize].b32.s0 = CR_CODE;
            error();
        }
    }
    if MEM[(cur_align + 5) as usize].b32.s0 != SPAN_CODE {
        unsave();
        new_save_level(GroupCode::ALIGN);
        if cur_list.mode == -104 {
            adjust_tail = cur_tail;
            pre_adjust_tail = cur_pre_tail;
            u = hpack(MEM[cur_list.head].b32.s1, 0, ADDITIONAL as small_number);
            w = MEM[(u + 1) as usize].b32.s1;
            cur_tail = adjust_tail;
            adjust_tail = TEX_NULL;
            cur_pre_tail = pre_adjust_tail;
            pre_adjust_tail = TEX_NULL
        } else {
            u = vpackage(MEM[cur_list.head].b32.s1, 0, ADDITIONAL as small_number, 0) as i32;
            w = MEM[(u + 3) as usize].b32.s1
        }
        n = 0;
        if cur_span != cur_align {
            /*827: */
            q = cur_span; /*normal *//*:684 */
            loop {
                n += 1; /*normal *//*:690 */
                q = MEM[MEM[q as usize].b32.s1 as usize].b32.s1; /*tab_skip_code 1 *//*:824 */
                if q == cur_align {
                    break;
                }
            }
            if n > u16::MAX as i32 {
                confusion(b"too many spans");
            }
            q = cur_span;
            while MEM[MEM[q as usize].b32.s0 as usize].b32.s1 < n {
                q = MEM[q as usize].b32.s0
            }
            if MEM[MEM[q as usize].b32.s0 as usize].b32.s1 > n {
                s = get_node(SPAN_NODE_SIZE) as i32;
                MEM[s as usize].b32.s0 = MEM[q as usize].b32.s0;
                MEM[s as usize].b32.s1 = n;
                MEM[q as usize].b32.s0 = s;
                MEM[(s + 1) as usize].b32.s1 = w
            } else if MEM[(MEM[q as usize].b32.s0 + 1) as usize].b32.s1 < w {
                MEM[(MEM[q as usize].b32.s0 + 1) as usize].b32.s1 = w
            }
        } else if w > MEM[(cur_align + 1) as usize].b32.s1 {
            MEM[(cur_align + 1) as usize].b32.s1 = w
        }
        *NODE_type(u as usize) = UNSET_NODE;
        MEM[u as usize].b16.s0 = n as u16;
        if total_stretch[FILLL as usize] != 0 {
            o = FILLL as glue_ord
        } else if total_stretch[FILL as usize] != 0 {
            o = FILL as glue_ord
        } else if total_stretch[FIL as usize] != 0 {
            o = FIL as glue_ord
        } else {
            o = 0 as glue_ord
        }
        MEM[(u + 5) as usize].b16.s0 = o as u16;
        MEM[(u + 6) as usize].b32.s1 = total_stretch[o as usize];
        if total_shrink[FILLL as usize] != 0 {
            o = FILLL as glue_ord
        } else if total_shrink[FILL as usize] != 0 {
            o = FILL as glue_ord
        } else if total_shrink[FIL as usize] != 0 {
            o = FIL as glue_ord
        } else {
            o = 0 as glue_ord
        }
        MEM[(u + 5) as usize].b16.s1 = o as u16;
        MEM[(u + 4) as usize].b32.s1 = total_shrink[o as usize];
        pop_nest();
        MEM[cur_list.tail].b32.s1 = u;
        cur_list.tail = u as usize;
        MEM[cur_list.tail].b32.s1 =
            new_glue(MEM[(MEM[cur_align as usize].b32.s1 + 1) as usize].b32.s0) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b16.s0 = 12;
        if MEM[(cur_align + 5) as usize].b32.s0 >= CR_CODE {
            return true;
        }
        init_span(p);
    }
    align_state = 1000000;
    loop {
        get_x_or_protected();
        if cur_cmd != SPACER as u8 {
            break;
        }
    }
    cur_align = p;
    init_col();
    false
}
pub(crate) unsafe fn fin_row() {
    let mut p: i32 = 0;
    if cur_list.mode as i32 == -104 {
        p = hpack(MEM[cur_list.head].b32.s1, 0, ADDITIONAL as small_number);
        pop_nest();
        if cur_pre_head != cur_pre_tail {
            MEM[cur_list.tail].b32.s1 = MEM[cur_pre_head as usize].b32.s1;
            cur_list.tail = cur_pre_tail as usize;
        }
        append_to_vlist(p as usize);
        if cur_head != cur_tail {
            MEM[cur_list.tail].b32.s1 = MEM[cur_head as usize].b32.s1;
            cur_list.tail = cur_tail as usize;
        }
    } else {
        p = vpackage(
            MEM[cur_list.head].b32.s1,
            0,
            ADDITIONAL as small_number,
            MAX_HALFWORD,
        ) as i32;
        pop_nest();
        MEM[cur_list.tail].b32.s1 = p;
        cur_list.tail = p as usize;
        cur_list.aux.b32.s0 = 1000i32
    }
    MEM[p as usize].b16.s1 = 13_u16;
    MEM[(p + 6) as usize].b32.s1 = 0;
    if !LOCAL(Local::every_cr).is_texnull() {
        begin_token_list(*LOCAL(Local::every_cr) as usize, EVERY_CR_TEXT);
    }
    align_peek();
}
pub(crate) unsafe fn fin_align() {
    let mut p: i32 = 0;
    let mut r: i32 = 0;
    let mut s: i32 = 0;
    let mut u: i32 = 0;
    let mut v: i32 = 0;
    let mut t: scaled_t = 0;
    let mut w: scaled_t = 0;
    let mut n: i32 = 0;
    let mut rule_save: scaled_t = 0;
    let mut aux_save: memory_word = memory_word {
        b32: b32x2 { s0: 0, s1: 0 },
    };
    if cur_group != GroupCode::ALIGN {
        confusion(b"align1");
    }
    unsave();
    if cur_group != GroupCode::ALIGN {
        confusion(b"align0");
    }
    unsave();
    let o = if NEST[(NEST_PTR - 1) as usize].mode == MMODE {
        *DIMENPAR(DimenPar::display_indent)
    } else {
        0
    };
    let mut q = MEM[MEM[ALIGN_HEAD].b32.s1 as usize].b32.s1;
    loop {
        flush_list(MEM[(q + 3) as usize].b32.s1.opt());
        flush_list(MEM[(q + 2) as usize].b32.s1.opt());
        p = MEM[MEM[q as usize].b32.s1 as usize].b32.s1;
        if MEM[(q + 1) as usize].b32.s1 == NULL_FLAG {
            /*831: */
            MEM[(q + 1) as usize].b32.s1 = 0;
            r = MEM[q as usize].b32.s1;
            s = MEM[(r + 1) as usize].b32.s0;
            if s != 0i32 {
                *GLUE_SPEC_ref_count(0) += 1;
                delete_glue_ref(s as usize);
                MEM[(r + 1) as usize].b32.s0 = 0
            }
        }
        if MEM[q as usize].b32.s0 != END_SPAN as i32 {
            /*832: */
            t = MEM[(q + 1) as usize].b32.s1
                + MEM[(MEM[(MEM[q as usize].b32.s1 + 1) as usize].b32.s0 + 1) as usize]
                    .b32
                    .s1; /*:833 */
            r = MEM[q as usize].b32.s0;
            s = END_SPAN as i32;
            MEM[s as usize].b32.s0 = p;
            n = 1;
            loop {
                MEM[(r + 1) as usize].b32.s1 = MEM[(r + 1) as usize].b32.s1 - t;
                u = MEM[r as usize].b32.s0;
                while MEM[r as usize].b32.s1 > n {
                    s = MEM[s as usize].b32.s0;
                    n = MEM[MEM[s as usize].b32.s0 as usize].b32.s1 + 1;
                }
                if MEM[r as usize].b32.s1 < n {
                    MEM[r as usize].b32.s0 = MEM[s as usize].b32.s0;
                    MEM[s as usize].b32.s0 = r;
                    MEM[r as usize].b32.s1 -= 1;
                    s = r
                } else {
                    if MEM[(r + 1) as usize].b32.s1
                        > MEM[(MEM[s as usize].b32.s0 + 1) as usize].b32.s1
                    {
                        MEM[(MEM[s as usize].b32.s0 + 1) as usize].b32.s1 =
                            MEM[(r + 1) as usize].b32.s1;
                    }
                    free_node(r as usize, SPAN_NODE_SIZE);
                }
                r = u;
                if r == END_SPAN as i32 {
                    break;
                }
            }
        }
        *NODE_type(q as usize) = UNSET_NODE;
        MEM[q as usize].b16.s0 = 0;
        MEM[(q + 3) as usize].b32.s1 = 0;
        MEM[(q + 2) as usize].b32.s1 = 0;
        MEM[(q + 5) as usize].b16.s0 = NORMAL;
        MEM[(q + 5) as usize].b16.s1 = NORMAL;
        MEM[(q + 6) as usize].b32.s1 = 0;
        MEM[(q + 4) as usize].b32.s1 = 0;
        q = p;
        if q.is_texnull() {
            break;
        }
    }
    SAVE_PTR -= 2;
    pack_begin_line = -cur_list.mode_line;
    if cur_list.mode == -1 {
        rule_save = *DIMENPAR(DimenPar::overfull_rule);
        *DIMENPAR(DimenPar::overfull_rule) = 0;
        p = hpack(
            MEM[ALIGN_HEAD].b32.s1,
            SAVE_STACK[SAVE_PTR + 1].b32.s1,
            SAVE_STACK[SAVE_PTR + 0].b32.s1 as small_number,
        );
        *DIMENPAR(DimenPar::overfull_rule) = rule_save
    } else {
        q = MEM[MEM[ALIGN_HEAD].b32.s1 as usize].b32.s1;
        loop {
            MEM[(q + 3) as usize].b32.s1 = MEM[(q + 1) as usize].b32.s1;
            MEM[(q + 1) as usize].b32.s1 = 0;
            q = MEM[MEM[q as usize].b32.s1 as usize].b32.s1;
            if q.is_texnull() {
                break;
            }
        }
        p = vpackage(
            MEM[ALIGN_HEAD].b32.s1,
            SAVE_STACK[SAVE_PTR + 1].b32.s1,
            SAVE_STACK[SAVE_PTR + 0].b32.s1 as small_number,
            MAX_HALFWORD,
        ) as i32;
        q = MEM[MEM[ALIGN_HEAD].b32.s1 as usize].b32.s1;
        loop {
            MEM[(q + 1) as usize].b32.s1 = MEM[(q + 3) as usize].b32.s1;
            MEM[(q + 3) as usize].b32.s1 = 0;
            q = MEM[MEM[q as usize].b32.s1 as usize].b32.s1;
            if q.is_texnull() {
                break;
            }
        }
    }
    pack_begin_line = 0;
    q = MEM[cur_list.head].b32.s1;
    s = cur_list.head as i32;
    while !q.is_texnull() {
        if !is_char_node(q) {
            if MEM[q as usize].b16.s1 == UNSET_NODE {
                /*836: */
                if cur_list.mode as i32 == -1 {
                    *NODE_type(q as usize) = HLIST_NODE;
                    MEM[(q + 1) as usize].b32.s1 = MEM[(p + 1) as usize].b32.s1;
                    if NEST[(NEST_PTR - 1) as usize].mode == MMODE {
                        MEM[q as usize].b16.s0 = DLIST
                    }
                } else {
                    *NODE_type(q as usize) = VLIST_NODE;
                    MEM[(q + 3) as usize].b32.s1 = MEM[(p + 3) as usize].b32.s1;
                }
                MEM[(q + 5) as usize].b16.s0 = MEM[(p + 5) as usize].b16.s0;
                MEM[(q + 5) as usize].b16.s1 = MEM[(p + 5) as usize].b16.s1;
                MEM[(q + 6) as usize].gr = MEM[(p + 6) as usize].gr;
                MEM[(q + 4) as usize].b32.s1 = o;
                r = MEM[MEM[(q + 5) as usize].b32.s1 as usize].b32.s1;
                s = MEM[MEM[(p + 5) as usize].b32.s1 as usize].b32.s1;
                loop {
                    /*837: */
                    n = MEM[r as usize].b16.s0 as i32; /*840: */
                    t = MEM[(s + 1) as usize].b32.s1;
                    w = t;
                    u = HOLD_HEAD as i32;
                    MEM[r as usize].b16.s0 = 0;
                    while n > 0 {
                        n -= 1;
                        s = *LLIST_link(s as usize);
                        v = MEM[(s + 1) as usize].b32.s0;
                        MEM[u as usize].b32.s1 = new_glue(v) as i32;
                        u = *LLIST_link(u as usize);
                        MEM[u as usize].b16.s0 = GluePar::tab_skip as u16 + 1;
                        t = t + MEM[(v + 1) as usize].b32.s1;
                        if MEM[(p + 5) as usize].b16.s1 == STRETCHING {
                            if MEM[v as usize].b16.s1 as i32 == MEM[(p + 5) as usize].b16.s0 as i32
                            {
                                t = t + tex_round(
                                    (*BOX_glue_set(p as usize))
                                        * MEM[(v + 2) as usize].b32.s1 as f64,
                                )
                            }
                        } else if MEM[(p + 5) as usize].b16.s1 == SHRINKING {
                            if MEM[v as usize].b16.s0 as i32 == MEM[(p + 5) as usize].b16.s0 as i32
                            {
                                t = t - tex_round(
                                    (*BOX_glue_set(p as usize))
                                        * MEM[(v + 3) as usize].b32.s1 as f64,
                                )
                            }
                        }
                        s = *LLIST_link(s as usize);
                        MEM[u as usize].b32.s1 = new_null_box() as i32;
                        u = *LLIST_link(u as usize);
                        t = t + MEM[(s + 1) as usize].b32.s1;
                        if cur_list.mode == -1 {
                            MEM[(u + 1) as usize].b32.s1 = MEM[(s + 1) as usize].b32.s1
                        } else {
                            *NODE_type(u as usize) = VLIST_NODE;
                            MEM[(u + 3) as usize].b32.s1 = MEM[(s + 1) as usize].b32.s1
                        }
                    }
                    if cur_list.mode == -1 {
                        /*839: */
                        MEM[(r + 3) as usize].b32.s1 = MEM[(q + 3) as usize].b32.s1;
                        MEM[(r + 2) as usize].b32.s1 = MEM[(q + 2) as usize].b32.s1;
                        if t == MEM[(r + 1) as usize].b32.s1 {
                            MEM[(r + 5) as usize].b16.s1 = NORMAL;
                            MEM[(r + 5) as usize].b16.s0 = NORMAL;
                            *BOX_glue_set(r as usize) = 0.;
                        } else if t > MEM[(r + 1) as usize].b32.s1 {
                            MEM[(r + 5) as usize].b16.s1 = STRETCHING;
                            if MEM[(r + 6) as usize].b32.s1 == 0 {
                                *BOX_glue_set(r as usize) = 0.;
                            } else {
                                *BOX_glue_set(r as usize) = (t - MEM[(r + 1) as usize].b32.s1)
                                    as f64
                                    / MEM[(r + 6) as usize].b32.s1 as f64
                            }
                        } else {
                            MEM[(r + 5) as usize].b16.s0 = MEM[(r + 5) as usize].b16.s1;
                            MEM[(r + 5) as usize].b16.s1 = SHRINKING;
                            if MEM[(r + 4) as usize].b32.s1 == 0 {
                                MEM[(r + 6) as usize].gr = 0.;
                            } else if MEM[(r + 5) as usize].b16.s0 == NORMAL
                                && MEM[(r + 1) as usize].b32.s1 - t > MEM[(r + 4) as usize].b32.s1
                            {
                                *BOX_glue_set(r as usize) = 1.;
                            } else {
                                *BOX_glue_set(r as usize) = (MEM[(r + 1) as usize].b32.s1 - t)
                                    as f64
                                    / MEM[(r + 4) as usize].b32.s1 as f64;
                            }
                        }
                        MEM[(r + 1) as usize].b32.s1 = w;
                        *NODE_type(r as usize) = HLIST_NODE;
                    } else {
                        MEM[(r + 1) as usize].b32.s1 = MEM[(q + 1) as usize].b32.s1;
                        if t == MEM[(r + 3) as usize].b32.s1 {
                            MEM[(r + 5) as usize].b16.s1 = NORMAL;
                            MEM[(r + 5) as usize].b16.s0 = NORMAL;
                            *BOX_glue_set(r as usize) = 0.;
                        } else if t > MEM[(r + 3) as usize].b32.s1 {
                            MEM[(r + 5) as usize].b16.s1 = STRETCHING;
                            if MEM[(r + 6) as usize].b32.s1 == 0 {
                                *BOX_glue_set(r as usize) = 0.;
                            } else {
                                *BOX_glue_set(r as usize) = (t - MEM[(r + 3) as usize].b32.s1)
                                    as f64
                                    / MEM[(r + 6) as usize].b32.s1 as f64
                            }
                        } else {
                            MEM[(r + 5) as usize].b16.s0 = MEM[(r + 5) as usize].b16.s1;
                            MEM[(r + 5) as usize].b16.s1 = SHRINKING;
                            if MEM[(r + 4) as usize].b32.s1 == 0 {
                                *BOX_glue_set(r as usize) = 0.0;
                            } else if MEM[(r + 5) as usize].b16.s0 == NORMAL
                                && MEM[(r + 3) as usize].b32.s1 - t > MEM[(r + 4) as usize].b32.s1
                            {
                                *BOX_glue_set(r as usize) = 1.;
                            } else {
                                *BOX_glue_set(r as usize) = (MEM[(r + 3) as usize].b32.s1 - t)
                                    as f64
                                    / MEM[(r + 4) as usize].b32.s1 as f64
                            }
                        }
                        MEM[(r + 3) as usize].b32.s1 = w;
                        *NODE_type(r as usize) = VLIST_NODE;
                    }
                    MEM[(r + 4) as usize].b32.s1 = 0;
                    if u != HOLD_HEAD as i32 {
                        MEM[u as usize].b32.s1 = MEM[r as usize].b32.s1;
                        MEM[r as usize].b32.s1 = MEM[HOLD_HEAD].b32.s1;
                        r = u;
                    }
                    r = MEM[MEM[r as usize].b32.s1 as usize].b32.s1;
                    s = MEM[MEM[s as usize].b32.s1 as usize].b32.s1;
                    if r.is_texnull() {
                        break;
                    }
                }
            } else if *NODE_type(q as usize) == RULE_NODE {
                /*835: */
                if MEM[(q + 1) as usize].b32.s1 == NULL_FLAG {
                    MEM[(q + 1) as usize].b32.s1 = MEM[(p + 1) as usize].b32.s1
                }
                if MEM[(q + 3) as usize].b32.s1 == NULL_FLAG {
                    MEM[(q + 3) as usize].b32.s1 = MEM[(p + 3) as usize].b32.s1
                }
                if MEM[(q + 2) as usize].b32.s1 == NULL_FLAG {
                    MEM[(q + 2) as usize].b32.s1 = MEM[(p + 2) as usize].b32.s1
                }
                if o != 0 {
                    r = MEM[q as usize].b32.s1;
                    MEM[q as usize].b32.s1 = TEX_NULL;
                    q = hpack(q, 0, ADDITIONAL as small_number);
                    MEM[(q + 4) as usize].b32.s1 = o;
                    MEM[q as usize].b32.s1 = r;
                    MEM[s as usize].b32.s1 = q
                }
            }
        }
        s = q;
        q = *LLIST_link(q as usize);
    }
    flush_node_list(p);
    pop_alignment();
    aux_save = cur_list.aux;
    p = MEM[cur_list.head].b32.s1;
    q = cur_list.tail as i32;
    pop_nest();
    if cur_list.mode == MMODE {
        /*1241: */
        do_assignments(); /*1232: */
        if cur_cmd != MATH_SHIFT as u8 {
            /*1242: */
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Missing $$ inserted");
            help_ptr = 2;
            help_line[1] = b"Displays can use special alignments (like \\eqalignno)";
            help_line[0] = b"only if nothing but the alignment itself is between $$\'s.";
            back_error();
        } else {
            get_x_token();
            if cur_cmd != MATH_SHIFT as u8 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Display math should end with $$");
                help_ptr = 2;
                help_line[1] = b"The `$\' that I just saw supposedly matches a previous `$$\'.";
                help_line[0] = b"So I shall assume that you typed `$$\' both times.";
                back_error();
            }
        }
        flush_node_list(cur_list.eTeX_aux);
        pop_nest();
        MEM[cur_list.tail].b32.s1 = new_penalty(*INTPAR(IntPar::pre_display_penalty)) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b32.s1 =
            new_param_glue(GluePar::above_display_skip as small_number) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b32.s1 = p;
        if !p.is_texnull() {
            cur_list.tail = q as usize;
        }
        MEM[cur_list.tail].b32.s1 = new_penalty(*INTPAR(IntPar::post_display_penalty)) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b32.s1 =
            new_param_glue(GluePar::below_display_skip as small_number) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        cur_list.aux.b32.s1 = aux_save.b32.s1;
        resume_after_display();
    } else {
        cur_list.aux = aux_save;
        MEM[cur_list.tail].b32.s1 = p;
        if !p.is_texnull() {
            cur_list.tail = q as usize;
        }
        if cur_list.mode == VMODE {
            build_page();
        }
    };
}
pub(crate) unsafe fn align_peek() {
    loop {
        align_state = 1000000;
        loop {
            get_x_or_protected();
            if cur_cmd != SPACER as u8 {
                break;
            }
        }
        if cur_cmd == NO_ALIGN as u8 {
            scan_left_brace();
            new_save_level(GroupCode::NO_ALIGN);
            if cur_list.mode as i32 == -1 {
                normal_paragraph();
            }
            break;
        } else if cur_cmd == RIGHT_BRACE as u8 {
            fin_align();
            break;
        } else {
            if cur_cmd == CAR_RET as u8 && cur_chr == CR_CR_CODE {
                continue;
            }
            init_row();
            init_col();
            break;
        }
    }
}
pub(crate) unsafe fn max_hyphenatable_length() -> i32 {
    if *INTPAR(IntPar::xetex_hyphenatable_length) > HYPHENATABLE_LENGTH_LIMIT {
        return HYPHENATABLE_LENGTH_LIMIT;
    }
    *INTPAR(IntPar::xetex_hyphenatable_length)
}
pub(crate) unsafe fn eTeX_enabled(mut b: bool, mut j: u16, mut k: i32) -> bool {
    if !b {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Improper ");
        print_cmd_chr(j, k);
        help_ptr = 1;
        help_line[0] = b"Sorry, this optional e-TeX feature has been disabled.";
        error();
    }
    b
}
pub(crate) unsafe fn show_save_groups() {
    unsafe fn do_loop(mut p: usize, mut a: i8) -> (bool, usize, i8) {
        print_nl_cstr(b"### ");
        print_group(true);
        if cur_group == GroupCode::BOTTOM_LEVEL {
            return (true, p, a);
        }
        let mut m: i16 = 0;
        loop {
            m = NEST[p].mode;
            if p > 0 {
                p -= 1
            } else {
                m = VMODE
            }
            if !(m == HMODE) {
                break;
            }
        }
        print_cstr(b" (");
        let mut s: &[u8] = &[];
        match cur_group {
            GroupCode::BOTTOM_LEVEL => unreachable!(),
            GroupCode::SIMPLE => {
                p += 1;
                return found2(p, a);
            }
            GroupCode::HBOX | GroupCode::ADJUSTED_HBOX => {
                s = b"hbox";
            }
            GroupCode::VBOX => {
                s = b"vbox";
            }
            GroupCode::VTOP => {
                s = b"vtop";
            }
            GroupCode::ALIGN => {
                if a == 0 {
                    if m == -VMODE {
                        s = b"halign"
                    } else {
                        s = b"valign"
                    }
                    a = 1;
                    return found1(s, p, a);
                } else {
                    if a == 1 {
                        print_cstr(b"align entry");
                    } else {
                        print_esc_cstr(b"cr");
                    }
                    if p as i32 >= a as i32 {
                        p = (p as i32 - a as i32) as usize
                    }
                    a = 0;
                    return found(p, a);
                }
            }
            GroupCode::NO_ALIGN => {
                p += 1;
                a = -1;
                print_esc_cstr(b"noalign");
                return found2(p, a);
            }
            GroupCode::OUTPUT => {
                print_esc_cstr(b"output");
                return found(p, a);
            }
            GroupCode::MATH => return found2(p, a),
            GroupCode::DISC | GroupCode::MATH_CHOICE => {
                if cur_group == GroupCode::DISC {
                    print_esc_cstr(b"discretionary");
                } else {
                    print_esc_cstr(b"mathchoice");
                }
                let mut i = 1;
                while i <= 3 {
                    if i <= SAVE_STACK[SAVE_PTR - 2].b32.s1 {
                        print_cstr(b"{}");
                    }
                    i += 1;
                }
                return found2(p, a);
            }
            GroupCode::INSERT => {
                if SAVE_STACK[SAVE_PTR - 2].b32.s1 == 255 {
                    print_esc_cstr(b"vadjust");
                } else {
                    print_esc_cstr(b"insert");
                    print_int(SAVE_STACK[SAVE_PTR - 2].b32.s1);
                }
                return found2(p, a);
            }
            GroupCode::VCENTER => {
                s = b"vcenter";
                return found1(s, p, a);
            }
            GroupCode::SEMI_SIMPLE => {
                p += 1;
                print_esc_cstr(b"begingroup");
                return found(p, a);
            }
            GroupCode::MATH_SHIFT => {
                if m == MMODE {
                    print_char('$' as i32);
                } else if NEST[p].mode == MMODE {
                    print_cmd_chr(EQ_NO, SAVE_STACK[SAVE_PTR - 2].b32.s1);
                    return found(p, a);
                }
                print_char('$' as i32);
                return found(p, a);
            }
            GroupCode::MATH_LEFT => {
                if MEM[NEST[p + 1].eTeX_aux as usize].b16.s1 == LEFT_NOAD {
                    print_esc_cstr(b"left");
                } else {
                    print_esc_cstr(b"middle");
                }
                return found(p, a);
            }
        }

        let mut i = SAVE_STACK[SAVE_PTR - 4].b32.s1;

        if i != 0 {
            if i < BOX_FLAG {
                let j = if NEST[p].mode.abs() == VMODE {
                    HMOVE
                } else {
                    VMOVE
                };
                if i > 0 {
                    print_cmd_chr(j, 0);
                } else {
                    print_cmd_chr(j, 1);
                }
                print_scaled(i.abs());
                print_cstr(b"pt");
            } else if i < SHIP_OUT_FLAG {
                if i >= GLOBAL_BOX_FLAG {
                    print_esc_cstr(b"global");
                    i = i - (GLOBAL_BOX_FLAG - BOX_FLAG)
                }
                print_esc_cstr(b"setbox");
                print_int(i - BOX_FLAG);
                print_char('=' as i32);
            } else {
                print_cmd_chr(LEADER_SHIP, i - (LEADER_FLAG - (A_LEADERS as i32)));
            }
        }
        found1(s, p, a)
    }

    unsafe fn found1(s: &[u8], p: usize, a: i8) -> (bool, usize, i8) {
        print_esc_cstr(s);
        if SAVE_STACK[SAVE_PTR - 2].b32.s1 != 0 {
            print_char(' ' as i32);
            if SAVE_STACK[SAVE_PTR - 3].b32.s1 == EXACTLY as i32 {
                print_cstr(b"to");
            } else {
                print_cstr(b"spread");
            }
            print_scaled(SAVE_STACK[SAVE_PTR - 2].b32.s1);
            print_cstr(b"pt");
        }
        found2(p, a)
    }

    unsafe fn found2(p: usize, a: i8) -> (bool, usize, i8) {
        print_char('{' as i32);
        found(p, a)
    }

    unsafe fn found(p: usize, a: i8) -> (bool, usize, i8) {
        print_char(')' as i32);
        cur_level = cur_level.wrapping_sub(1);
        cur_group = GroupCode::from(SAVE_STACK[SAVE_PTR].b16.s0);
        SAVE_PTR = SAVE_STACK[SAVE_PTR].b32.s1 as usize;
        (false, p, a)
    }

    unsafe fn done(v: i32, l: u16, c: GroupCode) {
        SAVE_PTR = v as usize;
        cur_level = l;
        cur_group = c;
    }

    let mut p = NEST_PTR;
    NEST[p] = cur_list;
    let v = SAVE_PTR as i32;
    let l = cur_level;
    let c = cur_group;
    SAVE_PTR = cur_boundary as usize;
    cur_level = cur_level.wrapping_sub(1);
    let mut a = 1_i8;
    print_nl_cstr(b"");
    print_ln();
    loop {
        let (is_done, new_p, new_a) = do_loop(p, a);
        p = new_p;
        a = new_a;
        if is_done {
            return done(v, l, c);
        }
    }
}
pub(crate) unsafe fn vert_break(mut p: i32, mut h: scaled_t, mut d: scaled_t) -> i32 {
    let mut current_block: u64;
    let mut q: i32 = 0;
    let mut pi: i32 = 0;
    let mut b: i32 = 0;
    let mut best_place: i32 = TEX_NULL;
    let mut prev_p = p;
    let mut least_cost = MAX_HALFWORD;
    active_width[1..].copy_from_slice(&[0; 6]);
    let mut prev_dp = 0;
    loop {
        if p.is_texnull() {
            pi = EJECT_PENALTY;
            current_block = 9007357115414505193;
        } else {
            /*1008: */
            match MEM[p as usize].b16.s1 as i32 {
                0 | 1 | 2 => {
                    active_width[1] = active_width[1] + prev_dp + MEM[(p + 3) as usize].b32.s1;
                    prev_dp = MEM[(p + 2) as usize].b32.s1;
                    current_block = 10249009913728301645;
                }
                8 => {
                    if MEM[p as usize].b16.s0 == PIC_NODE || MEM[p as usize].b16.s0 == PDF_NODE {
                        active_width[1] = active_width[1] + prev_dp + MEM[(p + 3) as usize].b32.s1;
                        prev_dp = MEM[(p + 2) as usize].b32.s1
                    }
                    current_block = 10249009913728301645;
                }
                10 => {
                    if is_non_discardable_node(prev_p) {
                        pi = 0;
                        current_block = 9007357115414505193;
                    } else {
                        current_block = 11492179201936201469;
                    }
                }
                11 => {
                    let t = if MEM[p as usize].b32.s1.is_texnull() {
                        PENALTY_NODE as i16
                    } else {
                        MEM[MEM[p as usize].b32.s1 as usize].b16.s1 as small_number
                    };
                    if t == GLUE_NODE as i16 {
                        pi = 0;
                        current_block = 9007357115414505193;
                    } else {
                        current_block = 11492179201936201469;
                    }
                }
                12 => {
                    pi = MEM[(p + 1) as usize].b32.s1;
                    current_block = 9007357115414505193;
                }
                4 | 3 => current_block = 10249009913728301645,
                _ => {
                    current_block = 5335814873276400744;
                    confusion(b"vertbreak");
                }
            }
        }
        match current_block {
            9007357115414505193 => {
                if pi < INF_PENALTY {
                    b = if active_width[1] < h {
                        if active_width[3] != 0 || active_width[4] != 0 || active_width[5] != 0 {
                            0
                        } else {
                            badness(h - active_width[1], active_width[2])
                        }
                    } else if active_width[1] - h > active_width[6] {
                        MAX_HALFWORD
                    } else {
                        badness(active_width[1] - h, active_width[6])
                    };
                    if b < MAX_HALFWORD {
                        if pi <= EJECT_PENALTY {
                            b = pi
                        } else if b < INF_BAD {
                            b = b + pi
                        } else {
                            b = 100000;
                        }
                    }
                    if b <= least_cost {
                        best_place = p;
                        least_cost = b;
                        best_height_plus_depth = active_width[1] + prev_dp
                    }
                    if b == MAX_HALFWORD || pi <= EJECT_PENALTY {
                        break;
                    }
                }
                if *NODE_type(p as usize) < GLUE_NODE || *NODE_type(p as usize) > KERN_NODE {
                    current_block = 10249009913728301645;
                } else {
                    current_block = 11492179201936201469;
                }
            }
            _ => {}
        }
        match current_block {
            11492179201936201469 => {
                /*update_heights *//*1011: */
                if *NODE_type(p as usize) == KERN_NODE {
                    q = p
                } else {
                    q = MEM[(p + 1) as usize].b32.s0; /*:1011 */
                    active_width[2 + MEM[q as usize].b16.s1 as usize] = active_width
                        [(2i32 + MEM[q as usize].b16.s1 as i32) as usize]
                        + MEM[(q + 2) as usize].b32.s1; /*:1014*/
                    active_width[6] = active_width[6] + MEM[(q + 3) as usize].b32.s1;
                    if MEM[q as usize].b16.s0 != NORMAL && MEM[(q + 3) as usize].b32.s1 != 0 {
                        if file_line_error_style_p != 0 {
                            print_file_line();
                        } else {
                            print_nl_cstr(b"! ");
                        }
                        print_cstr(b"Infinite glue shrinkage found in box being split");
                        help_ptr = 4;
                        help_line[3] = b"The box you are \\vsplitting contains some infinitely";
                        help_line[2] =
                            b"shrinkable glue, e.g., `\\vss\' or `\\vskip 0pt minus 1fil\'.";
                        help_line[1] =
                            b"Such glue doesn\'t belong there; but you can safely proceed,";
                        help_line[0] = b"since the offensive shrinkability has been made finite.";
                        error();
                        let r = new_spec(q as usize);
                        *GLUE_SPEC_shrink_order(r) = NORMAL;
                        delete_glue_ref(q as usize);
                        MEM[(p + 1) as usize].b32.s0 = r as i32;
                        q = r as i32;
                    }
                }
                active_width[1] = active_width[1] + prev_dp + MEM[(q + 1) as usize].b32.s1;
                prev_dp = 0;
            }
            _ => {}
        }
        if prev_dp > d {
            active_width[1] = active_width[1] + prev_dp - d;
            prev_dp = d
        }
        prev_p = p;
        p = MEM[prev_p as usize].b32.s1
    }
    best_place
}
pub(crate) unsafe fn vsplit(mut n: i32, mut h: scaled_t) -> i32 {
    cur_val = n;
    let v = if cur_val < 256 {
        *BOX_REG(cur_val as usize)
    } else {
        find_sa_element(4, cur_val, false);
        if cur_ptr.is_texnull() {
            TEX_NULL
        } else {
            MEM[(cur_ptr + 1) as usize].b32.s1
        }
    };
    flush_node_list(disc_ptr[VSPLIT_CODE as usize]);
    disc_ptr[VSPLIT_CODE as usize] = TEX_NULL;
    if !sa_root[MARK_VAL as usize].is_texnull() {
        if do_marks(0, 0, sa_root[MARK_VAL as usize]) {
            sa_root[MARK_VAL as usize] = TEX_NULL
        }
    }
    if !cur_mark[SPLIT_FIRST_MARK_CODE as usize].is_texnull() {
        delete_token_ref(cur_mark[SPLIT_FIRST_MARK_CODE as usize] as usize);
        cur_mark[SPLIT_FIRST_MARK_CODE as usize] = TEX_NULL;
        delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE as usize] as usize);
        cur_mark[SPLIT_BOT_MARK_CODE as usize] = TEX_NULL;
    }
    if v.is_texnull() {
        return TEX_NULL;
    }
    let v = v as usize;
    if *NODE_type(v as usize) != VLIST_NODE {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"");
        print_esc_cstr(b"vsplit");
        print_cstr(b" needs a ");
        print_esc_cstr(b"vbox");
        help_ptr = 2;
        help_line[1] = b"The box you are trying to split is an \\hbox.";
        help_line[0] = b"I can\'t split such a box, so I\'ll leave it alone.";
        error();
        return TEX_NULL;
    }
    let q = vert_break(MEM[v + 5].b32.s1, h, *DIMENPAR(DimenPar::split_max_depth));
    let mut p = MEM[v + 5].b32.s1;
    if p == q {
        MEM[v + 5].b32.s1 = TEX_NULL
    } else {
        loop {
            if *NODE_type(p as usize) == MARK_NODE {
                if MEM[(p + 1) as usize].b32.s0 != 0 {
                    /*1615: */
                    find_sa_element(MARK_VAL as small_number, MEM[(p + 1) as usize].b32.s0, true);
                    if MEM[(cur_ptr + 2) as usize].b32.s1.is_texnull() {
                        MEM[(cur_ptr + 2) as usize].b32.s1 = MEM[(p + 1) as usize].b32.s1;
                        MEM[MEM[(p + 1) as usize].b32.s1 as usize].b32.s0 += 1
                    } else {
                        delete_token_ref(MEM[(cur_ptr + 3) as usize].b32.s0 as usize);
                    }
                    MEM[(cur_ptr + 3) as usize].b32.s0 = MEM[(p + 1) as usize].b32.s1;
                    MEM[MEM[(p + 1) as usize].b32.s1 as usize].b32.s0 += 1;
                } else if cur_mark[SPLIT_FIRST_MARK_CODE as usize].is_texnull() {
                    cur_mark[SPLIT_FIRST_MARK_CODE as usize] = MEM[(p + 1) as usize].b32.s1;
                    cur_mark[SPLIT_BOT_MARK_CODE as usize] =
                        cur_mark[SPLIT_FIRST_MARK_CODE as usize];
                    MEM[cur_mark[SPLIT_FIRST_MARK_CODE as usize] as usize]
                        .b32
                        .s0 = MEM[cur_mark[SPLIT_FIRST_MARK_CODE as usize] as usize]
                        .b32
                        .s0
                        + 2
                } else {
                    delete_token_ref(cur_mark[SPLIT_BOT_MARK_CODE as usize] as usize);
                    cur_mark[SPLIT_BOT_MARK_CODE as usize] = MEM[(p + 1) as usize].b32.s1;
                    MEM[cur_mark[SPLIT_BOT_MARK_CODE as usize] as usize].b32.s0 += 1;
                }
            }
            if MEM[p as usize].b32.s1 == q {
                MEM[p as usize].b32.s1 = TEX_NULL;
                break;
            } else {
                p = *LLIST_link(p as usize);
            }
        }
    }
    let mut q = prune_page_top(q, *INTPAR(IntPar::saving_vdiscards) > 0);
    let p = MEM[v + 5].b32.s1;
    free_node(v, BOX_NODE_SIZE);
    if !q.is_texnull() {
        q = vpackage(q, 0, ADDITIONAL as small_number, MAX_HALFWORD) as i32
    }
    if cur_val < 256 {
        *BOX_REG(cur_val as usize) = q;
    } else {
        find_sa_element(4, cur_val, false);
        if !cur_ptr.is_texnull() {
            MEM[(cur_ptr + 1) as usize].b32.s1 = q;
            MEM[(cur_ptr + 1) as usize].b32.s0 += 1;
            delete_sa_ref(cur_ptr as usize);
        }
    }
    vpackage(p, h, EXACTLY as i16, *DIMENPAR(DimenPar::split_max_depth)) as i32
}
pub(crate) unsafe fn print_totals() {
    print_scaled(page_so_far[1]);
    if page_so_far[2] != 0 {
        print_cstr(b" plus ");
        print_scaled(page_so_far[2]);
        print_cstr(b"");
    }
    if page_so_far[3] != 0 {
        print_cstr(b" plus ");
        print_scaled(page_so_far[3]);
        print_cstr(b"fil");
    }
    if page_so_far[4] != 0 {
        print_cstr(b" plus ");
        print_scaled(page_so_far[4]);
        print_cstr(b"fill");
    }
    if page_so_far[5] != 0 {
        print_cstr(b" plus ");
        print_scaled(page_so_far[5]);
        print_cstr(b"filll");
    }
    if page_so_far[6] != 0 {
        print_cstr(b" minus ");
        print_scaled(page_so_far[6]);
    };
}
pub(crate) unsafe fn box_error(mut n: eight_bits) {
    error();
    begin_diagnostic();
    print_nl_cstr(b"The following box has been deleted:");
    show_box(*BOX_REG(n as usize));
    end_diagnostic(true);
    flush_node_list(*BOX_REG(n as usize));
    *BOX_REG(n as usize) = TEX_NULL;
}
pub(crate) unsafe fn app_space() {
    let q;
    if cur_list.aux.b32.s0 >= 2000 && *GLUEPAR(GluePar::xspace_skip) != 0i32 {
        q = new_param_glue(GluePar::xspace_skip as small_number)
    } else {
        if *GLUEPAR(GluePar::space_skip) != 0 {
            main_p = *GLUEPAR(GluePar::space_skip)
        } else {
            /*1077: */
            main_p = FONT_GLUE[EQTB[CUR_FONT_LOC].b32.s1 as usize]; /*:1079 */
            if main_p.is_texnull() {
                let main_p_ = new_spec(0);
                main_p = main_p_ as i32;
                main_k = PARAM_BASE[EQTB[CUR_FONT_LOC].b32.s1 as usize] + 2;
                MEM[main_p_ + 1].b32.s1 = FONT_INFO[main_k as usize].b32.s1;
                MEM[main_p_ + 2].b32.s1 = FONT_INFO[(main_k + 1) as usize].b32.s1;
                MEM[main_p_ + 3].b32.s1 = FONT_INFO[(main_k + 2) as usize].b32.s1;
                FONT_GLUE[EQTB[CUR_FONT_LOC].b32.s1 as usize] = main_p
            }
        }
        let main_p_ = new_spec(main_p as usize);
        main_p = main_p_ as i32;
        if cur_list.aux.b32.s0 >= 2000 {
            MEM[main_p_ + 1].b32.s1 = MEM[main_p_ + 1].b32.s1
                + FONT_INFO
                    [(EXTRA_SPACE_CODE + PARAM_BASE[EQTB[CUR_FONT_LOC].b32.s1 as usize]) as usize]
                    .b32
                    .s1
        }
        MEM[main_p_ + 2].b32.s1 = xn_over_d(MEM[main_p_ + 2].b32.s1, cur_list.aux.b32.s0, 1000);
        MEM[main_p_ + 3].b32.s1 = xn_over_d(MEM[main_p_ + 3].b32.s1, 1000, cur_list.aux.b32.s0);
        q = new_glue(main_p);
        MEM[main_p_].b32.s1 = TEX_NULL
    }
    MEM[cur_list.tail].b32.s1 = q as i32;
    cur_list.tail = q;
}
pub(crate) unsafe fn insert_dollar_sign() {
    back_input();
    cur_tok = MATH_SHIFT_TOKEN + 36;
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Missing $ inserted");
    help_ptr = 2;
    help_line[1] = b"I\'ve inserted a begin-math/end-math symbol since I think";
    help_line[0] = b"you left one out. Proceed, with fingers crossed.";
    ins_error();
}
pub(crate) unsafe fn you_cant() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"You can\'t use `");
    print_cmd_chr(cur_cmd as u16, cur_chr);
    print_in_mode(cur_list.mode as i32);
}
pub(crate) unsafe fn report_illegal_case() {
    you_cant();
    help_ptr = 4;
    help_line[3] = b"Sorry, but I\'m not programmed to handle this case;";
    help_line[2] = b"I\'ll just pretend that you didn\'t ask for it.";
    help_line[1] = b"If you\'re in the wrong mode, you might be able to";
    help_line[0] = b"return to the right one by typing `I}\' or `I$\' or `I\\par\'.";
    error();
}
pub(crate) unsafe fn privileged() -> bool {
    if cur_list.mode > 0 {
        true
    } else {
        report_illegal_case();
        false
    }
}
pub(crate) unsafe fn its_all_over() -> bool {
    if privileged() {
        if PAGE_HEAD == page_tail as usize && cur_list.head == cur_list.tail && dead_cycles == 0 {
            return true;
        }
        back_input();
        MEM[cur_list.tail].b32.s1 = new_null_box() as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail + 1].b32.s1 = *DIMENPAR(DimenPar::hsize);
        MEM[cur_list.tail].b32.s1 = new_glue(8) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b32.s1 = new_penalty(NULL_FLAG) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        build_page();
    }
    false
}
pub(crate) unsafe fn append_glue() {
    let s = cur_chr as small_number;
    match s as i32 {
        0 => cur_val = 4,
        1 => cur_val = 8,
        2 => cur_val = 12,
        3 => cur_val = 16,
        4 => scan_glue(GLUE_VAL as i16),
        5 => scan_glue(MU_VAL as i16),
        _ => {}
    }
    MEM[cur_list.tail].b32.s1 = new_glue(cur_val) as i32;
    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    if s >= SKIP_CODE as i16 {
        MEM[cur_val as usize].b32.s1 -= 1;
        if s > SKIP_CODE as i16 {
            MEM[cur_list.tail].b16.s0 = MU_GLUE;
        }
    };
}
pub(crate) unsafe fn append_kern() {
    let s = cur_chr as u16;
    scan_dimen(s == MU_GLUE as u16, false, false);
    MEM[cur_list.tail].b32.s1 = new_kern(cur_val) as i32;
    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    MEM[cur_list.tail].b16.s0 = s;
}
pub(crate) unsafe fn off_save() {
    if cur_group == GroupCode::BOTTOM_LEVEL {
        /*1101:*/
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Extra ");
        print_cmd_chr(cur_cmd as u16, cur_chr);
        help_ptr = 1;
        help_line[0] = b"Things are pretty mixed up, but I think the worst is over.";
        error();
    } else {
        back_input();
        let mut p = get_avail();
        MEM[TEMP_HEAD].b32.s1 = p as i32;
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Missing ");
        match cur_group {
            GroupCode::SEMI_SIMPLE => {
                MEM[p].b32.s0 = CS_TOKEN_FLAG + FROZEN_END_GROUP as i32;
                print_esc_cstr(b"endgroup");
            }
            GroupCode::MATH_SHIFT => {
                MEM[p].b32.s0 = MATH_SHIFT_TOKEN + '$' as i32;
                print_char('$' as i32);
            }
            GroupCode::MATH_LEFT => {
                MEM[p].b32.s0 = CS_TOKEN_FLAG + FROZEN_RIGHT as i32;
                MEM[p].b32.s1 = get_avail() as i32;
                p = *LLIST_link(p) as usize;
                MEM[p].b32.s0 = OTHER_TOKEN + '.' as i32;
                print_esc_cstr(b"right.");
            }
            _ => {
                MEM[p].b32.s0 = RIGHT_BRACE_TOKEN + '}' as i32;
                print_char('}' as i32);
            }
        }
        print_cstr(b" inserted");
        begin_token_list(MEM[TEMP_HEAD].b32.s1 as usize, INSERTED);
        help_ptr = 5;
        help_line[4] = b"I\'ve inserted something that you may have forgotten.";
        help_line[3] = b"(See the <inserted text> above.)";
        help_line[2] = b"With luck, this will get me unwedged. But if you";
        help_line[1] = b"really didn\'t forget anything, try typing `2\' now; then";
        help_line[0] = b"my insertion and my current dilemma will both disappear.";
        error();
    };
}
pub(crate) unsafe fn extra_right_brace() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Extra }, or forgotten ");
    match cur_group {
        GroupCode::SEMI_SIMPLE => print_esc_cstr(b"endgroup"),
        GroupCode::MATH_SHIFT => print_char('$' as i32),
        GroupCode::MATH_LEFT => print_esc_cstr(b"right"),
        _ => {}
    }
    help_ptr = 5;
    help_line[4] = b"I\'ve deleted a group-closing symbol because it seems to be";
    help_line[3] = b"spurious, as in `$x}$\'. But perhaps the } is legitimate and";
    help_line[2] = b"you forgot something else, as in `\\hbox{$x}\'. In such cases";
    help_line[1] = b"the way to recover is to insert both the forgotten and the";
    help_line[0] = b"deleted material, e.g., by typing `I$}\'.";
    error();
    align_state += 1;
}
pub(crate) unsafe fn normal_paragraph() {
    if *INTPAR(IntPar::looseness) != 0 {
        eq_word_define(INT_BASE as usize + (IntPar::looseness as usize), 0i32);
    }
    if *DIMENPAR(DimenPar::hang_indent) != 0 {
        eq_word_define(DIMEN_BASE as usize + (DimenPar::hang_indent as usize), 0i32);
    }
    if *INTPAR(IntPar::hang_after) != 1 {
        eq_word_define(INT_BASE as usize + (IntPar::hang_after as usize), 1i32);
    }
    if !LOCAL(Local::par_shape).is_texnull() {
        eq_define(
            LOCAL_BASE as usize + Local::par_shape as usize,
            SHAPE_REF,
            TEX_NULL,
        );
    }
    if !EQTB[INTER_LINE_PENALTIES_LOC].b32.s1.is_texnull() {
        eq_define(INTER_LINE_PENALTIES_LOC, SHAPE_REF, TEX_NULL);
    };
}
/*1110: "The box_end procedure does the right thing with cur_box, if
 * box_context represents the context as explained [as follows]." The
 * box_context is one of (1) a signed shift amount; (2) BOX_FLAG+N, signifying
 * a `\setbox<N>`; (3) GLOBAL_BOX_FLAG+N, signifying `\global\setbox<N>`; (4)
 * SHIP_OUT_FLAG, signifying `\shipout`; or (5) LEADER_FLAG+k, signifying (in
 * order) `\leaders`, `\cleaders`, or `\xleaders`. */
pub(crate) unsafe fn box_end(mut box_context: i32) {
    let mut a: small_number = 0;
    if box_context < BOX_FLAG {
        /*1111:*/
        if !cur_box.is_texnull() {
            MEM[(cur_box + 4) as usize].b32.s1 = box_context;
            if cur_list.mode.abs() == VMODE {
                if !pre_adjust_tail.is_texnull() {
                    if PRE_ADJUST_HEAD as i32 != pre_adjust_tail {
                        MEM[cur_list.tail].b32.s1 = MEM[PRE_ADJUST_HEAD].b32.s1;
                        cur_list.tail = pre_adjust_tail as usize;
                    }
                    pre_adjust_tail = TEX_NULL;
                }
                append_to_vlist(cur_box as usize);
                if !adjust_tail.is_texnull() {
                    if ADJUST_HEAD as i32 != adjust_tail {
                        MEM[cur_list.tail].b32.s1 = MEM[ADJUST_HEAD].b32.s1;
                        cur_list.tail = adjust_tail as usize;
                    }
                    adjust_tail = TEX_NULL
                }
                if cur_list.mode > 0 {
                    build_page();
                }
            } else {
                if cur_list.mode.abs() == HMODE {
                    cur_list.aux.b32.s0 = 1000
                } else {
                    let p = new_noad();
                    MEM[p + 1].b32.s1 = SUB_BOX;
                    MEM[p + 1].b32.s0 = cur_box;
                    cur_box = p as i32;
                }
                MEM[cur_list.tail].b32.s1 = cur_box;
                cur_list.tail = cur_box as usize;
            }
        }
    } else if box_context < SHIP_OUT_FLAG {
        /*1112:*/
        if box_context < GLOBAL_BOX_FLAG {
            cur_val = box_context - BOX_FLAG;
            a = 0
        } else {
            cur_val = box_context - GLOBAL_BOX_FLAG;
            a = 4
        }
        if cur_val < 256 {
            if a >= 4 {
                geq_define(BOX_BASE + cur_val as usize, BOX_REF, cur_box);
            } else {
                eq_define(BOX_BASE + cur_val as usize, BOX_REF, cur_box);
            }
        } else {
            find_sa_element(4, cur_val, true);
            if a >= 4 {
                gsa_def(cur_ptr as usize, cur_box);
            } else {
                sa_def(cur_ptr as usize, cur_box);
            }
        }
    } else if !cur_box.is_texnull() {
        if box_context > SHIP_OUT_FLAG {
            loop
            /*1113:*/
            {
                get_x_token();
                if !(cur_cmd == SPACER as u8 || cur_cmd == RELAX as u8) {
                    break;
                }
            }
            if cur_cmd == HSKIP as u8 && cur_list.mode.abs() != VMODE
                || cur_cmd == VSKIP as u8 && cur_list.mode.abs() == VMODE
            {
                append_glue();
                MEM[cur_list.tail].b16.s0 =
                    (box_context - (LEADER_FLAG - (A_LEADERS as i32))) as u16;
                MEM[cur_list.tail + 1].b32.s1 = cur_box;
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Leaders not followed by proper glue");
                help_ptr = 3;
                help_line[2] = b"You should say `\\leaders <box or rule><hskip or vskip>\'.";
                help_line[1] = b"I found the <box or rule>, but there\'s no suitable";
                help_line[0] = b"<hskip or vskip>, so I\'m ignoring these leaders.";
                back_error();
                flush_node_list(cur_box);
            }
        } else {
            ship_out(cur_box as usize);
        }
    };
}
pub(crate) unsafe fn begin_box(mut box_context: i32) {
    match cur_chr {
        BOX_CODE => {
            scan_register_num();
            if cur_val < 256 {
                cur_box = *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    cur_box = TEX_NULL
                } else {
                    cur_box = MEM[(cur_ptr + 1) as usize].b32.s1
                }
            }
            if cur_val < 256 {
                *BOX_REG(cur_val as usize) = TEX_NULL
            } else {
                find_sa_element(4, cur_val, false);
                if !cur_ptr.is_texnull() {
                    MEM[(cur_ptr + 1) as usize].b32.s1 = TEX_NULL;
                    MEM[(cur_ptr + 1) as usize].b32.s0 += 1;
                    delete_sa_ref(cur_ptr as usize);
                }
            }
        }
        COPY_CODE => {
            scan_register_num();
            let q = if cur_val < 256 {
                *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    TEX_NULL
                } else {
                    MEM[(cur_ptr + 1) as usize].b32.s1
                }
            };
            cur_box = copy_node_list(q)
        }
        LAST_BOX_CODE => {
            cur_box = TEX_NULL;
            if cur_list.mode.abs() == MMODE {
                you_cant();
                help_ptr = 1;
                help_line[0] = b"Sorry; this \\lastbox will be void.";
                error();
            } else if cur_list.mode == VMODE && cur_list.head == cur_list.tail {
                you_cant();
                help_ptr = 2;
                help_line[1] = b"Sorry...I usually can\'t take things from the current page.";
                help_line[0] = b"This \\lastbox will therefore be void.";
                error();
            } else {
                let mut current_block_79: u64;
                let mut tx = cur_list.tail as i32;
                if tx < hi_mem_min {
                    if *NODE_type(tx as usize) == MATH_NODE && MEM[tx as usize].b16.s0 == END_M_CODE
                    {
                        let mut r = cur_list.head as i32;
                        let mut q: i32 = 0;
                        loop {
                            q = r;
                            r = MEM[q as usize].b32.s1;
                            if !(r != tx) {
                                break;
                            }
                        }
                        tx = q
                    }
                }
                if tx < hi_mem_min {
                    if *NODE_type(tx as usize) == HLIST_NODE
                        || *NODE_type(tx as usize) == VLIST_NODE
                    {
                        /*1116:*/
                        let mut q = cur_list.head as i32;
                        let mut p = TEX_NULL;
                        let mut r: i32 = 0;
                        let mut fm: bool = false;
                        loop {
                            r = p;
                            p = q;
                            fm = false;
                            if q < hi_mem_min {
                                if *NODE_type(q as usize) == DISC_NODE {
                                    for _ in 1..=MEM[q as usize].b16.s0 {
                                        p = *LLIST_link(p as usize);
                                    }
                                    if p == tx {
                                        current_block_79 = 1209030638129645089;
                                        break;
                                    }
                                } else if *NODE_type(q as usize) == MATH_NODE
                                    && MEM[q as usize].b16.s0 == BEGIN_M_CODE
                                {
                                    fm = true;
                                }
                            }
                            q = MEM[p as usize].b32.s1;
                            if !(q != tx) {
                                current_block_79 = 12961834331865314435;
                                break;
                            }
                        }
                        match current_block_79 {
                            1209030638129645089 => {}
                            _ => {
                                q = MEM[tx as usize].b32.s1;
                                MEM[p as usize].b32.s1 = q;
                                MEM[tx as usize].b32.s1 = TEX_NULL;
                                if q.is_texnull() {
                                    if fm {
                                        confusion(b"tail1");
                                    } else {
                                        cur_list.tail = p as usize;
                                    }
                                } else if fm {
                                    cur_list.tail = r as usize;
                                    MEM[r as usize].b32.s1 = TEX_NULL;
                                    flush_node_list(p);
                                }
                                cur_box = tx;
                                MEM[(cur_box + 4) as usize].b32.s1 = 0
                            }
                        }
                    }
                }
            }
        }
        VSPLIT_CODE => {
            scan_register_num();
            let n = cur_val;
            if !scan_keyword(b"to") {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Missing `to\' inserted");
                help_ptr = 2;
                help_line[1] = b"I\'m working on `\\vsplit<box number> to <dimen>\';";
                help_line[0] = b"will look for the <dimen> next.";
                error();
            }
            scan_dimen(false, false, false);
            cur_box = vsplit(n, cur_val)
        }
        _ => {
            let mut k = cur_chr - 4;
            SAVE_STACK[SAVE_PTR + 0].b32.s1 = box_context;
            if k == HMODE as i32 {
                if box_context < BOX_FLAG && cur_list.mode.abs() == VMODE {
                    scan_spec(GroupCode::ADJUSTED_HBOX, true);
                } else {
                    scan_spec(GroupCode::HBOX, true);
                }
            } else {
                if k == VMODE as i32 {
                    scan_spec(GroupCode::VBOX, true);
                } else {
                    scan_spec(GroupCode::VTOP, true);
                    k = VMODE as i32;
                }
                normal_paragraph();
            }
            push_nest();
            cur_list.mode = -k as i16;
            if k == VMODE as i32 {
                cur_list.aux.b32.s1 = IGNORE_DEPTH;
                if !LOCAL(Local::every_vbox).is_texnull() {
                    begin_token_list(*LOCAL(Local::every_vbox) as usize, EVERY_VBOX_TEXT);
                }
            } else {
                cur_list.aux.b32.s0 = 1000;
                if !LOCAL(Local::every_hbox).is_texnull() {
                    begin_token_list(*LOCAL(Local::every_hbox) as usize, EVERY_HBOX_TEXT);
                }
            }
            return;
        }
    }
    box_end(box_context);
}
pub(crate) unsafe fn scan_box(mut box_context: i32) {
    loop {
        get_x_token();
        if !(cur_cmd == SPACER as u8 || cur_cmd == RELAX as u8) {
            break;
        }
    }
    if cur_cmd == MAKE_BOX as u8 {
        begin_box(box_context);
    } else if box_context >= LEADER_FLAG && (cur_cmd == HRULE as u8 || cur_cmd == VRULE as u8) {
        cur_box = scan_rule_spec() as i32;
        box_end(box_context);
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"A <box> was supposed to be here");
        help_ptr = 3;
        help_line[2] = b"I was expecting to see \\hbox or \\vbox or \\copy or \\box or";
        help_line[1] = b"something like that. So you might find something missing in";
        help_line[0] = b"your output. But keep trying; you can fix this later.";
        back_error();
    };
}
pub(crate) unsafe fn package(mut c: small_number) {
    let d = *DIMENPAR(DimenPar::box_max_depth);
    let u = *INTPAR(IntPar::xetex_upwards);
    unsave();
    SAVE_PTR -= 3;
    let v = *INTPAR(IntPar::xetex_upwards);
    *INTPAR(IntPar::xetex_upwards) = u;
    if cur_list.mode == -104 {
        cur_box = hpack(
            MEM[cur_list.head].b32.s1,
            SAVE_STACK[SAVE_PTR + 2].b32.s1,
            SAVE_STACK[SAVE_PTR + 1].b32.s1 as small_number,
        )
    } else {
        cur_box = vpackage(
            MEM[cur_list.head].b32.s1,
            SAVE_STACK[SAVE_PTR + 2].b32.s1,
            SAVE_STACK[SAVE_PTR + 1].b32.s1 as small_number,
            d,
        ) as i32;
        if c == VTOP_CODE as i16 {
            /*1122: */
            let mut h = 0;
            let p = MEM[(cur_box + 5) as usize].b32.s1;
            if !p.is_texnull() {
                if *NODE_type(p as usize) <= RULE_NODE {
                    h = MEM[(p + 3) as usize].b32.s1
                }
            }
            MEM[(cur_box + 2) as usize].b32.s1 =
                MEM[(cur_box + 2) as usize].b32.s1 - h + MEM[(cur_box + 3) as usize].b32.s1;
            MEM[(cur_box + 3) as usize].b32.s1 = h
        }
    }
    *INTPAR(IntPar::xetex_upwards) = v;
    pop_nest();
    box_end(SAVE_STACK[SAVE_PTR + 0].b32.s1);
}
pub(crate) unsafe fn norm_min(mut h: i32) -> small_number {
    (if h <= 0 {
        1
    } else if h >= 63 {
        63
    } else {
        h
    }) as small_number
}
pub(crate) unsafe fn new_graf(mut indented: bool) {
    cur_list.prev_graf = 0;
    if cur_list.mode == VMODE || cur_list.head != cur_list.tail {
        MEM[cur_list.tail].b32.s1 = new_param_glue(GluePar::par_skip as small_number) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    }
    push_nest();
    cur_list.mode = HMODE;
    cur_list.aux.b32.s0 = 1000;
    cur_lang = if *INTPAR(IntPar::language) <= 0 {
        0
    } else if *INTPAR(IntPar::language) > BIGGEST_LANG {
        0
    } else {
        *INTPAR(IntPar::language) as u8
    };
    cur_list.aux.b32.s1 = cur_lang as i32;
    cur_list.prev_graf = ((norm_min(*INTPAR(IntPar::left_hyphen_min)) as i32 * 64i32
        + norm_min(*INTPAR(IntPar::right_hyphen_min)) as i32) as i64
        * 65536
        + cur_lang as i64) as i32;
    if indented {
        cur_list.tail = new_null_box();
        MEM[cur_list.head].b32.s1 = cur_list.tail as i32;
        MEM[cur_list.tail + 1].b32.s1 = EQTB[DIMEN_BASE].b32.s1;
        if insert_src_special_every_par {
            insert_src_special();
        }
    }
    if !LOCAL(Local::every_par).is_texnull() {
        begin_token_list(*LOCAL(Local::every_par) as usize, EVERY_PAR_TEXT);
    }
    if NEST_PTR == 1 {
        build_page();
    };
}
pub(crate) unsafe fn indent_in_hmode() {
    if cur_chr > 0 {
        let mut p = new_null_box();
        MEM[p + 1].b32.s1 = EQTB[DIMEN_BASE].b32.s1;
        if cur_list.mode.abs() == HMODE {
            cur_list.aux.b32.s0 = 1000
        } else {
            let q = new_noad();
            MEM[q + 1].b32.s1 = SUB_BOX;
            MEM[q + 1].b32.s0 = p as i32;
            p = q;
        }
        MEM[cur_list.tail].b32.s1 = p as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    };
}
pub(crate) unsafe fn head_for_vmode() {
    if (cur_list.mode as i32) < 0 {
        if cur_cmd != HRULE as u8 {
            off_save();
        } else {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"You can\'t use `");
            print_esc_cstr(b"hrule");
            print_cstr(b"\' here except with leaders");
            help_ptr = 2;
            help_line[1] = b"To put a horizontal rule in an hbox or an alignment,";
            help_line[0] = b"you should use \\leaders or \\hrulefill (see The TeXbook).";
            error();
        }
    } else {
        back_input();
        cur_tok = par_token;
        back_input();
        cur_input.index = INSERTED;
    };
}
pub(crate) unsafe fn end_graf() {
    if cur_list.mode == HMODE {
        if cur_list.head == cur_list.tail {
            pop_nest();
        } else {
            line_break(false);
        }
        let aux = cur_list.eTeX_aux.opt();
        if let Some(_) = aux {
            flush_list(aux);
            cur_list.eTeX_aux = TEX_NULL;
        }
        normal_paragraph();
        error_count = 0;
    };
}
pub(crate) unsafe fn begin_insert_or_adjust() {
    if cur_cmd == VADJUST as u8 {
        cur_val = 255;
    } else {
        scan_eight_bit_int();
        if cur_val == 255 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"You can\'t ");
            print_esc_cstr(b"insert");
            print_int(255);
            help_ptr = 1;
            help_line[0] = b"I\'m changing to \\insert0; box 255 is special.";
            error();
            cur_val = 0;
        }
    }
    SAVE_STACK[SAVE_PTR + 0].b32.s1 = cur_val;
    if cur_cmd == VADJUST as u8 && scan_keyword(b"pre") {
        SAVE_STACK[SAVE_PTR + 1].b32.s1 = 1;
    } else {
        SAVE_STACK[SAVE_PTR + 1].b32.s1 = 0;
    }
    SAVE_PTR += 2;
    new_save_level(GroupCode::INSERT);
    scan_left_brace();
    normal_paragraph();
    push_nest();
    cur_list.mode = -1;
    cur_list.aux.b32.s1 = IGNORE_DEPTH;
}
pub(crate) unsafe fn make_mark() {
    let c = if cur_chr == 0 {
        0
    } else {
        scan_register_num();
        cur_val
    };
    let _p = scan_toks(false, true);
    let p = get_node(SMALL_NODE_SIZE);
    MEM[p + 1].b32.s0 = c;
    *NODE_type(p as usize) = MARK_NODE;
    MEM[p].b16.s0 = 0;
    MEM[p + 1].b32.s1 = def_ref as i32;
    MEM[cur_list.tail].b32.s1 = p as i32;
    cur_list.tail = p;
}
pub(crate) unsafe fn append_penalty() {
    scan_int();
    MEM[cur_list.tail].b32.s1 = new_penalty(cur_val) as i32;
    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    if cur_list.mode == VMODE {
        build_page();
    };
}
pub(crate) unsafe fn delete_last() {
    if cur_list.mode == VMODE && cur_list.tail == cur_list.head {
        /*1141: */
        if cur_chr != GLUE_NODE as i32 || last_glue != MAX_HALFWORD {
            you_cant();
            help_ptr = 2;
            help_line[1] = b"Sorry...I usually can\'t take things from the current page.";
            help_line[0] = b"Try `I\\vskip-\\lastskip\' instead.";
            if cur_chr == KERN_NODE as i32 {
                help_line[0] = b"Try `I\\kern-\\lastkern\' instead."
            } else if cur_chr != GLUE_NODE as i32 {
                help_line[0] = b"Perhaps you can make the output routine do it."
            }
            error();
        }
    } else {
        let mut tx = cur_list.tail as i32;
        if !is_char_node(tx) {
            if *NODE_type(tx as usize) == MATH_NODE && MEM[tx as usize].b16.s0 == END_M_CODE {
                let mut r = cur_list.head as i32;
                let mut q;
                loop {
                    q = r;
                    r = MEM[q as usize].b32.s1;
                    if r == tx {
                        break;
                    }
                }
                tx = q;
            }
        }
        if !is_char_node(tx) {
            if MEM[tx as usize].b16.s1 as i32 == cur_chr {
                let mut fm;
                let mut q = cur_list.head as i32;
                let mut p = TEX_NULL;
                let mut r;
                loop {
                    r = p;
                    p = q;
                    fm = false;
                    if !is_char_node(q) {
                        if *NODE_type(q as usize) == DISC_NODE {
                            let mut for_end: i32 = 0;
                            let mut m = 1_u16;
                            for_end = MEM[q as usize].b16.s0 as i32;
                            if m as i32 <= for_end {
                                loop {
                                    p = *LLIST_link(p as usize);
                                    let fresh77 = m;
                                    m = m.wrapping_add(1);
                                    if !((fresh77 as i32) < for_end) {
                                        break;
                                    }
                                }
                            }
                            if p == tx {
                                return;
                            }
                        } else if *NODE_type(q as usize) == MATH_NODE
                            && MEM[q as usize].b16.s0 == BEGIN_M_CODE
                        {
                            fm = true;
                        }
                    }
                    q = MEM[p as usize].b32.s1;
                    if q == tx {
                        break;
                    }
                }
                q = MEM[tx as usize].b32.s1;
                MEM[p as usize].b32.s1 = q;
                MEM[tx as usize].b32.s1 = TEX_NULL;
                if q.is_texnull() {
                    if fm {
                        confusion(b"tail1");
                    } else {
                        cur_list.tail = p as usize;
                    }
                } else if fm {
                    cur_list.tail = r as usize;
                    MEM[r as usize].b32.s1 = TEX_NULL;
                    flush_node_list(p);
                }
                flush_node_list(tx);
            }
        }
    };
}
pub(crate) unsafe fn unpackage() {
    let mut p: i32 = 0;
    let mut r: i32 = 0;
    let mut c: u8 = 0;
    if cur_chr > COPY_CODE {
        /*1651: */
        MEM[cur_list.tail].b32.s1 = disc_ptr[cur_chr as usize]; /*:1156 */
        disc_ptr[cur_chr as usize] = TEX_NULL;
    } else {
        c = cur_chr as u8;
        scan_register_num();
        if cur_val < 256 {
            p = *BOX_REG(cur_val as usize)
        } else {
            find_sa_element(4 as small_number, cur_val, false);
            if cur_ptr.is_texnull() {
                p = TEX_NULL;
            } else {
                p = MEM[(cur_ptr + 1) as usize].b32.s1;
            }
        }
        if p.is_texnull() {
            return;
        }
        if cur_list.mode.abs() == MMODE
            || cur_list.mode.abs() == VMODE && *NODE_type(p as usize) != VLIST_NODE
            || cur_list.mode.abs() == HMODE && *NODE_type(p as usize) != HLIST_NODE
        {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Incompatible list can\'t be unboxed");
            help_ptr = 3_u8;
            help_line[2] = b"Sorry, Pandora. (You sneaky devil.)";
            help_line[1] = b"I refuse to unbox an \\hbox in vertical mode or vice versa.";
            help_line[0] = b"And I can\'t open any boxes in math mode.";
            error();
            return;
        }
        if c == COPY_CODE as u8 {
            MEM[cur_list.tail].b32.s1 = copy_node_list(MEM[(p + 5) as usize].b32.s1)
        } else {
            MEM[cur_list.tail].b32.s1 = MEM[(p + 5) as usize].b32.s1;
            if cur_val < 256 {
                *BOX_REG(cur_val as usize) = TEX_NULL
            } else {
                find_sa_element(4, cur_val, false);
                if !cur_ptr.is_texnull() {
                    MEM[(cur_ptr + 1) as usize].b32.s1 = TEX_NULL;
                    MEM[(cur_ptr + 1) as usize].b32.s0 += 1;
                    delete_sa_ref(cur_ptr as usize);
                }
            }
            free_node(p as usize, BOX_NODE_SIZE);
        }
    }
    while !MEM[cur_list.tail].b32.s1.is_texnull() {
        r = MEM[cur_list.tail].b32.s1;
        if !is_char_node(r) && *NODE_type(r as usize) == MARGIN_KERN_NODE {
            MEM[cur_list.tail].b32.s1 = MEM[r as usize].b32.s1;
            free_node(r as usize, MARGIN_KERN_NODE_SIZE);
        }
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    }
}
pub(crate) unsafe fn append_italic_correction() {
    let mut p: i32 = 0;
    let mut f: internal_font_number = 0;
    if cur_list.tail != cur_list.head {
        if is_char_node(cur_list.tail as i32) {
            p = cur_list.tail as i32
        } else if MEM[cur_list.tail].b16.s1 == LIGATURE_NODE {
            p = cur_list.tail as i32 + 1;
        } else if *NODE_type(cur_list.tail) == WHATSIT_NODE {
            if MEM[cur_list.tail].b16.s0 == NATIVE_WORD_NODE
                || MEM[cur_list.tail].b16.s0 == NATIVE_WORD_NODE_AT
            {
                MEM[cur_list.tail].b32.s1 = new_kern(real_get_native_italic_correction(
                    &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                )) as i32;
                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                *NODE_subtype(cur_list.tail) = EXPLICIT
            } else if MEM[cur_list.tail].b16.s0 == GLYPH_NODE {
                MEM[cur_list.tail].b32.s1 = new_kern(real_get_native_glyph_italic_correction(
                    &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                )) as i32;
                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                *NODE_subtype(cur_list.tail) = EXPLICIT;
            }
            return;
        } else {
            return;
        }
        f = *CHAR_NODE_font(p as usize) as internal_font_number;
        MEM[cur_list.tail].b32.s1 = new_kern(*FONT_CHARINFO_ITALCORR(
            f,
            FONT_CHARACTER_INFO(
                f,
                effective_char(true, f, *CHAR_NODE_character(p as usize)) as usize,
            ),
        )) as i32;
        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
        MEM[cur_list.tail].b16.s0 = EXPLICIT
    };
}
pub(crate) unsafe fn append_discretionary() {
    MEM[cur_list.tail].b32.s1 = new_disc() as i32;
    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
    if cur_chr == 1 {
        let c = HYPHEN_CHAR[EQTB[CUR_FONT_LOC].b32.s1 as usize];
        if c >= 0 {
            if c <= BIGGEST_CHAR {
                MEM[cur_list.tail + 1].b32.s0 =
                    new_character(EQTB[CUR_FONT_LOC].b32.s1 as usize, c as UTF16_code).tex_int()
            }
        }
    } else {
        SAVE_PTR += 1;
        SAVE_STACK[SAVE_PTR - 1].b32.s1 = 0;
        new_save_level(GroupCode::DISC);
        scan_left_brace();
        push_nest();
        cur_list.mode = -104;
        cur_list.aux.b32.s0 = 1000;
    };
}
pub(crate) unsafe fn build_discretionary() {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut n: i32 = 0;
    unsave();
    q = cur_list.head as i32;
    p = MEM[q as usize].b32.s1;
    n = 0;
    while !p.is_texnull() {
        if !is_char_node(p) {
            if *NODE_type(p as usize) > RULE_NODE {
                if *NODE_type(p as usize) != KERN_NODE {
                    if *NODE_type(p as usize) != LIGATURE_NODE {
                        if *NODE_type(p as usize) != WHATSIT_NODE
                            || MEM[p as usize].b16.s0 != NATIVE_WORD_NODE
                                && MEM[p as usize].b16.s0 != NATIVE_WORD_NODE_AT
                                && MEM[p as usize].b16.s0 != GLYPH_NODE
                        {
                            if file_line_error_style_p != 0 {
                                print_file_line();
                            } else {
                                print_nl_cstr(b"! ");
                            }
                            print_cstr(b"Improper discretionary list");
                            help_ptr = 1;
                            help_line[0] =
                                b"Discretionary lists must contain only boxes and kerns.";
                            error();
                            begin_diagnostic();
                            print_nl_cstr(b"The following discretionary sublist has been deleted:");
                            show_box(p);
                            end_diagnostic(true);
                            flush_node_list(p);
                            MEM[q as usize].b32.s1 = TEX_NULL;
                            break;
                        }
                    }
                }
            }
        }
        q = p;
        p = MEM[q as usize].b32.s1;
        n += 1;
    }
    p = MEM[cur_list.head].b32.s1;
    pop_nest();
    match SAVE_STACK[SAVE_PTR - 1].b32.s1 {
        0 => MEM[cur_list.tail + 1].b32.s0 = p,
        1 => MEM[cur_list.tail + 1].b32.s1 = p,
        2 => {
            if n > 0 && cur_list.mode.abs() == MMODE {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Illegal math ");
                print_esc_cstr(b"discretionary");
                help_ptr = 2;
                help_line[1] = b"Sorry: The third part of a discretionary break must be";
                help_line[0] = b"empty, in math formulas. I had to delete your third part.";
                flush_node_list(p);
                n = 0;
                error();
            } else {
                MEM[cur_list.tail].b32.s1 = p
            }
            if n <= u16::MAX as i32 {
                MEM[cur_list.tail].b16.s0 = n as u16
            } else {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Discretionary list is too long");
                help_ptr = 2;
                help_line[1] = b"Wow---I never thought anybody would tweak me here.";
                help_line[0] = b"You can\'t seriously need such a huge discretionary list?";
                error();
            }
            if n > 0 {
                cur_list.tail = q as usize;
            }
            SAVE_PTR -= 1;
            return;
        }
        _ => {}
    }
    SAVE_STACK[SAVE_PTR - 1].b32.s1 += 1;
    new_save_level(GroupCode::DISC);
    scan_left_brace();
    push_nest();
    cur_list.mode = -104;
    cur_list.aux.b32.s0 = 1000;
}
pub(crate) unsafe fn make_accent() {
    scan_char_num();
    let mut f = EQTB[CUR_FONT_LOC].b32.s1 as usize;
    if let Some(mut p) = new_character(f, cur_val as UTF16_code) {
        let mut lsb: scaled_t = 0;
        let mut rsb: scaled_t = 0;
        let x = FONT_INFO[(X_HEIGHT_CODE + PARAM_BASE[f]) as usize].b32.s1;
        let s = FONT_INFO[(SLANT_CODE + PARAM_BASE[f]) as usize].b32.s1 as f64 / 65536.;
        let a = if FONT_AREA[f] as u32 == AAT_FONT_FLAG || FONT_AREA[f] as u32 == OTGR_FONT_FLAG {
            let a = MEM[p + 1].b32.s1;
            if a == 0 {
                get_native_char_sidebearings(f, cur_val, &mut lsb, &mut rsb);
            }
            a
        } else {
            *FONT_CHARACTER_WIDTH(
                f,
                effective_char(true, f, *CHAR_NODE_character(p as usize)) as usize,
            )
        };
        do_assignments();
        let mut q = None;
        f = EQTB[CUR_FONT_LOC].b32.s1 as usize;
        if cur_cmd == LETTER as u8 || cur_cmd == OTHER_CHAR as u8 || cur_cmd == CHAR_GIVEN as u8 {
            q = new_character(f, cur_chr as UTF16_code);
            cur_val = cur_chr
        } else if cur_cmd == CHAR_NUM as u8 {
            scan_char_num();
            q = new_character(f, cur_val as UTF16_code)
        } else {
            back_input();
        }
        if let Some(q) = q {
            /*1160: */
            let mut delta: scaled_t = 0;
            let mut h: scaled_t = 0;
            let mut w: scaled_t = 0;
            let mut i: b16x4 = b16x4 {
                s0: 0,
                s1: 0,
                s2: 0,
                s3: 0,
            };
            let t = FONT_INFO[(SLANT_CODE + PARAM_BASE[f]) as usize].b32.s1 as f64 / 65536.;
            if FONT_AREA[f] as u32 == AAT_FONT_FLAG || FONT_AREA[f] as u32 == OTGR_FONT_FLAG {
                w = MEM[q + 1].b32.s1;
                get_native_char_height_depth(f, cur_val, &mut h, &mut delta); // `delta` never used, BUG?
            } else {
                i = FONT_CHARACTER_INFO(
                    f,
                    effective_char(true, f, *CHAR_NODE_character(q)) as usize,
                );
                w = *FONT_CHARINFO_WIDTH(f, i);
                h = *FONT_CHARINFO_HEIGHT(f, i);
            }
            if h != x {
                p = hpack(p as i32, 0, ADDITIONAL as small_number) as usize;
                MEM[p + 4].b32.s1 = x - h;
            }
            let delta = if (FONT_AREA[f] as u32 == AAT_FONT_FLAG
                || FONT_AREA[f] as u32 == OTGR_FONT_FLAG)
                && a == 0
            {
                tex_round((w - lsb + rsb) as f64 / 2. + h as f64 * t - x as f64 * s)
            } else {
                tex_round((w - a) as f64 / 2. + h as f64 * t - x as f64 * s)
            };
            let r = new_kern(delta);
            *NODE_subtype(r as usize) = ACC_KERN;
            MEM[cur_list.tail].b32.s1 = r as i32;
            MEM[r as usize].b32.s1 = p as i32;
            cur_list.tail = new_kern(-a - delta);
            MEM[cur_list.tail].b16.s0 = ACC_KERN;
            MEM[p].b32.s1 = cur_list.tail as i32;
            p = q;
        }
        MEM[cur_list.tail].b32.s1 = p as i32;
        cur_list.tail = p;
        cur_list.aux.b32.s0 = 1000;
    };
}
pub(crate) unsafe fn align_error() {
    if align_state.abs() > 2 {
        /*1163: */
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Misplaced ");
        print_cmd_chr(cur_cmd as u16, cur_chr);
        if cur_tok == TAB_TOKEN + 38 {
            help_ptr = 6;
            help_line[5] = b"I can\'t figure out why you would want to use a tab mark";
            help_line[4] = b"here. If you just want an ampersand, the remedy is";
            help_line[3] = b"simple: Just type `I\\&\' now. But if some right brace";
            help_line[2] = b"up above has ended a previous alignment prematurely,";
            help_line[1] = b"you\'re probably due for more error messages, and you";
            help_line[0] = b"might try typing `S\' now just to see what is salvageable."
        } else {
            help_ptr = 5;
            help_line[4] = b"I can\'t figure out why you would want to use a tab mark";
            help_line[3] = b"or \\cr or \\span just now. If something like a right brace";
            help_line[2] = b"up above has ended a previous alignment prematurely,";
            help_line[1] = b"you\'re probably due for more error messages, and you";
            help_line[0] = b"might try typing `S\' now just to see what is salvageable."
        }
        error();
    } else {
        back_input();
        if align_state < 0 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Missing { inserted");
            align_state += 1;
            cur_tok = LEFT_BRACE_TOKEN + 123;
        } else {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Missing } inserted");
            align_state -= 1;
            cur_tok = RIGHT_BRACE_TOKEN + 125;
        }
        help_ptr = 3;
        help_line[2] = b"I\'ve put in what seems to be necessary to fix";
        help_line[1] = b"the current column of the current alignment.";
        help_line[0] = b"Try to go on, since this might almost work.";
        ins_error();
    };
}
pub(crate) unsafe fn no_align_error() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Misplaced ");
    print_esc_cstr(b"noalign");
    help_ptr = 2;
    help_line[1] = b"I expect to see \\noalign only after the \\cr of";
    help_line[0] = b"an alignment. Proceed, and I\'ll ignore this case.";
    error();
}
pub(crate) unsafe fn omit_error() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Misplaced ");
    print_esc_cstr(b"omit");
    help_ptr = 2;
    help_line[1] = b"I expect to see \\omit only after tab marks or the \\cr of";
    help_line[0] = b"an alignment. Proceed, and I\'ll ignore this case.";
    error();
}
pub(crate) unsafe fn do_endv() {
    BASE_PTR = INPUT_PTR;
    INPUT_STACK[BASE_PTR] = cur_input;
    while INPUT_STACK[BASE_PTR].index != V_TEMPLATE
        && INPUT_STACK[BASE_PTR].loc.is_texnull()
        && INPUT_STACK[BASE_PTR].state == TOKEN_LIST
    {
        BASE_PTR -= 1
    }
    if INPUT_STACK[BASE_PTR].index != V_TEMPLATE
        || !INPUT_STACK[BASE_PTR].loc.is_texnull()
        || INPUT_STACK[BASE_PTR].state != TOKEN_LIST
    {
        fatal_error(b"(interwoven alignment preambles are not allowed)");
    }
    if cur_group == GroupCode::ALIGN {
        end_graf();
        if fin_col() {
            fin_row();
        }
    } else {
        off_save();
    };
}
pub(crate) unsafe fn cs_error() {
    if file_line_error_style_p != 0 {
        print_file_line();
    } else {
        print_nl_cstr(b"! ");
    }
    print_cstr(b"Extra ");
    print_esc_cstr(b"endcsname");
    help_ptr = 1;
    help_line[0] = b"I\'m ignoring this, since I wasn\'t doing a \\csname.";
    error();
}
pub(crate) unsafe fn push_math(c: GroupCode) {
    push_nest();
    cur_list.mode = -207;
    cur_list.aux.b32.s1 = TEX_NULL;
    new_save_level(c);
}
pub(crate) unsafe fn just_copy(mut p: i32, mut h: usize, mut t: i32) {
    while !p.is_texnull() {
        let mut current_block_50: u64;
        let mut r: i32 = 0;
        let mut words = 1_u8;
        if is_char_node(p) {
            r = get_avail() as i32;
            current_block_50 = 2500484646272006982;
        } else {
            match *NODE_type(p as usize) {
                HLIST_NODE | VLIST_NODE => {
                    r = get_node(BOX_NODE_SIZE) as i32;
                    *SYNCTEX_tag(r as usize, BOX_NODE_SIZE) =
                        *SYNCTEX_tag(p as usize, BOX_NODE_SIZE);
                    *SYNCTEX_line(r as usize, BOX_NODE_SIZE) =
                        *SYNCTEX_line(p as usize, BOX_NODE_SIZE);
                    MEM[(r + 6) as usize] = MEM[(p + 6) as usize];
                    MEM[(r + 5) as usize] = MEM[(p + 5) as usize];
                    words = 5;
                    *BOX_list_ptr(r as usize) = TEX_NULL;
                    current_block_50 = 2500484646272006982;
                }
                RULE_NODE => {
                    r = get_node(RULE_NODE_SIZE) as i32;
                    words = RULE_NODE_SIZE as u8;
                    current_block_50 = 2500484646272006982;
                }
                LIGATURE_NODE => {
                    r = get_avail() as i32;
                    MEM[r as usize] = MEM[(p + 1) as usize];
                    current_block_50 = 1668590571950580537;
                }
                KERN_NODE | MATH_NODE => {
                    words = MEDIUM_NODE_SIZE as u8;
                    r = get_node(words as i32) as i32;
                    current_block_50 = 2500484646272006982;
                }
                GLUE_NODE => {
                    r = get_node(MEDIUM_NODE_SIZE) as i32;
                    *GLUE_SPEC_ref_count(*GLUE_NODE_glue_ptr(p as usize) as usize) += 1;
                    *SYNCTEX_tag(r as usize, MEDIUM_NODE_SIZE) =
                        *SYNCTEX_tag(p as usize, MEDIUM_NODE_SIZE);
                    *SYNCTEX_line(r as usize, MEDIUM_NODE_SIZE) =
                        *SYNCTEX_line(p as usize, MEDIUM_NODE_SIZE);
                    *GLUE_NODE_glue_ptr(r as usize) = *GLUE_NODE_glue_ptr(p as usize);
                    *GLUE_NODE_leader_ptr(r as usize) = TEX_NULL;
                    current_block_50 = 2500484646272006982;
                }
                WHATSIT_NODE => {
                    match *NODE_subtype(p as usize) {
                        OPEN_NODE => {
                            r = get_node(OPEN_NODE_SIZE) as i32;
                            words = OPEN_NODE_SIZE as u8;
                        }
                        WRITE_NODE | SPECIAL_NODE => {
                            r = get_node(WRITE_NODE_SIZE) as i32;
                            MEM[MEM[(p + 1) as usize].b32.s1 as usize].b32.s0 += 1;
                            words = WRITE_NODE_SIZE as u8;
                        }
                        CLOSE_NODE | LANGUAGE_NODE => {
                            r = get_node(SMALL_NODE_SIZE) as i32;
                            words = SMALL_NODE_SIZE as u8;
                        }
                        NATIVE_WORD_NODE | NATIVE_WORD_NODE_AT => {
                            words = *NATIVE_NODE_size(p as usize) as u8;
                            r = get_node(words as i32) as i32;

                            while words > 0 {
                                words -= 1;
                                MEM[(r + words as i32) as usize] = MEM[(p + words as i32) as usize]
                            }

                            *NATIVE_NODE_glyph_info_ptr(r as usize) = 0 as *mut libc::c_void;
                            *NATIVE_NODE_glyph_count(r as usize) = 0;
                            copy_native_glyph_info(p as usize, r as usize);
                        }
                        GLYPH_NODE => {
                            r = get_node(GLYPH_NODE_SIZE) as i32;
                            words = GLYPH_NODE_SIZE as u8;
                        }
                        PIC_NODE | PDF_NODE => {
                            words = (9i32 as u64).wrapping_add(
                                (MEM[(p + 4) as usize].b16.s1 as u64)
                                    .wrapping_add(::std::mem::size_of::<memory_word>() as u64)
                                    .wrapping_sub(1i32 as u64)
                                    .wrapping_div(::std::mem::size_of::<memory_word>() as u64),
                            ) as u8;
                            r = get_node(words as i32) as i32;
                        }
                        PDF_SAVE_POS_NODE => r = get_node(SMALL_NODE_SIZE) as i32,
                        _ => confusion(b"ext2"),
                    }
                    current_block_50 = 2500484646272006982;
                }
                _ => current_block_50 = 17768496421797376910,
            }
        }
        match current_block_50 {
            2500484646272006982 => {
                while words > 0 {
                    words -= 1;
                    MEM[(r + words as i32) as usize] = MEM[(p + words as i32) as usize]
                }
                current_block_50 = 1668590571950580537;
            }
            _ => {}
        }
        match current_block_50 {
            1668590571950580537 => {
                *LLIST_link(h) = r;
                h = r as usize
            }
            _ => {}
        }
        p = *LLIST_link(p as usize)
    }
    *LLIST_link(h) = t;
}
pub(crate) unsafe fn just_reverse(mut p: i32) {
    let mut q;
    let mut m = MIN_HALFWORD;
    let mut n = MIN_HALFWORD;
    if MEM[TEMP_HEAD].b32.s1.is_texnull() {
        just_copy(MEM[p as usize].b32.s1, TEMP_HEAD, TEX_NULL);
        q = MEM[TEMP_HEAD].b32.s1
    } else {
        q = MEM[p as usize].b32.s1;
        MEM[p as usize].b32.s1 = TEX_NULL;
        flush_node_list(MEM[TEMP_HEAD].b32.s1);
    }
    let mut t = new_edge(cur_dir, 0);
    let mut l = t as i32;
    cur_dir = (1 - cur_dir as i32) as small_number;
    while !q.is_texnull() {
        if is_char_node(q) {
            loop {
                p = q;
                q = MEM[p as usize].b32.s1;
                MEM[p as usize].b32.s1 = l;
                l = p;
                if !is_char_node(q) {
                    break;
                }
            }
        } else {
            p = q;
            q = MEM[p as usize].b32.s1;
            if *NODE_type(p as usize) == MATH_NODE {
                /*1527: */
                if MEM[p as usize].b16.s0 as i32 & 1 != 0 {
                    if MEM[LR_ptr as usize].b32.s0
                        != (L_CODE as i32) * (MEM[p as usize].b16.s0 as i32 / (L_CODE as i32)) + 3
                    {
                        *NODE_type(p as usize) = KERN_NODE;
                        LR_problems += 1;
                    } else {
                        temp_ptr = LR_ptr;
                        LR_ptr = MEM[temp_ptr as usize].b32.s1;
                        MEM[temp_ptr as usize].b32.s1 = avail;
                        avail = temp_ptr;
                        if n > MIN_HALFWORD {
                            n -= 1;
                            MEM[p as usize].b16.s0 -= 1;
                        } else if m > MIN_HALFWORD {
                            m -= 1;
                            *NODE_type(p as usize) = KERN_NODE;
                        } else {
                            MEM[t + 1].b32.s1 = MEM[(p + 1) as usize].b32.s1;
                            MEM[t].b32.s1 = q;
                            free_node(p as usize, MEDIUM_NODE_SIZE);
                            break;
                        }
                    }
                } else {
                    temp_ptr = get_avail() as i32;
                    MEM[temp_ptr as usize].b32.s0 =
                        (L_CODE as i32) * (MEM[p as usize].b16.s0 as i32 / (L_CODE as i32)) + 3;
                    MEM[temp_ptr as usize].b32.s1 = LR_ptr;
                    LR_ptr = temp_ptr;
                    if n > MIN_HALFWORD
                        || MEM[p as usize].b16.s0 as i32 / (R_CODE as i32) != cur_dir as i32
                    {
                        n += 1;
                        MEM[p as usize].b16.s0 += 1;
                    } else {
                        *NODE_type(p as usize) = KERN_NODE;
                        m += 1
                    }
                }
            }
            MEM[p as usize].b32.s1 = l;
            l = p
        }
    }
    MEM[TEMP_HEAD].b32.s1 = l;
}
pub(crate) unsafe fn get_r_token() {
    loop {
        loop {
            get_token();
            if !(cur_tok == SPACE_TOKEN) {
                break;
            }
        }
        if !(cur_cs == 0
            || cur_cs > EQTB_TOP as i32
            || cur_cs > FROZEN_CONTROL_SEQUENCE as i32 && cur_cs <= EQTB_SIZE as i32)
        {
            break;
        }
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Missing control sequence inserted");
        help_ptr = 5;
        help_line[4] = b"Please don\'t say `\\def cs{...}\', say `\\def\\cs{...}\'.";
        help_line[3] = b"I\'ve inserted an inaccessible control sequence so that your";
        help_line[2] = b"definition will be completed without mixing me up too badly.";
        help_line[1] = b"You can recover graciously from this error, if you\'re";
        help_line[0] = b"careful; see exercise 27.2 in The TeXbook.";
        if cur_cs == 0 {
            back_input();
        }
        cur_tok = CS_TOKEN_FLAG + FROZEN_PROTECTION as i32;
        ins_error();
    }
}
pub(crate) unsafe fn trap_zero_glue() {
    if MEM[(cur_val + 1) as usize].b32.s1 == 0
        && MEM[(cur_val + 2) as usize].b32.s1 == 0
        && MEM[(cur_val + 3) as usize].b32.s1 == 0
    {
        *GLUE_SPEC_ref_count(0) += 1;
        delete_glue_ref(cur_val as usize);
        cur_val = 0;
    };
}
pub(crate) unsafe fn do_register_command(mut a: small_number) {
    let mut current_block: u64;
    let mut l: i32 = TEX_NULL;
    let mut p: u8 = 0;
    let mut q = cur_cmd as usize;
    let mut e = false;
    if q != REGISTER as usize {
        get_x_token();
        if cur_cmd >= ASSIGN_INT as u8 && cur_cmd <= ASSIGN_MU_GLUE as u8 {
            l = cur_chr;
            p = cur_cmd - ASSIGN_INT as u8;
            current_block = 16534065480145571271;
        } else {
            if cur_cmd != REGISTER as u8 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"You can\'t use `");
                print_cmd_chr(cur_cmd as u16, cur_chr);
                print_cstr(b"\' after ");
                print_cmd_chr(q as u16, 0);
                help_ptr = 1;
                help_line[0] = b"I\'m forgetting what you said and not changing anything.";
                error();
                return;
            }
            current_block = 4808432441040389987;
        }
    } else {
        current_block = 4808432441040389987;
    }
    match current_block {
        4808432441040389987 => {
            if cur_chr < 0 || cur_chr > 19 {
                /*lo_mem_stat_max*/
                l = cur_chr;
                p = (MEM[l as usize].b16.s1 as i32 / 64) as u8;
                e = true
            } else {
                p = cur_chr as u8;
                scan_register_num();
                if cur_val > 255 {
                    find_sa_element(p as small_number, cur_val, true);
                    l = cur_ptr;
                    e = true
                } else {
                    match p {
                        INT_VAL => l = cur_val + COUNT_BASE as i32,
                        DIMEN_VAL => l = cur_val + SCALED_BASE as i32,
                        GLUE_VAL => l = cur_val + SKIP_BASE as i32,
                        MU_VAL => l = cur_val + MU_SKIP_BASE as i32,
                        _ => {}
                    }
                }
            }
        }
        _ => {}
    }
    let mut w = 0i32;
    let mut s = TEX_NULL;
    if p < GLUE_VAL {
        if e {
            w = MEM[(l + 2) as usize].b32.s1
        } else {
            w = EQTB[l as usize].b32.s1
        }
    } else if e {
        s = MEM[(l + 1) as usize].b32.s1
    } else {
        s = EQTB[l as usize].b32.s1
        /*:1272*/
    } /*1275:*/
    if q == REGISTER as usize {
        scan_optional_equals();
    } else {
        scan_keyword(b"by");
    }
    arith_error = false;
    if q < MULTIPLY as usize {
        /*1273:*/
        if p < GLUE_VAL {
            if p == INT_VAL {
                scan_int();
            } else {
                scan_dimen(false, false, false);
            }
            if q == ADVANCE as usize {
                cur_val = cur_val + w
            }
        } else {
            scan_glue(p as small_number);
            if q == ADVANCE as usize {
                /*1274:*/
                q = new_spec(cur_val as usize) as usize;
                let r = s;
                delete_glue_ref(cur_val as usize);
                MEM[q + 1].b32.s1 = MEM[q + 1].b32.s1 + MEM[(r + 1) as usize].b32.s1;
                if MEM[q + 2].b32.s1 == 0 {
                    MEM[q].b16.s1 = NORMAL
                }
                if MEM[q].b16.s1 == MEM[r as usize].b16.s1 {
                    MEM[q + 2].b32.s1 = MEM[q + 2].b32.s1 + MEM[(r + 2) as usize].b32.s1
                } else if MEM[q].b16.s1 < MEM[r as usize].b16.s1
                    && MEM[(r + 2) as usize].b32.s1 != 0
                {
                    MEM[q + 2].b32.s1 = MEM[(r + 2) as usize].b32.s1;
                    MEM[q].b16.s1 = MEM[r as usize].b16.s1
                }
                if MEM[q + 3].b32.s1 == 0 {
                    MEM[q].b16.s0 = NORMAL
                }
                if MEM[q].b16.s0 == MEM[r as usize].b16.s0 {
                    MEM[q + 3].b32.s1 = MEM[q + 3].b32.s1 + MEM[(r + 3) as usize].b32.s1
                } else if MEM[q].b16.s0 < MEM[r as usize].b16.s0
                    && MEM[(r + 3) as usize].b32.s1 != 0
                {
                    MEM[q + 3].b32.s1 = MEM[(r + 3) as usize].b32.s1;
                    MEM[q].b16.s0 = MEM[r as usize].b16.s0
                }
                cur_val = q as i32;
            }
        }
    } else {
        scan_int();
        if p < GLUE_VAL {
            if q == MULTIPLY as usize {
                if p == INT_VAL {
                    cur_val = mult_and_add(w, cur_val, 0, TEX_INFINITY)
                } else {
                    cur_val = mult_and_add(w, cur_val, 0, MAX_HALFWORD)
                }
            } else {
                cur_val = x_over_n(w, cur_val)
            }
        } else {
            let r = new_spec(s as usize);
            if q == MULTIPLY as usize {
                MEM[r + 1].b32.s1 =
                    mult_and_add(MEM[(s + 1) as usize].b32.s1, cur_val, 0, MAX_HALFWORD);
                MEM[r + 2].b32.s1 =
                    mult_and_add(MEM[(s + 2) as usize].b32.s1, cur_val, 0, MAX_HALFWORD);
                MEM[r + 3].b32.s1 =
                    mult_and_add(MEM[(s + 3) as usize].b32.s1, cur_val, 0, MAX_HALFWORD)
            } else {
                MEM[r + 1].b32.s1 = x_over_n(MEM[(s + 1) as usize].b32.s1, cur_val);
                MEM[r + 2].b32.s1 = x_over_n(MEM[(s + 2) as usize].b32.s1, cur_val);
                MEM[r + 3].b32.s1 = x_over_n(MEM[(s + 3) as usize].b32.s1, cur_val)
            }
            cur_val = r as i32;
        }
    }
    if arith_error {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Arithmetic overflow");
        help_ptr = 2;
        help_line[1] = b"I can\'t carry out that multiplication or division,";
        help_line[0] = b"since the result is out of range.";
        if p >= GLUE_VAL {
            delete_glue_ref(cur_val as usize);
        }
        error();
        return;
    }
    if p < GLUE_VAL {
        if e {
            if a >= 4 {
                gsa_w_def(l as usize, cur_val);
            } else {
                sa_w_def(l as usize, cur_val);
            }
        } else if a >= 4 {
            geq_word_define(l as usize, cur_val);
        } else {
            eq_word_define(l as usize, cur_val);
        }
    } else {
        trap_zero_glue();
        if e {
            if a >= 4 {
                gsa_def(l as usize, cur_val);
            } else {
                sa_def(l as usize, cur_val);
            }
        } else if a >= 4 {
            geq_define(l as usize, GLUE_REF, cur_val);
        } else {
            eq_define(l as usize, GLUE_REF, cur_val);
        }
    };
}
pub(crate) unsafe fn alter_aux() {
    if cur_chr != (cur_list.mode as i32).abs() {
        report_illegal_case();
    } else {
        let c = cur_chr;
        scan_optional_equals();
        if c == VMODE as i32 {
            scan_dimen(false, false, false);
            cur_list.aux.b32.s1 = cur_val
        } else {
            scan_int();
            if cur_val <= 0 || cur_val > 32767 {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Bad space factor");
                help_ptr = 1;
                help_line[0] = b"I allow only values in the range 1..32767 here.";
                int_error(cur_val);
            } else {
                cur_list.aux.b32.s0 = cur_val;
            }
        }
    };
}
pub(crate) unsafe fn alter_prev_graf() {
    NEST[NEST_PTR] = cur_list;
    let mut p = NEST_PTR;
    while NEST[p as usize].mode.abs() != VMODE {
        p -= 1;
    }
    scan_optional_equals();
    scan_int();
    if cur_val < 0 {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"Bad ");
        print_esc_cstr(b"prevgraf");
        help_ptr = 1;
        help_line[0] = b"I allow only nonnegative values here.";
        int_error(cur_val);
    } else {
        NEST[p as usize].prev_graf = cur_val;
        cur_list = NEST[NEST_PTR]
    };
}
pub(crate) unsafe fn alter_page_so_far() {
    let c = cur_chr as u8 as usize;
    scan_optional_equals();
    scan_dimen(false, false, false);
    page_so_far[c] = cur_val;
}
pub(crate) unsafe fn alter_integer() {
    let mut c: small_number = 0;
    c = cur_chr as small_number;
    scan_optional_equals();
    scan_int();
    if c == 0 {
        dead_cycles = cur_val
    } else if c == 2 {
        if cur_val < BATCH_MODE || cur_val > ERROR_STOP_MODE as i32 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Bad interaction mode");
            help_ptr = 2;
            help_line[1] = b"Modes are 0=batch, 1=nonstop, 2=scroll, and";
            help_line[0] = b"3=errorstop. Proceed, and I\'ll ignore this case.";
            int_error(cur_val);
        } else {
            cur_chr = cur_val;
            new_interaction();
        }
    } else {
        insert_penalties = cur_val
    };
}
pub(crate) unsafe fn alter_box_dimen() {
    let c = cur_chr as small_number;
    scan_register_num();
    let b = if cur_val < 256 {
        *BOX_REG(cur_val as usize)
    } else {
        find_sa_element(4i32 as small_number, cur_val, false);
        if cur_ptr.is_texnull() {
            TEX_NULL
        } else {
            MEM[(cur_ptr + 1) as usize].b32.s1
        }
    };
    scan_optional_equals();
    scan_dimen(false, false, false);
    if !b.is_texnull() {
        MEM[(b + c as i32) as usize].b32.s1 = cur_val;
    };
}
pub(crate) unsafe fn new_font(mut a: small_number) {
    let mut current_block: u64;
    let mut s: scaled_t = 0;
    if job_name == 0 {
        open_log_file();
    }
    get_r_token();
    let u = cur_cs;
    let mut t = if u >= HASH_BASE as i32 {
        (*hash.offset(u as isize)).s1
    } else if u >= SINGLE_BASE as i32 {
        if u == NULL_CS as i32 {
            maketexstring(b"FONT")
        } else {
            u - SINGLE_BASE as i32
        }
    } else {
        let old_setting_0 = selector;
        selector = Selector::NEW_STRING;
        print_cstr(b"FONT");
        print(u - 1);
        selector = old_setting_0;
        if pool_ptr + 1 > pool_size {
            overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
        }
        make_string()
    };
    if a >= 4 {
        geq_define(u as usize, SET_FONT, FONT_BASE);
    } else {
        eq_define(u as usize, SET_FONT, FONT_BASE);
    }
    scan_optional_equals();
    scan_file_name();
    name_in_progress = true;
    if scan_keyword(b"at") {
        /*1294: */
        scan_dimen(false, false, false); /*:1293 */
        s = cur_val; /*:79 */
        if s <= 0 || s >= 0x8000000 {
            if file_line_error_style_p != 0 {
                print_file_line(); /*1318: */
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Improper `at\' size (");
            print_scaled(s);
            print_cstr(b"pt), replaced by 10pt");
            help_ptr = 2;
            help_line[1] = b"I can only handle fonts at positive sizes that are";
            help_line[0] = b"less than 2048pt, so I\'ve changed what you said to 10pt.";
            error();
            s = (10_i64 * 65536) as scaled_t
        }
    } else if scan_keyword(b"scaled") {
        scan_int();
        s = -cur_val;
        if cur_val <= 0 || cur_val > 32768 {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Illegal magnification has been changed to 1000");
            help_ptr = 1;
            help_line[0] = b"The magnification ratio must be between 1 and 32768.";
            int_error(cur_val);
            s = -1000;
        }
    } else {
        s = -1000;
    }
    name_in_progress = false;
    let mut f = FONT_BASE as usize + 1;
    let mut for_end = FONT_PTR;
    if f <= for_end {
        current_block = 17075014677070940716;
    } else {
        current_block = 6838274324784804404;
    }
    loop {
        match current_block {
            6838274324784804404 => {
                f = read_font_info(u, cur_name, cur_area, s);
                break;
            }
            _ => {
                if str_eq_str(FONT_NAME[f], cur_name) as i32 != 0
                    && (length(cur_area) == 0
                        && (FONT_AREA[f] as u32 == AAT_FONT_FLAG
                            || FONT_AREA[f] as u32 == OTGR_FONT_FLAG)
                        || str_eq_str(FONT_AREA[f], cur_area) as i32 != 0)
                {
                    if s > 0 {
                        if s == FONT_SIZE[f] {
                            break;
                        }
                    } else if FONT_SIZE[f] == xn_over_d(FONT_DSIZE[f], -s, 1000) {
                        break;
                    }
                }
                append_str(cur_area);
                append_str(cur_name);
                append_str(cur_ext);
                if str_eq_str(FONT_NAME[f], make_string()) {
                    str_ptr -= 1;
                    pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize];
                    if FONT_AREA[f] as u32 == AAT_FONT_FLAG || FONT_AREA[f] as u32 == OTGR_FONT_FLAG
                    {
                        if s > 0 {
                            if s == FONT_SIZE[f] {
                                break;
                            }
                        } else if FONT_SIZE[f] == xn_over_d(FONT_DSIZE[f], -s, 1000) {
                            break;
                        }
                    }
                } else {
                    str_ptr -= 1;
                    pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize]
                }
                let fresh86 = f;
                f = f + 1;
                if fresh86 < for_end {
                    current_block = 17075014677070940716;
                } else {
                    current_block = 6838274324784804404;
                }
            }
        }
    }
    if a >= 4 {
        geq_define(u as usize, SET_FONT, f as i32);
    } else {
        eq_define(u as usize, SET_FONT, f as i32);
    }
    EQTB[(FROZEN_NULL_FONT + f) as usize] = EQTB[u as usize];
    (*hash.offset((FROZEN_NULL_FONT + f) as isize)).s1 = t;
}
pub(crate) unsafe fn new_interaction() {
    print_ln();
    interaction = cur_chr as u8;
    if interaction == BATCH_MODE as u8 {
        selector = Selector::NO_PRINT
    } else {
        selector = Selector::TERM_ONLY
    }
    if log_opened {
        selector = (u8::from(selector)).wrapping_add(2).into()
    };
}
pub(crate) unsafe fn issue_message() {
    let mut c: u8 = 0;
    let mut s: str_number = 0;
    c = cur_chr as u8;
    MEM[GARBAGE].b32.s1 = scan_toks(false, true) as i32;
    let old_setting_0 = selector;
    selector = Selector::NEW_STRING;
    token_show(Some(def_ref));
    selector = old_setting_0;
    flush_list(Some(def_ref));
    if pool_ptr + 1 > pool_size {
        overflow(b"pool size", (pool_size - init_pool_ptr) as usize);
    }
    s = make_string();
    if c == 0 {
        /*1315: */
        if term_offset + length(s) > max_print_line - 2 {
            print_ln();
        } else if term_offset > 0 || file_offset > 0 {
            print_char(' ' as i32);
        }
        print(s);
        rust_stdout.as_mut().unwrap().flush().unwrap();
    } else {
        if file_line_error_style_p != 0 {
            print_file_line();
        } else {
            print_nl_cstr(b"! ");
        }
        print_cstr(b"");
        print(s);
        if !LOCAL(Local::err_help).is_texnull() {
            use_err_help = true;
        } else if long_help_seen {
            help_ptr = 1;
            help_line[0] = b"(That was another \\errmessage.)"
        } else {
            if interaction < ERROR_STOP_MODE {
                long_help_seen = true;
            }
            help_ptr = 4;
            help_line[3] = b"This error message was generated by an \\errmessage";
            help_line[2] = b"command, so I can\'t give any explicit help.";
            help_line[1] = b"Pretend that you\'re Hercule Poirot: Examine all clues,";
            help_line[0] = b"and deduce the truth by order and method."
        }
        error();
        use_err_help = false
    }
    str_ptr -= 1;
    pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize];
}
pub(crate) unsafe fn shift_case() {
    let b = cur_chr;
    let _p = scan_toks(false, false);
    let mut p = MEM[def_ref].b32.s1;
    while !p.is_texnull() {
        let t = MEM[p as usize].b32.s0;
        if t < CS_TOKEN_FLAG + SINGLE_BASE as i32 {
            let c = t % MAX_CHAR_VAL;
            if EQTB[(b + c) as usize].b32.s1 != 0 {
                MEM[p as usize].b32.s0 = t - c + EQTB[(b + c) as usize].b32.s1
            }
        }
        p = *LLIST_link(p as usize)
    }
    begin_token_list(MEM[def_ref].b32.s1 as usize, BACKED_UP);
    MEM[def_ref].b32.s1 = avail;
    avail = def_ref as i32;
}
pub(crate) unsafe fn show_whatever() {
    let mut current_block: u64;
    let mut p: i32 = 0;
    match cur_chr {
        3 => {
            begin_diagnostic();
            show_activities();
            current_block = 7330218953828964527;
        }
        1 => {
            scan_register_num();
            if cur_val < 256 {
                p = *BOX_REG(cur_val as usize)
            } else {
                find_sa_element(4, cur_val, false);
                if cur_ptr.is_texnull() {
                    p = TEX_NULL;
                } else {
                    p = MEM[(cur_ptr + 1) as usize].b32.s1;
                }
            }
            begin_diagnostic();
            print_nl_cstr(b"> \\box");
            print_int(cur_val);
            print_char('=' as i32);
            if p.is_texnull() {
                print_cstr(b"void");
            } else {
                show_box(p);
            }
            current_block = 7330218953828964527;
        }
        0 => {
            get_token();
            print_nl_cstr(b"> ");
            if cur_cs != 0 {
                sprint_cs(cur_cs);
                print_char('=' as i32);
            }
            print_meaning();
            current_block = 6249296489108783913;
        }
        4 => {
            begin_diagnostic();
            show_save_groups();
            current_block = 7330218953828964527;
        }
        6 => {
            begin_diagnostic();
            print_nl_cstr(b"");
            print_ln();
            if cond_ptr.is_texnull() {
                print_nl_cstr(b"### ");
                print_cstr(b"no active conditionals");
            } else {
                p = cond_ptr;
                let mut n = 0;
                loop {
                    n += 1;
                    p = *LLIST_link(p as usize);
                    if p.is_texnull() {
                        break;
                    }
                }
                p = cond_ptr;
                let mut t = cur_if;
                let mut l = if_line;
                let mut m = if_limit;
                loop {
                    print_nl_cstr(b"### level ");
                    print_int(n);
                    print_cstr(b": ");
                    print_cmd_chr(IF_TEST, t as i32);
                    if m == FI_CODE {
                        print_esc_cstr(b"else");
                    }
                    if l != 0i32 {
                        print_cstr(b" entered on line ");
                        print_int(l);
                    }
                    n -= 1;
                    t = MEM[p as usize].b16.s0 as small_number;
                    l = MEM[(p + 1) as usize].b32.s1;
                    m = MEM[p as usize].b16.s1 as u8;
                    p = *LLIST_link(p as usize);
                    if p.is_texnull() {
                        break;
                    }
                }
            }
            current_block = 7330218953828964527;
        }
        _ => {
            p = the_toks() as i32;
            print_nl_cstr(b"> ");
            token_show(Some(TEMP_HEAD));
            flush_list(MEM[TEMP_HEAD].b32.s1.opt());
            current_block = 6249296489108783913;
        }
    }
    match current_block {
        7330218953828964527 => {
            end_diagnostic(true);
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"OK");
            if selector == Selector::TERM_AND_LOG {
                if *INTPAR(IntPar::tracing_online) <= 0i32 {
                    selector = Selector::TERM_ONLY;
                    print_cstr(b" (see the transcript file)");
                    selector = Selector::TERM_AND_LOG
                }
            }
        }
        _ => {}
    }
    if interaction < ERROR_STOP_MODE {
        help_ptr = 0;
        error_count -= 1;
    } else if *INTPAR(IntPar::tracing_online) > 0 {
        help_ptr = 3;
        help_line[2] = b"This isn\'t an error message; I\'m just \\showing something.";
        help_line[1] = b"Type `I\\show...\' to show more (e.g., \\show\\cs,";
        help_line[0] = b"\\showthe\\count10, \\showbox255, \\showlists)."
    } else {
        help_ptr = 5;
        help_line[4] = b"This isn\'t an error message; I\'m just \\showing something.";
        help_line[3] = b"Type `I\\show...\' to show more (e.g., \\show\\cs,";
        help_line[2] = b"\\showthe\\count10, \\showbox255, \\showlists).";
        help_line[1] = b"And type `I\\tracingonline=1\\show...\' to show boxes and";
        help_line[0] = b"lists on your terminal as well as in the transcript file."
    }
    error();
}
pub(crate) unsafe fn new_write_whatsit(mut w: small_number) {
    new_whatsit(cur_chr as small_number, w);
    if w != WRITE_NODE_SIZE as i16 {
        scan_four_bit_int();
    } else {
        scan_int();
        if cur_val < 0 {
            cur_val = 17;
        } else if cur_val > 15 && cur_val != 18 {
            cur_val = 16;
        }
    }
    MEM[cur_list.tail + 1].b32.s0 = cur_val;
}
pub(crate) unsafe fn scan_and_pack_name() {
    scan_file_name();
    pack_file_name(cur_name, cur_area, cur_ext);
}
pub(crate) unsafe fn do_extension() {
    let mut i: i32 = 0;
    let mut j: i32 = 0;
    let mut p: usize = 0;
    match cur_chr {
        0 => {
            new_write_whatsit(OPEN_NODE_SIZE as small_number);
            scan_optional_equals();
            scan_file_name();
            MEM[cur_list.tail + 1].b32.s1 = cur_name;
            MEM[cur_list.tail + 2].b32.s0 = cur_area;
            MEM[cur_list.tail + 2].b32.s1 = cur_ext
        }
        1 => {
            let k = cur_cs;
            new_write_whatsit(WRITE_NODE_SIZE as small_number);
            cur_cs = k;
            p = scan_toks(false, false);
            MEM[cur_list.tail + 1].b32.s1 = def_ref as i32
        }
        2 => {
            new_write_whatsit(WRITE_NODE_SIZE as small_number);
            MEM[cur_list.tail + 1].b32.s1 = TEX_NULL
        }
        3 => {
            new_whatsit(
                SPECIAL_NODE as small_number,
                WRITE_NODE_SIZE as small_number,
            );
            MEM[cur_list.tail + 1].b32.s0 = TEX_NULL;
            p = scan_toks(false, true);
            MEM[cur_list.tail + 1].b32.s1 = def_ref as i32;
        }
        4 => {
            get_x_token();
            if cur_cmd == EXTENSION as u8 && cur_chr <= CLOSE_NODE as i32 {
                p = cur_list.tail;
                do_extension();
                out_what(cur_list.tail as i32);
                flush_node_list(cur_list.tail as i32);
                cur_list.tail = p;
                MEM[p].b32.s1 = TEX_NULL;
            } else {
                back_input();
            }
        }
        5 => {
            if cur_list.mode.abs() != HMODE {
                report_illegal_case();
            } else {
                new_whatsit(
                    LANGUAGE_NODE as small_number,
                    SMALL_NODE_SIZE as small_number,
                );
                scan_int();
                if cur_val <= 0 {
                    cur_list.aux.b32.s1 = 0
                } else if cur_val > 255 {
                    cur_list.aux.b32.s1 = 0
                } else {
                    cur_list.aux.b32.s1 = cur_val
                }
                MEM[cur_list.tail + 1].b32.s1 = cur_list.aux.b32.s1;
                MEM[cur_list.tail + 1].b16.s1 = norm_min(*INTPAR(IntPar::left_hyphen_min)) as u16;
                MEM[cur_list.tail + 1].b16.s0 = norm_min(*INTPAR(IntPar::right_hyphen_min)) as u16
            }
        }
        41 => {
            if cur_list.mode.abs() == MMODE {
                report_illegal_case();
            } else {
                load_picture(false);
            }
        }
        42 => {
            if cur_list.mode.abs() == MMODE {
                report_illegal_case();
            } else {
                load_picture(true);
            }
        }
        43 => {
            if cur_list.mode.abs() == VMODE {
                back_input();
                new_graf(true);
            } else if cur_list.mode.abs() == MMODE {
                report_illegal_case();
            } else if FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == OTGR_FONT_FLAG
            {
                new_whatsit(GLYPH_NODE as small_number, GLYPH_NODE_SIZE as small_number);
                scan_int();
                if cur_val < 0 || cur_val > 65535 {
                    if file_line_error_style_p != 0 {
                        print_file_line();
                    } else {
                        print_nl_cstr(b"! ");
                    }
                    print_cstr(b"Bad glyph number");
                    help_ptr = 2;
                    help_line[1] = b"A glyph number must be between 0 and 65535.";
                    help_line[0] = b"I changed this one to zero.";
                    int_error(cur_val);
                    cur_val = 0;
                }
                MEM[cur_list.tail + 4].b16.s2 = EQTB[CUR_FONT_LOC].b32.s1 as u16;
                MEM[cur_list.tail + 4].b16.s1 = cur_val as u16;
                measure_native_glyph(
                    &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                    (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0) as i32,
                );
            } else {
                not_native_font_error(
                    EXTENSION,
                    GLYPH_CODE as i32,
                    EQTB[CUR_FONT_LOC].b32.s1 as usize,
                );
            }
        }
        44 => {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&mut j);
            if i == XETEX_INPUT_MODE_AUTO {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Encoding mode `auto\' is not valid for \\XeTeXinputencoding");
                help_ptr = 2;
                help_line[1] =
                    b"You can\'t use `auto\' encoding here, only for \\XeTeXdefaultencoding.";
                help_line[0] = b"I\'ll ignore this and leave the current encoding unchanged.";
                error();
            } else {
                set_input_file_encoding(INPUT_FILE[IN_OPEN], i, j);
            }
        }
        45 => {
            scan_and_pack_name();
            i = get_encoding_mode_and_info(&mut j);
            *INTPAR(IntPar::xetex_default_input_mode) = i;
            *INTPAR(IntPar::xetex_default_input_encoding) = j
        }
        46 => {
            scan_file_name();
            if length(cur_name) == 0 {
                *INTPAR(IntPar::xetex_linebreak_locale) = 0;
            } else {
                *INTPAR(IntPar::xetex_linebreak_locale) = cur_name;
            }
        }
        6 => new_whatsit(
            PDFTEX_FIRST_EXTENSION_CODE as small_number,
            SMALL_NODE_SIZE as small_number,
        ),
        _ => confusion(b"ext1"),
    };
}
pub(crate) unsafe fn fix_language() {
    let mut l: UTF16_code = if *INTPAR(IntPar::language) <= 0 {
        0
    } else if *INTPAR(IntPar::language) > 255 {
        0
    } else {
        *INTPAR(IntPar::language) as UTF16_code
    };
    if l as i32 != cur_list.aux.b32.s1 {
        new_whatsit(
            LANGUAGE_NODE as small_number,
            SMALL_NODE_SIZE as small_number,
        );
        MEM[cur_list.tail + 1].b32.s1 = l as i32;
        cur_list.aux.b32.s1 = l as i32;
        MEM[cur_list.tail + 1].b16.s1 = norm_min(*INTPAR(IntPar::left_hyphen_min)) as u16;
        MEM[cur_list.tail + 1].b16.s0 = norm_min(*INTPAR(IntPar::right_hyphen_min)) as u16
    };
}
pub(crate) unsafe fn insert_src_special() {
    if SOURCE_FILENAME_STACK[IN_OPEN] > 0
        && is_new_source(SOURCE_FILENAME_STACK[IN_OPEN], line) as i32 != 0
    {
        let toklist = get_avail();
        let p = toklist;
        MEM[p].b32.s0 = CS_TOKEN_FLAG + FROZEN_SPECIAL as i32;
        MEM[p].b32.s1 = get_avail() as i32;
        let p = *LLIST_link(p) as usize;
        MEM[p].b32.s0 = LEFT_BRACE_TOKEN + '{' as i32;
        let q = str_toks(make_src_special(SOURCE_FILENAME_STACK[IN_OPEN], line)) as usize;
        MEM[p].b32.s1 = MEM[TEMP_HEAD].b32.s1;
        let p = q;
        MEM[p].b32.s1 = get_avail() as i32;
        let p = *LLIST_link(p) as usize;
        MEM[p].b32.s0 = RIGHT_BRACE_TOKEN + '}' as i32;
        begin_token_list(toklist, INSERTED);
        remember_source_info(SOURCE_FILENAME_STACK[IN_OPEN], line);
    };
}
pub(crate) unsafe fn append_src_special() {
    if SOURCE_FILENAME_STACK[IN_OPEN] > 0
        && is_new_source(SOURCE_FILENAME_STACK[IN_OPEN], line) as i32 != 0
    {
        new_whatsit(
            SPECIAL_NODE as small_number,
            WRITE_NODE_SIZE as small_number,
        );
        MEM[cur_list.tail + 1].b32.s0 = 0;
        def_ref = get_avail();
        MEM[def_ref].b32.s0 = TEX_NULL;
        str_toks(make_src_special(SOURCE_FILENAME_STACK[IN_OPEN], line));
        MEM[def_ref].b32.s1 = MEM[TEMP_HEAD].b32.s1;
        MEM[cur_list.tail + 1].b32.s1 = def_ref as i32;
        remember_source_info(SOURCE_FILENAME_STACK[IN_OPEN], line);
    };
}
pub(crate) unsafe fn handle_right_brace() {
    let mut p: i32 = 0;
    let mut q: i32 = 0;
    let mut d: scaled_t = 0;
    let mut f: i32 = 0;
    match cur_group {
        GroupCode::SIMPLE => unsave(),
        GroupCode::BOTTOM_LEVEL => {
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Too many }\'s");
            help_ptr = 2_u8;
            help_line[1] = b"You\'ve closed more groups than you opened.";
            help_line[0] = b"Such booboos are generally harmless, so keep going.";
            error();
        }
        GroupCode::SEMI_SIMPLE | GroupCode::MATH_SHIFT | GroupCode::MATH_LEFT => {
            extra_right_brace()
        }
        GroupCode::HBOX => package(0),
        GroupCode::ADJUSTED_HBOX => {
            adjust_tail = ADJUST_HEAD as i32;
            pre_adjust_tail = PRE_ADJUST_HEAD as i32;
            package(0);
        }
        GroupCode::VBOX => {
            end_graf();
            package(0);
        }
        GroupCode::VTOP => {
            end_graf();
            package(VTOP_CODE as small_number);
        }
        GroupCode::INSERT => {
            end_graf();
            q = *GLUEPAR(GluePar::split_top_skip);
            MEM[q as usize].b32.s1 += 1;
            d = *DIMENPAR(DimenPar::split_max_depth);
            f = *INTPAR(IntPar::floating_penalty);
            unsave();
            SAVE_PTR -= 2;
            p = vpackage(
                MEM[cur_list.head].b32.s1,
                0,
                ADDITIONAL as small_number,
                MAX_HALFWORD,
            ) as i32;
            pop_nest();
            if SAVE_STACK[SAVE_PTR + 0].b32.s1 < 255 {
                MEM[cur_list.tail].b32.s1 = get_node(INS_NODE_SIZE) as i32;
                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                MEM[cur_list.tail].b16.s1 = INS_NODE;
                MEM[cur_list.tail].b16.s0 = SAVE_STACK[SAVE_PTR + 0].b32.s1 as u16;
                MEM[cur_list.tail + 3].b32.s1 =
                    MEM[(p + 3) as usize].b32.s1 + MEM[(p + 2) as usize].b32.s1;
                MEM[cur_list.tail + 4].b32.s0 = MEM[(p + 5) as usize].b32.s1;
                MEM[cur_list.tail + 4].b32.s1 = q;
                MEM[cur_list.tail + 2].b32.s1 = d;
                MEM[cur_list.tail + 1].b32.s1 = f
            } else {
                MEM[cur_list.tail].b32.s1 = get_node(SMALL_NODE_SIZE) as i32;
                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                MEM[cur_list.tail].b16.s1 = ADJUST_NODE;
                MEM[cur_list.tail].b16.s0 = SAVE_STACK[SAVE_PTR + 1].b32.s1 as u16;
                MEM[cur_list.tail + 1].b32.s1 = MEM[(p + 5) as usize].b32.s1;
                delete_glue_ref(q as usize);
            }
            free_node(p as usize, BOX_NODE_SIZE);
            if NEST_PTR == 0 {
                build_page();
            }
        }
        GroupCode::OUTPUT => {
            /*1062:*/
            if !cur_input.loc.is_texnull()
                || cur_input.index != OUTPUT_TEXT && cur_input.index != BACKED_UP
            {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Unbalanced output routine");
                help_ptr = 2;
                help_line[1] = b"Your sneaky output routine has problematic {\'s and/or }\'s.";
                help_line[0] = b"I can\'t handle that very well; good luck.";
                error();
                loop {
                    get_token();
                    if cur_input.loc.is_texnull() {
                        break;
                    }
                }
            }

            end_token_list();
            end_graf();
            unsave();
            output_active = false;
            insert_penalties = 0;

            if !BOX_REG(255).is_texnull() {
                if file_line_error_style_p != 0 {
                    print_file_line();
                } else {
                    print_nl_cstr(b"! ");
                }
                print_cstr(b"Output routine didn\'t use all of ");
                print_esc_cstr(b"box");
                print_int(255);
                help_ptr = 3;
                help_line[2] = b"Your \\output commands should empty \\box255,";
                help_line[1] = b"e.g., by saying `\\shipout\\box255\'.";
                help_line[0] = b"Proceed; I\'ll discard its present contents.";
                box_error(255);
            }
            if cur_list.tail != cur_list.head {
                MEM[page_tail as usize].b32.s1 = MEM[cur_list.head].b32.s1;
                page_tail = cur_list.tail as i32;
            }
            if !MEM[PAGE_HEAD].b32.s1.is_texnull() {
                if MEM[CONTRIB_HEAD].b32.s1.is_texnull() {
                    NEST[0].tail = page_tail as usize;
                }
                MEM[page_tail as usize].b32.s1 = MEM[CONTRIB_HEAD].b32.s1;
                MEM[CONTRIB_HEAD].b32.s1 = MEM[PAGE_HEAD].b32.s1;
                MEM[PAGE_HEAD].b32.s1 = TEX_NULL;
                page_tail = PAGE_HEAD as i32
            }
            flush_node_list(disc_ptr[LAST_BOX_CODE as usize]);
            disc_ptr[LAST_BOX_CODE as usize] = TEX_NULL;
            pop_nest();
            build_page();
        }
        GroupCode::DISC => build_discretionary(),
        GroupCode::ALIGN => {
            back_input();
            cur_tok = CS_TOKEN_FLAG + FROZEN_CR as i32;
            if file_line_error_style_p != 0 {
                print_file_line();
            } else {
                print_nl_cstr(b"! ");
            }
            print_cstr(b"Missing ");
            print_esc_cstr(b"cr");
            print_cstr(b" inserted");
            help_ptr = 1;
            help_line[0] = b"I\'m guessing that you meant to end an alignment here.";
            ins_error();
        }
        GroupCode::NO_ALIGN => {
            end_graf();
            unsave();
            align_peek();
        }
        GroupCode::VCENTER => {
            end_graf();
            unsave();
            SAVE_PTR -= 2;
            p = vpackage(
                MEM[cur_list.head].b32.s1,
                SAVE_STACK[SAVE_PTR + 1].b32.s1,
                SAVE_STACK[SAVE_PTR + 0].b32.s1 as small_number,
                MAX_HALFWORD,
            ) as i32;
            pop_nest();
            MEM[cur_list.tail].b32.s1 = new_noad() as i32;
            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
            MEM[cur_list.tail].b16.s1 = VCENTER_NOAD;
            MEM[cur_list.tail + 1].b32.s1 = SUB_BOX;
            MEM[cur_list.tail + 1].b32.s0 = p
        }
        GroupCode::MATH_CHOICE => build_choices(),
        GroupCode::MATH => {
            unsave();
            SAVE_PTR -= 1;
            MEM[SAVE_STACK[SAVE_PTR + 0].b32.s1 as usize].b32.s1 = SUB_MLIST;
            p = fin_mlist(TEX_NULL);
            MEM[SAVE_STACK[SAVE_PTR + 0].b32.s1 as usize].b32.s0 = p;
            if !p.is_texnull() {
                if MEM[p as usize].b32.s1.is_texnull() {
                    if MEM[p as usize].b16.s1 == ORD_NOAD {
                        if MEM[(p + 3) as usize].b32.s1 == EMPTY {
                            if MEM[(p + 2) as usize].b32.s1 == EMPTY {
                                MEM[SAVE_STACK[SAVE_PTR + 0].b32.s1 as usize].b32 =
                                    MEM[(p + 1) as usize].b32;
                                free_node(p as usize, NOAD_SIZE);
                            }
                        }
                    } else if MEM[p as usize].b16.s1 == ACCENT_NOAD {
                        if SAVE_STACK[SAVE_PTR + 0].b32.s1 == cur_list.tail as i32 + 1 {
                            if MEM[cur_list.tail].b16.s1 == ORD_NOAD {
                                /*1222:*/
                                q = cur_list.head as i32;
                                while MEM[q as usize].b32.s1 != cur_list.tail as i32 {
                                    q = *LLIST_link(q as usize);
                                }
                                MEM[q as usize].b32.s1 = p;
                                free_node(cur_list.tail, NOAD_SIZE);
                                cur_list.tail = p as usize;
                            }
                        }
                    }
                }
            }
        } //_ => confusion(b"rightbrace"),
    };
}
pub(crate) unsafe fn main_control() {
    let mut current_block: u64;
    if !LOCAL(Local::every_job).is_texnull() {
        begin_token_list(*LOCAL(Local::every_job) as usize, 13_u16);
    }
    'c_125208: loop {
        /* big_switch */
        get_x_token();
        loop {
            /*1066: */
            if *INTPAR(IntPar::tracing_commands) > 0i32 {
                show_cur_cmd_chr(); /*:1490 */
            }
            match cur_list.mode.abs() + cur_cmd as i16 {
                115 | 116 | 172 => {}
                120 => {
                    scan_usv_num();
                    cur_chr = cur_val;
                }
                169 => {
                    get_x_token();
                    if cur_cmd == LETTER as u8
                        || cur_cmd == OTHER_CHAR as u8
                        || cur_cmd == CHAR_GIVEN as u8
                        || cur_cmd == CHAR_NUM as u8
                    {
                        cancel_boundary = true;
                    }
                    continue;
                }
                _ => {
                    if cur_list.mode.abs() == HMODE {
                        if *INTPAR(IntPar::xetex_inter_char_tokens) > 0
                            && space_class != CHAR_CLASS_LIMIT
                            && prev_class != CHAR_CLASS_LIMIT - 1
                        {
                            prev_class = CHAR_CLASS_LIMIT - 1;
                            find_sa_element(
                                INTER_CHAR_VAL as small_number,
                                space_class * CHAR_CLASS_LIMIT + (CHAR_CLASS_LIMIT - 1),
                                false,
                            );
                            if !cur_ptr.is_texnull() {
                                if cur_cs == 0 {
                                    if cur_cmd == CHAR_NUM as u8 {
                                        cur_cmd = OTHER_CHAR as u8
                                    }
                                    cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
                                } else {
                                    cur_tok = CS_TOKEN_FLAG + cur_cs
                                }
                                back_input();
                                begin_token_list(
                                    MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                    INTER_CHAR_TEXT,
                                );
                                continue 'c_125208;
                            }
                        }
                    }
                    match cur_list.mode.abs() + cur_cmd as i16 {
                        114 => {
                            if cur_list.aux.b32.s0 == 1000 {
                                current_block = 1496671425652391013;
                                break;
                            } else {
                                current_block = 11459959175219260272;
                                break;
                            }
                        }
                        168 | 271 => {
                            current_block = 1496671425652391013;
                            break;
                        }
                        40 | 143 | 246 => {
                            if cur_chr == 0 {
                                loop {
                                    get_x_token();
                                    if !(cur_cmd == SPACER as u8) {
                                        break;
                                    }
                                }
                                continue;
                            } else {
                                let t = scanner_status as i32;
                                scanner_status = NORMAL as u8;
                                get_next();
                                scanner_status = t as u8;
                                if cur_cs < HASH_BASE as i32 {
                                    cur_cs = prim_lookup(cur_cs - SINGLE_BASE as i32)
                                } else {
                                    cur_cs = prim_lookup((*hash.offset(cur_cs as isize)).s1)
                                }
                                if !(cur_cs != UNDEFINED_PRIMITIVE) {
                                    continue 'c_125208;
                                }
                                cur_cmd = prim_eqtb[cur_cs as usize].b16.s1 as u8;
                                cur_chr = prim_eqtb[cur_cs as usize].b32.s1;
                                continue;
                            }
                        }
                        15 => {
                            if its_all_over() {
                                return;
                            }
                            continue 'c_125208;
                        }
                        23 | 125 | 228 | 72 | 175 | 278 | 39 | 45 | 49 | 152 | 7 | 110 | 213 => {
                            report_illegal_case();
                            continue 'c_125208;
                        }
                        8 | 111 | 9 | 112 | 18 | 121 | 70 | 173 | 71 | 174 | 51 | 154 | 16
                        | 119 | 50 | 153 | 53 | 156 | 67 | 170 | 54 | 157 | 55 | 158 | 57 | 160
                        | 56 | 159 | 31 | 134 | 52 | 155 | 29 | 132 | 47 | 150 | 216 | 220
                        | 221 | 234 | 231 | 240 | 243 => {
                            insert_dollar_sign();
                            continue 'c_125208;
                        }
                        37 | 139 | 242 => {
                            MEM[cur_list.tail].b32.s1 = scan_rule_spec() as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            if cur_list.mode.abs() == VMODE {
                                cur_list.aux.b32.s1 = IGNORE_DEPTH;
                            } else if cur_list.mode.abs() == HMODE {
                                cur_list.aux.b32.s0 = 1000;
                            }
                            continue 'c_125208;
                        }
                        28 | 130 | 233 | 235 => {
                            append_glue();
                            continue 'c_125208;
                        }
                        30 | 133 | 236 | 237 => {
                            append_kern();
                            continue 'c_125208;
                        }
                        2 | 105 => {
                            new_save_level(GroupCode::SIMPLE);
                            continue 'c_125208;
                        }
                        62 | 165 | 268 => {
                            new_save_level(GroupCode::SEMI_SIMPLE);
                            continue 'c_125208;
                        }
                        63 | 166 | 269 => {
                            if cur_group == GroupCode::SEMI_SIMPLE {
                                unsave();
                            } else {
                                off_save();
                            }
                            continue 'c_125208;
                        }
                        3 | 106 | 209 => {
                            handle_right_brace();
                            continue 'c_125208;
                        }
                        22 | 126 | 229 => {
                            let t = cur_chr;
                            scan_dimen(false, false, false);
                            if t == 0 {
                                scan_box(cur_val);
                            } else {
                                scan_box(-cur_val);
                            }
                            continue 'c_125208;
                        }
                        32 | 135 | 238 => {
                            scan_box(LEADER_FLAG - (A_LEADERS as i32) + cur_chr);
                            continue 'c_125208;
                        }
                        21 | 124 | 227 => {
                            begin_box(0);
                            continue 'c_125208;
                        }
                        44 => {
                            new_graf(cur_chr > 0);
                            continue 'c_125208;
                        }
                        12 | 13 | 17 | 69 | 4 | 24 | 36 | 46 | 48 | 27 | 34 | 65 | 66 => {
                            back_input();
                            new_graf(true);
                            continue 'c_125208;
                        }
                        147 | 250 => {
                            indent_in_hmode();
                            continue 'c_125208;
                        }
                        14 => {
                            normal_paragraph();
                            if cur_list.mode > 0 {
                                build_page();
                            }
                            continue 'c_125208;
                        }
                        117 => {
                            if align_state < 0 {
                                off_save();
                            }
                            end_graf();
                            if cur_list.mode == VMODE {
                                build_page();
                            }
                            continue 'c_125208;
                        }
                        118 | 131 | 140 | 128 | 136 => {
                            head_for_vmode();
                            continue 'c_125208;
                        }
                        38 | 141 | 244 | 142 | 245 => {
                            begin_insert_or_adjust();
                            continue 'c_125208;
                        }
                        19 | 122 | 225 => {
                            make_mark();
                            continue 'c_125208;
                        }
                        43 | 146 | 249 => {
                            append_penalty();
                            continue 'c_125208;
                        }
                        26 | 129 | 232 => {
                            delete_last();
                            continue 'c_125208;
                        }
                        25 | 127 | 230 => {
                            unpackage();
                            continue 'c_125208;
                        }
                        148 => {
                            append_italic_correction();
                            continue 'c_125208;
                        }
                        251 => {
                            MEM[cur_list.tail].b32.s1 = new_kern(0) as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            continue 'c_125208;
                        }
                        151 | 254 => {
                            append_discretionary();
                            continue 'c_125208;
                        }
                        149 => {
                            make_accent();
                            continue 'c_125208;
                        }
                        6 | 109 | 212 | 5 | 108 | 211 => {
                            align_error();
                            continue 'c_125208;
                        }
                        35 | 138 | 241 => {
                            no_align_error();
                            continue 'c_125208;
                        }
                        64 | 167 | 270 => {
                            omit_error();
                            continue 'c_125208;
                        }
                        33 => {
                            init_align();
                            continue 'c_125208;
                        }
                        137 => {
                            if cur_chr > 0 {
                                if eTeX_enabled(
                                    *INTPAR(IntPar::texxet) > 0,
                                    cur_cmd as u16,
                                    cur_chr,
                                ) {
                                    MEM[cur_list.tail].b32.s1 =
                                        new_math(0, cur_chr as small_number) as i32;
                                    cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                                }
                            } else {
                                init_align();
                            }
                            continue 'c_125208;
                        }
                        239 => {
                            if privileged() {
                                if cur_group == GroupCode::MATH_SHIFT {
                                    init_align();
                                } else {
                                    off_save();
                                }
                            }
                            continue 'c_125208;
                        }
                        10 | 113 => {
                            do_endv();
                            continue 'c_125208;
                        }
                        68 | 171 | 274 => {
                            cs_error();
                            continue 'c_125208;
                        }
                        107 => {
                            init_math();
                            continue 'c_125208;
                        }
                        255 => {
                            if privileged() {
                                if cur_group == GroupCode::MATH_SHIFT {
                                    start_eq_no();
                                } else {
                                    off_save();
                                }
                            }
                            continue 'c_125208;
                        }
                        208 => {
                            MEM[cur_list.tail].b32.s1 = new_noad() as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            back_input();
                            scan_math(cur_list.tail + 1);
                            continue 'c_125208;
                        }
                        218 | 219 | 275 => {
                            set_math_char(*MATH_CODE(cur_chr as usize));
                            continue 'c_125208;
                        }
                        223 => {
                            scan_char_num();
                            cur_chr = cur_val;
                            set_math_char(*MATH_CODE(cur_chr as usize));
                            continue 'c_125208;
                        }
                        224 => {
                            if cur_chr == 2 {
                                scan_math_class_int();
                                let t = set_class(cur_val);
                                scan_math_fam_int();
                                let t = t + set_family(cur_val);
                                scan_usv_num();
                                let t = t + cur_val;
                                set_math_char(t);
                            } else if cur_chr == 1 {
                                scan_xetex_math_char_int();
                                set_math_char(cur_val);
                            } else {
                                scan_fifteen_bit_int();
                                set_math_char(
                                    set_class(cur_val / 4096)
                                        + set_family((cur_val % 4096) / 256)
                                        + (cur_val % 256),
                                );
                            }
                            continue 'c_125208;
                        }
                        276 => {
                            set_math_char(
                                set_class(cur_chr / 4096)
                                    + set_family((cur_chr % 4096) / 256)
                                    + (cur_chr % 256),
                            );
                            continue 'c_125208;
                        }
                        277 => {
                            set_math_char(cur_chr);
                            continue 'c_125208;
                        }
                        222 => {
                            if cur_chr == 1i32 {
                                scan_math_class_int();
                                scan_math_class_int();
                                let t = set_class(cur_val);
                                scan_math_fam_int();
                                let t = t + set_family(cur_val);
                                scan_usv_num();
                                let t = t + cur_val;
                                set_math_char(t);
                            } else {
                                scan_delimiter_int();
                                cur_val = cur_val / 4096;
                                set_math_char(
                                    set_class(cur_val / 4096)
                                        + set_family((cur_val % 4096) / 256)
                                        + (cur_val % 256),
                                );
                            }
                            continue 'c_125208;
                        }
                        257 => {
                            MEM[cur_list.tail].b32.s1 = new_noad() as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            MEM[cur_list.tail].b16.s1 = cur_chr as u16;
                            scan_math(cur_list.tail + 1);
                            continue 'c_125208;
                        }
                        258 => {
                            math_limit_switch();
                            continue 'c_125208;
                        }
                        273 => {
                            math_radical();
                            continue 'c_125208;
                        }
                        252 | 253 => {
                            math_ac();
                            continue 'c_125208;
                        }
                        263 => {
                            scan_spec(GroupCode::VCENTER, false);
                            normal_paragraph();
                            push_nest();
                            cur_list.mode = -1;
                            cur_list.aux.b32.s1 = IGNORE_DEPTH;
                            if insert_src_special_every_vbox {
                                insert_src_special();
                            }
                            if !LOCAL(Local::every_vbox).is_texnull() {
                                begin_token_list(
                                    *LOCAL(Local::every_vbox) as usize,
                                    EVERY_VBOX_TEXT,
                                );
                            }
                            continue 'c_125208;
                        }
                        260 => {
                            MEM[cur_list.tail].b32.s1 = new_style(cur_chr as small_number) as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            continue 'c_125208;
                        }
                        262 => {
                            MEM[cur_list.tail].b32.s1 = new_glue(0) as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            MEM[cur_list.tail].b16.s0 = COND_MATH_GLUE;
                            continue 'c_125208;
                        }
                        261 => {
                            append_choices();
                            continue 'c_125208;
                        }
                        215 | 214 => {
                            sub_sup();
                            continue 'c_125208;
                        }
                        259 => {
                            math_fraction();
                            continue 'c_125208;
                        }
                        256 => {
                            math_left_right();
                            continue 'c_125208;
                        }
                        210 => {
                            if cur_group == GroupCode::MATH_SHIFT {
                                after_math();
                            } else {
                                off_save();
                            }
                            continue 'c_125208;
                        }
                        73 | 176 | 279 | 74 | 177 | 280 | 75 | 178 | 281 | 76 | 179 | 282 | 77
                        | 180 | 283 | 78 | 181 | 284 | 79 | 182 | 285 | 80 | 183 | 286 | 81
                        | 184 | 287 | 82 | 185 | 288 | 83 | 186 | 289 | 84 | 187 | 290 | 85
                        | 188 | 291 | 86 | 189 | 292 | 87 | 190 | 293 | 88 | 191 | 294 | 89
                        | 192 | 295 | 90 | 193 | 296 | 91 | 194 | 297 | 92 | 195 | 298 | 93
                        | 196 | 299 | 94 | 197 | 300 | 95 | 198 | 301 | 96 | 199 | 302 | 97
                        | 200 | 303 | 98 | 201 | 304 | 99 | 202 | 305 | 100 | 203 | 306 | 101
                        | 204 | 307 | 102 | 205 | 308 | 103 | 206 | 309 => {
                            prefixed_command();
                            continue 'c_125208;
                        }
                        41 | 144 | 247 => {
                            get_token();
                            after_token = cur_tok;
                            continue 'c_125208;
                        }
                        42 | 145 | 248 => {
                            get_token();
                            save_for_after(cur_tok);
                            continue 'c_125208;
                        }
                        61 | 164 | 267 => {
                            open_or_close_in();
                            continue 'c_125208;
                        }
                        59 | 162 | 265 => {
                            issue_message();
                            continue 'c_125208;
                        }
                        58 | 161 | 264 => {
                            shift_case();
                            continue 'c_125208;
                        }
                        20 | 123 | 226 => {
                            show_whatever();
                            continue 'c_125208;
                        }
                        60 | 163 | 266 => {
                            do_extension();
                            continue 'c_125208;
                        }
                        1 | 104 | 207 | 11 | 217 | 272 | _ => continue 'c_125208,
                    }
                }
            }
            /*main_loop *//*1069: */
            if cur_list.head == cur_list.tail && cur_list.mode > 0 {
                if insert_src_special_auto {
                    append_src_special();
                }
            }
            prev_class = CHAR_CLASS_LIMIT - 1;
            if FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == AAT_FONT_FLAG
                || FONT_AREA[EQTB[CUR_FONT_LOC].b32.s1 as usize] as u32 == OTGR_FONT_FLAG
            {
                if cur_list.mode > 0 {
                    if *INTPAR(IntPar::language) != cur_list.aux.b32.s1 {
                        fix_language();
                    }
                }
                main_h = 0;
                main_f = EQTB[CUR_FONT_LOC].b32.s1 as usize;
                native_len = 0;
                loop {
                    /*collect_native */
                    main_s = (*SF_CODE(cur_chr as usize) as i64 % 65536) as i32;
                    if main_s == 1000 {
                        cur_list.aux.b32.s0 = 1000;
                    } else if main_s < 1000 {
                        if main_s > 0 {
                            cur_list.aux.b32.s0 = main_s;
                        }
                    } else if cur_list.aux.b32.s0 < 1000 {
                        cur_list.aux.b32.s0 = 1000;
                    } else {
                        cur_list.aux.b32.s0 = main_s;
                    }
                    cur_ptr = TEX_NULL;
                    space_class = (*SF_CODE(cur_chr as usize) as i64 / 65536) as i32;
                    if *INTPAR(IntPar::xetex_inter_char_tokens) > 0 && space_class != 4096 {
                        if prev_class == CHAR_CLASS_LIMIT - 1 {
                            if cur_input.state != TOKEN_LIST || cur_input.index != BACKED_UP_CHAR {
                                find_sa_element(
                                    INTER_CHAR_VAL as small_number,
                                    (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT + space_class,
                                    false,
                                );
                                if !cur_ptr.is_texnull() {
                                    if cur_cmd != LETTER as u8 {
                                        cur_cmd = OTHER_CHAR as u8
                                    }
                                    cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                                    back_input();
                                    cur_input.index = BACKED_UP_CHAR;
                                    begin_token_list(
                                        MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                        INTER_CHAR_TEXT,
                                    );
                                    continue 'c_125208;
                                }
                            }
                        } else {
                            find_sa_element(
                                INTER_CHAR_VAL as small_number,
                                prev_class * CHAR_CLASS_LIMIT + space_class,
                                false,
                            );
                            if !cur_ptr.is_texnull() {
                                if cur_cmd != LETTER as u8 {
                                    cur_cmd = OTHER_CHAR as u8;
                                }
                                cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                                back_input();
                                cur_input.index = BACKED_UP_CHAR;
                                begin_token_list(
                                    MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                    INTER_CHAR_TEXT,
                                );
                                prev_class = CHAR_CLASS_LIMIT - 1;
                                current_block = 9706274459985797855;
                                break;
                            }
                        }
                        prev_class = space_class
                    }
                    if cur_chr as i64 > 65535 {
                        while native_text_size <= native_len + 2 {
                            native_text_size = native_text_size + 128;
                            native_text = xrealloc(
                                native_text as *mut libc::c_void,
                                (native_text_size as u64).wrapping_mul(::std::mem::size_of::<
                                    UTF16_code,
                                >(
                                )
                                    as u64) as _,
                            ) as *mut UTF16_code
                        }
                        *native_text.offset(native_len as isize) =
                            ((cur_chr as i64 - 65536) / 1024 + 0xd800) as UTF16_code;
                        native_len += 1;
                        *native_text.offset(native_len as isize) =
                            ((cur_chr as i64 - 65536) % 1024 + 0xdc00) as UTF16_code;
                        native_len += 1
                    } else {
                        while native_text_size <= native_len + 1 {
                            native_text_size = native_text_size + 128;
                            native_text = xrealloc(
                                native_text as *mut libc::c_void,
                                (native_text_size as u64).wrapping_mul(::std::mem::size_of::<
                                    UTF16_code,
                                >(
                                )
                                    as u64) as _,
                            ) as *mut UTF16_code
                        }
                        *native_text.offset(native_len as isize) = cur_chr as UTF16_code;
                        native_len += 1
                    }
                    is_hyph = cur_chr == HYPHEN_CHAR[main_f as usize]
                        || *INTPAR(IntPar::xetex_dash_break) > 0
                            && (cur_chr == 8212 || cur_chr == 8211);
                    if main_h == 0 && is_hyph as i32 != 0 {
                        main_h = native_len
                    }
                    get_next();
                    if cur_cmd == LETTER as u8
                        || cur_cmd == OTHER_CHAR as u8
                        || cur_cmd == CHAR_GIVEN as u8
                    {
                        continue;
                    }
                    x_token();
                    if cur_cmd == LETTER as u8
                        || cur_cmd == OTHER_CHAR as u8
                        || cur_cmd == CHAR_GIVEN as u8
                    {
                        continue;
                    }
                    if cur_cmd == CHAR_NUM as u8 {
                        scan_usv_num();
                        cur_chr = cur_val
                    } else if *INTPAR(IntPar::xetex_inter_char_tokens) > 0
                        && space_class != CHAR_CLASS_LIMIT
                        && prev_class != CHAR_CLASS_LIMIT - 1
                    {
                        current_block = 14170946608255986518;
                        break;
                    } else {
                        current_block = 9706274459985797855;
                        break;
                    }
                }
                match current_block {
                    14170946608255986518 => {
                        prev_class = CHAR_CLASS_LIMIT - 1;
                        find_sa_element(
                            INTER_CHAR_VAL as small_number,
                            space_class * CHAR_CLASS_LIMIT + (CHAR_CLASS_LIMIT - 1),
                            false,
                        );
                        if !cur_ptr.is_texnull() {
                            if cur_cs == 0 {
                                if cur_cmd == CHAR_NUM as u8 {
                                    cur_cmd = OTHER_CHAR as u8
                                }
                                cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
                            } else {
                                cur_tok = CS_TOKEN_FLAG + cur_cs
                            }
                            back_input();
                            begin_token_list(
                                MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                INTER_CHAR_TEXT,
                            );
                        }
                    }
                    _ => {}
                }
                /*collected */
                if !(FONT_MAPPING[main_f as usize]).is_null() {
                    main_k = apply_mapping(FONT_MAPPING[main_f as usize], native_text, native_len);
                    native_len = 0;
                    while native_text_size <= native_len + main_k {
                        native_text_size = native_text_size + 128;
                        native_text = xrealloc(
                            native_text as *mut libc::c_void,
                            (native_text_size as u64)
                                .wrapping_mul(::std::mem::size_of::<UTF16_code>() as u64)
                                as _,
                        ) as *mut UTF16_code
                    }
                    main_h = 0;
                    let mut for_end: i32 = 0;
                    main_p = 0;
                    for_end = main_k - 1;
                    if main_p <= for_end {
                        loop {
                            *native_text.offset(native_len as isize) =
                                *mapped_text.offset(main_p as isize);
                            native_len += 1;
                            if main_h == 0
                                && (*mapped_text.offset(main_p as isize) as i32
                                    == HYPHEN_CHAR[main_f as usize]
                                    || *INTPAR(IntPar::xetex_dash_break) > 0
                                        && (*mapped_text.offset(main_p as isize) == 8212
                                            || *mapped_text.offset(main_p as isize) == 8211))
                            {
                                main_h = native_len
                            }
                            let fresh88 = main_p;
                            main_p = main_p + 1;
                            if !(fresh88 < for_end) {
                                break;
                            }
                        }
                    }
                }
                if *INTPAR(IntPar::tracing_lost_chars) > 0 {
                    temp_ptr = 0;
                    while temp_ptr < native_len {
                        main_k = *native_text.offset(temp_ptr as isize) as font_index;
                        temp_ptr += 1;
                        if main_k >= 0xd800 && main_k < 0xdc00 {
                            main_k = (65536 + ((main_k - 0xd800) * 1024) as i64) as font_index;
                            main_k =
                                main_k + *native_text.offset(temp_ptr as isize) as i32 - 0xdc00;
                            temp_ptr += 1;
                        }
                        if map_char_to_glyph(main_f, main_k) == 0 {
                            char_warning(main_f, main_k);
                        }
                    }
                }
                main_k = native_len;
                main_pp = cur_list.tail as i32;
                if cur_list.mode == HMODE {
                    main_ppp = cur_list.head as i32;
                    if main_ppp != main_pp {
                        while MEM[main_ppp as usize].b32.s1 != main_pp {
                            if !is_char_node(main_ppp) && *NODE_type(main_ppp as usize) == DISC_NODE
                            {
                                temp_ptr = main_ppp;
                                let mut for_end_0: i32 = 0;
                                main_p = 1;
                                for_end_0 = MEM[temp_ptr as usize].b16.s0 as i32;
                                if main_p <= for_end_0 {
                                    loop {
                                        main_ppp = *LLIST_link(main_ppp as usize);
                                        let fresh89 = main_p;
                                        main_p = main_p + 1;
                                        if !(fresh89 < for_end_0) {
                                            break;
                                        }
                                    }
                                }
                            }
                            if main_ppp != main_pp {
                                main_ppp = *LLIST_link(main_ppp as usize);
                            }
                        }
                    }
                    temp_ptr = 0;
                    loop {
                        if main_h == 0 {
                            main_h = main_k
                        }
                        if !main_pp.is_texnull()
                            && !is_char_node(main_pp)
                            && *NODE_type(main_pp as usize) == WHATSIT_NODE
                            && (MEM[main_pp as usize].b16.s0 == NATIVE_WORD_NODE
                                || MEM[main_pp as usize].b16.s0 == NATIVE_WORD_NODE_AT)
                            && MEM[(main_pp + 4) as usize].b16.s2 as usize == main_f
                            && main_ppp != main_pp
                            && !is_char_node(main_ppp)
                            && *NODE_type(main_ppp as usize) != DISC_NODE
                        {
                            main_k = main_h + MEM[(main_pp + 4) as usize].b16.s1 as i32;
                            while native_text_size <= native_len + main_k {
                                native_text_size = native_text_size + 128;
                                native_text = xrealloc(
                                    native_text as *mut libc::c_void,
                                    (native_text_size as u64).wrapping_mul(::std::mem::size_of::<
                                        UTF16_code,
                                    >(
                                    )
                                        as u64) as _,
                                ) as *mut UTF16_code
                            }
                            save_native_len = native_len;
                            main_p = 0;
                            let mut for_end_1 = MEM[(main_pp + 4) as usize].b16.s1 as i32 - 1;
                            if main_p <= for_end_1 {
                                loop {
                                    *native_text.offset(native_len as isize) =
                                        *(&mut MEM[(main_pp + 6) as usize] as *mut memory_word
                                            as *mut u16)
                                            .offset(main_p as isize);
                                    native_len += 1;
                                    let fresh90 = main_p;
                                    main_p = main_p + 1;
                                    if !(fresh90 < for_end_1) {
                                        break;
                                    }
                                }
                            }
                            let mut for_end_2: i32 = 0;
                            main_p = 0i32;
                            for_end_2 = main_h - 1i32;
                            if main_p <= for_end_2 {
                                loop {
                                    *native_text.offset(native_len as isize) =
                                        *native_text.offset((temp_ptr + main_p) as isize);
                                    native_len += 1;
                                    let fresh91 = main_p;
                                    main_p = main_p + 1;
                                    if !(fresh91 < for_end_2) {
                                        break;
                                    }
                                }
                            }
                            do_locale_linebreaks(save_native_len, main_k);
                            native_len = save_native_len;
                            main_k = native_len - main_h - temp_ptr;
                            temp_ptr = main_h;
                            main_h = 0i32;
                            while main_h < main_k
                                && *native_text.offset((temp_ptr + main_h) as isize) as i32
                                    != HYPHEN_CHAR[main_f as usize]
                                && (!(*INTPAR(IntPar::xetex_dash_break) > 0)
                                    || *native_text.offset((temp_ptr + main_h) as isize) as i32
                                        != 8212
                                        && *native_text.offset((temp_ptr + main_h) as isize) as i32
                                            != 8211)
                            {
                                main_h += 1
                            }
                            if main_h < main_k {
                                main_h += 1
                            }
                            MEM[main_ppp as usize].b32.s1 = MEM[main_pp as usize].b32.s1;
                            MEM[main_pp as usize].b32.s1 = TEX_NULL;
                            flush_node_list(main_pp);
                            main_pp = cur_list.tail as i32;
                            while MEM[main_ppp as usize].b32.s1 != main_pp {
                                main_ppp = *LLIST_link(main_ppp as usize);
                            }
                        } else {
                            do_locale_linebreaks(temp_ptr, main_h);
                            temp_ptr = temp_ptr + main_h;
                            main_k = main_k - main_h;
                            main_h = 0;
                            while main_h < main_k
                                && *native_text.offset((temp_ptr + main_h) as isize) as i32
                                    != HYPHEN_CHAR[main_f as usize]
                                && (!(*INTPAR(IntPar::xetex_dash_break) > 0)
                                    || *native_text.offset((temp_ptr + main_h) as isize) as i32
                                        != 8212
                                        && *native_text.offset((temp_ptr + main_h) as isize) as i32
                                            != 8211)
                            {
                                main_h += 1
                            }
                            if main_h < main_k {
                                main_h += 1
                            }
                        }
                        if main_k > 0 || is_hyph as i32 != 0 {
                            MEM[cur_list.tail].b32.s1 = new_disc() as i32;
                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                            main_pp = cur_list.tail as i32;
                        }
                        if main_k == 0 {
                            break;
                        }
                    }
                } else {
                    main_ppp = cur_list.head as i32;
                    if main_ppp != main_pp {
                        while MEM[main_ppp as usize].b32.s1 != main_pp {
                            if !is_char_node(main_ppp) && *NODE_type(main_ppp as usize) == DISC_NODE
                            {
                                temp_ptr = main_ppp;
                                let mut for_end_3: i32 = 0;
                                main_p = 1i32;
                                for_end_3 = MEM[temp_ptr as usize].b16.s0 as i32;
                                if main_p <= for_end_3 {
                                    loop {
                                        main_ppp = *LLIST_link(main_ppp as usize);
                                        let fresh92 = main_p;
                                        main_p = main_p + 1;
                                        if !(fresh92 < for_end_3) {
                                            break;
                                        }
                                    }
                                }
                            }
                            if main_ppp != main_pp {
                                main_ppp = *LLIST_link(main_ppp as usize);
                            }
                        }
                    }
                    if !main_pp.is_texnull()
                        && !is_char_node(main_pp)
                        && *NODE_type(main_pp as usize) == WHATSIT_NODE
                        && (MEM[main_pp as usize].b16.s0 == NATIVE_WORD_NODE
                            || MEM[main_pp as usize].b16.s0 == NATIVE_WORD_NODE_AT)
                        && MEM[(main_pp + 4) as usize].b16.s2 as usize == main_f
                        && main_ppp != main_pp
                        && !is_char_node(main_ppp)
                        && *NODE_type(main_ppp as usize) != DISC_NODE
                    {
                        MEM[main_pp as usize].b32.s1 = new_native_word_node(
                            main_f,
                            main_k + MEM[(main_pp + 4) as usize].b16.s1 as i32,
                        ) as i32;
                        cur_list.tail = MEM[main_pp as usize].b32.s1 as usize;

                        let mut for_end_4: i32 = 0;
                        main_p = 0;
                        for_end_4 = MEM[(main_pp + 4) as usize].b16.s1 as i32 - 1;
                        if main_p <= for_end_4 {
                            loop {
                                *(&mut MEM[cur_list.tail + 6] as *mut memory_word as *mut u16)
                                    .offset(main_p as isize) = *(&mut MEM[(main_pp + 6) as usize]
                                    as *mut memory_word
                                    as *mut u16)
                                    .offset(main_p as isize);
                                let fresh93 = main_p;
                                main_p = main_p + 1;
                                if !(fresh93 < for_end_4) {
                                    break;
                                }
                            }
                        }
                        let mut for_end_5: i32 = 0;
                        main_p = 0i32;
                        for_end_5 = main_k - 1i32;
                        if main_p <= for_end_5 {
                            loop {
                                *(&mut MEM[cur_list.tail + 6] as *mut memory_word as *mut u16)
                                    .offset(
                                        (main_p + MEM[(main_pp + 4) as usize].b16.s1 as i32)
                                            as isize,
                                    ) = *native_text.offset(main_p as isize);
                                let fresh94 = main_p;
                                main_p = main_p + 1;
                                if !(fresh94 < for_end_5) {
                                    break;
                                }
                            }
                        }
                        measure_native_node(
                            &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                            (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0) as i32,
                        );
                        main_p = cur_list.head as i32;
                        if main_p != main_pp {
                            while MEM[main_p as usize].b32.s1 != main_pp {
                                main_p = *LLIST_link(main_p as usize);
                            }
                        }
                        MEM[main_p as usize].b32.s1 = MEM[main_pp as usize].b32.s1;
                        MEM[main_pp as usize].b32.s1 = TEX_NULL;
                        flush_node_list(main_pp);
                    } else {
                        MEM[main_pp as usize].b32.s1 = new_native_word_node(main_f, main_k) as i32;
                        cur_list.tail = MEM[main_pp as usize].b32.s1 as usize;
                        main_p = 0;
                        let mut for_end_6 = main_k - 1;
                        if main_p <= for_end_6 {
                            loop {
                                *(&mut MEM[cur_list.tail + 6] as *mut memory_word as *mut u16)
                                    .offset(main_p as isize) = *native_text.offset(main_p as isize);
                                let fresh95 = main_p;
                                main_p = main_p + 1;
                                if !(fresh95 < for_end_6) {
                                    break;
                                }
                            }
                        }
                        measure_native_node(
                            &mut MEM[cur_list.tail] as *mut memory_word as *mut libc::c_void,
                            (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0i32) as i32,
                        );
                    }
                }
                if *INTPAR(IntPar::xetex_interword_space_shaping) > 0 {
                    main_p = cur_list.head as i32;
                    main_pp = TEX_NULL;
                    while main_p != cur_list.tail as i32 {
                        if !main_p.is_texnull()
                            && !is_char_node(main_p)
                            && MEM[main_p as usize].b16.s1 == WHATSIT_NODE
                            && (MEM[main_p as usize].b16.s0 == NATIVE_WORD_NODE
                                || MEM[main_p as usize].b16.s0 == NATIVE_WORD_NODE_AT)
                        {
                            main_pp = main_p;
                        }
                        main_p = *LLIST_link(main_p as usize);
                    }
                    if !main_pp.is_texnull() {
                        if MEM[(main_pp + 4) as usize].b16.s2 as usize == main_f {
                            main_p = MEM[main_pp as usize].b32.s1;
                            while !is_char_node(main_p)
                                && (*NODE_type(main_p as usize) == PENALTY_NODE
                                    || *NODE_type(main_p as usize) == INS_NODE
                                    || *NODE_type(main_p as usize) == MARK_NODE
                                    || *NODE_type(main_p as usize) == ADJUST_NODE
                                    || *NODE_type(main_p as usize) == WHATSIT_NODE
                                        && MEM[main_p as usize].b16.s0 <= 4)
                            {
                                main_p = MEM[main_p as usize].b32.s1;
                            }
                            if !is_char_node(main_p) && *NODE_type(main_p as usize) == GLUE_NODE {
                                main_ppp = MEM[main_p as usize].b32.s1;
                                while !is_char_node(main_ppp)
                                    && (*NODE_type(main_ppp as usize) == PENALTY_NODE
                                        || *NODE_type(main_ppp as usize) == INS_NODE
                                        || *NODE_type(main_ppp as usize) == MARK_NODE
                                        || *NODE_type(main_ppp as usize) == ADJUST_NODE
                                        || *NODE_type(main_ppp as usize) == WHATSIT_NODE
                                            && MEM[main_ppp as usize].b16.s0 <= 4)
                                {
                                    main_ppp = *LLIST_link(main_ppp as usize)
                                }
                                if main_ppp == cur_list.tail as i32 {
                                    temp_ptr = new_native_word_node(
                                        main_f,
                                        MEM[(main_pp + 4) as usize].b16.s1 as i32
                                            + 1
                                            + MEM[cur_list.tail + 4].b16.s1 as i32,
                                    ) as i32;
                                    main_k = 0;
                                    let mut t = 0;
                                    let mut for_end_7 =
                                        MEM[(main_pp + 4) as usize].b16.s1 as i32 - 1;
                                    if t <= for_end_7 {
                                        loop {
                                            *(&mut MEM[(temp_ptr + 6) as usize]
                                                as *mut memory_word
                                                as *mut u16)
                                                .offset(main_k as isize) = *(&mut MEM
                                                [(main_pp + 6i32) as usize]
                                                as *mut memory_word
                                                as *mut u16)
                                                .offset(t as isize);
                                            main_k += 1;
                                            let fresh96 = t;
                                            t = t + 1;
                                            if !(fresh96 < for_end_7) {
                                                break;
                                            }
                                        }
                                    }
                                    *(&mut MEM[(temp_ptr + 6) as usize] as *mut memory_word
                                        as *mut u16)
                                        .offset(main_k as isize) = ' ' as i32 as u16;
                                    main_k += 1;
                                    t = 0;
                                    let mut for_end_8 = MEM[cur_list.tail + 4].b16.s1 as i32 - 1;
                                    if t <= for_end_8 {
                                        loop {
                                            *(&mut MEM[(temp_ptr + 6) as usize]
                                                as *mut memory_word
                                                as *mut u16)
                                                .offset(main_k as isize) = *(&mut MEM
                                                [cur_list.tail + 6]
                                                as *mut memory_word
                                                as *mut u16)
                                                .offset(t as isize);
                                            main_k += 1;
                                            let fresh97 = t;
                                            t = t + 1;
                                            if !(fresh97 < for_end_8) {
                                                break;
                                            }
                                        }
                                    }
                                    measure_native_node(
                                        &mut MEM[temp_ptr as usize] as *mut memory_word
                                            as *mut libc::c_void,
                                        (*INTPAR(IntPar::xetex_use_glyph_metrics) > 0i32) as i32,
                                    );
                                    t = MEM[(temp_ptr + 1) as usize].b32.s1
                                        - MEM[(main_pp + 1) as usize].b32.s1
                                        - MEM[cur_list.tail + 1].b32.s1;
                                    free_node(
                                        temp_ptr as usize,
                                        MEM[(temp_ptr + 4) as usize].b16.s3 as i32,
                                    );
                                    if t != MEM[(FONT_GLUE[main_f as usize] + 1i32) as usize].b32.s1
                                    {
                                        temp_ptr = new_kern(
                                            t - MEM[(FONT_GLUE[main_f as usize] + 1) as usize]
                                                .b32
                                                .s1,
                                        ) as i32;
                                        *NODE_subtype(temp_ptr as usize) = SPACE_ADJUSTMENT;
                                        MEM[temp_ptr as usize].b32.s1 = MEM[main_p as usize].b32.s1;
                                        MEM[main_p as usize].b32.s1 = temp_ptr
                                    }
                                }
                            }
                        }
                    }
                }
                if !cur_ptr.is_texnull() {
                    continue 'c_125208;
                }
            } else {
                main_s = (*SF_CODE(cur_chr as usize) as i64 % 65536) as i32;
                if main_s == 1000 {
                    cur_list.aux.b32.s0 = 1000
                } else if main_s < 1000 {
                    if main_s > 0 {
                        cur_list.aux.b32.s0 = main_s
                    }
                } else if cur_list.aux.b32.s0 < 1000 {
                    cur_list.aux.b32.s0 = 1000
                } else {
                    cur_list.aux.b32.s0 = main_s
                }
                cur_ptr = TEX_NULL;
                space_class = (*SF_CODE(cur_chr as usize) as i64 / 65536) as i32;
                if *INTPAR(IntPar::xetex_inter_char_tokens) > 0 && space_class != CHAR_CLASS_LIMIT {
                    if prev_class == CHAR_CLASS_LIMIT - 1 {
                        if cur_input.state != TOKEN_LIST || cur_input.index != BACKED_UP_CHAR {
                            find_sa_element(
                                INTER_CHAR_VAL as small_number,
                                (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT + space_class,
                                false,
                            );
                            if !cur_ptr.is_texnull() {
                                if cur_cmd != LETTER as u8 {
                                    cur_cmd = OTHER_CHAR as u8;
                                }
                                cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                                back_input();
                                cur_input.index = BACKED_UP_CHAR;
                                begin_token_list(
                                    MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                    INTER_CHAR_TEXT,
                                );
                                continue 'c_125208;
                            }
                        }
                    } else {
                        find_sa_element(
                            INTER_CHAR_VAL as small_number,
                            prev_class * CHAR_CLASS_LIMIT + space_class,
                            false,
                        );
                        if !cur_ptr.is_texnull() {
                            if cur_cmd != LETTER as u8 {
                                cur_cmd = OTHER_CHAR as u8;
                            }
                            cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                            back_input();
                            cur_input.index = BACKED_UP_CHAR;
                            begin_token_list(
                                MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                INTER_CHAR_TEXT,
                            );
                            prev_class = CHAR_CLASS_LIMIT - 1;
                            continue 'c_125208;
                        }
                    }
                    prev_class = space_class
                }
                main_f = EQTB[CUR_FONT_LOC].b32.s1 as usize;
                bchar = FONT_BCHAR[main_f as usize];
                false_bchar = FONT_FALSE_BCHAR[main_f as usize];
                if cur_list.mode > 0 {
                    if *INTPAR(IntPar::language) != cur_list.aux.b32.s1 {
                        fix_language();
                    }
                }
                lig_stack = avail;
                if lig_stack.is_texnull() {
                    lig_stack = get_avail() as i32;
                } else {
                    avail = MEM[lig_stack as usize].b32.s1;
                    MEM[lig_stack as usize].b32.s1 = TEX_NULL;
                }
                MEM[lig_stack as usize].b16.s1 = main_f as u16;
                cur_l = cur_chr;
                MEM[lig_stack as usize].b16.s0 = cur_l as u16;
                cur_q = cur_list.tail as i32;
                if cancel_boundary {
                    cancel_boundary = false;
                    main_k = NON_ADDRESS;
                } else {
                    main_k = BCHAR_LABEL[main_f as usize]
                }
                if main_k == NON_ADDRESS {
                    current_block = 249799543778823886;
                } else {
                    cur_r = cur_l;
                    cur_l = TOO_BIG_CHAR;
                    current_block = 13962460947151495567;
                }
                'c_125239: loop {
                    match current_block {
                        13962460947151495567 => {
                            // lab111
                            /*main_lig_loop 1 */
                            main_j = FONT_INFO[main_k as usize].b16;
                            current_block = 11331079115679122507;
                        }
                        _ =>
                        // lab92:
                        /*main_loop_move 2 */
                        {
                            if effective_char(false, main_f, cur_chr as u16)
                                > FONT_EC[main_f as usize] as i32
                                || effective_char(false, main_f, cur_chr as u16)
                                    < FONT_BC[main_f as usize] as i32
                            {
                                char_warning(main_f, cur_chr);
                                MEM[lig_stack as usize].b32.s1 = avail;
                                avail = lig_stack;
                                continue 'c_125208;
                            } else {
                                main_i = effective_char_info(main_f, cur_l as u16);
                                if !(main_i.s3 > 0) {
                                    char_warning(main_f, cur_chr);
                                    MEM[lig_stack as usize].b32.s1 = avail;
                                    avail = lig_stack;
                                    continue 'c_125208;
                                } else {
                                    MEM[cur_list.tail].b32.s1 = lig_stack;
                                    cur_list.tail = lig_stack as usize;
                                }
                            }
                            current_block = 18270385712206273994;
                        }
                    }
                    'c_125244: loop {
                        match current_block {
                            11331079115679122507 =>
                            // lab112
                            /*main_lig_loop 2 */
                            {
                                if main_j.s2 as i32 == cur_r {
                                    if main_j.s3 <= 128 {
                                        /*1075: */
                                        if main_j.s1 >= 128 {
                                            if cur_l < TOO_BIG_CHAR {
                                                if MEM[cur_q as usize].b32.s1 > TEX_NULL {
                                                    if MEM[cur_list.tail].b16.s0 as i32
                                                        == HYPHEN_CHAR[main_f as usize]
                                                    {
                                                        ins_disc = true
                                                    }
                                                }
                                                if ligature_present {
                                                    main_p = new_ligature(
                                                        main_f,
                                                        cur_l as u16,
                                                        MEM[cur_q as usize].b32.s1,
                                                    )
                                                        as i32;
                                                    if lft_hit {
                                                        MEM[main_p as usize].b16.s0 = 2;
                                                        lft_hit = false
                                                    }
                                                    if rt_hit {
                                                        if lig_stack.is_texnull() {
                                                            MEM[main_p as usize].b16.s0 += 1;
                                                            rt_hit = false
                                                        }
                                                    }
                                                    MEM[cur_q as usize].b32.s1 = main_p;
                                                    cur_list.tail = main_p as usize;
                                                    ligature_present = false
                                                }
                                                if ins_disc {
                                                    ins_disc = false;
                                                    if cur_list.mode as i32 > 0 {
                                                        MEM[cur_list.tail].b32.s1 =
                                                            new_disc() as i32;
                                                        cur_list.tail =
                                                            *LLIST_link(cur_list.tail) as usize;
                                                    }
                                                }
                                            }
                                            MEM[cur_list.tail].b32.s1 = new_kern(
                                                FONT_INFO[(KERN_BASE[main_f as usize]
                                                    + 256 * main_j.s1 as i32
                                                    + main_j.s0 as i32)
                                                    as usize]
                                                    .b32
                                                    .s1,
                                            )
                                                as i32;
                                            cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                                            current_block = 2772858075894446251;
                                        } else {
                                            if cur_l == TOO_BIG_CHAR {
                                                lft_hit = true
                                            } else if lig_stack.is_texnull() {
                                                rt_hit = true
                                            }
                                            match main_j.s1 {
                                                1 | 5 => {
                                                    cur_l = main_j.s0 as i32;
                                                    main_i = FONT_CHARACTER_INFO(
                                                        main_f,
                                                        effective_char(true, main_f, cur_l as u16)
                                                            as usize,
                                                    );
                                                    ligature_present = true;
                                                    current_block = 5062343687657450649;
                                                }
                                                2 | 6 => {
                                                    cur_r = main_j.s0 as i32;
                                                    if lig_stack.is_texnull() {
                                                        lig_stack =
                                                            new_lig_item(cur_r as u16) as i32;
                                                        bchar = TOO_BIG_CHAR
                                                    } else if is_char_node(lig_stack) {
                                                        main_p = lig_stack;
                                                        lig_stack =
                                                            new_lig_item(cur_r as u16) as i32;
                                                        MEM[(lig_stack + 1) as usize].b32.s1 =
                                                            main_p
                                                    } else {
                                                        MEM[lig_stack as usize].b16.s0 =
                                                            cur_r as u16
                                                    }
                                                    current_block = 5062343687657450649;
                                                }
                                                3 => {
                                                    cur_r = main_j.s0 as i32;
                                                    main_p = lig_stack;
                                                    lig_stack = new_lig_item(cur_r as u16) as i32;
                                                    MEM[lig_stack as usize].b32.s1 = main_p;
                                                    current_block = 5062343687657450649;
                                                }
                                                7 | 11 => {
                                                    if cur_l < TOO_BIG_CHAR {
                                                        if MEM[cur_q as usize].b32.s1 > TEX_NULL {
                                                            if MEM[cur_list.tail].b16.s0 as i32
                                                                == HYPHEN_CHAR[main_f as usize]
                                                            {
                                                                ins_disc = true
                                                            }
                                                        }
                                                        if ligature_present {
                                                            main_p = new_ligature(
                                                                main_f,
                                                                cur_l as u16,
                                                                MEM[cur_q as usize].b32.s1,
                                                            )
                                                                as i32;
                                                            if lft_hit {
                                                                MEM[main_p as usize].b16.s0 = 2;
                                                                lft_hit = false
                                                            }
                                                            MEM[cur_q as usize].b32.s1 = main_p;
                                                            cur_list.tail = main_p as usize;
                                                            ligature_present = false
                                                        }
                                                        if ins_disc {
                                                            ins_disc = false;
                                                            if cur_list.mode > 0 {
                                                                MEM[cur_list.tail].b32.s1 =
                                                                    new_disc() as i32;
                                                                cur_list.tail =
                                                                    *LLIST_link(cur_list.tail)
                                                                        as usize;
                                                            }
                                                        }
                                                    }
                                                    cur_q = cur_list.tail as i32;
                                                    cur_l = main_j.s0 as i32;
                                                    main_i = FONT_CHARACTER_INFO(
                                                        main_f,
                                                        effective_char(true, main_f, cur_l as u16)
                                                            as usize,
                                                    );
                                                    ligature_present = true;
                                                    current_block = 5062343687657450649;
                                                }
                                                _ => {
                                                    cur_l = main_j.s0 as i32;
                                                    ligature_present = true;
                                                    if lig_stack.is_texnull() {
                                                        current_block = 7236688557761431611;
                                                    } else {
                                                        current_block = 4014385708774270501;
                                                    }
                                                }
                                            }
                                            match current_block {
                                                7236688557761431611 => {}
                                                4014385708774270501 => {}
                                                _ => {
                                                    if main_j.s1 > 4 {
                                                        if main_j.s1 != 7 {
                                                            current_block = 7236688557761431611;
                                                        } else {
                                                            current_block = 17785146416239343017;
                                                        }
                                                    } else {
                                                        current_block = 17785146416239343017;
                                                    }
                                                    match current_block {
                                                        7236688557761431611 => {}
                                                        _ => {
                                                            if cur_l < TOO_BIG_CHAR {
                                                                current_block = 4700797278417140031;
                                                            } else {
                                                                main_k =
                                                                    BCHAR_LABEL[main_f as usize];
                                                                current_block =
                                                                    13962460947151495567;
                                                                break;
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        current_block = 17910696963991344696;
                                    }
                                } else {
                                    current_block = 17910696963991344696;
                                }
                                match current_block {
                                    2772858075894446251 => {}
                                    7236688557761431611 => {}
                                    4014385708774270501 => {}
                                    4700797278417140031 => {}
                                    _ => {
                                        if main_j.s3 as i32 == 0 {
                                            main_k += 1;
                                            current_block = 13962460947151495567;
                                            break;
                                        } else if !(main_j.s3 >= 128) {
                                            main_k = main_k + main_j.s3 as i32 + 1;
                                            current_block = 13962460947151495567;
                                            break;
                                        }
                                        current_block = 7236688557761431611;
                                    }
                                }
                            }
                            _ => {
                                // lab100:
                                /*main_loop_lookahead *//*1073: */
                                get_next();
                                if cur_cmd == LETTER as u8 {
                                    current_block = 10120566026430170701;
                                } else if cur_cmd == OTHER_CHAR as u8 {
                                    current_block = 10120566026430170701;
                                } else if cur_cmd == CHAR_GIVEN as u8 {
                                    current_block = 10120566026430170701;
                                } else {
                                    x_token();
                                    if cur_cmd == LETTER as u8 {
                                        current_block = 10120566026430170701;
                                    } else if cur_cmd == OTHER_CHAR as u8 {
                                        current_block = 10120566026430170701;
                                    } else if cur_cmd == CHAR_GIVEN as u8 {
                                        current_block = 10120566026430170701;
                                    } else if cur_cmd == CHAR_NUM as u8 {
                                        scan_char_num();
                                        cur_chr = cur_val;
                                        current_block = 10120566026430170701;
                                    } else {
                                        if cur_cmd == NO_BOUNDARY as u8 {
                                            bchar = TOO_BIG_CHAR;
                                        }
                                        cur_r = bchar;
                                        lig_stack = TEX_NULL;
                                        current_block = 4700797278417140031;
                                    }
                                }
                                match current_block {
                                    4700797278417140031 => {}
                                    _ => {
                                        // lab101:
                                        /*main_loop_lookahead 1 */
                                        main_s = (*SF_CODE(cur_chr as usize) as i64 % 65536) as i32; /*:1073 */
                                        if main_s == 1000 {
                                            cur_list.aux.b32.s0 = 1000
                                        } else if main_s < 1000 {
                                            if main_s > 0 {
                                                cur_list.aux.b32.s0 = main_s;
                                            }
                                        } else if cur_list.aux.b32.s0 < 1000 {
                                            cur_list.aux.b32.s0 = 1000;
                                        } else {
                                            cur_list.aux.b32.s0 = main_s;
                                        }
                                        cur_ptr = TEX_NULL;
                                        space_class =
                                            (*SF_CODE(cur_chr as usize) as i64 / 65536) as i32;
                                        if *INTPAR(IntPar::xetex_inter_char_tokens) > 0
                                            && space_class != CHAR_CLASS_LIMIT
                                        {
                                            if prev_class == CHAR_CLASS_LIMIT - 1 {
                                                if cur_input.state != TOKEN_LIST
                                                    || cur_input.index != BACKED_UP_CHAR
                                                {
                                                    find_sa_element(
                                                        INTER_CHAR_VAL as small_number,
                                                        (CHAR_CLASS_LIMIT - 1) * CHAR_CLASS_LIMIT
                                                            + space_class,
                                                        false,
                                                    );
                                                    if !cur_ptr.is_texnull() {
                                                        if cur_cmd != LETTER as u8 {
                                                            cur_cmd = OTHER_CHAR as u8
                                                        }
                                                        cur_tok =
                                                            cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                                                        back_input();
                                                        cur_input.index = BACKED_UP_CHAR;
                                                        begin_token_list(
                                                            MEM[(cur_ptr + 1) as usize].b32.s1
                                                                as usize,
                                                            INTER_CHAR_TEXT,
                                                        );
                                                        continue 'c_125208;
                                                    }
                                                }
                                            } else {
                                                find_sa_element(
                                                    INTER_CHAR_VAL as small_number,
                                                    prev_class * CHAR_CLASS_LIMIT + space_class,
                                                    false,
                                                );
                                                if !cur_ptr.is_texnull() {
                                                    if cur_cmd != LETTER as u8 {
                                                        cur_cmd = OTHER_CHAR as u8;
                                                    }
                                                    cur_tok =
                                                        cur_cmd as i32 * MAX_CHAR_VAL + cur_chr;
                                                    back_input();
                                                    cur_input.index = BACKED_UP_CHAR;
                                                    begin_token_list(
                                                        MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                                                        INTER_CHAR_TEXT,
                                                    );
                                                    prev_class = CHAR_CLASS_LIMIT - 1;
                                                    continue 'c_125208;
                                                }
                                            }
                                            prev_class = space_class
                                        }
                                        lig_stack = avail;
                                        if lig_stack.is_texnull() {
                                            lig_stack = get_avail() as i32;
                                        } else {
                                            avail = MEM[lig_stack as usize].b32.s1;
                                            MEM[lig_stack as usize].b32.s1 = TEX_NULL
                                        }
                                        MEM[lig_stack as usize].b16.s1 = main_f as u16;
                                        cur_r = cur_chr;
                                        MEM[lig_stack as usize].b16.s0 = cur_r as u16;
                                        if cur_r == false_bchar {
                                            cur_r = TOO_BIG_CHAR;
                                        }
                                        current_block = 4700797278417140031; // lab110
                                    }
                                }
                            }
                        }
                        loop {
                            match current_block {
                                7236688557761431611 => {
                                    // lab80:
                                    /*main_loop_wrapup *//*1070: */
                                    if cur_l < TOO_BIG_CHAR {
                                        if MEM[cur_q as usize].b32.s1 > TEX_NULL {
                                            if MEM[cur_list.tail].b16.s0 as i32
                                                == HYPHEN_CHAR[main_f as usize]
                                            {
                                                ins_disc = true
                                            }
                                        }
                                        if ligature_present {
                                            main_p = new_ligature(
                                                main_f,
                                                cur_l as u16,
                                                MEM[cur_q as usize].b32.s1 as i32,
                                            )
                                                as i32;
                                            if lft_hit {
                                                MEM[main_p as usize].b16.s0 = 2;
                                                lft_hit = false
                                            }
                                            if rt_hit {
                                                if lig_stack.is_texnull() {
                                                    MEM[main_p as usize].b16.s0 += 1;
                                                    rt_hit = false
                                                }
                                            }
                                            MEM[cur_q as usize].b32.s1 = main_p;
                                            cur_list.tail = main_p as usize;
                                            ligature_present = false
                                        }
                                        if ins_disc {
                                            ins_disc = false;
                                            if cur_list.mode > 0 {
                                                MEM[cur_list.tail].b32.s1 = new_disc() as i32;
                                                cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                                            }
                                        }
                                    }
                                    current_block = 2772858075894446251;
                                }
                                4700797278417140031 =>
                                /*main_lig_loop *//*1074: */
                                {
                                    if main_i.s1 as i32 % 4i32 != 1i32 {
                                        current_block = 7236688557761431611;
                                        continue;
                                    }
                                    if cur_r == 65536i32 {
                                        current_block = 7236688557761431611;
                                    } else {
                                        break;
                                    }
                                }
                                2772858075894446251 =>
                                // lab90:
                                /*main_loop_move *//*1071: */
                                {
                                    if lig_stack.is_texnull() {
                                        break 'c_125239;
                                    }
                                    cur_q = cur_list.tail as i32;
                                    cur_l = MEM[lig_stack as usize].b16.s0 as i32;
                                    current_block = 4014385708774270501;
                                }
                                _ =>
                                // lab91:
                                /*main_loop_move 1 */
                                {
                                    if is_char_node(lig_stack) {
                                        current_block = 249799543778823886; // lab92
                                        break 'c_125244;
                                    }
                                    // lab95:
                                    /*main_loop_move_lig *//*1072: */
                                    main_p = MEM[(lig_stack + 1) as usize].b32.s1;
                                    if main_p > TEX_NULL {
                                        MEM[cur_list.tail].b32.s1 = main_p;
                                        cur_list.tail = *LLIST_link(cur_list.tail) as usize;
                                    }
                                    temp_ptr = lig_stack;
                                    lig_stack = MEM[temp_ptr as usize].b32.s1;
                                    free_node(temp_ptr as usize, SMALL_NODE_SIZE);
                                    main_i = FONT_CHARACTER_INFO(
                                        main_f,
                                        effective_char(true, main_f, cur_l as u16) as usize,
                                    );
                                    ligature_present = true;
                                    if lig_stack.is_texnull() {
                                        if main_p > TEX_NULL {
                                            current_block = 18270385712206273994; // lab100
                                            continue 'c_125244;
                                        }
                                        cur_r = bchar;
                                        current_block = 4700797278417140031;
                                    } else {
                                        cur_r = MEM[lig_stack as usize].b16.s0 as i32;
                                        current_block = 4700797278417140031;
                                    }
                                }
                            }
                        }
                        main_k = LIG_KERN_BASE[main_f as usize] + main_i.s0 as i32;
                        main_j = FONT_INFO[main_k as usize].b16;
                        if main_j.s3 as i32 <= 128 {
                            current_block = 11331079115679122507; // lab112
                            continue;
                        }
                        main_k = ((LIG_KERN_BASE[main_f as usize]
                            + 256 * main_j.s1 as i32
                            + main_j.s0 as i32) as i64
                            + 32768
                            - (256 * 128) as i64) as font_index;
                        current_block = 13962460947151495567; // lab111
                        break;
                    }
                }
            }
        }
        match current_block {
            11459959175219260272 => app_space(),
            _ =>
            // lab100:
            /*append_normal_space */
            {
                if *INTPAR(IntPar::xetex_inter_char_tokens) > 0
                    && space_class != CHAR_CLASS_LIMIT
                    && prev_class != CHAR_CLASS_LIMIT - 1
                {
                    prev_class = CHAR_CLASS_LIMIT - 1;
                    find_sa_element(
                        INTER_CHAR_VAL as small_number,
                        space_class * CHAR_CLASS_LIMIT + (CHAR_CLASS_LIMIT - 1),
                        false,
                    );
                    if !cur_ptr.is_texnull() {
                        if cur_cs == 0 {
                            if cur_cmd == CHAR_NUM as u8 {
                                cur_cmd = OTHER_CHAR as u8;
                            }
                            cur_tok = cur_cmd as i32 * MAX_CHAR_VAL + cur_chr
                        } else {
                            cur_tok = CS_TOKEN_FLAG + cur_cs
                        }
                        back_input();
                        begin_token_list(
                            MEM[(cur_ptr + 1) as usize].b32.s1 as usize,
                            INTER_CHAR_TEXT,
                        );
                        continue;
                    }
                }
                if *GLUEPAR(GluePar::space_skip) == 0 {
                    main_p = FONT_GLUE[EQTB[CUR_FONT_LOC].b32.s1 as usize];
                    if main_p.is_texnull() {
                        let main_p_ = new_spec(0);
                        main_p = main_p_ as i32;
                        main_k = PARAM_BASE[EQTB[CUR_FONT_LOC].b32.s1 as usize] + 2;
                        MEM[main_p_ + 1].b32.s1 = FONT_INFO[main_k as usize].b32.s1;
                        MEM[main_p_ + 2].b32.s1 = FONT_INFO[(main_k + 1) as usize].b32.s1;
                        MEM[main_p_ + 3].b32.s1 = FONT_INFO[(main_k + 2) as usize].b32.s1;
                        FONT_GLUE[EQTB[CUR_FONT_LOC].b32.s1 as usize] = main_p
                    }
                    temp_ptr = new_glue(main_p) as i32
                } else {
                    temp_ptr = new_param_glue(GluePar::space_skip as small_number) as i32
                }
                MEM[cur_list.tail].b32.s1 = temp_ptr;
                cur_list.tail = temp_ptr as usize;
            }
        }
    }
}
pub(crate) unsafe fn give_err_help() {
    token_show(LOCAL(Local::err_help).opt());
}
pub(crate) unsafe fn close_files_and_terminate() {
    terminate_font_manager();
    for k in 0..=15 {
        if write_open[k] {
            ttstub_output_close(write_file[k].take().unwrap());
        }
    }
    finalize_dvi_file();
    synctex_terminate(log_opened);
    if log_opened {
        ttstub_output_putc(log_file.as_mut().unwrap(), '\n' as i32);
        ttstub_output_close(log_file.take().unwrap());
        log_file = None;
        selector = u8::from(selector).wrapping_sub(2).into();
        if selector == Selector::TERM_ONLY {
            print_nl_cstr(b"Transcript written on ");
            print(texmf_log_name);
            print_char('.' as i32);
        }
    }
    print_ln();
}
pub(crate) unsafe fn flush_str(mut s: str_number) {
    if s == str_ptr - 1 {
        str_ptr -= 1;
        pool_ptr = str_start[(str_ptr - TOO_BIG_CHAR) as usize]
    };
}
pub(crate) unsafe fn tokens_to_string(mut p: i32) -> str_number {
    if selector == Selector::NEW_STRING {
        pdf_error(
            b"tokens",
            b"tokens_to_string() called while selector = new_string",
        );
    }
    old_setting = selector;
    selector = Selector::NEW_STRING;
    show_token_list(MEM[p as usize].b32.s1, TEX_NULL, pool_size - pool_ptr);
    selector = old_setting;
    make_string()
}
pub(crate) unsafe fn scan_pdf_ext_toks() {
    scan_toks(false, true);
}
pub(crate) unsafe fn compare_strings() {
    unsafe fn done(s1: str_number, s2: str_number) {
        flush_str(s2);
        flush_str(s1);
        cur_val_level = 0_u8;
    }
    scan_toks(false, true);
    let s1 = tokens_to_string(def_ref as i32);
    delete_token_ref(def_ref);
    scan_toks(false, true);
    let s2 = tokens_to_string(def_ref as i32);
    delete_token_ref(def_ref);
    let mut i1 = str_start[(s1 as i64 - 65536) as usize];
    let j1 = str_start[((s1 + 1i32) as i64 - 65536) as usize];
    let mut i2 = str_start[(s2 as i64 - 65536) as usize];
    let j2 = str_start[((s2 + 1i32) as i64 - 65536) as usize];
    loop {
        if !(i1 < j1 && i2 < j2) {
            break;
        }
        if (str_pool[i1 as usize] as i32) < str_pool[i2 as usize] as i32 {
            cur_val = -1i32;
            return done(s1, s2);
        } else if str_pool[i1 as usize] as i32 > str_pool[i2 as usize] as i32 {
            cur_val = 1i32;
            return done(s1, s2);
        } else {
            i1 += 1;
            i2 += 1
        }
    }
    if i1 == j1 && i2 == j2 {
        cur_val = 0i32
    } else if i1 < j1 {
        cur_val = 1i32
    } else {
        cur_val = -1i32
    }
    done(s1, s2);
}
pub(crate) unsafe fn prune_page_top(mut p: i32, mut s: bool) -> i32 {
    let mut r: i32 = TEX_NULL;
    let mut prev_p = TEMP_HEAD;
    MEM[TEMP_HEAD].b32.s1 = p;
    while !p.is_texnull() {
        match MEM[p as usize].b16.s1 {
            HLIST_NODE | VLIST_NODE | RULE_NODE => {
                let q = new_skip_param(GluePar::split_top_skip as small_number);
                MEM[prev_p].b32.s1 = q as i32;
                MEM[q as usize].b32.s1 = p;
                if MEM[(temp_ptr + 1) as usize].b32.s1 > MEM[(p + 3) as usize].b32.s1 {
                    MEM[(temp_ptr + 1) as usize].b32.s1 =
                        MEM[(temp_ptr + 1) as usize].b32.s1 - MEM[(p + 3) as usize].b32.s1
                } else {
                    MEM[(temp_ptr + 1) as usize].b32.s1 = 0
                }
                p = TEX_NULL
            }
            WHATSIT_NODE | MARK_NODE | INS_NODE => {
                prev_p = p as usize;
                p = MEM[prev_p].b32.s1
            }
            GLUE_NODE | KERN_NODE | PENALTY_NODE => {
                let q = p;
                p = MEM[q as usize].b32.s1;
                MEM[q as usize].b32.s1 = TEX_NULL;
                MEM[prev_p].b32.s1 = p;
                if s {
                    if disc_ptr[VSPLIT_CODE as usize].is_texnull() {
                        disc_ptr[VSPLIT_CODE as usize] = q
                    } else {
                        MEM[r as usize].b32.s1 = q
                    }
                    r = q
                } else {
                    flush_node_list(q);
                }
            }
            _ => confusion(b"pruning"),
        }
    }
    MEM[TEMP_HEAD].b32.s1
}
pub(crate) unsafe fn do_marks(mut a: small_number, mut l: small_number, mut q: i32) -> bool {
    if l < 4 {
        for i in 0..=15 {
            if i as i32 & 1i32 != 0 {
                cur_ptr = MEM[(q + i / 2 + 1) as usize].b32.s1
            } else {
                cur_ptr = MEM[(q + i / 2 + 1) as usize].b32.s0
            }
            if !cur_ptr.is_texnull() {
                if do_marks(a, (l as i32 + 1i32) as small_number, cur_ptr) {
                    if i as i32 & 1i32 != 0 {
                        MEM[(q + i / 2 + 1) as usize].b32.s1 = TEX_NULL
                    } else {
                        MEM[(q + i / 2 + 1) as usize].b32.s0 = TEX_NULL
                    }
                    MEM[q as usize].b16.s0 -= 1;
                }
            }
        }
        if MEM[q as usize].b16.s0 == 0 {
            free_node(q as usize, INDEX_NODE_SIZE);
            q = TEX_NULL;
        }
    } else {
        match a {
            VSPLIT_INIT => {
                /*1614: */
                if !MEM[(q + 2) as usize].b32.s1.is_texnull() {
                    delete_token_ref(MEM[(q + 2) as usize].b32.s1 as usize);
                    MEM[(q + 2) as usize].b32.s1 = TEX_NULL;
                    delete_token_ref(MEM[(q + 3) as usize].b32.s0 as usize);
                    MEM[(q + 3) as usize].b32.s0 = TEX_NULL
                }
            }
            FIRE_UP_INIT => {
                if !MEM[(q + 2) as usize].b32.s0.is_texnull() {
                    if !MEM[(q + 1) as usize].b32.s0.is_texnull() {
                        delete_token_ref(MEM[(q + 1) as usize].b32.s0 as usize);
                    }
                    delete_token_ref(MEM[(q + 1) as usize].b32.s1 as usize);
                    MEM[(q + 1) as usize].b32.s1 = TEX_NULL;
                    if MEM[MEM[(q + 2) as usize].b32.s0 as usize]
                        .b32
                        .s1
                        .is_texnull()
                    {
                        delete_token_ref(MEM[(q + 2) as usize].b32.s0 as usize);
                        MEM[(q + 2) as usize].b32.s0 = TEX_NULL
                    } else {
                        MEM[MEM[(q + 2) as usize].b32.s0 as usize].b32.s0 += 1;
                    }
                    MEM[(q + 1) as usize].b32.s0 = MEM[(q + 2) as usize].b32.s0
                }
            }
            FIRE_UP_DONE => {
                if !MEM[(q + 1) as usize].b32.s0.is_texnull()
                    && MEM[(q + 1) as usize].b32.s1.is_texnull()
                {
                    MEM[(q + 1) as usize].b32.s1 = MEM[(q + 1) as usize].b32.s0;
                    MEM[MEM[(q + 1) as usize].b32.s0 as usize].b32.s0 += 1;
                }
            }
            DESTROY_MARKS => {
                for i in 0..=4 {
                    if i as i32 & 1i32 != 0 {
                        cur_ptr = MEM[(q + i / 2 + 1) as usize].b32.s1
                    } else {
                        cur_ptr = MEM[(q + i / 2 + 1) as usize].b32.s0
                    }
                    if !cur_ptr.is_texnull() {
                        delete_token_ref(cur_ptr as usize);
                        if i as i32 & 1i32 != 0 {
                            MEM[(q + i / 2 + 1) as usize].b32.s1 = TEX_NULL
                        } else {
                            MEM[(q + i / 2 + 1) as usize].b32.s0 = TEX_NULL
                        }
                    }
                }
            }
            _ => {}
        }
        if MEM[(q + 2) as usize].b32.s0.is_texnull() {
            if MEM[(q + 3) as usize].b32.s0.is_texnull() {
                free_node(q as usize, MARK_CLASS_NODE_SIZE);
                q = TEX_NULL
            }
        }
    }
    q.is_texnull()
}
pub(crate) unsafe fn do_assignments() {
    loop {
        loop {
            get_x_token();
            if !(cur_cmd == SPACER as u8 || cur_cmd == RELAX as u8) {
                break;
            }
        }
        if cur_cmd <= MAX_NON_PREFIXED_COMMAND as u8 {
            return;
        }
        set_box_allowed = false;
        prefixed_command();
        set_box_allowed = true
    }
}
/* the former xetexcoerce.h: */
pub(crate) unsafe fn new_whatsit(mut s: small_number, mut w: small_number) {
    let p = get_node(w as i32);
    *NODE_type(p) = WHATSIT_NODE;
    MEM[p].b16.s0 = s as u16;
    MEM[cur_list.tail].b32.s1 = p as i32;
    cur_list.tail = p;
}
