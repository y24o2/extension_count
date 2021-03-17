use std::env;

pub struct Options{
    pub path:String,
    pub recursion:bool,
    pub min:u32,
    pub ranked:bool,
    pub ignore_case:bool,
}

impl Options{
    pub fn default() -> Options{
        Options{
            path: String::from("."),
            recursion: false,
            min: 1,
            ranked: false,
            ignore_case:false,
        }
    }

    pub fn from_args(args:env::Args) -> Options{
        let mut options = Options::default();
        for arg in args{
            if arg == "-h" || arg == "--help" {eprintln!("Usage ..."); std::process::exit(0)}
            else if arg.starts_with("-") && !arg.starts_with("--"){
                if arg.contains("i") || arg.contains("c") {options.ignore_case = true}
                if arg.contains("n") {options.ranked = true}
                if arg.contains("r") {options.recursion = true}
            }
            else if  arg == "--caseinsensitive" {options.ignore_case = true}
            else if arg == "--ranked" {options.ranked = true}
            else if arg == "--recursion" {options.recursion = true}
            else if arg.starts_with("--min="){options.min = arg[6..arg.len()].parse::<u32>().unwrap_or(1)}
            else if arg.starts_with("min="){options.min = arg[4..arg.len()].parse::<u32>().unwrap_or(1)}
            else if arg.starts_with("--path="){options.path = String::from(&arg[7..arg.len()])}
            else if arg.starts_with("path="){options.path = String::from(&arg[5..arg.len()])}
        }
        options
    }
}