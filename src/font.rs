// Copyright 2015 The Rust-Windows Project Developers. See the
// COPYRIGHT file at the top-level directory of this distribution.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(non_camel_case_types)]

use std::ptr;
use std::default::Default;

use gdi32;
use winapi::{DWORD, HFONT, c_int};

use wchar::ToCU16Str;

#[derive(Copy)]
pub enum CharSet {
    ANSI_CHARSET = 0,
    DEFAULT_CHARSET = 1,
    SYMBOL_CHARSET = 2,
    SHIFTJIS_CHARSET = 128,
    // HANGEUL_CHARSET = 129,
    HANGUL_CHARSET =  129,
    GB2312_CHARSET = 134,
    CHINESEBIG5_CHARSET = 136,
    GREEK_CHARSET = 161,
    TURKISH_CHARSET = 162,
    HEBREW_CHARSET = 177,
    ARABIC_CHARSET = 178,
    BALTIC_CHARSET = 186,
    RUSSIAN_CHARSET = 204,
    THAI_CHARSET = 222,
    EASTEUROPE_CHARSET = 238,
    OEM_CHARSET = 255,
    JOHAB_CHARSET = 130,
    VIETNAMESE_CHARSET = 163,
    MAC_CHARSET = 77,
}

#[derive(Copy)]
pub enum OutputPrecision {
    OUT_DEFAULT_PRECIS = 0,
    OUT_STRING_PRECIS = 1,
    OUT_CHARACTER_PRECIS = 2,
    OUT_STROKE_PRECIS = 3,
    OUT_TT_PRECIS = 4,
    OUT_DEVICE_PRECIS = 5,
    OUT_RASTER_PRECIS = 6,
    OUT_TT_ONLY_PRECIS = 7,
    OUT_OUTLINE_PRECIS = 8,
    OUT_PS_ONLY_PRECIS = 10,
}

#[derive(Copy)]
pub enum ClipPrecision {
    CLIP_DEFAULT_PRECIS = 0,
    CLIP_CHARACTER_PRECIS = 1,
    CLIP_STROKE_PRECIS = 2,
    CLIP_MASK = 15,
    CLIP_LH_ANGLES = 16,
    CLIP_TT_ALWAYS = 32,
    CLIP_EMBEDDED = 128,
    // TODO:
    // CLIP_DFA_DISABLE
    // CLIP_DFA_OVERRIDE
}

#[derive(Copy)]
pub enum Quality {
    DEFAULT_QUALITY = 0,
    DRAFT_QUALITY = 1,
    PROOF_QUALITY = 2,
    NONANTIALIASED_QUALITY = 3,
    ANTIALIASED_QUALITY = 4,
    // #if _WIN32_WINNT >= 0x0500
    CLEARTYPE_QUALITY = 5,
}

#[derive(Copy)]
pub enum Pitch {
    DEFAULT_PITCH = 0,
    FIXED_PITCH = 1,
    VARIABLE_PITCH = 2,
}

#[derive(Copy)]
pub enum Family {
    FF_DECORATIVE = 80,
    FF_DONTCARE = 0,
    FF_MODERN = 48,
    FF_ROMAN = 16,
    FF_SCRIPT = 64,
    FF_SWISS = 32,
}

pub struct FontAttr {
    pub height: isize,
    pub width: isize,
    pub escapement: isize,
    pub orientation: isize,
    pub weight: isize,
    pub italic: bool,
    pub underline: bool,
    pub strike_out: bool,
    pub char_set: CharSet,
    pub output_precision: OutputPrecision,
    pub clip_precision: ClipPrecision,
    pub quality: Quality,
    pub pitch: Pitch,
    pub family: Family,
    pub face: Option<String>,
}

impl Default for FontAttr {
    fn default() -> FontAttr {
        FontAttr {
            height: 0,
            width: 0,
            escapement: 0,
            orientation: 0,
            weight: 400, // FW_NORMAL. TODO use FW_DONTCARE (0)?
            italic: false,
            underline: false,
            strike_out: false,
            char_set: CharSet::DEFAULT_CHARSET,
            output_precision: OutputPrecision::OUT_DEFAULT_PRECIS,
            clip_precision: ClipPrecision::CLIP_DEFAULT_PRECIS,
            quality: Quality::DEFAULT_QUALITY,
            pitch: Pitch::DEFAULT_PITCH,
            family: Family::FF_DONTCARE,
            face: None,
        }
    }
}

#[derive(Copy)]
pub struct Font {
    pub font: HFONT,
}

impl Clone for Font {
    fn clone(&self) -> Font {
        Font {
            font: self.font,
        }
    }
}

impl Font {
    pub fn new(attr: &FontAttr) -> Option<Font> {
        let face = attr.face.to_c_u16();
        let hfont = unsafe {
            gdi32::CreateFontW(
                attr.height as c_int,
                attr.width as c_int,
                attr.escapement as c_int,
                attr.orientation as c_int,
                attr.weight as c_int,
                attr.italic as DWORD,
                attr.underline as DWORD,
                attr.strike_out as DWORD,
                attr.char_set as DWORD,
                attr.output_precision as DWORD,
                attr.clip_precision as DWORD,
                attr.quality as DWORD,
                (attr.pitch as DWORD) | (attr.family as DWORD),
                if face.len()==0 { ptr::null_mut() } else { face.as_ptr() },
            )
        };
        if hfont == ptr::null_mut() {
            None
        }
        else {
            Some(Font { font: hfont })
        }
    }
}
