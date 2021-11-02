use crate::encodings::EncodingTable;

#[rustfmt::skip]
pub const ENCODING_MAC_CENTEURO: &EncodingTable = &[
    '\u{0000}', '\u{0001}', '\u{0002}', '\u{0003}', '\u{0004}', '\u{0005}', '\u{0006}', '\u{0007}', // 0x00 - 0x07
    '\u{0008}', '\u{0009}', '\u{000A}', '\u{000B}', '\u{000C}', '\u{000D}', '\u{000E}', '\u{000F}', // 0x08 - 0x0F
    '\u{0010}', '\u{0011}', '\u{0012}', '\u{0013}', '\u{0014}', '\u{0015}', '\u{0016}', '\u{0017}', // 0x10 - 0x17
    '\u{0018}', '\u{0019}', '\u{001A}', '\u{001B}', '\u{001C}', '\u{001D}', '\u{001E}', '\u{001F}', // 0x18 - 0x1F
    '\u{0020}', '\u{0021}', '\u{0022}', '\u{0023}', '\u{0024}', '\u{0025}', '\u{0026}', '\u{0027}', // 0x20 - 0x27
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
    '\u{0078}', '\u{0079}', '\u{007A}', '\u{007B}', '\u{007C}', '\u{007D}', '\u{007E}', '\u{007F}', // 0x78 - 0x7F

    '\u{00C4}', '\u{0100}', '\u{0101}', '\u{00C9}', '\u{0104}', '\u{00D6}', '\u{00DC}', '\u{00E1}', // 0x80 - 0x87
    '\u{0105}', '\u{010C}', '\u{00E4}', '\u{010D}', '\u{0106}', '\u{0107}', '\u{00E9}', '\u{0179}', // 0x88 - 0x8F
    '\u{017A}', '\u{010E}', '\u{00ED}', '\u{010F}', '\u{0112}', '\u{0113}', '\u{0116}', '\u{00F3}', // 0x90 - 0x97
    '\u{0117}', '\u{00F4}', '\u{00F6}', '\u{00F5}', '\u{00FA}', '\u{011A}', '\u{011B}', '\u{00FC}', // 0x98 - 0x9F
    '\u{2020}', '\u{00B0}', '\u{0118}', '\u{00A3}', '\u{00A7}', '\u{2022}', '\u{00B6}', '\u{00DF}', // 0xA0 - 0xA7
    '\u{00AE}', '\u{00A9}', '\u{2122}', '\u{0119}', '\u{00A8}', '\u{2260}', '\u{0123}', '\u{012E}', // 0xA8 - 0xAF
    '\u{012F}', '\u{012A}', '\u{2264}', '\u{2265}', '\u{012B}', '\u{0136}', '\u{2202}', '\u{2211}', // 0xB0 - 0xB7
    '\u{0142}', '\u{013B}', '\u{013C}', '\u{013D}', '\u{013E}', '\u{0139}', '\u{013A}', '\u{0145}', // 0xB8 - 0xBF
    '\u{0146}', '\u{0143}', '\u{00AC}', '\u{221A}', '\u{0144}', '\u{0147}', '\u{2206}', '\u{00AB}', // 0xC0 - 0xC7
    '\u{00BB}', '\u{2026}', '\u{00A0}', '\u{0148}', '\u{0150}', '\u{00D5}', '\u{0151}', '\u{014C}', // 0xC8 - 0xCF
    '\u{2013}', '\u{2014}', '\u{201C}', '\u{201D}', '\u{2018}', '\u{2019}', '\u{00F7}', '\u{25CA}', // 0xD0 - 0xD7
    '\u{014D}', '\u{0154}', '\u{0155}', '\u{0158}', '\u{2039}', '\u{203A}', '\u{0159}', '\u{0156}', // 0xD8 - 0xDF
    '\u{0157}', '\u{0160}', '\u{201A}', '\u{201E}', '\u{0161}', '\u{015A}', '\u{015B}', '\u{00C1}', // 0xE0 - 0xE7
    '\u{0164}', '\u{0165}', '\u{00CD}', '\u{017D}', '\u{017E}', '\u{016A}', '\u{00D3}', '\u{00D4}', // 0xE8 - 0xEF
    '\u{016B}', '\u{016E}', '\u{00DA}', '\u{016F}', '\u{0170}', '\u{0171}', '\u{0172}', '\u{0173}', // 0xF0 - 0xF7
    '\u{00DD}', '\u{00FD}', '\u{0137}', '\u{017B}', '\u{0141}', '\u{017C}', '\u{0122}', '\u{02C7}', // 0xF8 - 0xFF
];
