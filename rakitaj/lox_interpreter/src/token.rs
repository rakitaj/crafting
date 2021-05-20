pub enum TokenType {
    // Single-character tokens.                      
    LeftParen, RightParen, LeftBrace, RightBrace,
    Comma, Dot, Minus, Plus, SemiColon, Slash, Star,

    // One or two character tokens.                  
    Bang, BangEqual,
    Equal, EqualEqual,
    Greater, GreaterEqual,
    Less, LessEqual,

    // Literals.                                     
    Identifier(String), String(String), Number(f32),

    // Keywords.                                     
    And, Class, Else, False, Fun, For, If, Nil, Or,
    Print, Return, Super, This, True, Var, WHile,

    Eof
}

pub struct SourceCode {
    pub source: String,
    pub index: usize
}

impl SourceCode {
    pub fn new(source: String) -> Self {
        SourceCode {
            source: source,
            index: 0
        }
    }

    pub fn get(&self) -> Option<char> {
        self.source.chars().nth(self.index)
    }

    pub fn peek(self, n: usize) -> Option<char> {
        self.source.chars().nth(&self.index + n)
    }

    pub fn consume(&mut self) -> Option<char> {
        let c = &self.get();
        self.index += 1;
        return *c;
    }

    pub fn eof(&self) -> bool {
        match &self.get() {
            Some(_) => false,
            None => true
        }
    }
}

pub fn lex(source: SourceCode) -> Vec<TokenType> {
    let mut tokens: Vec<TokenType> = Vec::new();
    while !source.eof() {

    }
    return tokens;
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_eof_false() {
        let source = SourceCode::new("main() { return 0; }".to_string());
        assert_eq!(source.eof(), false);
    }

    #[test]
    fn test_eof_true() {
        let mut source = SourceCode::new("main() { return 0; }".to_string());
        source.index = 40;
        assert_eq!(source.eof(), true);
    }

    #[test]
    fn test_consume_should_increment() {
        let mut source = SourceCode::new("foobar".to_string());
        assert_eq!(source.index, 0);
        assert_eq!(source.consume().unwrap(), 'f');
        assert_eq!(source.index, 1);
    }
}