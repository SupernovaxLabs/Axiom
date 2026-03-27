# Axiom Technical Implementation Guide

## Complete Compiler, Kernel, Interpreter & Binary Generation

---

# Part 1: Language Syntax Design

## 1.1 Complete Grammar Specification (EBNF)

```ebnf
(* ============ CORE LANGUAGE GRAMMAR ============ *)

(* Program Structure *)
program          = module_decl, { import_decl }, { declaration } ;

module_decl      = "module", module_path ;
module_path      = identifier, { ".", identifier } ;

import_decl      = "import", import_path, [ "as", identifier ] ;
import_path      = identifier, { ".", identifier } 
                 | identifier, "::", "{", import_list, "}" ;
import_list      = identifier, { ",", identifier } ;

(* Declarations *)
declaration      = function_decl
                 | struct_decl
                 | enum_decl
                 | trait_decl
                 | impl_block
                 | const_decl
                 | static_decl
                 | type_decl ;

(* Function Declaration *)
function_decl    = [ "pub" ], [ "async" ], [ "unsafe" ], "fn",
                   identifier, [ type_params ], params,
                   [ ">", type ], [ where_clause ], block ;

params           = "(", [ param_list ], ")" ;
param_list       = param, { ",", param } ;
param            = identifier, ":", type, [ "=", expr ] ;

type_params      = "<", type_param, { ",", type_param }, ">" ;
type_param       = identifier, [ ":", type_bound ] ;
type_bound       = type, { "+", type } ;

(* Struct Declaration *)
struct_decl      = [ "pub" ], "struct", identifier, [ type_params ],
                   "{", { field }, "}" ;
field            = [ "pub" ], identifier, ":", type ;

(* Enum Declaration *)
enum_decl        = [ "pub" ], "enum", identifier, [ type_params ],
                   "{", variant_list, "}" ;
variant_list     = variant, { ",", variant } ;
variant          = identifier, [ variant_data ] ;
variant_data     = "(" type_list ")" | "{" field_list "}" ;

(* Types *)
type             = primitive_type
                 | identifier
                 | type, "[" expr "]"
                 | type, "."
                 | "(" type_list ")"
                 | "[" type, ";", expr "]"
                 | "&", [ "mut" ], type
                 | "*", [ "mut" ], type
                 | "?", type
                 | "fn", params, [ "->", type ]
                 | "impl", type_bound
                 | "dyn", type_bound ;

primitive_type   = "i8" | "i16" | "i32" | "i64" | "i128" | "isize"
                 | "u8" | "u16" | "u32" | "u64" | "u128" | "usize"
                 | "f32" | "f64"
                 | "bool" | "char" | "byte" | "string" | "void" ;

(* Expressions *)
expr             = assignment_expr ;
assignment_expr  = or_expr, [ assignment_op, expr ] ;
assignment_op    = "=" | "+=" | "-=" | "*=" | "/=" | "%=" 
                 | "&=" | "|=" | "^=" | "<<=" | ">>=" ;

or_expr          = and_expr, { "||", and_expr } ;
and_expr         = compare_expr, { "&&", compare_expr } ;
compare_expr     = bitwise_or, [ compare_op, bitwise_or ] ;
compare_op       = "==" | "!=" | "<" | ">" | "<=" | ">=" | "<=>" ;

bitwise_or       = bitwise_xor, { "|", bitwise_xor } ;
bitwise_xor      = bitwise_and, { "^", bitwise_and } ;
bitwise_and      = shift_expr, { "&", shift_expr } ;
shift_expr       = add_expr, { shift_op, add_expr } ;
shift_op         = "<<" | ">>" | ">>>" ;

add_expr         = mul_expr, { add_op, mul_expr } ;
add_op           = "+" | "-" ;
mul_expr         = unary_expr, { mul_op, unary_expr } ;
mul_op           = "*" | "/" | "%" | "//" ;

unary_expr       = unary_op, unary_expr | postfix_expr ;
unary_op         = "-" | "!" | "~" | "&" | "&mut" | "*" | "move" ;

postfix_expr     = primary_expr, { postfix_op } ;
postfix_op       = "." identifier
                 | "(" [ arg_list ] ")"
                 | "[" expr "]"
                 | "?"
                 | "!"
                 | ".." [ expr ]
                 | "..=" expr ;

primary_expr     = literal
                 | identifier
                 | "(" expr ")"
                 | "[" expr_list "]"
                 | block_expr
                 | if_expr
                 | match_expr
                 | loop_expr
                 | while_expr
                 | for_expr
                 | return_expr
                 | break_expr
                 | continue_expr
                 | lambda_expr
                 | struct_init ;

(* Literals *)
literal          = int_literal
                 | float_literal
                 | string_literal
                 | char_literal
                 | bool_literal
                 | "null" ;

int_literal      = digit, { digit | "_" }, [ int_suffix ] 
                 | "0x", hex_digit, { hex_digit | "_" }
                 | "0o", oct_digit, { oct_digit | "_" }
                 | "0b", bin_digit, { bin_digit | "_" } ;

float_literal    = digit, { digit | "_" }, ".", digit, { digit | "_" },
                   [ float_suffix ]
                 | digit, { digit | "_" }, ( "e" | "E" ),
                   [ "+" | "-" ], digit, { digit | "_" };

string_literal   = '"', { string_char | escape_seq }, '"'
                 | '`', { raw_string_char }, '`' ;

char_literal     = "'", ( char | escape_seq ), "'" ;

(* Statements *)
statement        = let_stmt
                 | var_stmt
                 | expr_stmt
                 | block
                 | empty_stmt ;

let_stmt         = "let", identifier, [ ":", type ], "=", expr, ";" ;
var_stmt         = "var", identifier, [ ":", type ], [ "=", expr ], ";" ;
expr_stmt        = expr, ";" ;
empty_stmt       = ";" ;
block            = "{", { statement }, [ expr ], "}" ;

(* Control Flow *)
if_expr          = "if", expr, block, 
                   { "else", "if", expr, block },
                   [ "else", block ] ;

match_expr       = "match", expr, "{", match_arm_list, "}" ;
match_arm_list   = match_arm, { ",", match_arm }, [ "," ] ;
match_arm        = pattern, [ "if", expr ], "=>", ( block | expr ) ;

pattern          = literal_pattern
                 | identifier_pattern
                 | wildcard_pattern
                 | range_pattern
                 | struct_pattern
                 | tuple_pattern
                 | or_pattern ;

literal_pattern  = literal ;
identifier_pattern = [ "ref" ], [ "mut" ], identifier, [ "@", pattern ] ;
wildcard_pattern = "_" ;
range_pattern    = pattern, "..=", pattern ;
struct_pattern   = identifier, "{", field_pattern_list, "}" ;
tuple_pattern    = "(", pattern_list, ")" ;
or_pattern       = pattern, { "|", pattern } ;

loop_expr        = [ label ":'" ], "loop", block ;
while_expr       = [ label ":'" ], "while", expr, block ;
for_expr         = [ label ":'" ], "for", identifier, "in", expr, block ;

label            = identifier ;

(* Lambda *)
lambda_expr      = "|", [ param_list ], "|", [ "->", type ], expr ;

(* Struct Init *)
struct_init      = identifier, "{", field_init_list, "}" ;
field_init_list  = field_init, { ",", field_init }, [ "," ] ;
field_init       = identifier, ":", expr 
                 | "..", expr ;
```

---

## 1.2 Operator Precedence Table

| Level | Operators | Associativity | Description |
|-------|-----------|---------------|-------------|
| 1 | `()` `[]` `.` `?` `!` | Left | Postfix |
| 2 | `-` `!` `~` `*` `&` `move` | Right | Prefix unary |
| 3 | `**` | Right | Power |
| 4 | `*` `/` `%` `//` | Left | Multiplicative |
| 5 | `+` `-` | Left | Additive |
| 6 | `<<` `>>` `>>>` | Left | Shift |
| 7 | `&` | Left | Bitwise AND |
| 8 | `^` | Left | Bitwise XOR |
| 9 | `\|` | Left | Bitwise OR |
| 10 | `==` `!=` `<` `>` `<=` `>=` `<=>` | Left | Comparison |
| 11 | `&&` | Left | Logical AND |
| 12 | `\|\|` | Left | Logical OR |
| 13 | `..` `..=` | Left | Range |
| 14 | `=` `+=` `-=` etc. | Right | Assignment |

---

# Part 2: Compiler Implementation

## 2.1 Compiler Architecture Overview

```
┌─────────────────────────────────────────────────────────────────────────────┐
│                           AXIOM COMPILER PIPELINE                           │
├─────────────────────────────────────────────────────────────────────────────┤
│                                                                             │
│  ┌─────────────┐    ┌─────────────┐    ┌─────────────────────────────────┐ │
│  │   SOURCE    │    │   LEXER     │    │         PARSER                  │ │
│  │   CODE      │───▶│             │───▶│                                 │ │
│  │  (.ax file) │    │  Tokens     │    │         AST                     │ │
│  └─────────────┘    └─────────────┘    └─────────────────────────────────┘ │
│                                                   │                         │
│                                                   ▼                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    SEMANTIC ANALYSIS                                 │   │
│  │  ┌──────────────┐  ┌──────────────┐  ┌──────────────────────────┐  │   │
│  │  │    NAME      │  │    TYPE      │  │      BORROW              │  │   │
│  │  │  RESOLVER    │─▶│   CHECKER    │─▶│      CHECKER             │  │   │
│  │  │              │  │              │  │                          │  │   │
│  │  └──────────────┘  └──────────────┘  └──────────────────────────┘  │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                   │                         │
│                                                   ▼                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    INTERMEDIATE REPRESENTATION                       │   │
│  │                                                                      │   │
│  │         AIR (Axiom Intermediate Representation) - SSA Form          │   │
│  │                                                                      │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                   │                         │
│                                                   ▼                         │
│  ┌─────────────────────────────────────────────────────────────────────┐   │
│  │                    OPTIMIZATION PASSES                               │   │
│  │  ┌────────┐ ┌────────┐ ┌────────┐ ┌────────┐ ┌────────────────┐   │   │
│  │  │Const   │ │Dead    │ │Inline  │ │Loop    │ │Escape Analysis │   │   │
│  │  │Fold    │ │Code    │ │        │ │Opt     │ │                │   │   │
│  │  └────────┘ └────────┘ └────────┘ └────────┘ └────────────────┘   │   │
│  └─────────────────────────────────────────────────────────────────────┘   │
│                                                   │                         │
│                         ┌─────────────────────────┼─────────────────────┐   │
│                         ▼                         ▼                     ▼   │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │
│  │    LLVM      │  │  CRANELIFT   │  │    WASM      │  │ INTERPRETER  │   │
│  │   BACKEND    │  │   BACKEND    │  │   BACKEND    │  │   (REPL)     │   │
│  │              │  │              │  │              │  │              │   │
│  │  Native x64  │  │  Fast Debug  │  │  Web Target  │  │  Direct AST  │   │
│  │  ARM64 etc.  │  │  Builds      │  │  Support     │  │  Execution   │   │
│  └──────────────┘  └──────────────┘  └──────────────┘  └──────────────┘   │
│                         │                         │                        │
│                         ▼                         ▼                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                         LINKER                                        │  │
│  │                  (ld, lld, link.exe)                                  │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                   │                                        │
│                                   ▼                                        │
│  ┌──────────────────────────────────────────────────────────────────────┐  │
│  │                    EXECUTABLE / LIBRARY                               │  │
│  │                   (.exe, .so, .dll, .wasm)                           │  │
│  └──────────────────────────────────────────────────────────────────────┘  │
│                                                                             │
└─────────────────────────────────────────────────────────────────────────────┘
```

