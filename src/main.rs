pub mod hsc;
pub mod variables;
use hsc::Hsc::Commands;
use std::{
    collections::HashMap,
    io::{self, Write},
};

fn main() {
    loop {
        print!("hsh > ");

        let _ = io::stdout().flush();
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Can not read line ...");
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let mut parts = input.split_whitespace();
        let command = parts.next().unwrap();
        let args: Vec<String> = parts.map(|s| resolve_variable(s, &variables::Variables::get_all_vars())).collect();

        if let Err(e) = command.parse::<Commands>() {
            eprintln!("{:?}", e);
            continue;
        }

        let command = command.parse::<Commands>().unwrap();
        Commands::commnd(command, args);
        // match command {
        //     "cd" => {
        //         let new_dir = args.first().map(|s| s.as_str()).unwrap_or("/");
        //         let root = Path::new(new_dir);

        //         if args.len() != 1 {
        //             println!("Usage cd <directory>");
        //         } else if args.len() > 1 {
        //             println!("It does not take more than one argument.")
        //         } else {
        //             if let Err(e) = env::set_current_dir(root) {
        //                 eprintln!("cd: {}", e);
        //             }
        //         }
        //     }

        //     "ls" => {
        //         let _ = ls_dir();
        //     }

        //     "set" => {
        //         if args.is_empty() {
        //             if vars.is_empty() {
        //                 println!("Not variables set.");
        //             } else {
        //                 for (key, value) in &vars {
        //                     println!("{}={}", key, value);
        //                 }
        //             }
        //         } else {
        //             for arg in args {
        //                 if let Some(eq_index) = arg.find('=') {
        //                     let key = &arg[..eq_index];
        //                     let value = &arg[eq_index + 1..];
        //                     vars.insert(key.to_string(), value.to_string());
        //                 } else {
        //                     println!("Invalid format: expected key=value")
        //                 }
        //             }
        //         }
        //     }

        //     "pwd" => match env::current_dir() {
        //         Ok(path) => println!("{}", path.display()),
        //         Err(e) => println!("Error getting current directory: {}", e),
        //     },

        //     "export" => {
        //         for arg in args {
        //             if let Some(eq_index) = arg.find('=') {
        //                 let key = &arg[..eq_index];
        //                 let value = &arg[eq_index + 1..];
        //                 vars.insert(key.to_string(), value.to_string());
        //                 unsafe { std::env::set_var(key, value) };
        //             } else {
        //                 println!("Invalid format in export: expected key=value got '{}'", arg);
        //             }
        //         }
        //     }

        //     "unset" => {
        //         for key in &args {
        //             vars.remove(key);
        //             unsafe { std::env::remove_var(key) };
        //         }
        //     }

        //     "env" => {
        //         for (key, value) in std::env::vars() {
        //             println!("{}={}", key, value);
        //         }
        //     }

        //     "clear" => {
        //         let _ = clear_screen();
        //     }

        //     "read" => {
        //         if args.len() != 1 {
        //             println!("Usage read <filename>");
        //         } else {
        //             let _ = read_command(args.first().unwrap());
        //         }
        //     }

        //     "exit" => {
        //         break;
        //     }

        //     "touch" => {
        //         let _ = touch_command(args.first().unwrap());
        //     }

        //     "echo" => {
        //         println!("{}", args.join(" "));
        //     }

        //     _ => {
        //         println!("Not Found!");
        //     }
    }
}




fn resolve_variable(input: &str, vars: &HashMap<String, String>) -> String {
    let mut result = String::new();
    let mut chars = input.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            let mut var_name = String::new();

            while let Some(&next_c) = chars.peek() {
                if next_c.is_alphanumeric() || next_c == '_' {
                    var_name.push(next_c);
                    chars.next();
                } else {
                    break;
                }
            }

            if !var_name.is_empty() {
                if let Some(value) = vars.get(&var_name) {
                    result.push_str(value);
                }
            } else {
                result.push('$');
            }
        } else {
            result.push(c);
        }
    }
    result
}
