
/// JSON token structure
#[derive(Debug, Clone)]
pub struct Token {
    pub kind: TokenKind,
    pub pos: usize
}

/// Type of JSON token
#[derive(Debug)]
pub enum TokenKind {
    ObjOpen,
    ObjClose,
    ArrayOpen,
    ArrayClose,
    Assign,
    Separator,
    Null,
    Text(String),
    Int(i64),
    Float(f64),
    Bool(bool),
}

impl fmt::Display for TokenKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let text = match self {
            TokenKind::ObjOpen => "{".to_string(),
            TokenKind::ObjClose => "}".to_string(),
            TokenKind::ArrayOpen => "[".to_string(),
            TokenKind::ArrayClose => "]".to_string(),
            TokenKind::Assign => ":".to_string(),
            TokenKind::Separator => ",".to_string(),
            TokenKind::Null => "null".to_string(),
            TokenKind::Text(t) => format!("\"{}\"", t),
            TokenKind::Int(i) => format!("{}", i),
            TokenKind::Float(f) => format!("{}", f),
            TokenKind::Bool(b) => format!("{}", b)
        };
        write!(f, "{}", text)
    }
}

impl Clone for TokenKind {
    fn clone(&self) -> TokenKind {
        match self {
            TokenKind::ObjOpen => TokenKind::ObjOpen,
            TokenKind::ObjClose => TokenKind::ObjClose,
            TokenKind::ArrayOpen => TokenKind::ArrayOpen,
            TokenKind::ArrayClose => TokenKind::ArrayClose,
            TokenKind::Assign => TokenKind::Assign,
            TokenKind::Separator => TokenKind::Separator,
            TokenKind::Null => TokenKind::Null,
            TokenKind::Text(t) => TokenKind::Text(t.clone()),
            TokenKind::Int(i) => TokenKind::Int(*i),
            TokenKind::Float(f) => TokenKind::Float(*f),
            TokenKind::Bool(b) => TokenKind::Bool(*b)
        }
    }
}
#[derive(Debug)]
pub enum JsonType {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    String(String),
    Object(HashMap<String, JsonType>),
    Array(Vec<JsonType>),
}
use std::char;
use std::str::Chars;

/// Internal state of the JSON lexer
enum State {
    Neutral,
    Text,
    Keyword,
    Number
}

/// JSON lexer, converts a JSON string to tokens
pub struct Lexer {
    state: State,
    buffer: String,
    tokens: Vec<Token>,
}

impl Lexer {

    pub fn new() -> Lexer {
        Lexer {
            state: State::Neutral,
            buffer: String::new(),
            tokens: Vec::new(),
        }
    }

    /// Convert a string to a series of tokens
    pub fn lex(&mut self, text: &str) -> Result<Vec<Token>, Error> {
        for (i, c) in text.chars().enumerate() {
            match self.state {
                State::Neutral => self.lex_neutral(i, c),
                State::Text => self.lex_text(i, c)?,
                State::Keyword => self.lex_keyword(i, c)?,
                State::Number => self.lex_number(i, c)?
            };
        }

        let pos = text.len();
        match self.state {
            State::Neutral => Ok(self.tokens.clone()),
            State::Text => {
                self.add_text(pos)?;
                Ok(self.tokens.clone())
            },
            State::Keyword => {
                self.add_keyword(pos)?;
                Ok(self.tokens.clone())
            },
            State::Number => {
                self.add_number(pos)?;
                Ok(self.tokens.clone())
            }
        }
    }

    /// Handle text when the lexer isn't in a specific state
    fn lex_neutral(&mut self, index: usize, character: char) {
        match character {
            '{' => self.tokens.push(Token{kind: TokenKind::ObjOpen, pos: index}),
            '}' => self.tokens.push(Token{kind: TokenKind::ObjClose, pos: index}),
            '[' => self.tokens.push(Token{kind: TokenKind::ArrayOpen, pos: index}),
            ']' => self.tokens.push(Token{kind: TokenKind::ArrayClose, pos: index}),
            ',' => self.tokens.push(Token{kind: TokenKind::Separator, pos: index}),
            ':' => self.tokens.push(Token{kind: TokenKind::Assign, pos: index}),
            '0'..= '9' | '-' => {
                self.state = State::Number;
                self.buffer.clear();
                self.buffer.push(character);
            }
            't' | 'f' | 'n' => {
                self.state = State::Keyword;
                self.buffer.clear();
                self.buffer.push(character);
            }
            '"' => {
                self.state = State::Text;
                self.buffer.clear();
                self.buffer.push(character);
            },
            _ => {}
        }
    }

