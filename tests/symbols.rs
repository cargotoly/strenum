//! This module contains the `Symbol` enum, which is used to
//! represent all the symbols in an example programming language.
//! It  was created with the #[StringEnum] macro.

use strenum::StringEnum;

// TODO
#[derive(Debug, PartialEq, Eq, Clone)]
#[StringEnum]
pub enum Symbol {
    // (1) Assign
    Assign = "=",

    // (1) Sequence
    Semicolon = ";",
    Comma = ",",
    Dot = ".",
    Colon = ":",
    Question = "?",

    // (1) Arithmetic / Logical
    Plus = "+",
    Minus = "-",
    Star = "*",
    Slash = "/",
    Caret = "^",
    Pipe = "|",
    Ampersand = "&",
    Percent = "%",
    Exclamation = "!",

    // (1) Scope Management
    BracketRight = "[",
    BracketLeft = "]",
    CurlyRight = "{",
    CurlyLeft = "}",
    RoundRight = "(",
    RoundLeft = ")",
    AngleRight = ">",
    AngleLeft = "<",

    // (1) Reserved
    Tilde = "~",
    At = "@",

    // (2)
    ControlAssign = "?=",
    Declare = ":=",

    // (2) Arithmetic / Logical Assign
    DotAssign = ".=",
    PlusAssign = "+=",
    MinusAssign = "-=",
    StarAssign = "*=",
    SlashAssign = "/=",
    CaretAssign = "^=",
    PipeAssign = "|=",
    AmpersandAssign = "&=",
    PercentAssign = "%=",

    // (2) Arithmetic
    ShiftLeft = "<<",
    ShiftRight = ">>",

    // (2) Conditional
    Equals = "==",
    NotEquals = "!=",
    Pipe2 = "||",
    Ampersand2 = "&&",

    // (2) Lambda
    DoubleArrow = "=>",
    Arrow = "->",
    LeadsTo = "~>",

    // (2) Reserved
    Dot2 = "..",
    Colon2 = "::",
    AtAssign = "@=",
    TildeAssign = "~=",

    // (3) Arithmetic Assign
    ShiftLeftAssign = "<<=",
    ShiftRightAssign = ">>=",

    // (3) Reserved
    Dot3 = "...",
    Question3 = "???",
}
