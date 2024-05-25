const KEYWORDS: &[&str] = &["type", "streaming", "sync"];
const PUNCTUATIONS: &[char] = &['{', '}', '.', ',', ':', ';'];

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum TokenType {
    Keyword,
    Identifier,
    Punctuation,
    Defer,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Token {
    pub ty: TokenType,
    pub value: String,
}

impl Token {
    fn new() -> Self {
        Self {
            ty: TokenType::Defer,
            value: String::new(),
        }
    }
    fn push(&mut self, c: char) {
        self.value.push(c);
    }
    fn is_empty(&self) -> bool {
        self.value.is_empty()
    }
    fn finish(&mut self) {
        if self.value.is_empty() {
            panic!("Token is empty");
        }
        if self.ty != TokenType::Defer {
            panic!("Token has been finished already");
        }
        if KEYWORDS.contains(&self.value.as_str()) {
            self.ty = TokenType::Keyword;
        } else if PUNCTUATIONS.contains(&self.value.chars().next().unwrap()) {
            if self.value.len() != 1 {
                panic!("Punctuation token must be a single character");
            }
            self.ty = TokenType::Punctuation;
        } else {
            self.ty = TokenType::Identifier;
        }
    }
}

pub(crate) fn tokenize(schema: &str) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut current_token = Token::new();
    for c in schema.chars() {
        match c {
            ' ' | '\t' | '\n' => {
                if !current_token.is_empty() {
                    current_token.finish();
                    tokens.push(current_token);
                    current_token = Token::new();
                }
            }
            '{' | '}' | '.' | ',' | ':' | ';' => {
                if !current_token.is_empty() {
                    current_token.finish();
                    tokens.push(current_token);
                    current_token = Token::new();
                }
                current_token.push(c);
                current_token.finish();
                tokens.push(current_token);
                current_token = Token::new();
            }
            _ => {
                current_token.push(c);
            }
        }
    }
    if !current_token.is_empty() {
        current_token.finish();
        tokens.push(current_token);
    }
    tokens
}