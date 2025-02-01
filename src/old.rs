use std::{collections::HashMap, time::Duration};

#[derive(Clone, Copy)]
enum Register {
    Real(f32),
    Int(i32),
}

fn main() {
    let file = include_str!("./test.rot");
    let lines = file.lines().collect::<Vec<&str>>();
    let mut me = Register::Real(0.0f32);
    let mut you = Register::Real(0.0f32);
    let mut last_success = String::new();
    let mut last_failure = String::new();
    let mut yapper = false;
    let mut yappings = String::new();
    let mut arrays = HashMap::<String, Vec<Register>>::default();

    macro_rules! quit {
        ($a:expr) => {
            last_failure = uwuifier::uwuify_str_sse($a)
        };
    }

    macro_rules! out {
        ($a:expr) => {
            last_success = uwuifier::uwuify_str_sse($a)
        };
    }

    let mut i = 0;
    let mut nuhuh = 696969;
    let mut ofcbruh = 696969;
    while i < lines.len() {
        let line = lines[i];
        i += 1;
        let words = line.split_whitespace().collect::<Vec<&str>>();

        if words.len() == 0  {
            continue;
        }

        if yapper && line != "mew" {
            yappings.push_str(line);
            }

        // REGISTER DEFINITION
        if words[0] == "let" {
            let reg = match words[1] {
                "you" => &mut you,
                "me" => &mut me,
                _ => { quit!("Failed to fetch register type. I don't know why!"); continue },
            };

            let bytes = match reg {
                Register::Real(x) => x.to_ne_bytes(),
                Register::Int(x) => x.to_ne_bytes(),
            };

            match words[3] {
                "fr" => *reg = Register::Real(f32::from_ne_bytes(bytes)),
                "inting" => *reg = Register::Int(i32::from_ne_bytes(bytes)),
                _ => quit!("Failed to fetch register target!"),
            };

            out!("Changed register type! I don't know which one but surely at least one!")
        }

        // IO RESULT OUTPUT
        if line.ends_with("in the chat") {
            match words[0] {
                "ws" => println!("{}", last_success),
                "ls" => println!("{}", last_failure),
                _ => {}
            }
        }

        // IO OUTPUT
        if line == "mew" {
            yapper = false;
            print!("{}", yappings);
            yappings.clear();
        } else if line == "yap" {
            yapper = true;
            continue;
        }

        // IO INPUT
        if line.starts_with("chat, is this") {
            println!("{}", line.trim_start_matches("chat, "));
            let input = std::io::stdin().lines().next().unwrap().unwrap();
            let float = input.parse::<f32>().unwrap_or_default();
            let int = input.parse::<i32>().unwrap_or_default();

            match words[3] {
                "real?" => {
                    if let Register::Real(x) = &mut you {
                        *x = float;
                    }

                    if let Register::Real(x) = &mut me {
                        *x = float;
                    }
                },
                "inting?" => {
                    if let Register::Int(x) = &mut you {
                        *x = int;
                    }

                    if let Register::Int(x) = &mut me {
                        *x = int;
                    }
                },
                _ => { quit!("Failed to fetch output register target"); continue }
            }
        }

        // MATH
        if line.starts_with("lemme rizz you") {
            let offset = match words[3] {
                "up" => 1,
                "down" => -1,
                "bruh" => 0,
                "silently" => 0,
                _ => { quit!("Not a fucking rizz option bruh!!!"); 0 },
            };

            let out = match &mut you {
                Register::Real(x) => { *x += offset as f32; format!("{x}") },
                Register::Int(x) => { *x += offset; format!("{x}") },
            };

            let input = format!("Changed value of \"you\" register to {}!!", out);
            out!(&input);
        }



        // CONDITIONAL MARKERS
        match line {
            "nuhuh" => nuhuh = i-1,
            "ofcbruh" => ofcbruh = i-1,
            _ => {}
        }

        // SLEEP
        if line.starts_with("eep for") {
            let eep = words[2].parse::<u64>().unwrap();
            let mul: u64 = match words[3] {
                "bazillions" => 10,
                "billions" => 100,
                "fucks" => 1000,
                _ => { quit!("Hang yourself you insignificant FUCK"); 0 },
            };

            std::thread::sleep(Duration::from_millis(eep * mul))
        }

        // VAL CHECKS
        if line.contains("mogging") {
            let a = words[1];
            let b = words[3];

            let cp = i;
            match (a, b) {
                ("you", "me") => {
                    match (&you, &me) {
                        (Register::Real(a_), Register::Real(b_)) if a_ > b_ => i = ofcbruh,
                        (Register::Int(a_), Register::Int(b_)) if a_ > b_ => i = ofcbruh,
                        _ => i = nuhuh
                    };
                },
                ("me", "you") => {
                    match (&you, &me) {
                        (Register::Real(a_), Register::Real(b_)) if a_ < b_ => i = ofcbruh,
                        (Register::Int(a_), Register::Int(b_)) if a_ < b_ => i = ofcbruh,
                        _ => i = nuhuh
                    };
                },
                _ => quit!("You should end yourself you fucking normie")
            }

            nuhuh = 696969;
            ofcbruh = 696969;
            if i == 696969 {
                i = cp
            }
        }

        // TYPE CHECKS
        if line.starts_with("is") && !line.contains("mogging") {
            let a = words[1];
            let b = words[2];

            if match (a, b) {
                ("me", "inting") => matches!(me, Register::Int(_)),
                ("me", "fr") => matches!(me, Register::Real(_)),
                ("you", "inting") => matches!(you, Register::Int(_)),
                ("you", "fr") => matches!(you, Register::Real(_)),
                _ => { quit!("Are you rizzing me up rn??!?!?! BAKA!!"); true }
            } {
                i = ofcbruh;
            } else {
                i = nuhuh;
            }
        }

        // ARRAY DEFINTIONS
        if line.starts_with("sussy") && words.len() == 5 {
            let a = words[1];
            let name = words[2];

            let register = match a {
                "inting" => Register::Int(0),
                "real" => Register::Real(0f32),
                _ => {
                    quit!("not a real sussny type timing!!");
                    continue;
                }
            };

            if arrays.contains_key(name)  || ["sussy", "real", "inting", "me", "i", "you"].contains(&name) {
                quit!("I already sussied you!!!");
                continue;
            } else {
                if name.len() > 5 {
                    quit!("Sorgy, but name size is too big bro!!!");
                    continue;
                } else {
                    arrays.insert(name.to_string(), vec![register; name.len()]);
                }
            }
        }

        // COPY DEFINITIONS
        if line.contains("bussed") && line.contains("to") && words.len() == 3 {
            let index = if words.len() == 6 {
                match words[4] {
                    "one" => 0,
                    "two" => 1,
                    "three" => 2,
                    "four" => 3,
                    "five" => 4,
                    _ => 420,
                }
            } else {
                420
            };

            let rizz = |register: &mut Register| {};

            let gyat = move |resolve: String, arrays: &mut HashMap<String, Vec<Register>>| {
                let a = {
                    if arrays.contains_key(&resolve) {
                        if index != 420 || index > resolve.len() {
                            quit!("I love giving error messages that do no help in the slightest bit");
                            return None;
                        } else {
                            let a = arrays.get_mut(&resolve);
                            let b = a.map(|x| x.get_mut(index));
                            return b.flatten();
                        }
                    } else if resolve == "me" {
                        //Some(&mut me)
                        None
                    } else if resolve == "you" {
                        //Some(&mut you)
                        None
                    } else {
                        None
                    }
                }
                
            }


            let first = words[0];
            let second = words[3];
        }
    }
}