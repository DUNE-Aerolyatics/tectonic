/* tectonic/xetex-xetexd.h -- many, many XeTeX symbol definitions
   Copyright 2016-2018 The Tectonic Project
   Licensed under the MIT License.
*/
#![allow(
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
)]

use super::xetex_consts::{
    get_int_par, IntPar, ACTIVE_BASE, BIGGEST_USV, CAT_CODE, DIMEN_VAL_LIMIT, EQTB_SIZE, HASH_BASE,
    NULL_CS, SCRIPT_SIZE, SINGLE_BASE, TEXT_SIZE, UNDEFINED_CONTROL_SEQUENCE,
};
use crate::cmd::Cmd;
use crate::node::NativeWord;
use crate::xetex_scaledmath::Scaled;
use crate::xetex_stringpool::{PoolString, TOO_BIG_CHAR};

use super::xetex_ini::Selector;
use super::xetex_ini::{
    dig, doing_special, error_line, file_offset, hash, line, log_file, max_print_line, pool_ptr,
    pool_size, rust_stdout, selector, str_pool, str_ptr, tally, term_offset, trick_buf,
    trick_count, write_file, EQTB_TOP, FULL_SOURCE_FILENAME_STACK, IN_OPEN, LINE_STACK, MEM,
};

/* Extra stuff used in various change files for various reasons.  */
/* Array allocations. Add 1 to size to account for Pascal indexing convention. */
/*11:*/
/*18: */
pub(crate) type UTF16_code = u16;
pub(crate) type str_number = i32;
/* xetex-output */
/* tectonic/output.c -- functions related to outputting messages
 * Copyright 2016 the Tectonic Project
 * Licensed under the MIT License.
*/
pub(crate) unsafe fn print_ln() {
    use std::io::Write;
    match selector {
        Selector::TERM_AND_LOG => {
            write!(rust_stdout.as_mut().unwrap(), "\n").unwrap();
            write!(log_file.as_mut().unwrap(), "\n").unwrap();
            term_offset = 0;
            file_offset = 0;
        }
        Selector::LOG_ONLY => {
            write!(log_file.as_mut().unwrap(), "\n").unwrap();
            file_offset = 0;
        }
        Selector::TERM_ONLY => {
            write!(rust_stdout.as_mut().unwrap(), "\n").unwrap();
            term_offset = 0;
        }
        Selector::NO_PRINT | Selector::PSEUDO | Selector::NEW_STRING => {}
        Selector::File(u) => {
            write!(write_file[u as usize].as_mut().unwrap(), "\n").unwrap();
        }
    };
}
pub(crate) unsafe fn print_raw_char(s: UTF16_code) {
    match selector {
        Selector::NEW_STRING => {
            if pool_ptr < pool_size {
                str_pool[pool_ptr as usize] = s;
                pool_ptr += 1
            }
        }
        _ => unreachable!(),
    }
}
pub(crate) unsafe fn print_rust_char(s: char) {
    use std::io::Write;
    match selector {
        Selector::TERM_AND_LOG => {
            let stdout = rust_stdout.as_mut().unwrap();
            let lg = log_file.as_mut().unwrap();
            write!(stdout, "{}", s).unwrap();
            write!(lg, "{}", s).unwrap();
            term_offset += 1;
            file_offset += 1;
            if term_offset == max_print_line {
                write!(stdout, "\n").unwrap();
                term_offset = 0;
            }
            if file_offset == max_print_line {
                write!(lg, "\n").unwrap();
                file_offset = 0;
            }
        }
        Selector::LOG_ONLY => {
            write!(log_file.as_mut().unwrap(), "{}", s).unwrap();
            file_offset += 1;
            if file_offset == max_print_line {
                print_ln();
            }
        }
        Selector::TERM_ONLY => {
            write!(rust_stdout.as_mut().unwrap(), "{}", s).unwrap();
            term_offset += 1;
            if term_offset == max_print_line {
                print_ln();
            }
        }
        Selector::NO_PRINT => {}
        Selector::PSEUDO => {
            if tally < trick_count {
                trick_buf[(tally % error_line) as usize] = s;
            }
        }
        Selector::NEW_STRING => unreachable!(),
        Selector::File(u) => {
            write!(write_file[u as usize].as_mut().unwrap(), "{}", s).unwrap();
        }
    }
    tally += 1;
}
// TODO: optimize
pub(crate) unsafe fn print_rust_string(s: &str) {
    use std::io::Write;
    let mut count = s.chars().count() as i32;
    match selector {
        Selector::TERM_AND_LOG => {
            if file_offset + count <= max_print_line && term_offset + count <= max_print_line {
                let stdout = rust_stdout.as_mut().unwrap();
                let lg = log_file.as_mut().unwrap();
                write!(stdout, "{}", s).unwrap();
                write!(lg, "{}", s).unwrap();
                term_offset += count;
                file_offset += count;
                if term_offset == max_print_line {
                    write!(stdout, "\n").unwrap();
                    term_offset = 0;
                }
                if file_offset == max_print_line {
                    write!(lg, "\n").unwrap();
                    file_offset = 0;
                }
            } else {
                for c in s.chars() {
                    print_rust_char(c);
                }
                return;
            }
        }
        Selector::LOG_ONLY => {
            if file_offset + count <= max_print_line {
                write!(log_file.as_mut().unwrap(), "{}", s).unwrap();
                file_offset += count;
                if file_offset == max_print_line {
                    print_ln();
                }
            } else {
                for c in s.chars() {
                    print_rust_char(c);
                }
                return;
            }
        }
        Selector::TERM_ONLY => {
            if term_offset + count <= max_print_line {
                write!(rust_stdout.as_mut().unwrap(), "{}", s).unwrap();
                term_offset += count;
                if term_offset == max_print_line {
                    print_ln();
                }
            } else {
                for c in s.chars() {
                    print_rust_char(c);
                }
                return;
            }
        }
        Selector::NO_PRINT => {}
        Selector::PSEUDO => {
            count = 0;
            for (t, c) in s
                .chars()
                .take((trick_count - tally).max(0) as usize)
                .enumerate()
            {
                trick_buf[((tally as usize + t) % (error_line as usize))] = c;
                count += 1;
            }
        }
        Selector::NEW_STRING => unreachable!(),
        Selector::File(u) => {
            write!(write_file[u as usize].as_mut().unwrap(), "{}", s).unwrap();
        }
    }
    tally += count;
}
pub(crate) unsafe fn print_char(s: i32) {
    print_chr(std::char::from_u32(s as u32).unwrap())
}
pub(crate) unsafe fn print_chr(s: char) {
    if selector == Selector::NEW_STRING {
        if !doing_special {
            let mut b = [0; 2];
            for i in s.encode_utf16(&mut b) {
                print_raw_char(*i);
            }
            return;
        } else {
            let mut b = [0; 4];
            for c in s.encode_utf8(&mut b).bytes() {
                print_raw_char(c as u16);
            }
            return;
        }
    }
    if (s as i32) == get_int_par(IntPar::new_line_char) {
        if selector != Selector::PSEUDO {
            print_ln();
            return;
        }
    }
    match s {
        '\u{0}'..='\u{1f}' => {
            print_rust_string("^^");
            print_rust_char(char::from((s as u8) + 0x40));
        }
        '\u{7f}' => print_rust_string("^^?"),
        '\u{80}'..='\u{9f}' => {
            print_rust_string("^^");
            let s = s as u8;
            for &l in &[(s / 16), (s % 16)] {
                if l < 10 {
                    print_rust_char(char::from(b'0' + l));
                } else {
                    print_rust_char(char::from(b'a' + l - 10));
                }
            }
        }
        _ => print_rust_char(s),
    }
}
pub(crate) unsafe fn print(s: i32) {
    if s < 0 || s >= str_ptr {
        print_cstr("???");
    } else if s < TOO_BIG_CHAR {
        print_char(s);
    } else {
        for c in std::char::decode_utf16(PoolString::from(s).as_slice().iter().cloned()) {
            print_chr(c.unwrap())
        }
    }
}
pub(crate) unsafe fn print_cstr(slice: &str) {
    for &s in slice.as_bytes() {
        print_char(s as i32);
    }
}