---

## 2.2 Lexer Implementation

### 2.2.1 Token Definition

```axiom
// src/compiler/lexer/token.ax

/// Token type enumeration
pub enum TokenKind {
    // Keywords (28 total)
    FN, LET, VAR, CONST, IF, ELSE, MATCH, FOR, WHILE, LOOP,
    RETURN, BREAK, CONTINUE, STRUCT, ENUM, IMPL, TRAIT, PUB,
    IMPORT, EXPORT, MODULE, ASYNC, AWAIT, UNSAFE, MOVE,
    TRUE, FALSE, NULL, SELF, SELF_TYPE, SUPER, WHERE,
    
    // Literals
    INT_LITERAL(i64),
    FLOAT_LITERAL(f64),
    STRING_LITERAL(string),
    CHAR_LITERAL(char),
    
    // Identifiers
    IDENT(string),
    
    // Arithmetic Operators
    PLUS,           // +
    MINUS,          // -
    STAR,           // *
    SLASH,          // /
    PERCENT,        // %
    STAR_STAR,      // **
    SLASH_SLASH,    // //
    
    // Comparison Operators
    EQ_EQ,          // ==
    BANG_EQ,        // !=
    LT,             // <
    GT,             // >
    LT_EQ,          // <=
    GT_EQ,          // >=
    SPACESHIP,      // <=>
    
    // Logical Operators
    AMP_AMP,        // &&
    PIPE_PIPE,      // ||
    BANG,           // !
    
    // Bitwise Operators
    AMP,            // &
    PIPE,           // |
    CARET,          // ^
    TILDE,          // ~
    LT_LT,          // <<
    GT_GT,          // >>
    GT_GT_GT,       // >>>
    
    // Assignment Operators
    EQ,             // =
    PLUS_EQ,        // +=
    MINUS_EQ,       // -=
    STAR_EQ,        // *=
    SLASH_EQ,       // /=
    PERCENT_EQ,     // %=
    AMP_EQ,         // &=
    PIPE_EQ,        // |=
    CARET_EQ,       // ^=
    LT_LT_EQ,       // <<=
    GT_GT_EQ,       // >>=
    
    // Punctuation
    LPAREN,         // (
    RPAREN,         // )
    LBRACE,         // {
    RBRACE,         // }
    LBRACKET,       // [
    RBRACKET,       // ]
    COMMA,          // ,
    SEMICOLON,      // ;
    COLON,          // :
    COLON_COLON,    // ::
    DOT,            // .
    DOT_DOT,        // ..
    DOT_DOT_EQ,     // ..=
    ARROW,          // ->
    FAT_ARROW,      // =>
    HASH,           // #
    QUESTION,       // ?
    AT,             // @
    
    // Special
    EOF,
    ERROR(string)
}

/// Token with location information
pub struct Token {
    pub kind: TokenKind,
    pub span: Span,
    pub value: TokenValue,
}

/// Source code location
pub struct Span {
    pub start: Position,
    pub end: Position,
}

pub struct Position {
    pub offset: usize,    // Byte offset in file
    pub line: usize,      // Line number (1-based)
    pub column: usize,    // Column number (1-based)
}

/// Token value for literals
pub enum TokenValue {
    None,
    Int(i64, Radix),
    Float(f64),
    String(string),
    Char(char),
}

pub enum Radix {
    Decimal,
    Hexadecimal,
    Octal,
    Binary,
}

/// Keywords table
const KEYWORDS: HashMap<string, TokenKind> = {
    "fn": TokenKind::FN,
    "let": TokenKind::LET,
    "var": TokenKind::VAR,
    "const": TokenKind::CONST,
    "if": TokenKind::IF,
    "else": TokenKind::ELSE,
    "match": TokenKind::MATCH,
    "for": TokenKind::FOR,
    "while": TokenKind::WHILE,
    "loop": TokenKind::LOOP,
    "return": TokenKind::RETURN,
    "break": TokenKind::BREAK,
    "continue": TokenKind::CONTINUE,
    "struct": TokenKind::STRUCT,
    "enum": TokenKind::ENUM,
    "impl": TokenKind::IMPL,
    "trait": TokenKind::TRAIT,
    "pub": TokenKind::PUB,
    "import": TokenKind::IMPORT,
    "export": TokenKind::EXPORT,
    "module": TokenKind::MODULE,
    "async": TokenKind::ASYNC,
    "await": TokenKind::AWAIT,
    "unsafe": TokenKind::UNSAFE,
    "move": TokenKind::MOVE,
    "true": TokenKind::TRUE,
    "false": TokenKind::FALSE,
    "null": TokenKind::NULL,
    "self": TokenKind::SELF,
    "Self": TokenKind::SELF_TYPE,
    "super": TokenKind::SUPER,
    "where": TokenKind::WHERE,
};
```

### 2.2.2 Lexer Core Implementation

