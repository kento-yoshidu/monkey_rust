#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TokenType {
    // トークンや文字が未知であること
    Illegal,
    // ファイル終端
    Eof,
    // 識別子 + リテラル
    Ident,
    Int,
    // 演算子
    Assign,
    Plus,
    // デリミター
    Comma,
    Semicolon,
    Lparen,
    Rparen,
    Lbrace,
    Rbrace,
    // キーワード
    Function,
    Let,
}
