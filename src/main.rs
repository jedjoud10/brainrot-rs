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

    for line in lines {
        let words = line.split_whitespace().collect::<Vec<&str>>();

        if words.len() == 0 {
            continue;
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

        // IO OUTPUT
        if line.ends_with("in the chat") {
            match words[0] {
                "ws" => println!("{}", last_success),
                "ls" => println!("{}", last_failure),
                _ => {}
            }
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
                _ => { quit!("Not a fucking rizz option bruh!!!"); 0 },
            };

            let out = match &mut you {
                Register::Real(x) => { *x += offset as f32; format!("{x}") },
                Register::Int(x) => { *x += offset; format!("{x}") },
            };

            let input = format!("Changed value of \"you\" register to {}", out);
            out!(&input);
        }
    }
}