```axiom
// src/compiler/lexer/lexer.ax

/// The Lexer transforms source code text into tokens
pub struct Lexer {
    /// Source code being tokenized
    source: string,
    
    /// Current position in source
    position: Position,
    
    /// Current character being processed
    current: Option<char>,
    
    /// Lookahead character
    lookahead: Option<char>,
    
    /// Token start position
    token_start: Position,
    
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    
    /// File ID for diagnostics
    file_id: FileId,
}

impl Lexer {
    /// Create a new lexer for the given source code
    pub fn new(source: string, file_id: FileId) -> Self {
        let mut lexer = Self {
            source,
            position: Position { offset: 0, line: 1, column: 1 },
            current: None,
            lookahead: None,
            token_start: Position { offset: 0, line: 1, column: 1 },
            diagnostics: Vec::new(),
            file_id,
        };
        
        // Initialize first two characters
        lexer.current = lexer.read_char();
        lexer.lookahead = lexer.read_char();
        
        lexer
    }
    
    /// Get the next token from the source
    pub fn next_token(&mut self) -> Token {
        // Skip whitespace and comments
        self.skip_whitespace_and_comments();
        
        // Record token start position
        self.token_start = self.position;
        
        match self.current {
            // End of file
            None => self.make_token(TokenKind::EOF),
            
            // Identifiers and keywords
            Some(c) if c.is_id_start() => self.lex_identifier(),
            
            // Numbers
            Some(c) if c.is_digit() => self.lex_number(),
            
            // Strings
            Some('"') => self.lex_string(),
            Some('`') => self.lex_raw_string(),
            
            // Characters
            Some('\'') => self.lex_char(),
            
            // Operators and punctuation
            Some('+') => self.lex_plus(),
            Some('-') => self.lex_minus(),
            Some('*') => self.lex_star(),
            Some('/') => self.lex_slash(),
            Some('%') => self.lex_percent(),
            Some('=') => self.lex_equal(),
            Some('!') => self.lex_bang(),
            Some('<') => self.lex_less(),
            Some('>') => self.lex_greater(),
            Some('&') => self.lex_ampersand(),
            Some('|') => self.lex_pipe(),
            Some('^') => self.lex_caret(),
            Some('~') => self.lex_tilde(),
            Some('(') => self.lex_single(TokenKind::LPAREN),
            Some(')') => self.lex_single(TokenKind::RPAREN),
            Some('{') => self.lex_single(TokenKind::LBRACE),
            Some('}') => self.lex_single(TokenKind::RBRACE),
            Some('[') => self.lex_single(TokenKind::LBRACKET),
            Some(']') => self.lex_single(TokenKind::RBRACKET),
            Some(',') => self.lex_single(TokenKind::COMMA),
            Some(';') => self.lex_single(TokenKind::SEMICOLON),
            Some(':') => self.lex_colon(),
            Some('.') => self.lex_dot(),
            Some('#') => self.lex_single(TokenKind::HASH),
            Some('?') => self.lex_single(TokenKind::QUESTION),
            Some('@') => self.lex_single(TokenKind::AT),
            
            // Unknown character
            Some(c) => self.error_token(format!("unexpected character: '{}'", c)),
        }
    }
    
    /// Read the next character from source
    fn read_char(&mut self) -> Option<char> {
        if self.position.offset >= self.source.len() {
            return None;
        }
        
        let c = self.source.chars().nth(self.position.offset)?;
        self.position.offset += c.len_utf8();
        
        if c == '\n' {
            self.position.line += 1;
            self.position.column = 1;
        } else {
            self.position.column += 1;
        }
        
        Some(c)
    }
    
    /// Advance to the next character
    fn advance(&mut self) {
        self.current = self.lookahead;
        self.lookahead = self.read_char();
    }
    
    /// Peek at the lookahead character
    fn peek(&self) -> Option<char> {
        self.lookahead
    }
    
    /// Check if current character matches
    fn check(&self, expected: char) -> bool {
        self.current == Some(expected)
    }
    
    /// Check if lookahead matches
    fn check_next(&self, expected: char) -> bool {
        self.lookahead == Some(expected)
    }
    
    /// Make a token with the current span
    fn make_token(&self, kind: TokenKind) -> Token {
        Token {
            kind,
            span: Span {
                start: self.token_start,
                end: self.position,
            },
            value: TokenValue::None,
        }
    }
    
    /// Make a token with a value
    fn make_token_with_value(&self, kind: TokenKind, value: TokenValue) -> Token {
        Token {
            kind,
            span: Span {
                start: self.token_start,
                end: self.position,
            },
            value,
        }
    }
    
    /// Create an error token
    fn error_token(&self, message: string) -> Token {
        self.make_token(TokenKind::ERROR(message))
    }
    
    /// Skip whitespace and comments
    fn skip_whitespace_and_comments(&mut self) {
        loop {
            match self.current {
                // Whitespace
                Some(' ') | Some('\t') | Some('\r') | Some('\n') => {
                    self.advance();
                }
                
                // Line comment
                Some('/') if self.check_next('/') => {
                    self.skip_line_comment();
                }
                
                // Block comment
                Some('/') if self.check_next('*') => {
                    self.skip_block_comment();
                }
                
                // Done
                _ => break,
            }
        }
    }
    
    /// Skip a line comment
    fn skip_line_comment(&mut self) {
        // Skip //
        self.advance();
        self.advance();
        
        // Skip until end of line or file
        while !self.check('\n') && self.current.is_some() {
            self.advance();
        }
    }
    
    /// Skip a block comment (supports nesting)
    fn skip_block_comment(&mut self) {
        let mut depth = 1;
        
        // Skip /*
        self.advance();
        self.advance();
        
        while depth > 0 && self.current.is_some() {
            if self.check('/') && self.check_next('*') {
                depth += 1;
                self.advance();
                self.advance();
            } else if self.check('*') && self.check_next('/') {
                depth -= 1;
                self.advance();
                self.advance();
            } else {
                self.advance();
            }
        }
    }
    
    /// Lex an identifier or keyword
    fn lex_identifier(&mut self) -> Token {
        let start = self.position.offset;
        
        // Read identifier characters
        while let Some(c) = self.current {
            if !c.is_id_continue() {
                break;
            }
            self.advance();
        }
        
        let end = self.position.offset;
        let text = self.source[start..end];
        
        // Check for keyword
        let kind = match KEYWORDS.get(text) {
            Some(kind) => *kind,
            None => TokenKind::IDENT(text.to_string()),
        };
        
        self.make_token(kind)
    }
    
    /// Lex a number literal
    fn lex_number(&mut self) -> Token {
        let start = self.position.offset;
        
        // Check for hex, octal, binary
        if self.check('0') {
            self.advance();
            
            match self.current {
                Some('x') | Some('X') => return self.lex_hex_number(),
                Some('o') | Some('O') => return self.lex_octal_number(),
                Some('b') | Some('B') => return self.lex_binary_number(),
                _ => {}
            }
        }
        
        // Decimal number
        let mut is_float = false;
        
        // Integer part
        while let Some(c) = self.current {
            if !c.is_digit() && c != '_' {
                break;
            }
            self.advance();
        }
        
        // Fractional part
        if self.check('.') && self.peek().map_or(false, |c| c.is_digit()) {
            is_float = true;
            self.advance(); // consume '.'
            
            while let Some(c) = self.current {
                if !c.is_digit() && c != '_' {
                    break;
                }
                self.advance();
            }
        }
        
        // Exponent
        if self.check('e') || self.check('E') {
            is_float = true;
            self.advance();
            
            if self.check('+') || self.check('-') {
                self.advance();
            }
            
            while let Some(c) = self.current {
                if !c.is_digit() && c != '_' {
                    break;
                }
                self.advance();
            }
        }
        
        // Type suffix
        let suffix = self.lex_number_suffix();
        
        let end = self.position.offset;
        let text = self.source[start..end].replace("_", "");
        
        if is_float {
            let value = text.parse::<f64>().unwrap_or(0.0);
            self.make_token_with_value(TokenKind::FLOAT_LITERAL(value), TokenValue::Float(value))
        } else {
            let value = text.parse::<i64>().unwrap_or(0);
            self.make_token_with_value(TokenKind::INT_LITERAL(value), TokenValue::Int(value, Radix::Decimal))
        }
    }
    
    /// Lex hexadecimal number
    fn lex_hex_number(&mut self) -> Token {
        self.advance(); // consume 'x'
        
        let start = self.position.offset;
        
        while let Some(c) = self.current {
            if !c.is_hex_digit() && c != '_' {
                break;
            }
            self.advance();
        }
        
        let text = self.source[start..self.position.offset].replace("_", "");
        let value = i64::from_str_radix(text, 16).unwrap_or(0);
        
        self.make_token_with_value(TokenKind::INT_LITERAL(value), TokenValue::Int(value, Radix::Hexadecimal))
    }
    
    /// Lex octal number
    fn lex_octal_number(&mut self) -> Token {
        self.advance(); // consume 'o'
        
        let start = self.position.offset;
        
        while let Some(c) = self.current {
            if !c.is_octal_digit() && c != '_' {
                break;
            }
            self.advance();
        }
        
        let text = self.source[start..self.position.offset].replace("_", "");
        let value = i64::from_str_radix(text, 8).unwrap_or(0);
        
        self.make_token_with_value(TokenKind::INT_LITERAL(value), TokenValue::Int(value, Radix::Octal))
    }
    
    /// Lex binary number
    fn lex_binary_number(&mut self) -> Token {
        self.advance(); // consume 'b'
        
        let start = self.position.offset;
        
        while let Some(c) = self.current {
            if c != '0' && c != '1' && c != '_' {
                break;
            }
            self.advance();
        }
        
        let text = self.source[start..self.position.offset].replace("_", "");
        let value = i64::from_str_radix(text, 2).unwrap_or(0);
        
        self.make_token_with_value(TokenKind::INT_LITERAL(value), TokenValue::Int(value, Radix::Binary))
    }
    
    /// Lex string literal
    fn lex_string(&mut self) -> Token {
        self.advance(); // consume opening '"'
        
        let mut value = string::new();
        
        loop {
            match self.current {
                None => {
                    return self.error_token("unterminated string literal");
                }
                Some('"') => {
                    self.advance();
                    break;
                }
                Some('\\') => {
                    self.advance();
                    match self.lex_escape_sequence() {
                        Ok(c) => value.push(c),
                        Err(msg) => return self.error_token(msg),
                    }
                }
                Some(c) => {
                    value.push(c);
                    self.advance();
                }
            }
        }
        
        self.make_token_with_value(TokenKind::STRING_LITERAL(value.clone()), TokenValue::String(value))
    }
    
    /// Lex raw string literal
    fn lex_raw_string(&mut self) -> Token {
        self.advance(); // consume opening '`'
        
        let start = self.position.offset;
        
        loop {
            match self.current {
                None => {
                    return self.error_token("unterminated raw string literal");
                }
                Some('`') => {
                    self.advance();
                    break;
                }
                Some(_) => {
                    self.advance();
                }
            }
        }
        
        let value = self.source[start..self.position.offset - 1];
        self.make_token_with_value(TokenKind::STRING_LITERAL(value.to_string()), TokenValue::String(value.to_string()))
    }
    
    /// Lex character literal
    fn lex_char(&mut self) -> Token {
        self.advance(); // consume opening '\''
        
        let value = match self.current {
            None => return self.error_token("unterminated character literal"),
            Some('\\') => {
                self.advance();
                match self.lex_escape_sequence() {
                    Ok(c) => c,
                    Err(msg) => return self.error_token(msg),
                }
            }
            Some(c) => {
                self.advance();
                c
            }
        };
        
        if !self.check('\'') {
            return self.error_token("expected closing quote for character literal");
        }
        self.advance();
        
        self.make_token_with_value(TokenKind::CHAR_LITERAL(value), TokenValue::Char(value))
    }
    
    /// Lex escape sequence
    fn lex_escape_sequence(&mut self) -> Result<char, string> {
        let c = self.current.ok_or("unexpected end of escape sequence")?;
        self.advance();
        
        match c {
            'n' => Ok('\n'),
            'r' => Ok('\r'),
            't' => Ok('\t'),
            '\\' => Ok('\\'),
            '"' => Ok('"'),
            '\'' => Ok('\''),
            '0' => Ok('\0'),
            'x' => self.lex_hex_escape(2),
            'u' => self.lex_unicode_escape(),
            _ => Err(format!("invalid escape sequence: '\\{}'", c)),
        }
    }
    
    /// Lex hex escape
    fn lex_hex_escape(&mut self, digits: usize) -> Result<char, string> {
        let mut value = 0u32;
        
        for _ in 0..digits {
            let c = self.current.ok_or("unexpected end of hex escape")?;
            let digit = c.to_digit(16).ok_or("invalid hex digit")?;
            value = (value << 4) | digit;
            self.advance();
        }
        
        char::from_u32(value).ok_or("invalid hex escape value")
    }
    
    /// Lex unicode escape
    fn lex_unicode_escape(&mut self) -> Result<char, string> {
        if !self.check('{') {
            return Err("expected '{' after '\\u'".to_string());
        }
        self.advance();
        
        let mut value = 0u32;
        let mut digits = 0;
        
        while !self.check('}') {
            let c = self.current.ok_or("unexpected end of unicode escape")?;
            let digit = c.to_digit(16).ok_or("invalid hex digit in unicode escape")?;
            value = (value << 4) | digit;
            digits += 1;
            self.advance();
            
            if digits > 6 {
                return Err("unicode escape has too many digits".to_string());
            }
        }
        self.advance(); // consume '}'
        
        char::from_u32(value).ok_or("invalid unicode escape value")
    }
}

/// Character classification extensions
impl char {
    fn is_id_start(self: Self) -> bool {
        unicode::xid_start(self) || self == '_'
    }
    
    fn is_id_continue(self: Self) -> bool {
        unicode::xid_continue(self) || self == '_'
    }
    
    fn is_hex_digit(self: Self) -> bool {
        self.is_digit() || ('a'..='f').contains(&self) || ('A'..='F').contains(&self)
    }
    
    fn is_octal_digit(self: Self) -> bool {
        ('0'..='7').contains(&self)
    }
}
```

---

## 2.3 Parser Implementation

### 2.3.1 AST Node Definitions

```axiom
// src/compiler/ast/nodes.ax

/// Root AST node
pub struct Module {
    pub span: Span,
    pub name: Option<string>,
    pub declarations: Vec<Declaration>,
    pub imports: Vec<Import>,
}

/// Import declaration
pub struct Import {
    pub span: Span,
    pub path: ImportPath,
    pub alias: Option<string>,
}

pub enum ImportPath {
    Simple(Vec<string>),
    Selective(Vec<string>, Vec<string>),
}

/// Top-level declarations
pub enum Declaration {
    Function(FunctionDecl),
    Struct(StructDecl),
    Enum(EnumDecl),
    Trait(TraitDecl),
    Impl(ImplBlock),
    Const(ConstDecl),
    Static(StaticDecl),
    Type(TypeDecl),
}

/// Function declaration
pub struct FunctionDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_params: Vec<TypeParam>,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Option<Block>,
    pub is_async: bool,
    pub is_unsafe: bool,
    pub where_clause: Vec<WherePredicate>,
    pub attributes: Vec<Attribute>,
}

/// Identifier with span
pub struct Ident {
    pub span: Span,
    pub name: string,
}

/// Visibility modifier
pub enum Visibility {
    Private,
    Public,
    Crate,
    Super,
    In(string),
}

/// Type parameter
pub struct TypeParam {
    pub span: Span,
    pub name: Ident,
    pub bounds: Vec<Type>,
}

/// Function parameter
pub struct Param {
    pub span: Span,
    pub name: Ident,
    pub type_: Type,
    pub default: Option<Expr>,
}

/// Type representation
pub enum Type {
    // Named types
    Named(TypePath),
    
    // Primitives
    Primitive(PrimitiveType, Span),
    
    // Array: [T; N]
    Array(Box<Type>, Box<Expr>, Span),
    
