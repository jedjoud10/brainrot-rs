use regex::Regex;
use crate::tokens::*;

pub fn tokenize_raw(raw: String) -> Result<Vec<TokenType>, ()> {
    // Thanks ChatGPT my beloved <333
    let re = Regex::new(r"(\w+|[^\w\s])").unwrap();
    let mut tokens = Vec::<TokenType>::new();
    let mut extra = Vec::<TokenExtra>::new();
    
    let mut line_index = 0;
    for line in raw.lines() {
        tokens.push(TokenType::Marker(Marker::LineStart));
        for word in re.find_iter(line).map(|m| m.as_str())  {
            let _type = match word {
                "let" => TokenType::Assignment,

                // Registers...
                "zawg" => TokenType::Register(Register::Zawg),
                "blud" => TokenType::Register(Register::Blud),
                "vro" => TokenType::Register(Register::Vro),
                "cro" => TokenType::Register(Register::Cro),
                "zro" => TokenType::Register(Register::Zro),
                "tro" => TokenType::Register(Register::Tro),
                
                // Conditional...
                "is" => TokenType::Conditional,
                "mogging" => TokenType::LessThanEqualCheck,
                "say" => TokenType::WriteBitwise,
                "g" => TokenType::BitwiseRegister(0),
                "ja" => TokenType::BitwiseRegister(1),
                "god" => TokenType::BitwiseRegister(2),
                "ohio" => TokenType::BitwiseRegister(3),
                "gyats" => TokenType::BitwiseRegister(4),
                "rizzer" => TokenType::BitwiseRegister(5),
                "skibidi" => TokenType::BitwiseRegister(6),
                "fanumtax" => TokenType::BitwiseRegister(7),

                // Slop-maxination...
                "be" => TokenType::Slop(Slop::Be),
                "this" => TokenType::Slop(Slop::This),
                "to" => TokenType::Slop(Slop::To),
                "on" => TokenType::Slop(Slop::On),

                // Types...
                "fr" => TokenType::Type(Type::Float),
                "inting" => TokenType::Type(Type::Int),

                // Chat input...
                "chat" => TokenType::Io,
                "listen" => TokenType::IoDirection(IoDirection::Listen),
                "talk" => TokenType::IoDirection(IoDirection::Talk),

                // Special...
                "," => TokenType::Special(Special::Comma),
                ":" => TokenType::Special(Special::Colon),
                "?" => TokenType::Special(Special::Question),
                
                // TODO: idk how to do this with if let or better pattern matching
                _ if word.parse::<u32>().is_ok() => TokenType::Parsed(ParsedType::Int(word.parse::<i32>().unwrap())),
                _ if word.parse::<f32>().is_ok() => TokenType::Parsed(ParsedType::Float(word.parse::<f32>().unwrap())),

                // Comments
                "#" => { 
                    tokens.push(TokenType::Comment);
                    extra.push(TokenExtra { line: line_index });
                    break;
                },

                // Debug...
                "debugginate" => TokenType::Debug,

                // Moments...
                "rn" => TokenType::Moment(Moment::Instant),
                "later" => TokenType::Moment(Moment::Later),

                _ => {
                    eprintln!("Reached unrecognized token \"{word}\"");
                    TokenType::Unrecognized
                },
            };
            tokens.push(_type);
            extra.push(TokenExtra { line: line_index });
        }
        tokens.push(TokenType::Marker(Marker::LineEnd));
        extra.push(TokenExtra { line: line_index });
        line_index += 1;
    }
    
    tokens.push(TokenType::Marker(Marker::FileEnd));
    extra.push(TokenExtra { line: line_index });
    return Ok(tokens);
}