    /// Handle text for a keyword such as 'true', 'false', 'null'
    fn lex_keyword(&mut self, index: usize, character: char) -> Result<(), Error> {
        match character {
            'r' | 'u' | 'a' | 'l' | 's' | 'e' => {
                self.buffer.push(character);
                Ok(())
            },
            _ => {
                self.add_keyword(index)?;
                self.state = State::Neutral;
                self.lex_neutral(index, character);
                Ok(())
            }
        }
    }

    /// Read the keyword in the buffer and convert it to a token
    fn add_keyword(&mut self, index: usize) -> Result<(), Error> {
        match &self.buffer[..] {
            "true" => {
                self.tokens.push(
                    Token{kind: TokenKind::Bool(true), pos: index - 4}
                );
                Ok(())
            },
            "false" => {
                self.tokens.push(
                    Token{kind: TokenKind::Bool(false), pos: index - 5}
                );
                Ok(())
            },
            "null" => {
                self.tokens.push(
                    Token{kind: TokenKind::Null, pos: index - 4}
                );
                Ok(())
            }
            _ => {
                Err(Error::invalid(index, "keyword", &self.buffer))
            }
        }
    }

    /// Handle text for a numeric value
    fn lex_number(&mut self, index: usize, character: char) -> Result<(), Error> {
        match character {
            '0'..= '9' => {
                self.buffer.push(character);
                Ok(())
            },
            '-' | '.' | 'e' => {
                self.buffer.push(character);
                Ok(())
            },
            _ => {
                self.add_number(index)?;
                self.state = State::Neutral;
                self.lex_neutral(index, character);
                Ok(())
            }
        }
    }

    /// Read the number in the buffer and convert to a token
    fn add_number(&mut self, index: usize) -> Result<(), Error> {
        let number: f64 = self.buffer.parse()
            .map_err(|_| Error::invalid(index, "digit", &self.buffer))
            ?;

        if self.buffer.contains('.') {
            self.tokens.push(Token{
                kind: TokenKind::Float(number), 
                pos: index - self.buffer.len()
            });
        } else {
            self.tokens.push(Token{
                kind: TokenKind::Int(number as i64), 
                pos: index - self.buffer.len()
            });
        }

        Ok(())
    }

    /// Handle text inside a string
    fn lex_text(&mut self, index: usize, character: char) -> Result<(), Error> {
        self.buffer.push(character);
        if character == '"' && !self.buffer.ends_with("\\\"") {
            self.add_text(index)?;
            self.state = State::Neutral;
        }
        Ok(())
    }

    /// Convert text in the buffer to a string
    fn add_text(&mut self, index: usize) -> Result<(), Error> {
        if !self.buffer.ends_with("\"") {
            return Err(Error::invalid(index, "string", &self.buffer));
        }

        self.tokens.push(Token{
            kind: TokenKind::Text(self.convert_string(index)?),
            pos: index - self.buffer.len()
        });
        Ok(())
    }

    /// Convert special characters such as \r \n \t etc
    fn convert_string(&self, index: usize) -> Result<String, Error> {
        let mut iter = self.buffer[1..self.buffer.len()-1].chars();
        let mut converted = String::new();
        loop {
            match &iter.next() {
                None => return Ok(converted),
                Some('\\') => {
                    match &iter.next() {
                        Some('n') => converted.push('\n'),
                        Some('t') => converted.push('\t'),
                        Some('r') => converted.push('\r'),
                        Some('u') => converted.push(Lexer::convert_unicode(index, &mut iter)?),
                        Some(c) => converted.push(*c),
                        None => return Ok(converted)
                    }
                },
                Some(c) => converted.push(*c)
            }
        }
    }

