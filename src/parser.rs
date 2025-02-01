use std::{any::TypeId, mem::transmute};
use crate::tokens::*;

macro_rules! match_token {
    ($tokens:expr, $index:expr, $pattern:pat => $var:ident) => {
        if let $pattern = &$tokens[$index] {
            $var
        } else {
            panic!(
                "Error at index {}: Expected {}",
                $index,
                stringify!($pattern)
            );
        }
    };

    ($tokens:expr, $index:expr, $pattern:pat) => {
        if let $pattern = &$tokens[$index] {
        } else {
            panic!(
                "Error at index {}: Expected {}",
                $index,
                stringify!($pattern)
            );
        }
    };
}

fn switch_type_to(data: ParsedType, _type: Type) -> ParsedType {
    let bytes = match data {
        ParsedType::Int(x) => x.to_le_bytes(),
        ParsedType::Float(x) => x.to_le_bytes(),
    };

    match _type {
        Type::Int => ParsedType::Int(i32::from_le_bytes(bytes)),
        Type::Float => ParsedType::Float(f32::from_le_bytes(bytes)),
    }
}

fn handle_moment<F: FnOnce()>(function: F, moment: Moment) {
    match moment {
        Moment::Instant => function(),
        Moment::Later => todo!(),
    }
}

fn get_register_index(register: &Register) -> usize {
    return unsafe { std::mem::transmute::<_, u32>(*register) } as usize;
}

pub fn parse_and_execute(tokens: Vec<TokenType>) {
    let mut index = 0;
    let mut registers = [ParsedType::Float(0.0f32); 5];

    loop {
        let current = &tokens[index];

        match current {
            // let _ be _ _;
            TokenType::Assignment => {
                let register = match_token!(tokens, index + 1, TokenType::Register(register) => register);
                let _type = match_token!(tokens, index + 3, TokenType::Type(_type) => _type);
                let moment = match_token!(tokens, index + 4, TokenType::Moment(moment) => moment);
                match_token!(tokens, index + 2, TokenType::Slop(Slop::Be));

                // I love unsafe maxxing... (kill me)
                handle_moment(|| {
                    let register_index = get_register_index(register);
                    registers[register_index] = switch_type_to(registers[register_index], *_type);
                }, *moment);                

                index += 5;
            },

            // chat, _ to _ _;
            TokenType::Io => {
                match_token!(tokens, index + 1, TokenType::Special(Special::Comma));
                match_token!(tokens, index + 3, TokenType::Slop(Slop::To));
                let direction = match_token!(tokens, index + 2, TokenType::IoDirection(dir) => dir);
                let moment = match_token!(tokens, index + 5, TokenType::Moment(moment) => moment);
                let register = match_token!(tokens, index + 4, TokenType::Register(register) => register);

                handle_moment(|| {
                    let register_index = get_register_index(register);
                    match direction {
                        IoDirection::Listen => {
                            match registers[register_index] {
                                ParsedType::Int(val) => println!("{val}"),
                                ParsedType::Float(val) => println!("{val}"),
                            }
                        },
                        IoDirection::Talk => {
                            let input = std::io::stdin().lines().next().unwrap().unwrap();
                            let float = input.parse::<f32>().unwrap_or_default();
                            let int = input.parse::<i32>().unwrap_or_default();

                            match &mut registers[register_index] {
                                ParsedType::Int(int_ref) => *int_ref = int,
                                ParsedType::Float(float_ref) => *float_ref = float,
                            }
                        },
                    }
                    
                }, *moment);   


                index += 6;
            }

            TokenType::Marker(Marker::LineEnd) | TokenType::Marker(Marker::LineStart) | TokenType::Comment => index += 1,
            TokenType::Marker(Marker::FileEnd) => break,
            _ => panic!("{:?}", current),
        }
    }
}