use std::{
    collections::HashMap,
    io::{self, Write},
};

use hsh::{
    hsc::Hsc::Commands,
    variables::{self, Variables},
};

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

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    loop {
        print!("hsh > ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;

        let inp = input.trim();
        if inp.is_empty() {
            continue;
        }

        let parts: Vec<&str> = inp.split_whitespace().collect();

        let (command_str, args) = match parts.split_first() {
            Some((cmd, args)) => (*cmd, args),
            None => continue,
        };

        let resolved_args = args
            .iter()
            .map(|s| resolve_variable(s, &Variables::get_all_vars()))
            .collect();

        
        let command = if let Ok(cmd) = command_str.parse::<Commands>() {
            cmd
        } else {
            eprintln!("Invalid command!");
            continue;
        };

        Commands::commnd(command, resolved_args);
    }
}
