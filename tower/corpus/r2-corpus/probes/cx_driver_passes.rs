// cx12 driver slice witness (ourc construct-generation, 2026-10-25): derived PartialOrd on an enum (sort_by
// partial_cmp); Result<Vec<String>,_> Ok payload; a moved Vec FIELD local extended; Option<&str>.map(format!)
// .unwrap_or_default(); map_err closure param typed from a fn-pointer tuple binder (`->` inside Vec<(..)> types).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] enum Level { Note, Warning, Error }
struct Diag { level: Level, msg: String, code: Option<&'static str> }
struct Cx { log: Vec<String>, n: u32 }
fn parse(cx: &mut Cx) -> Result<(), String> { cx.n += 1; cx.log.push(format!("parse {}", cx.n)); Ok(()) }
fn check(cx: &mut Cx) -> Result<(), String> { if cx.n > 0 { Err(format!("{} errors", cx.n)) } else { Ok(()) } }
fn run(stop: bool) -> Result<Vec<String>, String> {
    let mut cx = Cx { log: vec![], n: 0 };
    let passes: Vec<(&str, fn(&mut Cx) -> Result<(), String>)> = vec![("parse", parse), ("check", check)];
    for (name, p) in &passes { if stop || *name != "check" { p(&mut cx).map_err(|e| format!("[{}] {}", name, e))?; } }
    let ds = vec![Diag { level: Level::Note, msg: "n".into(), code: None }, Diag { level: Level::Error, msg: "e".into(), code: Some("E1") }, Diag { level: Level::Warning, msg: "w".into(), code: None }];
    let mut d: Vec<&Diag> = ds.iter().collect(); d.sort_by(|a, b| b.level.partial_cmp(&a.level).unwrap());
    let mut out = cx.log; out.extend(d.iter().map(|d| format!("{:?}{}: {}", d.level, d.code.map(|c| format!("[{}]", c)).unwrap_or_default(), d.msg))); Ok(out)
}
fn main() { for s in [false, true] { match run(s) { Ok(l) => println!("OK {:?}", l), Err(e) => println!("ERR {}", e) } } }
