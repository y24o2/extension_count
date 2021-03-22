use std::env;

pub enum Format{
    Text,
    Json,
    PrettyJson, 
}

pub struct Options{
    pub path:String,
    pub recursion:bool,
    pub min:u32,
    pub ranked:bool,
    pub ignore_case:bool,
    pub format:Format,
}

impl Options{
    pub fn default() -> Options{
        Options{
            path: String::from("."),
            recursion: false,
            min: 1,
            ranked: false,
            ignore_case:false,
            format:Format::Text,
        }
    }

    pub fn from_args(args:env::Args) -> Options{
        let usage = "\
Usage: extension_count [flags] [options]
   flags:
        -h, --help  show help message and exit
        -r          read subdirs
        -i, -c      case insensitiv
        -j, --json  json (one line || pretty)
        -n          list extensions ordered by count
    options:
        --min=1     minimum amount (default = 1)
        --path=\".\"  set path (default = \".\")\n";

        let mut options = Options::default();
        for arg in args{
            if arg == "-h" || arg == "--help" {eprintln!("{}", &usage); std::process::exit(0)}
            else if arg.starts_with("-") && !arg.starts_with("--"){
                if arg.contains("i") || arg.contains("c") {options.ignore_case = true}
                if arg.contains("n") {options.ranked = true}
                if arg.contains("r") {options.recursion = true}
                if arg.contains("j") {options.format = Format::Json}
            }
            else if  arg == "--caseinsensitive" {options.ignore_case = true}
            else if arg == "--json" {options.format = Format::PrettyJson}
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