    // Slice: [T]
    Slice(Box<Type>, Span),
    
    // Tuple: (T1, T2, ...)
    Tuple(Vec<Type>, Span),
    
    // Reference: &T or &mut T
    Reference(Box<Type>, Mutability, Span),
    
    // Raw pointer: *T or *mut T
    Pointer(Box<Type>, Mutability, Span),
    
    // Optional: ?T
    Optional(Box<Type>, Span),
    
    // Function type
    Function(FunctionType, Span),
    
    // Generic arguments: Path<T1, T2>
    Generic(TypePath, Vec<Type>, Span),
    
    // Infer: _
    Infer(Span),
}

/// Primitive types
pub enum PrimitiveType {
    I8, I16, I32, I64, I128, ISize,
    U8, U16, U32, U64, U128, USize,
    F32, F64,
    Bool, Char, Byte, String, Void,
}

/// Mutability qualifier
pub enum Mutability {
    Immutable,
    Mutable,
}

/// Function type
pub struct FunctionType {
    pub params: Vec<Type>,
    pub return_type: Option<Type>,
    pub is_unsafe: bool,
    pub is_async: bool,
}

/// Struct declaration
pub struct StructDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_params: Vec<TypeParam>,
    pub fields: Vec<Field>,
    pub attributes: Vec<Attribute>,
}

/// Struct field
pub struct Field {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_: Type,
}

/// Enum declaration
pub struct EnumDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_params: Vec<TypeParam>,
    pub variants: Vec<Variant>,
    pub attributes: Vec<Attribute>,
}

/// Enum variant
pub struct Variant {
    pub span: Span,
    pub name: Ident,
    pub data: VariantData,
}

/// Variant data
pub enum VariantData {
    Unit,
    Tuple(Vec<Type>),
    Struct(Vec<Field>),
}

/// Trait declaration
pub struct TraitDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_params: Vec<TypeParam>,
    pub super_traits: Vec<Type>,
    pub items: Vec<TraitItem>,
    pub attributes: Vec<Attribute>,
}

/// Trait item
pub enum TraitItem {
    Function(TraitFunction),
    Const(TraitConst),
    Type(TraitType),
}

pub struct TraitFunction {
    pub span: Span,
    pub signature: FunctionDecl,
    pub default: Option<Block>,
}

/// Impl block
pub struct ImplBlock {
    pub span: Span,
    pub type_params: Vec<TypeParam>,
    pub trait_type: Option<Type>,
    pub for_type: Type,
    pub items: Vec<ImplItem>,
}

/// Impl item
pub enum ImplItem {
    Function(FunctionDecl),
    Const(ConstDecl),
    Type(TypeDecl),
}

/// Constant declaration
pub struct ConstDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_: Type,
    pub value: Expr,
    pub attributes: Vec<Attribute>,
}

/// Static declaration
pub struct StaticDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_: Type,
    pub value: Expr,
    pub is_mutable: bool,
    pub attributes: Vec<Attribute>,
}

/// Type alias declaration
pub struct TypeDecl {
    pub span: Span,
    pub name: Ident,
    pub visibility: Visibility,
    pub type_params: Vec<TypeParam>,
    pub type_: Type,
}

/// Block (sequence of statements)
pub struct Block {
    pub span: Span,
    pub statements: Vec<Statement>,
    pub final_expr: Option<Expr>,
}

/// Statement
pub enum Statement {
    Let(LetStmt),
    Var(VarStmt),
    Expr(Expr),
    Empty,
}

/// Let statement
pub struct LetStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub type_: Option<Type>,
    pub init: Option<Expr>,
}

/// Var statement  
pub struct VarStmt {
    pub span: Span,
    pub pattern: Pattern,
    pub type_: Option<Type>,
    pub init: Option<Expr>,
}

/// Expression
pub enum Expr {
    // Literals
    Literal(Literal, Span),
    
    // Identifiers
    Ident(Ident),
    Path(TypePath, Span),
    
    // Operations
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Assign(AssignExpr),
    
    // Access
    Field(FieldExpr),
    Index(IndexExpr),
    Call(CallExpr),
    Method(MethodExpr),
    
    // Control flow
    If(IfExpr),
    Match(MatchExpr),
    Block(Block),
    Loop(LoopExpr),
    While(WhileExpr),
    For(ForExpr),
    Return(ReturnExpr),
    Break(BreakExpr),
    Continue(ContinueExpr),
    
    // Lambda
    Lambda(LambdaExpr),
    
    // Struct construction
    Struct(StructExpr),
    
    // Reference
    Reference(ReferenceExpr),
    
    // Dereference
    Deref(DerefExpr),
    
    // Range
    Range(RangeExpr),
    
    // Tuple
    Tuple(TupleExpr),
    
    // Array
    Array(ArrayExpr),
    
    // Error
    Error(Span, string),
}

/// Literal value
pub enum Literal {
    Int(i64, Option<IntSuffix>),
    Float(f64, Option<FloatSuffix>),
    String(string),
    Char(char),
    Bool(bool),
    Null,
}

pub enum IntSuffix {
    I8, I16, I32, I64, I128, ISize,
    U8, U16, U32, U64, U128, USize,
}

pub enum FloatSuffix {
    F32, F64,
}

/// Binary expression
pub struct BinaryExpr {
    pub span: Span,
    pub op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

pub enum BinaryOp {
    // Arithmetic
    Add, Sub, Mul, Div, Mod, Pow, IntDiv,
    
    // Comparison
    Eq, Ne, Lt, Gt, Le, Ge, Spaceship,
    
    // Logical
    And, Or,
    
    // Bitwise
    BitAnd, BitOr, BitXor, Shl, Shr, UShr,
}

/// Unary expression
pub struct UnaryExpr {
    pub span: Span,
    pub op: UnaryOp,
    pub operand: Box<Expr>,
}

pub enum UnaryOp {
    Neg, Not, BitNot, Deref, Ref, RefMut, Move,
}

/// Assignment expression
pub struct AssignExpr {
    pub span: Span,
    pub target: Box<Expr>,
    pub op: Option<BinaryOp>,
    pub value: Box<Expr>,
}

/// Field access expression
pub struct FieldExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub field: Ident,
}

/// Index expression
pub struct IndexExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub index: Box<Expr>,
}

/// Call expression
pub struct CallExpr {
    pub span: Span,
    pub func: Box<Expr>,
    pub args: Vec<Expr>,
}

/// Method call expression
pub struct MethodExpr {
    pub span: Span,
    pub base: Box<Expr>,
    pub method: Ident,
    pub type_args: Vec<Type>,
    pub args: Vec<Expr>,
}

/// If expression
pub struct IfExpr {
    pub span: Span,
    pub condition: Box<Expr>,
    pub then_block: Block,
    pub else_block: Option<Box<Expr>>,
}

/// Match expression
pub struct MatchExpr {
    pub span: Span,
    pub value: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

/// Match arm
pub struct MatchArm {
    pub span: Span,
    pub pattern: Pattern,
    pub guard: Option<Expr>,
    pub body: MatchBody,
}

pub enum MatchBody {
    Expr(Expr),
    Block(Block),
}

/// Pattern
pub enum Pattern {
    Wildcard(Span),
    Literal(Literal, Span),
    Ident(Ident, Mutability),
    Struct(StructPattern),
    Tuple(TuplePattern),
    Or(OrPattern),
    Range(RangePattern),
    Rest(Span),
}

pub struct StructPattern {
    pub span: Span,
    pub path: TypePath,
    pub fields: Vec<FieldPattern>,
    pub rest: bool,
}

pub struct FieldPattern {
    pub span: Span,
    pub name: Ident,
    pub pattern: Option<Pattern>,
}

pub struct TuplePattern {
    pub span: Span,
    pub elements: Vec<Pattern>,
}

pub struct OrPattern {
    pub span: Span,
    pub alternatives: Vec<Pattern>,
}

pub struct RangePattern {
    pub span: Span,
    pub start: Literal,
    pub end: Literal,
}

/// Loop expression
pub struct LoopExpr {
    pub span: Span,
    pub label: Option<string>,
    pub body: Block,
}

/// While expression
pub struct WhileExpr {
    pub span: Span,
    pub label: Option<string>,
    pub condition: Box<Expr>,
    pub body: Block,
}

/// For expression
pub struct ForExpr {
    pub span: Span,
    pub label: Option<string>,
    pub pattern: Pattern,
    pub iterable: Box<Expr>,
    pub body: Block,
}

/// Return expression
pub struct ReturnExpr {
    pub span: Span,
    pub value: Option<Expr>,
}

/// Break expression
pub struct BreakExpr {
    pub span: Span,
    pub label: Option<string>,
    pub value: Option<Expr>,
}

/// Continue expression
pub struct ContinueExpr {
    pub span: Span,
    pub label: Option<string>,
}

/// Lambda expression
pub struct LambdaExpr {
    pub span: Span,
    pub params: Vec<Param>,
    pub return_type: Option<Type>,
    pub body: Box<Expr>,
    pub captures: CaptureMode,
}

pub enum CaptureMode {
    Ref,
    RefMut,
    Move,
}

/// Struct construction expression
pub struct StructExpr {
    pub span: Span,
    pub path: TypePath,
    pub fields: Vec<FieldInit>,
    pub spread: Option<Expr>,
}

pub struct FieldInit {
    pub span: Span,
    pub name: Ident,
    pub value: Expr,
}

/// Reference expression
pub struct ReferenceExpr {
    pub span: Span,
    pub mutability: Mutability,
    pub operand: Box<Expr>,
}

/// Dereference expression
pub struct DerefExpr {
    pub span: Span,
    pub operand: Box<Expr>,
}

/// Range expression
pub struct RangeExpr {
    pub span: Span,
    pub start: Option<Expr>,
    pub end: Option<Expr>,
    pub inclusive: bool,
}

/// Tuple expression
pub struct TupleExpr {
    pub span: Span,
    pub elements: Vec<Expr>,
}

/// Array expression
pub struct ArrayExpr {
    pub span: Span,
    pub elements: Vec<Expr>,
}

/// Attribute
pub struct Attribute {
    pub span: Span,
    pub name: string,
    pub args: Vec<AttributeArg>,
}

pub enum AttributeArg {
    Literal(Literal),
    Ident(string),
    KeyValue(string, Literal),
}
```

### 2.3.2 Parser Core Implementation

```axiom
// src/compiler/parser/parser.ax

/// The Parser transforms tokens into an Abstract Syntax Tree
pub struct Parser {
    /// Token stream
    tokens: Vec<Token>,
    
    /// Current position in token stream
    position: usize,
    
    /// Current token
    current: Token,
    
    /// Lookahead token
    lookahead: Token,
    
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    
    /// File ID for diagnostics
    file_id: FileId,
    
