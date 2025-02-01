use regex::Regex;
use crate::tokens::*;

pub fn tokenize_raw(raw: String) -> Vec<TokenType> {
    // Thanks ChatGPT my beloved <333
    let re = Regex::new(r"(\w+|[^\w\s])").unwrap();
    let mut tokens = Vec::<TokenType>::new();
    
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

                // Slop-maxination...
                "be" => TokenType::Slop(Slop::Be),
                "is" => TokenType::Slop(Slop::Is),
                "this" => TokenType::Slop(Slop::This),
                "to" => TokenType::Slop(Slop::To),

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

                "#" => { 
                    tokens.push(TokenType::Comment);
                    break;
                },

                // Moments...
                "rn" => TokenType::Moment(Moment::Instant),
                "later" => TokenType::Moment(Moment::Later),

                _ => panic!("Reached unrecognized token {word}"),
            };
            tokens.push(_type);
        }
        tokens.push(TokenType::Marker(Marker::LineEnd));
    }
    
    tokens.push(TokenType::Marker(Marker::FileEnd));
    return tokens;
}