mod cork;
mod cp254;
mod cp320;
mod cp437;
mod cp850;
mod cp852;
mod cwi_1;
mod cwi_2;
mod ep_hfont;
mod ep_hun;
mod ep_plus_hun;
mod iso_646_hu;
mod iso_8859_1;
mod iso_8859_2;
mod mac_centeuro;
mod mac_roman;
mod szki;
mod tvc;
mod windows_1250;
mod windows_1252;

use strum::{Display, EnumIter, EnumString, EnumVariantNames, VariantNames};

pub type EncodingTable = [char; 256];

pub struct EncodingDefinition {
    pub encoding_table: EncodingTable,
    pub is_common: bool,
}

#[allow(non_camel_case_types)]
#[allow(clippy::upper_case_acronyms)]
#[derive(Copy, Clone, Display, EnumIter, EnumString, EnumVariantNames, Eq, PartialEq)]
#[strum(serialize_all = "SCREAMING-KEBAB-CASE", ascii_case_insensitive)]
pub enum Encoding {
    CORK,
    CP254,
    CP320,
    CP437,
    CP850,
    CP852,
    CWI_1,
    CWI_2,
    EP_HFONT,
    EP_HUN,
    EP_PLUS_HUN,
    ISO_646_HU,
    ISO_8859_1,
    ISO_8859_2,
    MAC_CENTEURO,
    MAC_ROMAN,
    SZKI,
    TVC,
    WINDOWS_1250,
    WINDOWS_1252,
}

impl Encoding {
    pub fn encoding_definition(&self) -> &EncodingDefinition {
        match self {
            Self::CORK => &cork::ENCODING_CORK,
            Self::CP254 => &cp254::ENCODING_CP254,
            Self::CP320 => &cp320::ENCODING_CP320,
            Self::CP437 => &cp437::ENCODING_CP437,
            Self::CP850 => &cp850::ENCODING_CP850,
            Self::CP852 => &cp852::ENCODING_CP852,
            Self::CWI_1 => &cwi_1::ENCODING_CWI_1,
            Self::CWI_2 => &cwi_2::ENCODING_CWI_2,
            Self::EP_HFONT => &ep_hfont::ENCODING_EP_HFONT,
            Self::EP_HUN => &ep_hun::ENCODING_EP_HUN,
            Self::EP_PLUS_HUN => &ep_plus_hun::ENCODING_EP_PLUS_HUN,
            Self::ISO_646_HU => &iso_646_hu::ENCODING_ISO_646_HU,
            Self::ISO_8859_1 => &iso_8859_1::ENCODING_ISO_8859_1,
            Self::ISO_8859_2 => &iso_8859_2::ENCODING_ISO_8859_2,
            Self::MAC_CENTEURO => &mac_centeuro::ENCODING_MAC_CENTEURO,
            Self::MAC_ROMAN => &mac_roman::ENCODING_MAC_ROMAN,
            Self::SZKI => &szki::ENCODING_SZKI,
            Self::TVC => &tvc::ENCODING_TVC,
            Self::WINDOWS_1250 => &windows_1250::ENCODING_WINDOWS_1250,
            Self::WINDOWS_1252 => &windows_1252::ENCODING_WINDOWS_1252,
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
        let encoding_table = self.encoding_definition().encoding_table;
        charset.iter().all(|c| encoding_table.contains(c))
    }

    pub fn is_common(&self) -> bool {
        self.encoding_definition().is_common
    }

    pub fn encode(&self, input: char) -> u8 {
        self.encoding_definition()
            .encoding_table
            .iter()
            .position(|&p| p == input)
            .unwrap_or(0) as u8
    }

    pub fn decode(&self, input: u8) -> char {
        self.encoding_definition().encoding_table[input as usize]
    }
}

pub trait Encoder<'a> {
    fn encode(self, encoding: Encoding) -> Box<dyn Iterator<Item = u8> + 'a>;
}

impl<'a, T, F> Encoder<'a> for T
where
    T: Iterator<Item = F> + 'a,
    F: std::borrow::Borrow<char>,
{
    fn encode(self, encoding: Encoding) -> Box<dyn Iterator<Item = u8> + 'a> {
        Box::new(self.map(move |c| encoding.encode(*c.borrow())))
    }
}

pub trait Decoder<'a> {
    fn decode(self, encoding: Encoding) -> Box<dyn Iterator<Item = char> + 'a>;
}

impl<'a, T, F> Decoder<'a> for T
where
    T: Iterator<Item = F> + 'a,
    F: std::borrow::Borrow<u8>,
{
    fn decode(self, encoding: Encoding) -> Box<dyn Iterator<Item = char> + 'a> {
        Box::new(self.map(move |b| encoding.decode(*b.borrow())))
    }
}