    /// Expected token stack for error recovery
    expected_stack: Vec<TokenKind>,
}

impl Parser {
    /// Create a new parser from tokens
    pub fn new(tokens: Vec<Token>, file_id: FileId) -> Self {
        let mut tokens = tokens;
        
        // Ensure EOF token at end
        if tokens.last().map_or(true, |t| t.kind != TokenKind::EOF) {
            tokens.push(Token {
                kind: TokenKind::EOF,
                span: Span::default(),
                value: TokenValue::None,
            });
        }
        
        let mut parser = Self {
            tokens,
            position: 0,
            current: Token::default(),
            lookahead: Token::default(),
            diagnostics: Vec::new(),
            file_id,
            expected_stack: Vec::new(),
        };
        
        // Initialize lookahead
        parser.current = parser.tokens[0].clone();
        parser.lookahead = parser.tokens.get(1).cloned().unwrap_or_default();
        
        parser
    }
    
    /// Parse a complete module
    pub fn parse_module(&mut self) -> Result!Module {
        let mut module = Module {
            span: Span::default(),
            name: None,
            imports: Vec::new(),
            declarations: Vec::new(),
        };
        
        // Parse module declaration (optional)
        if self.check_keyword("module") {
            module.name = Some(self.parse_module_decl()?);
        }
        
        // Parse imports
        while self.check_keyword("import") {
            module.imports.push(self.parse_import()?);
        }
        
        // Parse declarations
        while !self.at_eof() {
            match self.parse_declaration() {
                Ok(decl) => module.declarations.push(decl),
                Err(e) => {
                    self.diagnostics.push(e.into());
                    self.recover_to_declaration();
                }
            }
        }
        
        module.span = Span::merge(
            module.declarations.first().map(|d| d.span()).unwrap_or_default(),
            module.declarations.last().map(|d| d.span()).unwrap_or_default(),
        );
        
        Ok(module)
    }
    
    // ============ TOKEN HANDLING ============
    
    /// Check if current token is EOF
    fn at_eof(&self) -> bool {
        matches!(self.current.kind, TokenKind::EOF)
    }
    
    /// Check current token kind
    fn check(&self, kind: TokenKind) -> bool {
        std::mem::discriminant(&self.current.kind) == std::mem::discriminant(&kind)
    }
    
    /// Check if current is a keyword
    fn check_keyword(&self, keyword: &str) -> bool {
        match &self.current.kind {
            TokenKind::IDENT(name) if name == keyword => true,
            _ => false,
        }
    }
    
    /// Check if current is an identifier
    fn check_ident(&self) -> bool {
        matches!(self.current.kind, TokenKind::IDENT(_))
    }
    
    /// Check if current is a literal
    fn check_literal(&self) -> bool {
        matches!(self.current.kind, 
            TokenKind::INT_LITERAL(_) | 
            TokenKind::FLOAT_LITERAL(_) | 
            TokenKind::STRING_LITERAL(_) | 
            TokenKind::CHAR_LITERAL(_) |
            TokenKind::TRUE | 
            TokenKind::FALSE |
            TokenKind::NULL
        )
    }
    
    /// Advance to next token
    fn advance(&mut self) -> Token {
        let token = std::mem::take(&mut self.current);
        
        self.position += 1;
        self.current = self.lookahead.clone();
        self.lookahead = self.tokens.get(self.position + 1).cloned().unwrap_or_default();
        
        token
    }
    
    /// Expect a specific token kind
    fn expect(&mut self, kind: TokenKind) -> Result!Token {
        if self.check(kind.clone()) {
            return Ok(self.advance());
        }
        
        Err(ParseError::expected(
            format!("{:?}", kind),
            self.current.kind.clone(),
            self.current.span.clone(),
        ))
    }
    
    /// Expect an identifier
    fn expect_ident(&mut self) -> Result!Ident {
        if self.check_ident() {
            let token = self.advance();
            let name = match token.kind {
                TokenKind::IDENT(n) => n,
                _ => unreachable!(),
            };
            return Ok(Ident { span: token.span, name });
        }
        
        Err(ParseError::expected(
            "identifier".to_string(),
            format!("{:?}", self.current.kind),
            self.current.span.clone(),
        ))
    }
    
    /// Optional token consumption
    fn eat(&mut self, kind: TokenKind) -> Option<Token> {
        if self.check(kind) {
            Some(self.advance())
        } else {
            None
        }
    }
    
    // ============ DECLARATION PARSING ============
    
    /// Parse a declaration
    fn parse_declaration(&mut self) -> Result!Declaration {
        // Parse attributes
        let attrs = self.parse_attributes()?;
        
        // Parse visibility
        let visibility = self.parse_visibility();
        
        // Peek at next tokens to determine declaration type
        match &self.current.kind {
            TokenKind::IDENT(name) => match name.as_str() {
                "fn" => self.parse_function(visibility, attrs),
                "struct" => self.parse_struct(visibility, attrs),
                "enum" => self.parse_enum(visibility, attrs),
                "trait" => self.parse_trait(visibility, attrs),
                "impl" => self.parse_impl(),
                "const" => self.parse_const(visibility, attrs),
                "static" => self.parse_static(visibility, attrs),
                "type" => self.parse_type_alias(visibility),
                _ => Err(ParseError::unexpected(
                    format!("{:?}", name),
                    self.current.span.clone(),
                )),
            },
            _ => Err(ParseError::unexpected(
                format!("{:?}", self.current.kind),
                self.current.span.clone(),
            )),
        }
    }
    
    /// Parse function declaration
    fn parse_function(&mut self, visibility: Visibility, attrs: Vec<Attribute>) -> Result!Declaration {
        let start = self.current.span.start;
        self.expect_keyword("fn")?;
        
        let name = self.expect_ident()?;
        let type_params = self.parse_type_params()?;
        let params = self.parse_params()?;
        let return_type = self.parse_return_type()?;
        let where_clause = self.parse_where_clause()?;
        
        let body = if self.check(TokenKind::LBRACE) {
            Some(self.parse_block()?)
        } else {
            self.expect(TokenKind::SEMICOLON)?;
            None
        };
        
        Ok(Declaration::Function(FunctionDecl {
            span: Span::from_start_end(start, self.current.span.end),
            name,
            visibility,
            type_params,
            params,
            return_type,
            body,
            is_async: false,
            is_unsafe: false,
            where_clause,
            attributes: attrs,
        }))
    }
    
    /// Parse function parameters
    fn parse_params(&mut self) -> Result!Vec<Param> {
        self.expect(TokenKind::LPAREN)?;
        
        let mut params = Vec::new();
        
        while !self.check(TokenKind::RPAREN) {
            let name = self.expect_ident()?;
            self.expect(TokenKind::COLON)?;
            let type_ = self.parse_type()?;
            
            let default = if self.eat(TokenKind::EQ).is_some() {
                Some(self.parse_expr()?)
            } else {
                None
            };
            
            params.push(Param {
                span: Span::merge(name.span, self.current.span),
                name,
                type_,
                default,
            });
            
            if !self.eat(TokenKind::COMMA).is_some() {
                break;
            }
        }
        
        self.expect(TokenKind::RPAREN)?;
        Ok(params)
    }
    
    /// Parse return type
    fn parse_return_type(&mut self) -> Result!Option<Type> {
        if self.eat(TokenKind::ARROW).is_some() {
            Ok(Some(self.parse_type()?))
        } else {
            Ok(None)
        }
    }
    
    /// Parse type parameters
    fn parse_type_params(&mut self) -> Result!Vec<TypeParam> {
        if self.eat(TokenKind::LT).is_none() {
            return Ok(Vec::new());
        }
        
        let mut params = Vec::new();
        
        while !self.check(TokenKind::GT) {
            let name = self.expect_ident()?;
            
            let bounds = if self.eat(TokenKind::COLON).is_some() {
                self.parse_type_bounds()?
            } else {
                Vec::new()
            };
            
            params.push(TypeParam {
                span: name.span.clone(),
                name,
                bounds,
            });
            
            if !self.eat(TokenKind::COMMA).is_some() {
                break;
            }
        }
        
        self.expect(TokenKind::GT)?;
        Ok(params)
    }
    
    /// Parse type bounds
    fn parse_type_bounds(&mut self) -> Result!Vec<Type> {
        let mut bounds = Vec::new();
        
        bounds.push(self.parse_type()?);
        
        while self.eat(TokenKind::PLUS).is_some() {
            bounds.push(self.parse_type()?);
        }
        
        Ok(bounds)
    }
    
    /// Parse a block
    fn parse_block(&mut self) -> Result!Block {
        let start = self.current.span.start;
        self.expect(TokenKind::LBRACE)?;
        
        let mut statements = Vec::new();
        let mut final_expr = None;
        
        while !self.check(TokenKind::RBRACE) {
            // Try to parse a statement
            let stmt = self.parse_statement()?;
            
            // Check if this is a final expression (no semicolon)
            if self.check(TokenKind::RBRACE) {
                if let Statement::Expr(expr) = stmt {
                    final_expr = Some(expr);
                    break;
                }
            }
            
            statements.push(stmt);
        }
        
        self.expect(TokenKind::RBRACE)?;
        
        Ok(Block {
            span: Span::from_start_end(start, self.current.span.end),
            statements,
            final_expr,
        })
    }
    
    /// Parse a statement
    fn parse_statement(&mut self) -> Result!Statement {
        match &self.current.kind {
            TokenKind::IDENT(name) => match name.as_str() {
                "let" => self.parse_let_statement(),
                "var" => self.parse_var_statement(),
                _ => self.parse_expr_statement(),
            },
            _ => self.parse_expr_statement(),
        }
    }
    
    /// Parse let statement
    fn parse_let_statement(&mut self) -> Result!Statement {
        let start = self.current.span.start;
        self.expect_keyword("let")?;
        
        let pattern = self.parse_pattern()?;
        let type_ = if self.eat(TokenKind::COLON).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let init = if self.eat(TokenKind::EQ).is_some() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        self.expect(TokenKind::SEMICOLON)?;
        
        Ok(Statement::Let(LetStmt {
            span: Span::from_start_end(start, self.current.span.end),
            pattern,
            type_,
            init,
        }))
    }
    
    /// Parse var statement
    fn parse_var_statement(&mut self) -> Result!Statement {
        let start = self.current.span.start;
        self.expect_keyword("var")?;
        
        let pattern = self.parse_pattern()?;
        let type_ = if self.eat(TokenKind::COLON).is_some() {
            Some(self.parse_type()?)
        } else {
            None
        };
        
        let init = if self.eat(TokenKind::EQ).is_some() {
            Some(self.parse_expr()?)
        } else {
            None
        };
        
        self.expect(TokenKind::SEMICOLON)?;
        
        Ok(Statement::Var(VarStmt {
            span: Span::from_start_end(start, self.current.span.end),
            pattern,
            type_,
            init,
        }))
    }
    
