pub(super) trait Digit {}
pub(super) trait HexDigit {}

pub(super) trait NumericLiteral: Digit + HexDigit {}

pub(super) trait LiteralValue: NumericLiteral {}
