#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    // Special
    EoF,
    // Identifiers
    Ident(&'a str),
    Nat(u32),
    Char(char),
    // Brackets
    LfAngle,
    RtAngle,
    LfBracket,
    RtBracket,
    // Delimiters
    Colon,
    Semicolon,
    Comma,
    FwdSlash,
    // Options
    Hashtag,
}
