mod cork;
mod cp1250;
mod cp1252;
mod cp254;
mod cp320;
mod cp437;
mod cp850;
mod cp852;
mod cwi1;
mod cwi2;
mod enterprise_hfont;
mod enterprise_hun;
mod enterprise_plus_hun;
mod iso646hu;
mod iso88591;
mod iso88592;
mod mac_centeuro;
mod mac_roman;
mod szki;
mod tvc;

pub use cork::ENCODING_CORK;
pub use cp1250::ENCODING_CP1250;
pub use cp1252::ENCODING_CP1252;
pub use cp254::ENCODING_CP254;
pub use cp320::ENCODING_CP320;
pub use cp437::ENCODING_CP437;
pub use cp850::ENCODING_CP850;
pub use cp852::ENCODING_CP852;
pub use cwi1::ENCODING_CWI1;
pub use cwi2::ENCODING_CWI2;
pub use enterprise_hfont::ENCODING_ENTERPRISE_HFONT;
pub use enterprise_hun::ENCODING_ENTERPRISE_HUN;
pub use enterprise_plus_hun::ENCODING_ENTERPRISE_PLUS_HUN;
pub use iso646hu::ENCODING_ISO646HU;
pub use iso88591::ENCODING_ISO88591;
pub use iso88592::ENCODING_ISO88592;
pub use mac_centeuro::ENCODING_MAC_CENTEURO;
pub use mac_roman::ENCODING_MAC_ROMAN;
pub use szki::ENCODING_SZKI;
pub use tvc::ENCODING_TVC;

pub type EncodingTable = [char; 256];

// rustfmt still doesn't properly support vertical alignment.
#[rustfmt::skip]
pub const ENCODINGS: &[(&str, &EncodingTable, bool)] = &[
    ("CORK",         ENCODING_CORK,                false),
    ("CP1250",       ENCODING_CP1250,              true ),
    ("CP1252",       ENCODING_CP1252,              true ),
    ("CP254",        ENCODING_CP254,               false),
    ("CP320",        ENCODING_CP320,               false),
    ("CP437",        ENCODING_CP437,               true ),
    ("CP850",        ENCODING_CP850,               true ),
    ("CP852",        ENCODING_CP852,               true ),
    ("CWI-1",        ENCODING_CWI1,                false),
    ("CWI-2",        ENCODING_CWI2,                true ),
    ("EP-HFONT",     ENCODING_ENTERPRISE_HFONT,    false),
    ("EP-HUN",       ENCODING_ENTERPRISE_HUN,      false),
    ("EP-PLUS-HUN",  ENCODING_ENTERPRISE_PLUS_HUN, false),
    ("ISO-646-HU",   ENCODING_ISO646HU,            false),
    ("ISO-8859-1",   ENCODING_ISO88591,            true ),
    ("ISO-8859-2",   ENCODING_ISO88592,            true ),
    ("MAC-CENTEURO", ENCODING_MAC_CENTEURO,        false),
    ("MAC-ROMAN",    ENCODING_MAC_ROMAN,           false),
    ("SZKI",         ENCODING_SZKI,                false),
    ("TVC",          ENCODING_TVC,                 false),
];

// TODO: const fn
pub fn max_encoding_name_width() -> usize {
    ENCODINGS
        .iter()
        .map(|(encoding_name, _, _)| encoding_name.len())
        .max()
        .unwrap_or(0)
}

pub fn from_str(name: &str) -> Option<&EncodingTable> {
    let name = name.to_uppercase();
    for (encoding_name, encoding_table, _) in ENCODINGS {
        if *encoding_name == name {
            return Some(encoding_table);
        }
    }
    None
}

pub fn supports_charset(encoding_table: &EncodingTable, charset: &[char]) -> bool {
    charset.iter().all(|c| encoding_table.contains(c))
}
