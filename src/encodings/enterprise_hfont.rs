use super::EncodingTable;

#[rustfmt::skip]
pub const ENCODING_ENTERPRISE_HFONT: &EncodingTable = &[
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0x00 - 0x07
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{000A}', '\u{0000}', '\u{0000}', // 0x08 - 0x0F
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0x10 - 0x17
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0x18 - 0x1F
    '\u{0020}', '\u{0021}', '\u{0022}', '\u{00A3}', '\u{0024}', '\u{0025}', '\u{0026}', '\u{0027}', // 0x20 - 0x27
    '\u{0028}', '\u{0029}', '\u{002A}', '\u{002B}', '\u{002C}', '\u{002D}', '\u{002E}', '\u{002F}', // 0x28 - 0x2F
    '\u{0030}', '\u{0031}', '\u{0032}', '\u{0033}', '\u{0034}', '\u{0035}', '\u{0036}', '\u{0037}', // 0x30 - 0x37
    '\u{0038}', '\u{0039}', '\u{003A}', '\u{003B}', '\u{003C}', '\u{003D}', '\u{003E}', '\u{003F}', // 0x38 - 0x3F
    '\u{0040}', '\u{0041}', '\u{0042}', '\u{0043}', '\u{0044}', '\u{0045}', '\u{0046}', '\u{0047}', // 0x40 - 0x47
    '\u{0048}', '\u{0049}', '\u{004A}', '\u{004B}', '\u{004C}', '\u{004D}', '\u{004E}', '\u{004F}', // 0x48 - 0x4F
    '\u{0050}', '\u{0051}', '\u{0052}', '\u{0053}', '\u{0054}', '\u{0055}', '\u{0056}', '\u{0057}', // 0x50 - 0x57
    '\u{0058}', '\u{0059}', '\u{005A}', '\u{005B}', '\u{005C}', '\u{005D}', '\u{005E}', '\u{005F}', // 0x58 - 0x5F
    '\u{0060}', '\u{0061}', '\u{0062}', '\u{0063}', '\u{0064}', '\u{0065}', '\u{0066}', '\u{0067}', // 0x60 - 0x67
    '\u{0068}', '\u{0069}', '\u{006A}', '\u{006B}', '\u{006C}', '\u{006D}', '\u{006E}', '\u{006F}', // 0x68 - 0x6F
    '\u{0070}', '\u{0071}', '\u{0072}', '\u{0073}', '\u{0074}', '\u{0075}', '\u{0076}', '\u{0077}', // 0x70 - 0x77
    '\u{0078}', '\u{0079}', '\u{007A}', '\u{007B}', '\u{007C}', '\u{007D}', '\u{007E}', '\u{0000}', // 0x78 - 0x7F

    '\u{0000}', '\u{00E1}', '\u{0000}', '\u{0000}', '\u{00ED}', '\u{00CD}', '\u{00F3}', '\u{00F6}', // 0x80 - 0x87
    '\u{0151}', '\u{00DC}', '\u{00FA}', '\u{00FC}', '\u{0171}', '\u{0000}', '\u{0000}', '\u{0170}', // 0x88 - 0x8F
    '\u{03C0}', '\u{00C1}', '\u{00D3}', '\u{00E9}', '\u{00D6}', '\u{00DA}', '\u{0000}', '\u{00C9}', // 0x90 - 0x97
    '\u{0000}', '\u{0150}', '\u{0000}', '\u{2193}', '\u{0000}', '\u{0000}', '\u{2190}', '\u{2588}', // 0x98 - 0x9F
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xA0 - 0xA7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xA8 - 0xAF
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xB0 - 0xB7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xB8 - 0xBF
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xC0 - 0xC7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xC8 - 0xCF
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xD0 - 0xD7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xD8 - 0xDF
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xE0 - 0xE7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xE8 - 0xEF
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xF0 - 0xF7
    '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', '\u{0000}', // 0xF8 - 0xFF
];
