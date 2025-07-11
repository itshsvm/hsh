// Hsh Core ...
pub mod Hsc {

    use crate::variables::{self, Variables};
    use std::{
        collections::{HashMap, HashSet},
        env, fmt,
        fs::{self, File},
        io::{self, ErrorKind, Read},
        path::Path,
        str,
    };

    use libc::exit;

    #[derive(Debug)]
    pub enum Commands {
        Ls,
        Cd,
        Clear,
        Pwd,
        Echo,
        Set,
        Unset,
        Export,
        Read,
        Exit,
        Touch,
        Env,
        RmDir,
        Rm,
    }

    #[derive(Debug, Clone)]
    pub struct CommandFlag {
        flags: HashSet<char>,
    }

    impl CommandFlag {
        pub fn from_arg(args: &[String]) -> (CommandFlag, Vec<String>) {
            let mut flags = HashSet::new();
            let mut remaining_args = Vec::new();

            for arg in args {
                if arg.starts_with('-') && arg.len() > 1 {
                    for ch in arg.chars().skip(1) {
                        flags.insert(ch);
                    }
                } else {
                    remaining_args.push(arg.clone());
                }
            }

            (CommandFlag { flags }, remaining_args)
        }

        pub fn has(&self, flag: char) -> bool {
            self.flags.contains(&flag)
        }

        pub fn all(&self) -> &HashSet<char> {
            &self.flags
        }
    }

    #[derive(Debug)]
    pub enum CommandError {
        CmdErr(String),
    }

