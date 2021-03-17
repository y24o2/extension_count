use std::fs;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::OsString;
use std::env;

mod ec_options;

fn list_file_extension(dir: &Path, recursion: bool, ignore_case: bool) -> HashMap<OsString, u32>{
    let mut res: HashMap<OsString, u32> = HashMap::new();
    let entries = match fs::read_dir(&dir) {
        Ok(x) => x,
        _ => return res,
    };

    for e in entries{
        let e = e.unwrap();
        if e.path().is_dir() && recursion{
            for (key, value) in list_file_extension(&e.path(), recursion, ignore_case){
                if let Some(x) = res.get_mut(&key) {
                    *x += value;
                }
                else{
                    res.insert(key, value);
                }
            }
        }
        else{
            if e.path().extension().is_some(){
                let mut ex = OsString::from(e.path().extension().unwrap());
                if ignore_case { ex = OsString::from(ex.to_str().unwrap().to_lowercase()) }
                if let Some(x) = res.get_mut(&ex) {
                    *x += 1;
                }
                else{
                    res.insert(ex, 1);
                }
            }
        }
    }

    res
}

fn main() {
    let options = ec_options::Options::from_args(env::args());   
    let res = list_file_extension(Path::new(&options.path), options.recursion, options.ignore_case);
    let mut keys:Vec<OsString> = Vec::new();

    for (k, v) in &res{
        if v >= &options.min { keys.push(OsString::from(k.to_str().unwrap())) };
    }
    keys.sort();
    if options.ranked{
        let mut nums:Vec<u32> = Vec::new();
        for (_, v) in &res{
            if v >= &options.min && !nums.contains(v) { nums.push(*v) };
        }
        nums.sort();
        nums.reverse();
        for n in nums{
            for (k, v) in &res{
                if n == *v {println!("{}: {}", &k.to_str().unwrap(), *v);}
            }
        }
    }
    else{
        for k in keys{
            println!("{}: {}", &k.to_str().unwrap(), &res.get(&k).unwrap());
        }
    }
}