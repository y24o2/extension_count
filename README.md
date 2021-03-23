# extension_count
[![Rust](https://img.shields.io/badge/Built%20with-Rust-Purple)](https://www.rust-lang.org/)

read directory, count and list file extensions

## examples
list files in current directory:
```
PS C:\Users\Daniel\Coding\rust\extension_count> .\target\debug\extension_count.exe
lock: 1
md: 1
toml: 1
```

list files in other directory, read subdirs, ordered by count:
```
PS C:\Users\Daniel\Coding\rust\extension_count> .\target\debug\extension_count.exe -rn path="C:\Users\Daniel\Coding\Python"
py: 95
pyc: 19
txt: 7
js: 3
lnk: 2
```

## options

option | description
--------|-----------
-h | show help message and exit
-r | read subdirs
-i or -c | case insensitiv
-j | json (one line)
-n | list extensions ordered by count
--json | json (pretty)
--min=1 | minimum amount (default = 1)
--path="." | set path (default = ".")
