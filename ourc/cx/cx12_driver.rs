// cx12 driver/session: rustc_driver/rustc_session-shaped option parsing, a pass pipeline of fn pointers
// over a mutable compiler context, diagnostics with error counting + abort_if_errors, `?` propagation.
#[derive(Debug, Default)] struct Opts { opt_level: u8, crate_name: Option<String>, cfgs: Vec<(String, Option<String>)>, emit: Vec<String>, edition: u16 }
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)] enum Level { Note, Warning, Error }
struct Diag { level: Level, msg: String, code: Option<&'static str> }
#[derive(Default)] struct Sess { opts: Opts, diags: Vec<Diag> }
impl Sess { fn emit(&mut self, level: Level, msg: impl Into<String>, code: Option<&'static str>) { self.diags.push(Diag { level, msg: msg.into(), code }); }
    fn err_count(&self) -> usize { self.diags.iter().filter(|d| d.level == Level::Error).count() }
    fn abort_if_errors(&self) -> Result<(), String> { if self.err_count() > 0 { Err(format!("aborting due to {} previous error{}", self.err_count(), if self.err_count() == 1 { "" } else { "s" })) } else { Ok(()) } } }
fn parse_opts(args: &[&str]) -> Result<Opts, String> {
    let mut o = Opts { edition: 2015, ..Default::default() }; let mut it = args.iter().peekable();
    while let Some(&a) = it.next() {
        match a {
            "-O" => o.opt_level = 2,
            "--crate-name" => o.crate_name = Some(it.next().ok_or("--crate-name needs a value")?.to_string()),
            "--cfg" => { let v = it.next().ok_or("--cfg needs a value")?; o.cfgs.push(match v.split_once('=') { Some((k, v)) => (k.to_string(), Some(v.trim_matches('"').to_string())), None => (v.to_string(), None) }); }
            s if s.starts_with("--emit=") => o.emit.extend(s["--emit=".len()..].split(',').map(String::from)),
            s if s.starts_with("--edition=") => o.edition = s[10..].parse().map_err(|e| format!("bad edition: {:?}", e))?,
            s if s.starts_with("-C") => { let kv = if s.len() > 2 { &s[2..] } else { it.next().copied().unwrap_or("") }; if let Some(l) = kv.strip_prefix("opt-level=") { o.opt_level = l.parse().unwrap_or(0); } }
            other => return Err(format!("unknown option `{}`", other)),
        }
    }
    Ok(o)
}
struct Cx { sess: Sess, items: Vec<String>, log: Vec<String> }
fn parse(cx: &mut Cx) -> Result<(), String> { cx.items = vec!["fn main".into(), "struct S".into(), "fn bad".into()]; cx.log.push(format!("parse {}", cx.items.len())); Ok(()) }
fn resolve(cx: &mut Cx) -> Result<(), String> { if cx.sess.opts.cfgs.iter().any(|(k, _)| k == "deny_bad") { if let Some(i) = cx.items.iter().position(|s| s.ends_with("bad")) { let m = format!("cannot find `{}`", &cx.items[i][3..]); cx.sess.emit(Level::Error, m, Some("E0425")); } } cx.sess.emit(Level::Note, "resolved", None); cx.log.push("resolve".into()); Ok(()) }
fn typeck(cx: &mut Cx) -> Result<(), String> { cx.sess.abort_if_errors()?; cx.sess.emit(Level::Warning, "unused struct `S`", None); cx.log.push("typeck".into()); Ok(()) }
fn codegen(cx: &mut Cx) -> Result<(), String> { let e = if cx.sess.opts.emit.is_empty() { vec!["link".to_string()] } else { cx.sess.opts.emit.clone() }; cx.log.push(format!("codegen O{} {:?}", cx.sess.opts.opt_level, e)); Ok(()) }
fn run(args: &[&str]) -> Result<Vec<String>, String> {
    let opts = parse_opts(args)?; let mut cx = Cx { sess: Sess { opts, diags: vec![] }, items: vec![], log: vec![] };
    let passes: Vec<(&str, fn(&mut Cx) -> Result<(), String>)> = vec![("parse", parse), ("resolve", resolve), ("typeck", typeck), ("codegen", codegen)];
    for (name, p) in &passes { p(&mut cx).map_err(|e| format!("[{}] {}", name, e))?; }
    let mut out = cx.log; let mut d: Vec<&Diag> = cx.sess.diags.iter().collect(); d.sort_by(|a, b| b.level.partial_cmp(&a.level).unwrap());
    out.extend(d.iter().map(|d| format!("{:?}{}: {}", d.level, d.code.map(|c| format!("[{}]", c)).unwrap_or_default(), d.msg))); Ok(out)
}
fn main() {
    let cases: Vec<Vec<&str>> = vec![vec!["-O", "--crate-name", "foo", "--emit=obj,metadata"], vec!["--cfg", "feature=\"x\"", "-C", "opt-level=3", "--edition=2021"], vec!["--cfg", "deny_bad"], vec!["--edition=20x"], vec!["--crate-name"], vec!["-Z"]];
    for c in &cases { match run(c) { Ok(l) => println!("OK {:?}", l), Err(e) => println!("ERR {}", e) } }
    println!("{:?}", parse_opts(&["--cfg", "feature=\"x\"", "--edition=2021"]).map(|o| (o.cfgs, o.edition)));
}