    /// Parse expression statement
    fn parse_expr_statement(&mut self) -> Result!Statement {
        let expr = self.parse_expr()?;
        
        if self.eat(TokenKind::SEMICOLON).is_some() {
            Ok(Statement::Expr(expr))
        } else if self.check(TokenKind::RBRACE) {
            // Final expression in block
            Ok(Statement::Expr(expr))
        } else {
            Err(ParseError::expected(
                "semicolon".to_string(),
                format!("{:?}", self.current.kind),
                self.current.span.clone(),
            ))
        }
    }
    
    // ============ EXPRESSION PARSING ============
    
    /// Parse expression (entry point)
    fn parse_expr(&mut self) -> Result!Expr {
        self.parse_assignment_expr()
    }
    
    /// Parse assignment expression (lowest precedence)
    fn parse_assignment_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut expr = self.parse_or_expr()?;
        
        if let Some(op) = self.parse_assignment_op() {
            let value = self.parse_assignment_expr()?;
            
            expr = Expr::Assign(AssignExpr {
                span: Span::from_start_end(start, self.current.span.end),
                target: Box::new(expr),
                op,
                value: Box::new(value),
            });
        }
        
        Ok(expr)
    }
    
    /// Parse assignment operator
    fn parse_assignment_op(&mut self) -> Option<Option<BinaryOp>> {
        let op = match &self.current.kind {
            TokenKind::EQ => None,
            TokenKind::PLUS_EQ => Some(BinaryOp::Add),
            TokenKind::MINUS_EQ => Some(BinaryOp::Sub),
            TokenKind::STAR_EQ => Some(BinaryOp::Mul),
            TokenKind::SLASH_EQ => Some(BinaryOp::Div),
            TokenKind::PERCENT_EQ => Some(BinaryOp::Mod),
            TokenKind::AMP_EQ => Some(BinaryOp::BitAnd),
            TokenKind::PIPE_EQ => Some(BinaryOp::BitOr),
            TokenKind::CARET_EQ => Some(BinaryOp::BitXor),
            TokenKind::LT_LT_EQ => Some(BinaryOp::Shl),
            TokenKind::GT_GT_EQ => Some(BinaryOp::Shr),
            _ => return None,
        };
        
        self.advance();
        Some(op)
    }
    
    /// Parse OR expression
    fn parse_or_expr(&mut self) -> Result!Expr {
        self.parse_binary_expr(
            |p| p.parse_and_expr(),
            TokenKind::PIPE_PIPE,
            BinaryOp::Or,
        )
    }
    
    /// Parse AND expression
    fn parse_and_expr(&mut self) -> Result!Expr {
        self.parse_binary_expr(
            |p| p.parse_equality_expr(),
            TokenKind::AMP_AMP,
            BinaryOp::And,
        )
    }
    
    /// Parse equality expression
    fn parse_equality_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut left = self.parse_comparison_expr()?;
        
        loop {
            let op = match &self.current.kind {
                TokenKind::EQ_EQ => BinaryOp::Eq,
                TokenKind::BANG_EQ => BinaryOp::Ne,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_comparison_expr()?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    /// Parse comparison expression
    fn parse_comparison_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut left = self.parse_bitwise_or_expr()?;
        
        loop {
            let op = match &self.current.kind {
                TokenKind::LT => BinaryOp::Lt,
                TokenKind::GT => BinaryOp::Gt,
                TokenKind::LT_EQ => BinaryOp::Le,
                TokenKind::GT_EQ => BinaryOp::Ge,
                TokenKind::SPACESHIP => BinaryOp::Spaceship,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_bitwise_or_expr()?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    /// Parse bitwise OR
    fn parse_bitwise_or_expr(&mut self) -> Result!Expr {
        self.parse_binary_expr(
            |p| p.parse_bitwise_xor_expr(),
            TokenKind::PIPE,
            BinaryOp::BitOr,
        )
    }
    
    /// Parse bitwise XOR
    fn parse_bitwise_xor_expr(&mut self) -> Result!Expr {
        self.parse_binary_expr(
            |p| p.parse_bitwise_and_expr(),
            TokenKind::CARET,
            BinaryOp::BitXor,
        )
    }
    
    /// Parse bitwise AND
    fn parse_bitwise_and_expr(&mut self) -> Result!Expr {
        self.parse_binary_expr(
            |p| p.parse_shift_expr(),
            TokenKind::AMP,
            BinaryOp::BitAnd,
        )
    }
    
    /// Parse shift expression
    fn parse_shift_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut left = self.parse_additive_expr()?;
        
        loop {
            let op = match &self.current.kind {
                TokenKind::LT_LT => BinaryOp::Shl,
                TokenKind::GT_GT => BinaryOp::Shr,
                TokenKind::GT_GT_GT => BinaryOp::UShr,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_additive_expr()?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    /// Parse additive expression
    fn parse_additive_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut left = self.parse_multiplicative_expr()?;
        
        loop {
            let op = match &self.current.kind {
                TokenKind::PLUS => BinaryOp::Add,
                TokenKind::MINUS => BinaryOp::Sub,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_multiplicative_expr()?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    /// Parse multiplicative expression
    fn parse_multiplicative_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut left = self.parse_power_expr()?;
        
        loop {
            let op = match &self.current.kind {
                TokenKind::STAR => BinaryOp::Mul,
                TokenKind::SLASH => BinaryOp::Div,
                TokenKind::PERCENT => BinaryOp::Mod,
                TokenKind::SLASH_SLASH => BinaryOp::IntDiv,
                _ => break,
            };
            
            self.advance();
            let right = self.parse_power_expr()?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op,
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    /// Parse power expression (right associative)
    fn parse_power_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let left = self.parse_unary_expr()?;
        
        if self.check(TokenKind::STAR_STAR) {
            self.advance();
            let right = self.parse_power_expr()?; // Right associative
            
            return Ok(Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op: BinaryOp::Pow,
                left: Box::new(left),
                right: Box::new(right),
            }));
        }
        
        Ok(left)
    }
    
    /// Parse unary expression
    fn parse_unary_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        
        let op = match &self.current.kind {
            TokenKind::MINUS => UnaryOp::Neg,
            TokenKind::BANG => UnaryOp::Not,
            TokenKind::TILDE => UnaryOp::BitNot,
            TokenKind::STAR => UnaryOp::Deref,
            TokenKind::AMP => {
                self.advance();
                let mutability = if self.check_keyword("mut") {
                    self.advance();
                    Mutability::Mutable
                } else {
                    Mutability::Immutable
                };
                let operand = self.parse_unary_expr()?;
                return Ok(Expr::Reference(ReferenceExpr {
                    span: Span::from_start_end(start, self.current.span.end),
                    mutability,
                    operand: Box::new(operand),
                }));
            }
            _ => return self.parse_postfix_expr(),
        };
        
        self.advance();
        let operand = self.parse_unary_expr()?;
        
        Ok(Expr::Unary(UnaryExpr {
            span: Span::from_start_end(start, self.current.span.end),
            op,
            operand: Box::new(operand),
        }))
    }
    
    /// Parse postfix expression
    fn parse_postfix_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        let mut expr = self.parse_primary_expr()?;
        
        loop {
            match &self.current.kind {
                // Method call: .method(args)
                TokenKind::DOT => {
                    self.advance();
                    let method = self.expect_ident()?;
                    
                    let type_args = if self.check(TokenKind::COLON_COLON) {
                        self.advance();
                        self.parse_type_args()?
                    } else {
                        Vec::new()
                    };
                    
                    let args = self.parse_args()?;
                    
                    expr = Expr::Method(MethodExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        base: Box::new(expr),
                        method,
                        type_args,
                        args,
                    });
                }
                
                // Field access: .field
                TokenKind::DOT if !self.check_next(TokenKind::DOT) => {
                    self.advance();
                    let field = self.expect_ident()?;
                    
                    expr = Expr::Field(FieldExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        base: Box::new(expr),
                        field,
                    });
                }
                
                // Index: [index]
                TokenKind::LBRACKET => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(TokenKind::RBRACKET)?;
                    
                    expr = Expr::Index(IndexExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        base: Box::new(expr),
                        index: Box::new(index),
                    });
                }
                
                // Call: (args)
                TokenKind::LPAREN => {
                    let args = self.parse_args()?;
                    
                    expr = Expr::Call(CallExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        func: Box::new(expr),
                        args,
                    });
                }
                
                // Optional unwrap: ?
                TokenKind::QUESTION => {
                    self.advance();
                    // Wrap in optional unwrap
                    expr = Expr::Unary(UnaryExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        op: UnaryOp::Unwrap,
                        operand: Box::new(expr),
                    });
                }
                
                // Range: .. or ..=
                TokenKind::DOT_DOT => {
                    self.advance();
                    let end = if !self.check(TokenKind::RBRACE) && 
                               !self.check(TokenKind::RPAREN) &&
                               !self.check(TokenKind::COMMA) {
                        Some(self.parse_range_expr()?)
                    } else {
                        None
                    };
                    
                    expr = Expr::Range(RangeExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        start: Some(expr),
                        end,
                        inclusive: false,
                    });
                }
                
                TokenKind::DOT_DOT_EQ => {
                    self.advance();
                    let end = self.parse_range_expr()?;
                    
                    expr = Expr::Range(RangeExpr {
                        span: Span::from_start_end(start, self.current.span.end),
                        start: Some(expr),
                        end: Some(end),
                        inclusive: true,
                    });
                }
                
