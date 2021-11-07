use crate::encodings::EncodingDefinition;

#[rustfmt::skip]
pub const ENCODING_CWI_2: EncodingDefinition = EncodingDefinition {
    encoding_table: [
        '\u{0000}', '\u{263A}', '\u{263B}', '\u{2665}', '\u{2666}', '\u{2663}', '\u{2660}', '\u{2022}', // 0x00 - 0x07
        '\u{25D8}', '\u{25CB}', '\u{25D9}', '\u{2642}', '\u{2640}', '\u{266A}', '\u{266B}', '\u{263C}', // 0x08 - 0x0F
        '\u{25BA}', '\u{25C4}', '\u{2195}', '\u{203C}', '\u{00B6}', '\u{00A7}', '\u{25AC}', '\u{21A8}', // 0x10 - 0x17
        '\u{2191}', '\u{2193}', '\u{2192}', '\u{2190}', '\u{221F}', '\u{2194}', '\u{25B2}', '\u{25BC}', // 0x18 - 0x1F
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

        '\u{00C7}', '\u{00FC}', '\u{00E9}', '\u{00E2}', '\u{00E4}', '\u{00E0}', '\u{00E5}', '\u{00E7}', // 0x80 - 0x87
        '\u{00EA}', '\u{00EB}', '\u{00E8}', '\u{00EF}', '\u{00EE}', '\u{00CD}', '\u{00C4}', '\u{00C1}', // 0x88 - 0x8F
        '\u{00C9}', '\u{00E6}', '\u{00C6}', '\u{0151}', '\u{00F6}', '\u{00D3}', '\u{0171}', '\u{00DA}', // 0x90 - 0x97
        '\u{0170}', '\u{00D6}', '\u{00DC}', '\u{00A2}', '\u{00A3}', '\u{00A5}', '\u{20A7}', '\u{0192}', // 0x98 - 0x9F
        '\u{00E1}', '\u{00ED}', '\u{00F3}', '\u{00FA}', '\u{00F1}', '\u{00D1}', '\u{00AA}', '\u{0150}', // 0xA0 - 0xA7
        '\u{00BF}', '\u{2310}', '\u{00AC}', '\u{00BD}', '\u{00BC}', '\u{00A1}', '\u{00AB}', '\u{00BB}', // 0xA8 - 0xAF
        '\u{2591}', '\u{2592}', '\u{2593}', '\u{2502}', '\u{2524}', '\u{2561}', '\u{2562}', '\u{2556}', // 0xB0 - 0xB7
        '\u{2555}', '\u{2563}', '\u{2551}', '\u{2557}', '\u{255D}', '\u{255C}', '\u{255B}', '\u{2510}', // 0xB8 - 0xBF
        '\u{2514}', '\u{2534}', '\u{252C}', '\u{251C}', '\u{2500}', '\u{253C}', '\u{255E}', '\u{255F}', // 0xC0 - 0xC7
        '\u{255A}', '\u{2554}', '\u{2569}', '\u{2566}', '\u{2560}', '\u{2550}', '\u{256C}', '\u{2567}', // 0xC8 - 0xCF
        '\u{2568}', '\u{2564}', '\u{2565}', '\u{2559}', '\u{2558}', '\u{2552}', '\u{2553}', '\u{256B}', // 0xD0 - 0xD7
        '\u{256A}', '\u{2518}', '\u{250C}', '\u{2588}', '\u{2584}', '\u{258C}', '\u{2590}', '\u{2580}', // 0xD8 - 0xDF
        '\u{03B1}', '\u{00DF}', '\u{0393}', '\u{03C0}', '\u{03A3}', '\u{03C3}', '\u{00B5}', '\u{03C4}', // 0xE0 - 0xE7
        '\u{03A6}', '\u{0398}', '\u{03A9}', '\u{03B4}', '\u{221E}', '\u{03C6}', '\u{03B5}', '\u{2229}', // 0xE8 - 0xEF
        '\u{2261}', '\u{00B1}', '\u{2265}', '\u{2264}', '\u{2320}', '\u{2321}', '\u{00F7}', '\u{2248}', // 0xF0 - 0xF7
        '\u{00B0}', '\u{2219}', '\u{00B7}', '\u{221A}', '\u{207F}', '\u{00B2}', '\u{25A0}', '\u{00A0}', // 0xF8 - 0xFF
    ],
    is_common: true,
};