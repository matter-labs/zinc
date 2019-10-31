#[repr(u8)]
pub enum OpCode {
    NoOp,

    Push,
    Pop,
    Copy,
    Swap,

    Add,
    Sub,
    Mul,
    Div,
    Rem,

    Select,
}
