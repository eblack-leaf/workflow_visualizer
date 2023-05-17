use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum Sendable {
    ExitConfirmed,
    TokenAdded(TokenName),
    TokenRemoved(TokenName),
    TokenOtp((TokenName, TokenOtp)),
}

#[derive(Debug, Clone)]
pub enum Receivable {
    ExitRequest,
    AddToken((TokenName, Token)),
    GenerateOtp(TokenName),
    RemoveToken(TokenName),
}
#[derive(Debug, Clone)]
pub struct TokenOtp(pub String);
#[derive(Debug, Clone)]
pub struct TokenName(pub String);
#[derive(Debug, Clone)]
pub struct Token(pub String);
pub struct Engen {
    pub tokens: HashMap<TokenName, Token>,
}
impl Engen {
    pub fn new() -> Self {
        Self {
            tokens: HashMap::new(),
        }
    }
}