                _ => break,
            }
        }
        
        Ok(expr)
    }
    
    /// Parse primary expression
    fn parse_primary_expr(&mut self) -> Result!Expr {
        let start = self.current.span.start;
        
        match &self.current.kind {
            // Literals
            TokenKind::INT_LITERAL(v) => {
                let v = *v;
                self.advance();
                Ok(Expr::Literal(Literal::Int(v, None), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::FLOAT_LITERAL(v) => {
                let v = *v;
                self.advance();
                Ok(Expr::Literal(Literal::Float(v, None), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::STRING_LITERAL(v) => {
                let v = v.clone();
                self.advance();
                Ok(Expr::Literal(Literal::String(v), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::CHAR_LITERAL(v) => {
                let v = *v;
                self.advance();
                Ok(Expr::Literal(Literal::Char(v), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::TRUE => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(true), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::FALSE => {
                self.advance();
                Ok(Expr::Literal(Literal::Bool(false), Span::from_start_end(start, self.current.span.end)))
            }
            TokenKind::NULL => {
                self.advance();
                Ok(Expr::Literal(Literal::Null, Span::from_start_end(start, self.current.span.end)))
            }
            
            // Identifier or path
            TokenKind::IDENT(_) => self.parse_path_or_ident(),
            
            // Parenthesized expression or tuple
            TokenKind::LPAREN => self.parse_tuple_or_parens(),
            
            // Array
            TokenKind::LBRACKET => self.parse_array(),
            
            // Block
            TokenKind::LBRACE => self.parse_block_expr(),
            
            // If expression
            _ if self.check_keyword("if") => self.parse_if_expr(),
            
            // Match expression
            _ if self.check_keyword("match") => self.parse_match_expr(),
            
            // Loop expression
            _ if self.check_keyword("loop") => self.parse_loop_expr(),
            
            // While expression
            _ if self.check_keyword("while") => self.parse_while_expr(),
            
            // For expression
            _ if self.check_keyword("for") => self.parse_for_expr(),
            
            // Return expression
            _ if self.check_keyword("return") => self.parse_return_expr(),
            
            // Break expression
            _ if self.check_keyword("break") => self.parse_break_expr(),
            
            // Continue expression
            _ if self.check_keyword("continue") => self.parse_continue_expr(),
            
            // Lambda
            TokenKind::PIPE => self.parse_lambda(),
            
            _ => Err(ParseError::unexpected(
                format!("{:?}", self.current.kind),
                self.current.span.clone(),
            )),
        }
    }
    
    /// Generic binary expression parser
    fn parse_binary_expr<F>(
        &mut self,
        parse_operand: F,
        token: TokenKind,
        op: BinaryOp,
    ) -> Result!Expr
    where
        F: Fn(&mut Self) -> Result!Expr,
    {
        let start = self.current.span.start;
        let mut left = parse_operand(self)?;
        
        while self.check(token.clone()) {
            self.advance();
            let right = parse_operand(self)?;
            
            left = Expr::Binary(BinaryExpr {
                span: Span::from_start_end(start, self.current.span.end),
                op: op.clone(),
                left: Box::new(left),
                right: Box::new(right),
            });
        }
        
        Ok(left)
    }
    
    // ============ ERROR RECOVERY ============
    
    /// Recover to the next declaration boundary
    fn recover_to_declaration(&mut self) {
        while !self.at_eof() {
            match &self.current.kind {
                // Declaration keywords
                TokenKind::IDENT(name) => {
                    match name.as_str() {
                        "fn" | "struct" | "enum" | "trait" | "impl" | 
                        "const" | "static" | "type" | "pub" => return,
                        _ => {}
                    }
                }
                TokenKind::RBRACE => return,
                _ => {}
            }
            self.advance();
        }
    }
}
```

---

# Part 3: Type Checker Implementation

## 3.1 Type System Core

```axiom
// src/compiler/typeck/types.ax

/// Representable types in the type system
pub enum Ty {
    // Primitive types
    Int(IntTy),
    Float(FloatTy),
    Bool,
    Char,
    Byte,
    String,
    Void,
    
    // Named types (structs, enums, type aliases)
    Named(NamedTy),
    
    // Type variables (for inference)
    Var(TypeVar),
    
    // Generic parameters
    Param(TypeParamId),
    
    // Composite types
    Array(Box<Ty>, ConstInt),
    Slice(Box<Ty>),
    Tuple(Vec<Ty>),
    Ref(Region, Box<Ty>, Mutability),
    RawPtr(Box<Ty>, Mutability),
    Optional(Box<Ty>),
    Function(FunctionTy),
    
    // Trait objects
    TraitObject(Vec<TraitBound>),
    
    // Projection (associated types)
    Projection(ProjectionTy),
    
    // Error type
    Error,
    
    // Never type
    Never,
}

/// Integer types
pub enum IntTy {
    I8, I16, I32, I64, I128, ISize,
    U8, U16, U32, U64, U128, USize,
}

/// Float types
pub enum FloatTy {
    F32, F64,
}

/// Named type reference
pub struct NamedTy {
    pub def_id: DefId,
    pub args: Vec<Ty>,
}

/// Type variable for inference
pub struct TypeVar {
    pub id: TypeVarId,
    pub name: string,
}

/// Region (lifetime) representation
pub enum Region {
    /// Static lifetime
    Static,
    /// Named lifetime parameter
    EarlyBound(LifetimeParamId),
    /// Anonymous lifetime from inference
    LateBound(RegionVarId),
    /// Re erased ('_)
    ReErased,
    /// Concrete region
    ReConcrete(RegionId),
}

/// Function type
pub struct FunctionTy {
    pub params: Vec<Ty>,
    pub return_type: Box<Ty>,
    pub regions: Vec<Region>,
    pub is_unsafe: bool,
    pub is_async: bool,
    pub abi: Abi,
}

/// Trait bound
pub struct TraitBound {
    pub trait_id: TraitId,
    pub args: Vec<Ty>,
}

/// Projection type (associated type)
pub struct ProjectionTy {
    pub trait_bound: TraitBound,
    pub associated_type: string,
}

/// Type kind (for error messages)
pub enum TyKind {
    Int,
    Float,
    Bool,
    Char,
    String,
    Array,
    Slice,
    Tuple,
    Function,
    Struct,
    Enum,
    Trait,
    Reference,
    Pointer,
    Optional,
    Generic,
    Error,
}
```

## 3.2 Type Inference Engine

```axiom
// src/compiler/typeck/inference.ax

/// Type inference engine
pub struct TypeInference {
    /// Type variable unification table
    unification_table: UnificationTable<TypeVarId, Ty>,
    
    /// Pending constraints
    constraints: Vec<Constraint>,
    
    /// Region constraints
    region_constraints: Vec<RegionConstraint>,
    
    /// Type variable counter
    next_type_var: TypeVarId,
    
    /// Region variable counter
    next_region_var: RegionVarId,
}

impl TypeInference {
    /// Create new inference context
    pub fn new() -> Self {
        Self {
            unification_table: UnificationTable::new(),
            constraints: Vec::new(),
            region_constraints: Vec::new(),
            next_type_var: TypeVarId(0),
            next_region_var: RegionVarId(0),
        }
    }
    
    /// Create a fresh type variable
    pub fn fresh_type_var(&mut self) -> Ty {
        let id = self.next_type_var;
        self.next_type_var = TypeVarId(id.0 + 1);
        Ty::Var(TypeVar { id, name: format!("_T{}", id.0) })
    }
    
    /// Create a fresh region variable
    pub fn fresh_region_var(&mut self) -> Region {
        let id = self.next_region_var;
        self.next_region_var = RegionVarId(id.0 + 1);
        Region::LateBound(id)
    }
    
    /// Unify two types
    pub fn unify(&mut self, expected: &Ty, actual: &Ty) -> Result!() {
        // Resolve type variables
        let expected = self.resolve(expected);
        let actual = self.resolve(actual);
        
        match (expected, actual) {
            // Same type
            (Ty::Int(a), Ty::Int(b)) if a == b => Ok(()),
            (Ty::Float(a), Ty::Float(b)) if a == b => Ok(()),
            (Ty::Bool, Ty::Bool) => Ok(()),
            (Ty::Char, Ty::Char) => Ok(()),
            (Ty::Void, Ty::Void) => Ok(()),
            
            // Type variable unification
            (Ty::Var(v1), Ty::Var(v2)) if v1.id == v2.id => Ok(()),
            (Ty::Var(v), ty) | (ty, Ty::Var(v)) => {
                self.bind_type_var(v.id, ty)
            }
            
            // Array unification
            (Ty::Array(e1, n1), Ty::Array(e2, n2)) if n1 == n2 => {
                self.unify(&e1, &e2)
            }
            
            // Slice unification
            (Ty::Slice(e1), Ty::Slice(e2)) => {
                self.unify(&e1, &e2)
            }
            
            // Tuple unification
            (Ty::Tuple(elems1), Ty::Tuple(elems2)) if elems1.len() == elems2.len() => {
                for (e1, e2) in elems1.iter().zip(elems2.iter()) {
                    self.unify(e1, e2)?;
                }
                Ok(())
            }
            
            // Reference unification
            (Ty::Ref(r1, t1, m1), Ty::Ref(r2, t2, m2)) if m1 == m2 => {
                self.unify_regions(&r1, &r2)?;
                self.unify(&t1, &t2)
            }
            
            // Optional unification
            (Ty::Optional(t1), Ty::Optional(t2)) => {
                self.unify(&t1, &t2)
            }
            
            // Function unification
            (Ty::Function(f1), Ty::Function(f2)) => {
                self.unify_functions(&f1, &f2)
            }
            
            // Named type unification
            (Ty::Named(n1), Ty::Named(n2)) if n1.def_id == n2.def_id => {
                if n1.args.len() != n2.args.len() {
                    return Err(TypeError::arg_count_mismatch(n1.args.len(), n2.args.len()));
                }
                for (a1, a2) in n1.args.iter().zip(n2.args.iter()) {
                    self.unify(a1, a2)?;
                }
                Ok(())
            }
            
            // Error type
            (Ty::Error, _) | (_, Ty::Error) => Ok(()),
            
            // Type mismatch
            (expected, actual) => {
                Err(TypeError::mismatch(expected, actual))
            }
        }
    }
    
    /// Bind a type variable to a type
    fn bind_type_var(&mut self, var: TypeVarId, ty: Ty) -> Result!() {
        // Check for occurs (infinite types)
        if self.occurs_check(var, &ty) {
            return Err(TypeError::infinite_type(var, ty));
        }
        
        self.unification_table.insert(var, ty);
        Ok(())
    }
    
    /// Occurs check - prevent infinite types
    fn occurs_check(&self, var: TypeVarId, ty: &Ty) -> bool {
        match ty {
            Ty::Var(v) if v.id == var => true,
            Ty::Array(elem, _) => self.occurs_check(var, elem),
            Ty::Slice(elem) => self.occurs_check(var, elem),
            Ty::Tuple(elems) => elems.iter().any(|e| self.occurs_check(var, e)),
            Ty::Ref(_, elem, _) => self.occurs_check(var, elem),
            Ty::Optional(elem) => self.occurs_check(var, elem),
            Ty::Function(func) => {
                func.params.iter().any(|p| self.occurs_check(var, p)) ||
                self.occurs_check(var, &func.return_type)
            }
            Ty::Named(named) => named.args.iter().any(|a| self.occurs_check(var, a)),
            _ => false,
        }
    }
    
    /// Resolve a type to its canonical form
    pub fn resolve(&self, ty: &Ty) -> Ty {
        match ty {
            Ty::Var(var) => {
                match self.unification_table.get(var.id) {
                    Some(resolved) => self.resolve(resolved),
                    None => ty.clone(),
                }
            }
            _ => ty.clone(),
        }
    }
    
    /// Finalize all type variables
    pub fn finalize(&mut self) -> Result!HashMap<TypeVarId, Ty> {
        let mut result = HashMap::new();
        
        for (var, ty) in self.unification_table.iter() {
            result.insert(var, self.resolve(ty));
        }
        
        Ok(result)
    }
}

/// Constraints for trait bounds
pub enum Constraint {
    /// Type must implement trait
    TraitBound(Ty, TraitBound),
    /// Types must be equal
    TypeEqual(Ty, Ty),
    /// Type must be sized
    Sized(Ty),
    /// Type must be copy
    Copy(Ty),
}

/// Region constraints
pub enum RegionConstraint {
    /// Region outlives another
    Outlives(Region, Region),
    /// Region is longer than given bound
    LongerThan(Region, usize),
}
```

## 3.3 Type Checker Implementation

```axiom
// src/compiler/typeck/typeck.ax

/// Main type checker
pub struct TypeChecker {
    /// Type inference engine
    inference: TypeInference,
    
    /// Symbol table
    symbols: SymbolTable,
    
    /// Current function return type
    return_type: Option<Ty>,
    
    /// Diagnostic messages
    diagnostics: Vec<Diagnostic>,
    
    /// Trait implementations cache
    impl_cache: ImplCache,
}

impl TypeChecker {
    /// Type check a module
    pub fn check_module(&mut self, module: &Module) -> Result!() {
        // First pass: collect all declarations
        for decl in &module.declarations {
            self.collect_declaration(decl)?;
        }
        
        // Second pass: check all declarations
        for decl in &module.declarations {
            self.check_declaration(decl)?;
        }
        
        Ok(())
    }
    
    /// Collect declaration signatures
    fn collect_declaration(&mut self, decl: &Declaration) -> Result!() {
        match decl {
            Declaration::Function(func) => {
                let signature = self.make_function_signature(func)?;
                self.symbols.insert_function(func.name.name.clone(), signature);
            }
            Declaration::Struct(s) => {
                self.symbols.insert_struct(s.name.name.clone(), s);
            }
            Declaration::Enum(e) => {
                self.symbols.insert_enum(e.name.name.clone(), e);
            }
            Declaration::Trait(t) => {
                self.symbols.insert_trait(t.name.name.clone(), t);
            }
            _ => {}
        }
        Ok(())
    }
    
    /// Check a declaration
    fn check_declaration(&mut self, decl: &Declaration) -> Result!() {
        match decl {
            Declaration::Function(func) => self.check_function(func),
            Declaration::Struct(s) => self.check_struct(s),
            Declaration::Enum(e) => self.check_enum(e),
            Declaration::Trait(t) => self.check_trait(t),
            Declaration::Impl(i) => self.check_impl(i),
            Declaration::Const(c) => self.check_const(c),
            Declaration::Static(s) => self.check_static(s),
            _ => Ok(()),
        }
    }
    
    /// Type check a function
    fn check_function(&mut self, func: &FunctionDecl) -> Result!() {
        // Create new scope
        self.symbols.push_scope();
        
        // Add type parameters
        for param in &func.type_params {
            let ty = Ty::Param(param.id);
            self.symbols.insert_type_param(param.name.name.clone(), ty);
        }
        
        // Add parameters to scope
        for param in &func.params {
            let ty = self.convert_type(&param.type_)?;
            self.symbols.insert_var(param.name.name.clone(), ty);
        }
        
        // Set return type
        let return_type = match &func.return_type {
            Some(t) => self.convert_type(t)?,
            None => Ty::Void,
        };
        self.return_type = Some(return_type.clone());
        
        // Check body
        if let Some(body) = &func.body {
            let body_type = self.check_block(body)?;
            
            // Verify return type matches
            self.inference.unify(&return_type, &body_type)?;
        }
        
        self.symbols.pop_scope();
        Ok(())
    }
    
    /// Type check an expression
    pub fn check_expr(&mut self, expr: &Expr, expected: &Ty) -> Result!Ty {
        match expr {
            Expr::Literal(lit, _) => self.check_literal(lit, expected),
            
            Expr::Ident(ident) => self.check_ident(ident, expected),
            
            Expr::Binary(binary) => self.check_binary(binary, expected),
            
            Expr::Unary(unary) => self.check_unary(unary, expected),
            
            Expr::Call(call) => self.check_call(call, expected),
            
            Expr::Field(field) => self.check_field(field, expected),
            
            Expr::Index(index) => self.check_index(index, expected),
            
            Expr::If(if_expr) => self.check_if(if_expr, expected),
            
            Expr::Match(match_expr) => self.check_match(match_expr, expected),
            
            Expr::Block(block) => self.check_block(block),
            
            Expr::Reference(ref_expr) => self.check_reference(ref_expr, expected),
            
            Expr::Lambda(lambda) => self.check_lambda(lambda, expected),
            
            Expr::Struct(struct_expr) => self.check_struct_expr(struct_expr, expected),
            
            Expr::Array(arr) => self.check_array(arr, expected),
            
            Expr::Tuple(tuple) => self.check_tuple(tuple, expected),
            
            Expr::Range(range) => self.check_range(range),
            
            Expr::Error(span, msg) => {
                self.diagnostics.push(Diagnostic::error(span.clone(), msg.clone()));
                Ok(Ty::Error)
            }
            
            _ => Ok(Ty::Error),
        }
    }
    
    /// Infer type of expression
    pub fn infer_expr(&mut self, expr: &Expr) -> Result!Ty {
        let ty = match expr {
            Expr::Literal(lit, _) => self.infer_literal(lit),
            
            Expr::Ident(ident) => self.infer_ident(ident),
            
            Expr::Binary(binary) => self.infer_binary(binary),
            
            Expr::Unary(unary) => self.infer_unary(unary),
            
            Expr::Call(call) => self.infer_call(call),
            
            Expr::Field(field) => self.infer_field(field),
            
            Expr::Index(index) => self.infer_index(index),
            
            Expr::If(if_expr) => self.infer_if(if_expr),
            
            Expr::Match(match_expr) => self.infer_match(match_expr),
            
            Expr::Block(block) => self.check_block(block),
            
            Expr::Reference(ref_expr) => self.infer_reference(ref_expr),
            
            Expr::Lambda(lambda) => self.infer_lambda(lambda),
            
            Expr::Struct(struct_expr) => self.infer_struct_expr(struct_expr),
            
            Expr::Array(arr) => self.infer_array(arr),
            
            Expr::Tuple(tuple) => self.infer_tuple(tuple),
            
            Expr::Range(range) => self.infer_range(range),
            
            _ => Ty::Error,
        };
        
        Ok(ty)
    }
    
    /// Check literal type
    fn check_literal(&mut self, lit: &Literal, expected: &Ty) -> Result!Ty {
        match lit {
            Literal::Int(value, suffix) => {
                let ty = match suffix {
                    Some(IntSuffix::I8) => Ty::Int(IntTy::I8),
                    Some(IntSuffix::I16) => Ty::Int(IntTy::I16),
                    Some(IntSuffix::I32) => Ty::Int(IntTy::I32),
                    Some(IntSuffix::I64) => Ty::Int(IntTy::I64),
                    Some(IntSuffix::I128) => Ty::Int(IntTy::I128),
                    Some(IntSuffix::ISize) => Ty::Int(IntTy::ISize),
                    Some(IntSuffix::U8) => Ty::Int(IntTy::U8),
                    Some(IntSuffix::U16) => Ty::Int(IntTy::U16),
                    Some(IntSuffix::U32) => Ty::Int(IntTy::U32),
                    Some(IntSuffix::U64) => Ty::Int(IntTy::U64),
                    Some(IntSuffix::U128) => Ty::Int(IntTy::U128),
                    Some(IntSuffix::USize) => Ty::Int(IntTy::USize),
                    None => {
                        // Infer from expected type
                        match expected {
                            Ty::Int(_) => expected.clone(),
                            _ => Ty::Int(IntTy::I32), // Default
                        }
                    }
                };
                Ok(ty)
            }
            Literal::Float(value, suffix) => {
                let ty = match suffix {
                    Some(FloatSuffix::F32) => Ty::Float(FloatTy::F32),
                    Some(FloatSuffix::F64) => Ty::Float(FloatTy::F64),
                    None => {
                        match expected {
                            Ty::Float(_) => expected.clone(),
                            _ => Ty::Float(FloatTy::F64),
                        }
                    }
                };
                Ok(ty)
            }
            Literal::String(_) => Ok(Ty::String),
            Literal::Char(_) => Ok(Ty::Char),
            Literal::Bool(_) => Ok(Ty::Bool),
            Literal::Null => {
                // Null must have expected optional type
                match expected {
                    Ty::Optional(_) => Ok(expected.clone()),
                    _ => Err(TypeError::null_without_expected()),
                }
            }
        }
    }
    
    /// Check binary expression
    fn check_binary(&mut self, binary: &BinaryExpr, expected: &Ty) -> Result!Ty {
        let left_type = self.infer_expr(&binary.left)?;
        let right_type = self.infer_expr(&binary.right)?;
        
        let result_type = match binary.op {
            // Arithmetic
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | 
            BinaryOp::Div | BinaryOp::Mod | BinaryOp::Pow | BinaryOp::IntDiv => {
                self.inference.unify(&left_type, &right_type)?;
                self.inference.unify(&left_type, expected)?;
                left_type
            }
            
            // Comparison
            BinaryOp::Eq | BinaryOp::Ne | BinaryOp::Lt | BinaryOp::Gt | 
            BinaryOp::Le | BinaryOp::Ge | BinaryOp::Spaceship => {
                self.inference.unify(&left_type, &right_type)?;
                Ty::Bool
            }
            
            // Logical
            BinaryOp::And | BinaryOp::Or => {
                self.inference.unify(&left_type, &Ty::Bool)?;
                self.inference.unify(&right_type, &Ty::Bool)?;
                Ty::Bool
            }
            
            // Bitwise
            BinaryOp::BitAnd | BinaryOp::BitOr | BinaryOp::BitXor => {
                self.inference.unify(&left_type, &right_type)?;
                left_type
            }
            
            BinaryOp::Shl | BinaryOp::Shr | BinaryOp::UShr => {
                // Left must be integer, right must be integer
                self.expect_integer(&left_type)?;
                self.expect_integer(&right_type)?;
                left_type
            }
        };
        
        Ok(result_type)
    }
    
    /// Check function call
    fn check_call(&mut self, call: &CallExpr, expected: &Ty) -> Result!Ty {
        let func_type = self.infer_expr(&call.func)?;
        
        match func_type {
            Ty::Function(func_ty) => {
                // Check argument count
                if call.args.len() != func_ty.params.len() {
                    return Err(TypeError::arg_count_mismatch(
                        func_ty.params.len(),
                        call.args.len()
                    ));
                }
                
                // Check argument types
                for (arg, param_ty) in call.args.iter().zip(func_ty.params.iter()) {
                    self.check_expr(arg, param_ty)?;
                }
                
                // Unify return type with expected
                self.inference.unify(&func_ty.return_type, expected)?;
                
                Ok(*func_ty.return_type)
            }
            _ => Err(TypeError::not_callable(func_type)),
        }
    }
}
```

---

This is Part 1 of the technical documentation. Let me continue with Part 2 covering the Borrow Checker, IR Generation, Optimization, and Binary Generation...
