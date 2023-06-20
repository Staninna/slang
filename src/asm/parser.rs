use nom::IResult;
use strum::{IntoEnumIterator, VariantNames};
use vm::opcodes::Opcode;

pub fn opcode(input: &[u8]) -> IResult<&[u8], Opcode> {
    // Iterate through each opcode and check if the input starts with it
    for (opcode, name) in Opcode::iter().zip(Opcode::VARIANTS.iter()) {
        if input.starts_with(name.as_bytes()) {
            return Ok((&input[name.len()..], opcode));
        }
    }

    // If no opcode was found, return an error
    Err(nom::Err::Error(nom::error::Error::new(
        input,
        nom::error::ErrorKind::Tag,
    )))
}
