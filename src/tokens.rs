pub enum Tokens {
    // Brackets
    CurlyBraceOpen, CurlyBraceClose,
    SquareBraceOpen, SquareBraceClose,
    RoundBraceOpen, RoundBraceClose,

    // Operators
    Plus, Minus, SlashForward, Asterisk,
    Equal, EqualEqual,
    Bang, BangEqual,
    GreaterThan, GreaterEqual,
    LessThan, LessEqual,
    AmpersandDouble, PipeDouble,

    // Keywords
    Struct, Impl,
    For, While, Break, Continue,
    If, Else,
    Define, DashArrow,
    True, False, None,
    Return,

    // Quotes
    QuoteSingle, QuoteDouble,
    QuoteSingleLatex, QuoteDoubleLatex,   // L"\begin{bmatrix}1\\2\\3\end{bmatrix}"
    QuoteSingleFormat, QuoteSingleFormat, // f""
    QuoteSingleRaw, QuoteDoubleRaw,       // r""

    // Characters
    Backslash, Semicolon, Comma,
    DoubleColon, Dot,                     // paamayim nekudotayim
//  QuestionMark, Colon,

    // Literals
    LiteralNumber(value), LiteralString(value), LiteralStringLatex(value),

    // Identifier
    Identifier(name),

    EndOfFile
}