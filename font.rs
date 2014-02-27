#[allow(non_camel_case_types)];

use std::ptr;
use std::default::Default;

use ll::*;
use wchar::*;

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

pub enum Quality {
    DEFAULT_QUALITY = 0,
    DRAFT_QUALITY = 1,
    PROOF_QUALITY = 2,
    NONANTIALIASED_QUALITY = 3,
    ANTIALIASED_QUALITY = 4,
    // #if _WIN32_WINNT >= 0x0500
    CLEARTYPE_QUALITY = 5,
}

pub enum Pitch {
    DEFAULT_PITCH = 0,
    FIXED_PITCH = 1,
    VARIABLE_PITCH = 2,
}

pub enum Family {
    FF_DECORATIVE = 80,
    FF_DONTCARE = 0,
    FF_MODERN = 48,
    FF_ROMAN = 16,
    FF_SCRIPT = 64,
    FF_SWISS = 32,
}

pub struct FontAttr {
    height: int,
    width: int,
    escapement: int,
    orientation: int,
    weight: int,
    italic: bool,
    underline: bool,
    strike_out: bool,
    char_set: CharSet,
    output_precision: OutputPrecision,
    clip_precision: ClipPrecision,
    quality: Quality,
    pitch: Pitch,
    family: Family,
    face: Option<~str>,
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
            char_set: DEFAULT_CHARSET,
            output_precision: OUT_DEFAULT_PRECIS,
            clip_precision: CLIP_DEFAULT_PRECIS,
            quality: DEFAULT_QUALITY,
            pitch: DEFAULT_PITCH,
            family: FF_DONTCARE,
            face: None,
        }
    }
}

pub struct Font {
    font: HFONT,
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
            CreateFontW(
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
                face.as_ptr()
            )
        };
        if hfont == ptr::mut_null() {
            None
        }
        else {
            Some(Font { font: hfont })
        }
    }
}
