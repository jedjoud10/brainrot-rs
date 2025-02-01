use std::{any::TypeId, mem::transmute};
use crate::tokens::*;

macro_rules! read {
    ($tokens:expr, $index:expr) => {
        if index >= $tokens.len() {
            eprintln!(
                "Missing token at index {}",
                $index
            );
            return Err(());
        } 
    };
}

macro_rules! smart_expected {
    ($pattern:pat) => {
        {
            stringify!($pattern)
            /*
            let next = stringify!($pattern).split("::").next().unwrap();
            next
            */
        }
        //<$enum>::iter().map(|v| format!("{:?}", v)).collect()
    };
}

macro_rules! match_token {
    ($tokens:expr, $index:expr, $pattern:pat => $var:ident) => {
        {
            if $index >= $tokens.len() {
                eprintln!(
                    "Missing token at index {}: Expected {}",
                    $index,
                    smart_expected!($pattern)
                );
                return Err(());
            };
    
            if let $pattern = &$tokens[$index] {
                $var
            } else {
                eprintln!(
                    "Error at index {}: Expected {}",
                    $index,
                    smart_expected!($pattern)
                );
                return Err(());
            }
        }
    };

    ($tokens:expr, $index:expr, $pattern:pat) => {
        {
            if $index >= $tokens.len() {
                eprintln!(
                    "Missing token at index {}: Expected {}",
                    $index,
                    smart_expected!($pattern)
                );
                return Err(());
            };

            if let $pattern = &$tokens[$index] {
            } else {
                eprintln!(
                    "Error at index {}: Expected {}",
                    $index,
                    smart_expected!($pattern)
                );
                return Err(());
            }
        }
    };
}

fn switch_type_to(data: ParsedType, _type: Type) -> ParsedType {
    let floating = match data {
        ParsedType::Int(x) => x as f32,
        ParsedType::Float(x) => x,
    };

    match _type {
        Type::Int => ParsedType::Int(floating.round() as i32),
        Type::Float => ParsedType::Float(floating),
    }
}

fn compare(src: ParsedType, other: ParsedType) -> bool {
    let floating = match src {
        ParsedType::Int(x) => x as f32,
        ParsedType::Float(x) => x,
    };

    let floating2 = match other {
        ParsedType::Int(x) => x as f32,
        ParsedType::Float(x) => x,
    }; 

    floating > floating2
}

fn handle_moment<F: FnOnce()>(function: F, moment: Moment) {
    match moment {
        Moment::Instant => function(),
        Moment::Later => todo!(),
    }
}

fn get_register_index(register: &Register) -> usize {
                    // I love unsafe maxxing... (kill me)
    return unsafe { std::mem::transmute::<_, u32>(*register) } as usize;
}

fn get_register_by_index(index: usize) -> Register {
    return unsafe { std::mem::transmute::<u32, Register>(index as u32) };
}

fn get_bit_register_name_by_index(index: usize) -> &'static str {
    ["g","ja","god","ohio","gyats","rizzer","skibidi","fanumtax"][index]
}

#[derive(Default)]
pub struct World {
    pub registers: [ParsedType; 5],
    pub bitwise: u8,
}

pub fn parse_and_execute(tokens: Vec<TokenType>, world: &mut World) -> Result<(), ()> {
    let mut index = 0;
    let World {
        registers,
        bitwise,
    } = world;

    loop {
        let current = &tokens[index];

        match current {
            // let _ be _ rn;
            TokenType::Assignment => {
                let register = match_token!(tokens, index + 1, TokenType::Register(register) => register);
                match_token!(tokens, index + 2, TokenType::Slop(Slop::Be));
                let _type = match_token!(tokens, index + 3, TokenType::Type(_type) => _type);
                match_token!(tokens, index + 4, TokenType::Moment(Moment::Instant));

                let register_index = get_register_index(register);
                registers[register_index] = switch_type_to(registers[register_index], *_type);                

                index += 5;
            },

            // chat, _ to _ rn;
            TokenType::Io => {
                match_token!(tokens, index + 1, TokenType::Special(Special::Comma));
                let direction = match_token!(tokens, index + 2, TokenType::IoDirection(dir) => dir);
                match_token!(tokens, index + 3, TokenType::Slop(Slop::To));
                let register = match_token!(tokens, index + 4, TokenType::Register(register) => register);
                match_token!(tokens, index + 5, TokenType::Moment(Moment::Instant));

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

                        dbg!(float);
                        dbg!(int);
                    },
                } 


                index += 6;
            }

            // Check:
            // is _ _ rn? (ex: is blud inting rn?)
            // is _ mogging _ rn? (ex: is blud mogging cro rn?)
            // Storage:
            // say on _.
            TokenType::Conditional => {
                let src = match_token!(tokens, index + 1, TokenType::Register(register) => register);

                // Check if this is a type check or value check
                let result;
                let next_index_offset;
                if matches!(tokens[index + 2], TokenType::LessThanEqualCheck) {
                    let src1 = match_token!(tokens, index + 3, TokenType::Register(register) => register);
                    match_token!(tokens, index + 4, TokenType::Moment(Moment::Instant));
                    match_token!(tokens, index + 5, TokenType::Special(Special::Question));

                    // value check, cmp
                    let first = &registers[get_register_index(src)];
                    let second = &registers[get_register_index(src1)];
                    result = compare(*first, *second);
                    next_index_offset = 6;
                } else {
                    let _type = match_token!(tokens, index + 2, TokenType::Type(_type) => _type);     
                    match_token!(tokens, index + 3, TokenType::Moment(Moment::Instant));
                    match_token!(tokens, index + 4, TokenType::Special(Special::Question));    

                    // type check, cmp   
                    result = match (_type, registers[get_register_index(src)]) {
                        (Type::Float, ParsedType::Float(_)) | (Type::Int, ParsedType::Int(_)) => true,
                        _ => false,
                    };
                    next_index_offset = 5;
                }

                // Write to storage frfr
                match_token!(tokens, index + next_index_offset, TokenType::WriteBitwise);
                match_token!(tokens, index + next_index_offset + 1, TokenType::Slop(Slop::On));
                let bit_index = match_token!(tokens, index + next_index_offset + 2, TokenType::BitwiseRegister(bit_register) => bit_register);
                
                // We do writing frfrfrfr
                if result {
                    *bitwise |= 1 << bit_index;
                } else {
                    *bitwise &= !(1 << bit_index);
                }

                index += next_index_offset + 3;
            }
            

            TokenType::Marker(Marker::LineEnd) | TokenType::Marker(Marker::LineStart) | TokenType::Comment => index += 1,
            TokenType::Debug => {
                println!("\n");
                println!("============================");
                println!("Registers: ");
                for (index, register) in registers.iter().enumerate() {
                    let name = format!("{:?}", get_register_by_index(index));
                    println!("  {name} [{index}]: {:?}", register);
                }
                
                println!("Bit Register: ");
                for i in 0..8 {
                    let name = get_bit_register_name_by_index(i);
                    let bit = (*bitwise >> i) & 1;
                    println!("  {name} [{i}]: {bit}");
                }
                
                println!("============================");
                println!("\n");

                index += 1
            },
            TokenType::Marker(Marker::FileEnd) => break,
            _ => {
                return Err(());
            },
        }
    }

    Ok(())
}