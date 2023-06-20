use crate::parser::opcode;
use strum::VariantNames;
use vm::opcodes::Opcode;

#[test]
fn test_opcode() {
    // Get the list of opcodes from src\vm\opcodes.rs
    let mut inputs = vec![];
    for opcode in Opcode::VARIANTS.iter() {
        inputs.push(opcode);
    }

    // Test each opcode
    for test_case in inputs {
        let result = opcode(test_case.as_bytes());
        assert!(result.is_ok(), "Failed to parse '{}'", test_case);
    }
}