    /// Convert unicode code points such as \u0123
    fn convert_unicode(index: usize, iter: &mut Chars) -> Result<char, Error> {
        let digit: String = iter.take(3).collect();

        let code: u32 = digit.parse()
            .map_err(|_| Error::invalid(index, "string", &digit))?;

        char::from_u32(code)
            .ok_or(Error::invalid(index, "string", &digit))
    }
}


/// Error handler for all errors in the json module
#[derive(Debug)]
pub struct Error {
    pub pos: usize,
    pub message: String
}

impl Error {

    /// A JSON token that has a syntax error
    pub fn invalid(pos: usize, kind: &str, value: &str) -> Error {
        Error {
            pos: pos,
            message: format!("Invalid type {}: '{}'", kind, value)
        }
    }

    /// A JSON token that wasn't used in the right place
    pub fn unexpected(kind: &str, token: &Token) -> Error {
        Error {
            pos: token.pos,
            message: format!("Expecting type {}, got '{}'", kind, token.kind)
        }
    }

    /// A JSON token that's missing from the object
    pub fn missing(kind: &str) -> Error {
        Error {
            pos: 0,
            message: format!("Expecting {}, but no more tokens", kind)
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Parse error at {}: {}", self.pos, self.message)
    }
}

impl From<Error> for String {
   fn from(e: Error) -> Self {
       format!("{}", e)
   }
}
use std::slice::Iter;

/// Type for representing our converted JSON structure

/// Convert a string to its JSON representation
pub fn parse(text: &str) -> Result<JsonType, Error> {
    let mut lexer = Lexer::new();
    let tokens = lexer.lex(text)?;
    let mut iterator = tokens.iter();
    parse_tokens(&mut iterator)
}

/// Read JSON tokens and convert them to a JSON data type. Can be called recursively
fn parse_tokens(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let item = tokens.next().ok_or(Error::missing("token"))?;
    match &item.kind {
        TokenKind::Null => Ok(JsonType::Null),
        TokenKind::Bool(b) => Ok(JsonType::Bool(*b)),
        TokenKind::Int(i) => Ok(JsonType::Int(*i)),
        TokenKind::Float(f) => Ok(JsonType::Float(*f)),
        TokenKind::Text(t) => Ok(JsonType::String(t.clone())),
        TokenKind::ArrayOpen => parse_array(tokens),
        TokenKind::ObjOpen => parse_object(tokens),
        _ => Err(Error::unexpected("value", item))
    }
}

/// Read and convert tokens forming an array
fn parse_array(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let mut items: Vec<JsonType> = Vec::new();
    loop {
        items.push(parse_tokens(tokens)?);

        let item = tokens.next().ok_or(Error::missing("array"))?;
        match item.kind {
            TokenKind::Separator => {},
            TokenKind::ArrayClose => return Ok(JsonType::Array(items)),
            _ => return Err(Error::unexpected("separator", item))
        }
    }
}

/// Read and convert tokens forming an object
fn parse_object(tokens: &mut Iter<Token>) -> Result<JsonType, Error> {
    let mut items: HashMap<String, JsonType> = HashMap::new();

    loop {
        //the key as in {"key": "value"}
        let key = match tokens.next() {
            Some(token) => {
                match &token.kind {
                    TokenKind::Text(t) => t.to_string(),
                    _ => return Err(Error::unexpected("string", token))
                }
            }
            None => return Err(Error::missing("assignment"))
        };

        //make sure there is a ":" after the key
        match tokens.next() {
            Some(token) => {
                match token.kind {
                    TokenKind::Assign => {},
                    _ => return Err(Error::unexpected("assignment", token)),
                }
            },
            _ => return Err(Error::missing("assignment"))
        };

        //convert the value
        let value = parse_tokens(tokens)?;

        // handle a "," or "}"
        match tokens.next() {
            Some(token) => {
                match token.kind {
                    TokenKind::Separator => items.insert(key, value),
                    TokenKind::ObjClose => {
                        items.insert(key, value);
                        return Ok(JsonType::Object(items))
                    },
                    _ => return Err(Error::unexpected("object close or separator", token))
                }
            }
            None => return Err(Error::missing("object close or separator"))
        };
    }
}