pub(crate) unsafe fn print_nl(s: str_number) {
    match selector {
        Selector::TERM_ONLY | Selector::TERM_AND_LOG if term_offset > 0 => print_ln(),
        Selector::LOG_ONLY | Selector::TERM_AND_LOG if file_offset > 0 => print_ln(),
        _ => {}
    }
    print(s);
}
pub(crate) unsafe fn print_nl_cstr(slice: &str) {
    match selector {
        Selector::TERM_ONLY | Selector::TERM_AND_LOG if term_offset > 0 => print_ln(),
        Selector::LOG_ONLY | Selector::TERM_AND_LOG if file_offset > 0 => print_ln(),
        _ => {}
    }
    print_cstr(slice);
}
pub(crate) unsafe fn print_esc(s: str_number) {
    let c = get_int_par(IntPar::escape_char);
    if c >= 0 && c <= BIGGEST_USV as i32 {
        print_char(c);
    }
    print(s);
}
pub(crate) unsafe fn print_esc_cstr(s: &str) {
    let c = get_int_par(IntPar::escape_char);
    if c >= 0 && c <= BIGGEST_USV as i32 {
        print_char(c);
    }
    print_cstr(s);
}
unsafe fn print_the_digs(mut k: u8) {
    while k as i32 > 0 {
        k -= 1;
        if (dig[k as usize] as i32) < 10 {
            print_char('0' as i32 + dig[k as usize] as i32);
        } else {
            print_char(55 + dig[k as usize] as i32);
        }
    }
}
pub(crate) unsafe fn print_int(mut n: i32) {
    let mut k = 0_u8;
    if n < 0 {
        print_chr('-');
        if n as i64 > -100000000 {
            n = -n
        } else {
            let mut m = -1 - n;
            n = m / 10;
            m = m % 10 + 1;
            k = 1_u8;
            if m < 10 {
                dig[0] = m as u8
            } else {
                dig[0] = 0;
                n += 1;
            }
        }
    }
    loop {
        dig[k as usize] = (n % 10) as u8;
        n = n / 10;
        k += 1;
        if n == 0 {
            break;
        }
    }
    print_the_digs(k);
}
pub(crate) unsafe fn print_cs(p: i32) {
    if p < HASH_BASE as i32 {
        if p >= SINGLE_BASE as i32 {
            if p == NULL_CS as i32 {
                print_esc_cstr("csname");
                print_esc_cstr("endcsname");
                print_chr(' ');
            } else {
                print_esc(p - SINGLE_BASE as i32);
                if *CAT_CODE(p as usize - SINGLE_BASE) == Cmd::Letter as _ {
                    print_chr(' ');
                }
            }
        } else if p < ACTIVE_BASE as i32 {
            print_esc_cstr("IMPOSSIBLE.");
        } else {
            print_char(p - 1);
        }
    } else if p >= UNDEFINED_CONTROL_SEQUENCE as i32 && p <= EQTB_SIZE as i32 || p > EQTB_TOP as i32
    {
        print_esc_cstr("IMPOSSIBLE.");
    } else if (*hash.offset(p as isize)).s1 >= str_ptr {
        print_esc_cstr("NONEXISTENT.");
    } else {
        print_esc((*hash.offset(p as isize)).s1);
        print_chr(' ');
    };
}
pub(crate) unsafe fn sprint_cs(p: i32) {
    if p < HASH_BASE as i32 {
        if p < SINGLE_BASE as i32 {
            print_char(p - 1);
        } else if p < NULL_CS as i32 {
            print_esc(p - SINGLE_BASE as i32);
        } else {
            print_esc_cstr("csname");
            print_esc_cstr("endcsname");
        }
    } else {
        print_esc((*hash.offset(p as isize)).s1);
    };
}
pub(crate) unsafe fn print_file_name(n: i32, a: i32, e: i32) {
    let mut must_quote: bool = false;
    let mut quote_char = None;
    if a != 0 {
        for &j in PoolString::from(a).as_slice() {
            if must_quote && quote_char.is_some() {
                break;
            }
            if j as i32 == ' ' as i32 {
                must_quote = true;
            } else if j as i32 == '\"' as i32 {
                must_quote = true;
                quote_char = Some('\'');
            } else if j as i32 == '\'' as i32 {
                must_quote = true;
                quote_char = Some('\"');
            }
        }
    }
    if n != 0 {
        for &j in PoolString::from(n).as_slice() {
            if must_quote && quote_char.is_some() {
                break;
            }
            if j as i32 == ' ' as i32 {
                must_quote = true;
            } else if j as i32 == '\"' as i32 {
                must_quote = true;
                quote_char = Some('\'');
            } else if j as i32 == '\'' as i32 {
                must_quote = true;
                quote_char = Some('\"');
            }
        }
    }
    if e != 0 {
        for &j in PoolString::from(e).as_slice() {
            if must_quote && quote_char.is_some() {
                break;
            }
            if j as i32 == ' ' as i32 {
                must_quote = true;
            } else if j as i32 == '\"' as i32 {
                must_quote = true;
                quote_char = Some('\'');
            } else if j as i32 == '\'' as i32 {
                must_quote = true;
                quote_char = Some('\"');
            }
        }
    }
    if must_quote {
        if let Some(qc) = quote_char {
            print_chr(qc);
        } else {
            quote_char = Some('\"');
            print_chr('\"');
        }
    }
    if a != 0 {
        for j in std::char::decode_utf16(PoolString::from(a).as_slice().iter().cloned()) {
            let j = j.unwrap();
            if Some(j) == quote_char {
                print_chr(j);
                let c = match j {
                    '\"' => '\'',
                    '\'' => '\"',
                    _ => unreachable!(),
                };
                quote_char = Some(c);
                print_chr(c);
            }
            print_chr(j);
        }
    }
    if n != 0 {
        for j in std::char::decode_utf16(PoolString::from(n).as_slice().iter().cloned()) {
            let j = j.unwrap();
            if Some(j) == quote_char {
                print_chr(j);
                let c = match j {
                    '\"' => '\'',
                    '\'' => '\"',
                    _ => unreachable!(),
                };
                quote_char = Some(c);
                print_chr(c);
            }
            print_chr(j);
        }
    }
    if e != 0 {
        for j in std::char::decode_utf16(PoolString::from(e).as_slice().iter().cloned()) {
            let j = j.unwrap();
            if Some(j) == quote_char {
                print_chr(j);
                let c = match j {
                    '\"' => '\'',
                    '\'' => '\"',
                    _ => unreachable!(),
                };
                quote_char = Some(c);
                print_chr(c);
            }
            print_chr(j);
        }
    }
    if let Some(qc) = quote_char {
        print_chr(qc);
    };
}
pub(crate) unsafe fn print_size(s: i32) {
    if s == TEXT_SIZE as i32 {
        print_esc_cstr("textfont");
    } else if s == SCRIPT_SIZE as i32 {
        print_esc_cstr("scriptfont");
    } else {
        print_esc_cstr("scriptscriptfont");
    };
}
pub(crate) unsafe fn print_write_whatsit(s: &str, p: usize) {
    print_esc_cstr(s);
    if MEM[p + 1].b32.s0 < 16 {
        print_int(MEM[p + 1].b32.s0);
    } else if MEM[p + 1].b32.s0 == 16 {
        print_chr('*');
    } else {
        print_chr('-');
    };
}
pub(crate) unsafe fn print_native_word(p: &NativeWord) {
    for c in std::char::decode_utf16(p.text().iter().cloned()) {
        if let Ok(c) = c {
            print_char(c as i32);
        } else {
            print('.' as i32);
        }
    }
}
pub(crate) unsafe fn print_sa_num(mut q: usize) {
    let mut n;
    if MEM[q].b16.s1 < DIMEN_VAL_LIMIT {
        n = MEM[q + 1].b32.s1
    } else {
        n = MEM[q].b16.s1 as i32 % 64;
        q = MEM[q].b32.s1 as usize;
        n = n + 64 * MEM[q].b16.s1 as i32;
        q = MEM[q].b32.s1 as usize;
        n = n + 64 * 64 * (MEM[q].b16.s1 as i32 + 64 * MEM[MEM[q].b32.s1 as usize].b16.s1 as i32)
    }
    print_int(n);
}
pub(crate) unsafe fn print_file_line() {
    let mut level = IN_OPEN;
    while level > 0 && FULL_SOURCE_FILENAME_STACK[level] == 0 {
        level -= 1
    }
    if level == 0 {
        print_nl_cstr("! ");
    } else {
        print_nl_cstr("");
        print(FULL_SOURCE_FILENAME_STACK[level]);
        print(':' as i32);
        if level == IN_OPEN {
            print_int(line);
        } else {
            print_int(LINE_STACK[level + 1]);
        }
        print_cstr(": ");
    };
}

