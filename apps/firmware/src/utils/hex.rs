use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::Tuple,
    IResult, Parser,
};


use crate::color::Color;

fn from_hex(input: &str) -> Result<u8, core::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_ascii_hexdigit()
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex).parse(input)
}

pub fn hex_to_rgbw(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, result) = (hex_primary, hex_primary, hex_primary, hex_primary).parse(input)?;
    Ok((input, result.into()))
}