    impl fmt::Display for Commands {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::Ls => write!(f, "ls"),
                Self::Cd => write!(f, "cd"),
                Self::Clear => write!(f, "clear"),
                Self::Pwd => write!(f, "pwd"),
                Self::Echo => write!(f, "echo"),
                Self::Set => write!(f, "set"),
                Self::Unset => write!(f, "unset"),
                Self::Export => write!(f, "export"),
                Self::Read => write!(f, "read"),
                Self::Exit => write!(f, "exit"),
                Self::Touch => write!(f, "touch"),
                Self::Env => write!(f, "env"),
                Self::Rm => write!(f, "rm"),
                Self::RmDir => write!(f, "rmdir"),
            }
        }
    }

    impl str::FromStr for Commands {
        type Err = CommandError;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s {
                "ls" => Ok(Self::Ls),
                "cd" => Ok(Self::Cd),
                "clear" => Ok(Self::Clear),
                "exit" => Ok(Self::Exit),
                "set" => Ok(Self::Set),
                "unset" => Ok(Self::Unset),
                "export" => Ok(Self::Export),
                "echo" => Ok(Self::Echo),
                "pwd" => Ok(Self::Pwd),
                "read" => Ok(Self::Read),
                "touch" => Ok(Self::Touch),
                "env" => Ok(Self::Env),
                "rm" => Ok(Self::Rm),
                "rmdir" => Ok(Self::RmDir),

                e => Err(CommandError::CmdErr(e.to_string())),
            }
        }
    }

    impl Commands {
        pub fn commnd(input: Self, args: Vec<String>) {
            match input {
                Self::Env => {
                    for (key, value) in std::env::vars() {
                        println!("{}={}", key, value);
                    }
                }

                Self::Rm => {
                    let _ = ExecuteCommand::execute_rm(args);
                }
                Self::RmDir => {
                    let _ = ExecuteCommand::execute_rmdir(args);
                }

                Self::Ls => {
                    let _ = ExecuteCommand::execute_ls(args);
                }
                Self::Cd => {
                    let _ = ExecuteCommand::execute_cd(args);
                }
                Self::Clear => {
                    clearscreen::clear().unwrap();
                }
                Self::Pwd => match env::current_dir() {
                    Ok(dir) => println!("{}", dir.display()),
                    Err(e) => println!("Error getting current directory: {}", e),
                },
                Self::Echo => println!("{}", args.join(" ")),
                Self::Set => {
                    let _ = ExecuteCommand::execute_set(args);
                }
                Self::Unset => {
                    let _ = ExecuteCommand::execute_unset(args);
                }
                Self::Export => {
                    let _ = ExecuteCommand::execute_export(args);
                }
                Self::Read => {
                    if args.len() != 1 {
                        println!("Usage read <filename>");
                    } else {
                        let _ = ExecuteCommand::execute_read(args.first().unwrap());
                    }
                }
                Self::Exit => {
                    unsafe { exit(1) };
                }
                Self::Touch => {
                    if args.len() != 1 {
                        println!("Usage touch <filename>");
                    } else {
                        let _ = ExecuteCommand::execute_touch(args.first().unwrap());
                    }
                }
            }
        }
    }

    struct ExecuteCommand;

    impl ExecuteCommand {
        pub fn execute_ls(args: Vec<String>) -> io::Result<()> {
            let (flags, files) = CommandFlag::from_arg(&args);

            let show_all = flags.has('a');
            let long_format = flags.has('l');

            let mut list_dir = fs::read_dir(".")?
                .map(|e| e.map(|res| res.file_name()))
                .collect::<Result<Vec<_>, io::Error>>()?;

            if !show_all {
                list_dir.retain(|oss| oss.to_str().map_or(true, |s| !s.starts_with('.')));
            }

            list_dir.sort();

            if long_format {
                for name in &list_dir {
                    if let Some(name_str) = name.to_str() {
                        println!("{}", name_str);
                    }
                }
            } else {
                for (i, name) in list_dir.iter().enumerate() {
                    if let Some(name_str) = name.to_str() {
                        print!("{}", name_str);
                        if i < list_dir.len() - 1 {
                            print!("    ");
                        }
                    }
                }
                println!();
            }

            Ok(())
        }

        pub fn execute_rm(args: Vec<String>) -> io::Result<()> {
            let (flags, files) = CommandFlag::from_arg(&args);
            let force = flags.has('f');
            let verbose = flags.has('v');

            if files.is_empty() {
                if !force {
                    println!("rm: missing operand");
                }
                return Ok(());
            }

            for file in files {
                match fs::remove_file(&file) {
                    Ok(_) => {
                        if verbose {
                            println!("removed '{}'", file);
                        }
                    }
                    Err(e) => {
                        if !force {
                            eprintln!("rm: cannot remove '{}': {}", file, e);
                        }
                    }
                }
            }

            Ok(())
        }

        pub fn execute_rmdir(args: Vec<String>) -> io::Result<()> {
            let (flags, dirs) = CommandFlag::from_arg(&args);
            let verbose = flags.has('v');

            if dirs.is_empty() {
                println!("rmdir: missing operand");
                return Ok(());
            }

            for dir in dirs {
                match fs::remove_dir(&dir) {
                    Ok(_) => {
                        if verbose {
                            println!("removed directory '{}'", dir);
                        }
                    }
                    Err(e) => {
                        eprintln!("rmdir: failed to remove '{}': {}", dir, e);
                    }
                }
            }

            Ok(())
        }

        pub fn execute_read(filename: &str) -> io::Result<()> {
            let fopen = File::open(filename);

            let mut file = match fopen {
                Ok(content) => content,
                Err(e) => {
                    if e.kind() == ErrorKind::NotFound {
                        println!("oh dear file does not exists!");
                        return Err(e);
                    } else {
                        println!("An error occurred while opening the file: {:?}", e);
                        return Err(e); // Propagate other errors too
                    }
                }
            };
            let mut buf = String::new();
            let _ = file.read_to_string(&mut buf);
            println!("{}", buf);
            Ok(())
        }
        pub fn execute_touch(file_name: &str) -> io::Result<()> {
            let path = Path::new(file_name);

            fs::OpenOptions::new()
                .create(true)
                .append(true)
                .open(&path)?;

            Ok(())
        }
        pub fn execute_set(var: Vec<String>) {
            if var.is_empty() {
                let all = Variables::get_all_vars();
                if all.is_empty() {
                    println!("Not variables set.");
                } else {
                    for (key, value) in &all {
                        println!("{}={}", key, value);
                    }
                }
            } else {
                for arg in var {
                    if let Some(eq_index) = arg.find('=') {
                        let key = &arg[..eq_index];
                        let value = &arg[eq_index + 1..];
                        Variables::set_var(key, value);
                    } else {
                        println!("Invalid format: expected key=value")
                    }
                }
            }
        }

        pub fn execute_export(args: Vec<String>) {
            for arg in args {
                if let Some(eq_index) = arg.find('=') {
                    let key = &arg[..eq_index];
                    let value = &arg[eq_index + 1..];
                    Variables::export_var(key, value);
                } else {
                    println!("Invalid format in export: expected key=value got '{}'", arg);
                }
            }
        }

        pub fn execute_unset(args: Vec<String>) {
            for key in &args {
                Variables::remove_var(key);
                unsafe { std::env::remove_var(key) };
            }
        }

        pub fn execute_cd(args: Vec<String>) {
            let new_dir = args.first().map(|s| s.as_str()).unwrap_or("/");
            let root = Path::new(new_dir);

            if args.len() != 1 {
                println!("Usage cd <directory>");
            } else if args.len() > 1 {
                println!("It does not take more than one argument.")
            } else {
                if let Err(e) = env::set_current_dir(root) {
                    eprintln!("cd: {}", e);
                }
            }
        }
    }
}
