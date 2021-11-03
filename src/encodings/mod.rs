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

use strum::{Display, EnumIter, EnumString, EnumVariantNames, VariantNames};

pub type EncodingTable = [char; 256];

#[derive(Display, EnumIter, EnumString, EnumVariantNames, Eq, PartialEq)]
pub enum Encoding {
    #[strum(serialize = "CORK")]
    CORK,

    #[strum(serialize = "CP1250")]
    CP1250,

    #[strum(serialize = "CP1252")]
    CP1252,

    #[strum(serialize = "CP254")]
    CP254,

    #[strum(serialize = "CP320")]
    CP320,

    #[strum(serialize = "CP437")]
    CP437,

    #[strum(serialize = "CP850")]
    CP850,

    #[strum(serialize = "CP852")]
    CP852,

    #[strum(serialize = "CWI-1")]
    CWI1,

    #[strum(serialize = "CWI-2")]
    CWI2,

    #[strum(serialize = "EP-HFONT")]
    EnterpriseHfont,

    #[strum(serialize = "EP-HUN")]
    EnterpriseHun,

    #[strum(serialize = "EP-PLUS-HUN")]
    EnterprisePlusHun,

    #[strum(serialize = "ISO-646-HU")]
    ISO646HU,

    #[strum(serialize = "ISO-8859-1")]
    ISO88591,

    #[strum(serialize = "ISO-8859-2")]
    ISO88592,

    #[strum(serialize = "MAC-CENTEURO")]
    MacCenteuro,

    #[strum(serialize = "MAC-ROMAN")]
    MacRoman,

    #[strum(serialize = "SZKI")]
    SZKI,

    #[strum(serialize = "TVC")]
    TVC,
}

impl Encoding {
    pub fn get_encoding_table(&self) -> &EncodingTable {
        match self {
            Self::CORK => cork::ENCODING_CORK,
            Self::CP1250 => cp1250::ENCODING_CP1250,
            Self::CP1252 => cp1252::ENCODING_CP1252,
            Self::CP254 => cp254::ENCODING_CP254,
            Self::CP320 => cp320::ENCODING_CP320,
            Self::CP437 => cp437::ENCODING_CP437,
            Self::CP850 => cp850::ENCODING_CP850,
            Self::CP852 => cp852::ENCODING_CP852,
            Self::CWI1 => cwi1::ENCODING_CWI1,
            Self::CWI2 => cwi2::ENCODING_CWI2,
            Self::EnterpriseHfont => enterprise_hfont::ENCODING_ENTERPRISE_HFONT,
            Self::EnterpriseHun => enterprise_hun::ENCODING_ENTERPRISE_HUN,
            Self::EnterprisePlusHun => enterprise_plus_hun::ENCODING_ENTERPRISE_PLUS_HUN,
            Self::ISO646HU => iso646hu::ENCODING_ISO646HU,
            Self::ISO88591 => iso88591::ENCODING_ISO88591,
            Self::ISO88592 => iso88592::ENCODING_ISO88592,
            Self::MacCenteuro => mac_centeuro::ENCODING_MAC_CENTEURO,
            Self::MacRoman => mac_roman::ENCODING_MAC_ROMAN,
            Self::SZKI => szki::ENCODING_SZKI,
            Self::TVC => tvc::ENCODING_TVC,
        }
    }

    pub fn max_encoding_name_width() -> usize {
        Self::VARIANTS
            .iter()
            .map(|encoding_name| encoding_name.chars().count())
            .max()
            .unwrap_or(0)
    }

    pub fn supports_charset(&self, charset: &[char]) -> bool {
        charset.iter().all(|c| self.get_encoding_table().contains(c))
    }

    pub fn is_common(&self) -> bool {
        true
    }

    pub fn encode(&self, input: char) -> u8 {
        self.get_encoding_table().iter().position(|&p| p == input).unwrap_or(0) as u8
    }

    pub fn decode(&self, input: u8) -> char {
        self.get_encoding_table()[input as usize]
    }
}
