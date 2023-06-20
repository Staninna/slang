use nom::{branch::alt, bytes::complete::tag_no_case, combinator::value, IResult};

use vm::opcodes::Opcode;

pub fn opcode(input: &[u8]) -> IResult<&[u8], Opcode> {
    use Opcode::*;

    let misc = alt((
        value(Nop, tag_no_case("nop")),
        value(Hlt, tag_no_case("hlt")),
    ));

    let load_store = alt((
        value(Mov, tag_no_case("mov")),
        value(Lod, tag_no_case("lod")),
        value(Str, tag_no_case("str")),
    ));

    let arithmetic = alt((
        value(Add, tag_no_case("add")),
        value(Sub, tag_no_case("sub")),
        value(Mul, tag_no_case("mul")),
        value(Div, tag_no_case("div")),
        value(Inc, tag_no_case("inc")),
        value(Dec, tag_no_case("dec")),
    ));

    let bitwise = alt((
        value(And, tag_no_case("and")),
        value(Or, tag_no_case("or")),
        value(Xor, tag_no_case("xor")),
        value(Not, tag_no_case("not")),
        value(Shl, tag_no_case("shl")),
        value(Shr, tag_no_case("shr")),
    ));

    let branching = alt((
        value(Jmp, tag_no_case("jmp")),
        value(Jeq, tag_no_case("jeq")),
        value(Jne, tag_no_case("jne")),
        value(Jgt, tag_no_case("jgt")),
        value(Jlt, tag_no_case("jlt")),
        value(Jge, tag_no_case("jge")),
        value(Jle, tag_no_case("jle")),
        value(Jnz, tag_no_case("jnz")),
        value(Jz, tag_no_case("jz")),
    ));

    let stack = alt((
        value(Psh, tag_no_case("psh")),
        value(Pop, tag_no_case("pop")),
        value(Dup, tag_no_case("dup")),
        value(Swp, tag_no_case("swp")),
        value(Clr, tag_no_case("clr")),
        value(Ret, tag_no_case("ret")),
        value(Cal, tag_no_case("cal")),
    ));

    alt((misc, load_store, arithmetic, bitwise, branching, stack))(input)
}
