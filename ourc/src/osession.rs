pub struct Opts {
    pub out: String,
    pub emit_ir: bool,
    pub opt: u32,
    pub input: String,
    pub emit_meta: bool, // R2a P5: write this crate's metadata unit (to stdout) instead of compiling a program
    pub externs: Vec<String>, // R2a P5: `--extern NAME=PATH` units to load as crates
    pub link: bool, // V1.2 DRIVE-LINK: `-o PATH` was given -- build an executable there (llc + cc)
    pub time_limit: i64, // V5.3 BUDGET: seconds before a compile is refused with a diagnostic (0 = no limit)
    pub mem_limit: i64, // V5.3 BUDGET: GiB of address space before a compile is refused (0 = no cap)
}
pub fn opts_new() -> Opts {
    Opts {
        out: String::from("a.out"),
        emit_ir: false,
        opt: 0,
        input: String::from(""),
        emit_meta: false,
        externs: Vec::new(),
        link: false,
        time_limit: 25,
        mem_limit: 8,
    }
}
pub fn parse_args(args: &Vec<String>) -> Opts {
    let mut o = opts_new();
    let mut i = 0;
    while i < args.len() {
        let a = &args[i];
        if a.as_str() == "-o" {
            i = i + 1;
            if i < args.len() {
                o.out = String::from(args[i].as_str());
                o.link = true;
            }
        } else if a.as_str() == "--mem-limit" {
            i = i + 1;
            if i < args.len() {
                let db = args[i].as_bytes();
                let mut n: i64 = 0;
                let mut k = 0;
                while k < db.len() && db[k] >= 48 && db[k] <= 57 {
                    n = n * 10 + (db[k] - 48) as i64;
                    k = k + 1;
                }
                o.mem_limit = n;
            }
        } else if a.as_str() == "--time-limit" {
            i = i + 1;
            if i < args.len() {
                let db = args[i].as_bytes();
                let mut n: i64 = 0;
                let mut k = 0;
                while k < db.len() && db[k] >= 48 && db[k] <= 57 {
                    n = n * 10 + (db[k] - 48) as i64;
                    k = k + 1;
                }
                o.time_limit = n;
            }
        } else if a.as_str() == "--emit-ir" {
            o.emit_ir = true;
        } else if a.as_str() == "-O" {
            o.opt = 2;
        } else if a.as_str() == "--emit-meta" {
            o.emit_meta = true;
        } else if a.as_str() == "--extern" {
            i = i + 1;
            if i < args.len() {
                o.externs.push(String::from(args[i].as_str()));
            }
        } else {
            o.input = String::from(a.as_str());
        }
        i = i + 1;
    }
    o
}
pub fn opt_name(o: &Opts) -> String {
    if o.opt == 2 {
        String::from("O2")
    } else {
        String::from("O0")
    }
}