pub(crate) unsafe fn print_hex(mut n: i32) {
    let mut k: u8 = 0_u8;
    print_chr('\"');
    loop {
        dig[k as usize] = (n % 16) as u8;
        n = n / 16;
        k = k.wrapping_add(1);
        if n == 0 {
            break;
        }
    }
    print_the_digs(k);
}
pub(crate) unsafe fn print_roman_int(mut n: i32) {
    const roman_data: &[u8] = b"m2d5c2l5x2v5i";
    let mut j: u8 = 0_u8;
    let mut v = 1000i32;
    loop {
        while n >= v {
            print_char(roman_data[j as usize] as i32);
            n = n - v
        }
        if n <= 0 {
            return;
        }
        let mut k = j + 2;
        let mut u = v / (roman_data[k as usize - 1] as i32 - '0' as i32);
        if roman_data[k as usize - 1] as i32 == '2' as i32 {
            k += 2;
            u = u / (roman_data[k as usize - 1] as i32 - '0' as i32)
        }
        if n + u >= v {
            print_char(roman_data[k as usize] as i32);
            n = n + u
        } else {
            j += 2;
            v = v / (roman_data[j as usize - 1] as i32 - '0' as i32)
        }
    }
}
pub(crate) unsafe fn print_current_string() {
    for c in std::char::decode_utf16(PoolString::current().as_slice().iter().cloned()) {
        print_chr(c.unwrap());
    }
}
pub(crate) unsafe fn print_scaled(s: Scaled) {
    let mut s = s.0;
    if s < 0 {
        print_chr('-');
        s = s.wrapping_neg(); // TODO: check
    }
    print_int(s / 0x10000);
    print_chr('.');
    s = 10 * (s % 0x10000) + 5;
    let mut delta = 10;
    loop {
        if delta > 0x10000 {
            s = s + 0x8000 - 50000
        }
        print_chr(char::from(b'0' + (s / 0x10000) as u8));
        s = 10 * (s % 0x10000);
        delta *= 10;
        if !(s > delta) {
            break;
        }
    }
}
