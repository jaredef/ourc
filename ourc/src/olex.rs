#[derive(Clone)]
pub enum Tk {
    OpEq(String),
    At,
    PubT,
    Label(String),
    LexErr,
    LoopT,
    Dollar,
    DeriveCopy,
    WhereT,
    TypeT,
    UnsafeT,
    Bang2,
    UseT,
    ExternT,
    CrateT,
    Caret,
    Percent,
    Shl,
    Shr,
    AsT,
    Amp,
    ConstT,
    StaticT,
    BreakT,
    ContT,
    PlusEq,
    MinusEq,
    StarEq,
    SlashEq,
    ModT,
    Quest,
    DotDot,
    InT,
    Bar,
    DynT,
    Ret,
    Trait,
    ForT,
    StrL(String),
    Num(i64),
    ChL(i64),
    FloatL(String),
    Ident(String),
    Fn,
    While,
    Mut,
    Struct,
    Enum,
    Impl,
    Match,
    FatArrow,
    PathSep,
    LBrack,
    RBrack,
    Dot,
    Colon,
    Gt,
    Le,
    Ge,
    Ne,
    AndAnd,
    OrOr,
    Bang,
    Slash,
    Comma,
    Plus,
    Minus,
    Star,
    Lt,
    EqEq,
    LParen,
    RParen,
    LBrace,
    RBrace,
    Semi,
    Eq,
    Let,
    If,
    Else,
    TrueT,
    FalseT,
    Eof,
}
pub struct Tok {
    pub k: Tk,
    pub lo: u32,
    pub hi: u32,
}
// does this attribute's text derive Copy?
fn oast_is_num_suffix(t: &str) -> bool {
    t == "u8"
        || t == "u16"
        || t == "u32"
        || t == "u64"
        || t == "usize"
        || t == "i8"
        || t == "i16"
        || t == "i32"
        || t == "i64"
        || t == "isize"
        || t == "f64"
} /* F32-INT-LIT: `3f32` was consumed as `(3 as f32)` -- f32 has no type here, so it printed f64 BITS as an int
  (4613937818241073152) and `3f32*3f32` gave 0 (rustc 9). Left unconsumed it refuses, like `3.0f32` (F-3 is its own rung). */
fn has_copy(t: &str) -> bool {
    let b = t.as_bytes();
    let mut i = 0;
    while i + 3 < b.len() {
        if b[i] == 67 && b[i + 1] == 111 && b[i + 2] == 112 && b[i + 3] == 121 {
            return true;
        }
        i = i + 1;
    }
    false
}
fn is_a(c: char) -> bool {
    let x = c as u32;
    (x >= 97 && x <= 122) || (x >= 65 && x <= 90) || x == 95
}
fn is_d(c: char) -> bool {
    let x = c as u32;
    x >= 48 && x <= 57
}
fn hexv(ch: char) -> i64 {
    let x = ch as u32;
    if x >= 48 && x <= 57 {
        (x - 48) as i64
    } else if x >= 97 && x <= 102 {
        (x - 97 + 10) as i64
    } else if x >= 65 && x <= 70 {
        (x - 65 + 10) as i64
    } else {
        0 - 1
    }
}
// LEX-ESC (rustc_lexer unescape): the ONE escape decoder for char, byte and string literals. cs[i] is the
// backslash. Returns the escape's value, -1 for an escape rustc rejects (`\q`, `\x` without two hex digits,
// `\u{}` malformed), or -2 for a line continuation (backslash-newline: the newline and the following
// whitespace produce nothing). esc_len gives how many chars the escape spans.
fn esc_val(cs: &Vec<char>, i: usize) -> i64 {
    if i + 1 >= cs.len() {
        return 0 - 1;
    }
    let e = cs[i + 1];
    if e == 'n' {
        return 10;
    }
    if e == 't' {
        return 9;
    }
    if e == 'r' {
        return 13;
    }
    if e == '0' {
        return 0;
    }
    if e == '\\' {
        return 92;
    }
    if e == '\'' {
        return 39;
    }
    if e == '"' {
        return 34;
    }
    if e == '\n' {
        return 0 - 2;
    }
    if e == 'x' {
        if i + 3 < cs.len() && hexv(cs[i + 2]) >= 0 && hexv(cs[i + 3]) >= 0 {
            return hexv(cs[i + 2]) * 16 + hexv(cs[i + 3]);
        }
        return 0 - 1;
    }
    if e == 'u' {
        if i + 2 >= cs.len() || cs[i + 2] != '{' {
            return 0 - 1;
        }
        let mut j = i + 3;
        let mut v: i64 = 0;
        let mut nd = 0;
        while j < cs.len() && cs[j] != '}' {
            if cs[j] != '_' {
                let h = hexv(cs[j]);
                if h < 0 {
                    return 0 - 1;
                }
                v = v * 16 + h;
                nd = nd + 1;
            }
            j = j + 1;
        }
        if j >= cs.len() || nd == 0 || nd > 6 || v > 1114111 || (v >= 55296 && v <= 57343) {
            return 0 - 1;
        }
        return v;
    } // UESC-SURR: a surrogate is not a char (rustc rejects; SEM0 char::from_u32 does not)
    0 - 1
}
fn esc_len(cs: &Vec<char>, i: usize) -> usize {
    let e = cs[i + 1];
    if e == 'x' {
        return 4;
    }
    if e == 'u' {
        let mut j = i + 3;
        while j < cs.len() && cs[j] != '}' {
            j = j + 1;
        }
        return j + 1 - i;
    }
    if e == '\n' {
        let mut j = i + 2;
        while j < cs.len() && (cs[j] == ' ' || cs[j] == '\n' || cs[j] == '\t' || cs[j] == '\r') {
            j = j + 1;
        }
        return j - i;
    }
    2
}
pub fn lex(src: &str) -> Vec<Tok> {
    // REGRESS-E: a program that loads a crate (`extern crate test_macros;`) may bring its own derive macros -- an
    // unknown derive is refused only when there is no such crate (it was refusing ~19 passing proc-macro tests)
    let has_extern_crate = src.contains("extern crate");
    let cs: Vec<char> = {
        let mut v: Vec<char> = Vec::new();
        for c in src.chars() {
            v.push(c);
        }
        v
    };
    let mut out: Vec<Tok> = Vec::new();
    let mut i = 0;
    let srm = sig_region_mismatch(&cs);
    if srm < cs.len() {
        // A8a SIG-REGION: a returned parameter outlives-mismatch, judged on the text
        out.push(Tok {
            k: Tk::LexErr,
            lo: srm as u32,
            hi: srm as u32 + 1,
        });
    }
    let has_as_usize = src.contains("as usize");
    let has_layout_read = has_as_usize
        || src.contains("size_of")
        || src.contains("align_of")
        || src.contains("offset_of");
    if hr_lt_output_only(&cs) {
        // REGRESS-5 E0582: lifetimes are dropped by this lexer, so the check runs here on the text
        out.push(Tok {
            k: Tk::LexErr,
            lo: 0,
            hi: 0,
        });
    }
    while i < cs.len() {
        let st = i as u32;
        let c = cs[i];
        if c == ' ' || c == '\n' || c == '\t' || c == '\r' {
            i = i + 1;
        } else if c == '#' {
            // an ATTRIBUTE: `#[..]` or `#![..]`, skipped to the matching `]`
            i = i + 1;
            let mut inner_attr = false;
            if i < cs.len() && cs[i] == '!' {
                i = i + 1;
                inner_attr = true;
            }
            if i < cs.len() && cs[i] == '[' {
                let mut d = 0;
                let abeg = i;
                while i < cs.len() {
                    if cs[i] == '[' {
                        d = d + 1;
                    }
                    if cs[i] == ']' {
                        d = d - 1;
                        if d == 0 {
                            i = i + 1;
                            break;
                        }
                    }
                    i = i + 1;
                }
                {
                    let mut txt = String::from("");
                    let mut q = abeg;
                    while q < i {
                        // V4.3e RAW-ATTR: a raw identifier in an attribute names the same thing (`#[r#repr(r#C, r#packed)]`
                        // is `#[repr(C, packed)]`); the attribute tests below read the plain spelling. Not a raw STRING (`r#"`).
                        if cs[q] == 'r'
                            && q + 2 < i
                            && cs[q + 1] == '#'
                            && is_a(cs[q + 2])
                            && (q == abeg || !srm_idc(cs[q - 1]))
                        {
                            q = q + 2;
                            continue;
                        }
                        txt.push(cs[q]);
                        q = q + 1;
                    }
                    if has_copy(&txt) {
                        out.push(Tok {
                            k: Tk::DeriveCopy,
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    if derive_unknown(&txt) && !has_extern_crate {
                        // REGRESS-5: a derive macro that is not builtin (a proc-macro) -- we have no proc macros
                        out.push(Tok {
                            k: Tk::LexErr,
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    if has_word(&txt, "PartialEq") && txt.contains("derive") {
                        // CONST-PAT-TY widening: a structural-PartialEq type (its consts are patterns)
                        out.push(Tok {
                            k: Tk::Ident(String::from("__derive_peq__")),
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    // (only on an ITEM: on a variant inside an enum body the marker token read as a variant name, and
                    // variants_same_crate's match lost its variants -- a false refusal found by sweep-valid at the merge)
                    if txt.as_str() == "[non_exhaustive]" && nonexh_on_item(&cs, i) {
                        // R2a P6 NONEXH: `#[non_exhaustive]` -- across a crate boundary a match on the item needs a wildcard (E0004)
                        out.push(Tok {
                            k: Tk::Ident(String::from("__nonexh__")),
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    if has_derive_ord(&txt) {
                        // R13 DERIVE-ORD: `#[derive(.., Ord, ..)]` -- the parser synthesizes the lexicographic comparator
                        out.push(Tok {
                            k: Tk::Ident(String::from("__derive_ord__")),
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    if !inner_attr
                        && cfg_pre(txt.as_bytes(), "[cfg(")
                        && txt.as_bytes().len() > 7
                        && txt.as_bytes()[txt.as_bytes().len() - 1] == 93
                        && txt.as_bytes()[txt.as_bytes().len() - 2] == 41
                    {
                        // CFG-ITEM (W2): `#[cfg(PRED)]` on an item -- a FALSE predicate removes the item (parse_prog strips
                        // the marker + the item). Both cfg'd definitions were kept and the wrong one won.
                        let inner = cfg_sub(txt.as_bytes(), 5, txt.as_bytes().len() - 2);
                        if cfg_eval(inner.as_str()) == 0 {
                            out.push(Tok {
                                k: Tk::Ident(String::from("__cfg_off__")),
                                lo: st,
                                hi: i as u32,
                            });
                        }
                    }
                    if !inner_attr && txt.starts_with("[repr(") {
                        // V1 ALIGN-OBSERVE (refusal > WRONG; found by run f): `align(N)` is not honoured (aggregates are 8-aligned
                        // heap handles), so a program reading an address as an integer (`p as usize % 256`) saw a misaligned one
                        // V4.3e PACKED (refusal > WRONG): `packed` is not honoured either -- fields keep their alignment, so
                        // `size_of` of a `#[repr(C, packed)]` struct answered 16 where rustc says 9 (raw-identifiers/attr). Only a program that
                        // can READ layout (size_of / align_of / offset_of / an address) is refused: sweep-valid found 13 that do not
                        if (txt.contains("align(") && has_as_usize)
                            || (has_word(&txt, "packed") && has_layout_read)
                        {
                            out.push(Tok {
                                k: Tk::LexErr,
                                lo: st,
                                hi: i as u32,
                            });
                        }
                        // A2 ENUM-DISCR-WIDTH: `#[repr(u8)]` / `#[repr(i64)]` -- an enum's discriminant type (its suffixed
                        // discriminants must match it); emitted as `__repr_<width>__`, skipped by the item loop
                        // a macro-parameter repr (`#[repr($r)]`) names no width here: `any` (not judged)
                        let w = if txt.contains("$") {
                            String::from("any")
                        } else {
                            repr_int_width(&txt)
                        };
                        if w.len() > 0 {
                            let mut m = String::from("__repr_");
                            m.push_str(w.as_str());
                            m.push_str("__");
                            out.push(Tok {
                                k: Tk::Ident(m),
                                lo: st,
                                hi: i as u32,
                            });
                        }
                    }
                    if !inner_attr && (txt.starts_with("[repr(C") || txt.starts_with("[repr(C,")) {
                        // R4a LAYOUT: `#[repr(C)]` fixes declaration-order layout (size_of answers differ from repr(Rust))
                        out.push(Tok {
                            k: Tk::Ident(String::from("__repr_c__")),
                            lo: st,
                            hi: i as u32,
                        });
                    }
                    if inner_attr && txt.starts_with("[feature") {
                        out.push(Tok {
                            k: Tk::Ident(String::from("__feature_gate__")),
                            lo: st,
                            hi: i as u32,
                        });
                    } // A1 FEATURE-GATE: `#![feature(..)]` is rejected on stable (rustc E0554) -- the parser reports it
                    if txt.as_str() == "[default]" {
                        out.push(Tok {
                            k: Tk::Ident(String::from("__default_variant__")),
                            lo: st,
                            hi: i as u32,
                        });
                    }
                }
            }
        }
        // DFLT-ENUM: `#[default]` marks the enum`s default variant
        else if c == '/' && i + 1 < cs.len() && cs[i + 1] == '/' {
            while i < cs.len() && cs[i] != '\n' {
                i = i + 1;
            }
        }
        // LEX-NEST: block comments NEST in Rust (`/* a /* b */ c */` is one comment)
        else if c == '/' && i + 1 < cs.len() && cs[i + 1] == '*' {
            i = i + 2;
            let mut d = 1;
            while i < cs.len() && d > 0 {
                if cs[i] == '/' && i + 1 < cs.len() && cs[i + 1] == '*' {
                    d = d + 1;
                    i = i + 2;
                } else if cs[i] == '*' && i + 1 < cs.len() && cs[i + 1] == '/' {
                    d = d - 1;
                    i = i + 2;
                } else {
                    i = i + 1;
                }
            }
            if d > 0 {
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
            }
        }
        // a BYTE literal `b'x'` is an INTEGER (`println!("{}", b'A')` prints 65, not A), so it is a Num
        else if c == 'b' && i + 1 < cs.len() && cs[i + 1] == '\'' {
            let mut j = i + 2;
            let mut v: i64 = 0;
            if j < cs.len() && cs[j] == '\\' {
                v = esc_val(&cs, j);
                if v > 255 {
                    v = 0 - 1;
                }
                j = j + esc_len(&cs, j);
            } else if j < cs.len() {
                v = cs[j] as i64;
                j = j + 1;
            }
            if v < 0 || j >= cs.len() || cs[j] != '\'' {
                i = j;
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
                continue;
            }
            i = j + 1;
            out.push(Tok {
                k: Tk::Num(v),
                lo: st,
                hi: i as u32,
            });
            continue;
        }
        // LEX-BSTR: a byte string `b"AB"` is a `&[u8; N]` -- lexed as the array literal `[65, 66]`
        else if c == 'b' && i + 1 < cs.len() && cs[i + 1] == '"' {
            i = i + 2;
            let mut vs: Vec<i64> = Vec::new();
            let mut bad = false;
            while i < cs.len() && cs[i] != '"' {
                if cs[i] == '\\' {
                    let v = esc_val(&cs, i);
                    if v == 0 - 1 || v > 255 || (i + 1 < cs.len() && cs[i + 1] == 'u') {
                        bad = true;
                    } /* BSTR-NO-UESC: a byte string takes no \u escape */
                    if v >= 0 {
                        vs.push(v);
                    }
                    i = i + esc_len(&cs, i);
                } else {
                    vs.push(cs[i] as i64);
                    i = i + 1;
                }
            }
            if i >= cs.len() || bad {
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
                continue;
            }
            i = i + 1;
            out.push(Tok {
                k: Tk::LBrack,
                lo: st,
                hi: st,
            });
            let mut q = 0;
            while q < vs.len() {
                if q > 0 {
                    out.push(Tok {
                        k: Tk::Comma,
                        lo: st,
                        hi: st,
                    });
                }
                out.push(Tok {
                    k: Tk::Num(vs[q]),
                    lo: st,
                    hi: st,
                });
                q = q + 1;
            }
            out.push(Tok {
                k: Tk::RBrack,
                lo: st,
                hi: i as u32,
            });
            continue;
        }
        // LEX-RAW: raw strings `r"..."` / `r#"..."#` (no escapes; the closing quote must carry as many `#`) and
        // raw identifiers `r#match` (a keyword used as a plain name)
        else if c == 'r'
            && i + 1 < cs.len()
            && (cs[i + 1] == '"'
                || (cs[i + 1] == '#' && i + 2 < cs.len() && (cs[i + 2] == '"' || cs[i + 2] == '#')))
        {
            let mut j = i + 1;
            let mut nh = 0;
            while j < cs.len() && cs[j] == '#' {
                nh = nh + 1;
                j = j + 1;
            }
            if j >= cs.len() || cs[j] != '"' {
                i = j;
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
                continue;
            }
            j = j + 1;
            let mut s = String::from("");
            let mut done = false;
            while j < cs.len() {
                if cs[j] == '"' {
                    let mut k = 0;
                    while k < nh && j + 1 + k < cs.len() && cs[j + 1 + k] == '#' {
                        k = k + 1;
                    }
                    if k == nh {
                        j = j + 1 + nh;
                        done = true;
                        break;
                    }
                }
                s.push(cs[j]);
                j = j + 1;
            }
            i = j;
            if !done {
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
                continue;
            }
            out.push(Tok {
                k: Tk::StrL(s),
                lo: st,
                hi: i as u32,
            });
            continue;
        } else if c == 'r' && i + 2 < cs.len() && cs[i + 1] == '#' && is_a(cs[i + 2]) {
            i = i + 2;
            let mut s = String::from("");
            while i < cs.len() && (is_a(cs[i]) || is_d(cs[i])) {
                s.push(cs[i]);
                i = i + 1;
            }
            out.push(Tok {
                k: Tk::Ident(s),
                lo: st,
                hi: i as u32,
            });
            continue;
        } else if c == '\'' {
            // a char literal: one char or one escape, then the closing quote. Anything else after `'` is a
            // lifetime/label (`'a`), skipped. The one-char case must not claim `'\''`.
            if i + 2 < cs.len() && cs[i + 2] == '\'' && cs[i + 1] != '\\' && cs[i + 1] != '\'' {
                let v = cs[i + 1] as u32;
                i = i + 3;
                out.push(Tok {
                    k: Tk::ChL(v as i64),
                    lo: st,
                    hi: i as u32,
                });
            } else if i + 1 < cs.len() && cs[i + 1] == '\\' {
                let v = esc_val(&cs, i + 1);
                let j = i + 1 + esc_len(&cs, i + 1);
                if v < 0 || j >= cs.len() || cs[j] != '\'' {
                    i = j;
                    out.push(Tok {
                        k: Tk::LexErr,
                        lo: st,
                        hi: i as u32,
                    });
                } else {
                    i = j + 1;
                    out.push(Tok {
                        k: Tk::ChL(v),
                        lo: st,
                        hi: i as u32,
                    });
                }
            } else if i + 1 < cs.len() && cs[i + 1] == '\'' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
            }
            // `''`: empty char literal
            else {
                i = i + 1;
                let mut nm = String::from("'");
                while i < cs.len() && (is_a(cs[i]) || is_d(cs[i])) {
                    nm.push(cs[i]);
                    i = i + 1;
                }
                // LABEL: `'outer:` before a loop, or `'outer` after break/continue, is a LOOP LABEL token; every
                // other `'a` is a lifetime, which ourc's types ignore, so it stays skipped
                let mut kk = i;
                while kk < cs.len() && (cs[kk] == ' ' || cs[kk] == '\n' || cs[kk] == '\t') {
                    kk = kk + 1;
                }
                let defn =
                    kk < cs.len() && cs[kk] == ':' && !(kk + 1 < cs.len() && cs[kk + 1] == ':');
                let mut afterbk = false;
                if out.len() > 0 {
                    match out[out.len() - 1].k {
                        Tk::BreakT => {
                            afterbk = true;
                        }
                        Tk::ContT => {
                            afterbk = true;
                        }
                        _ => {}
                    }
                }
                if defn || afterbk {
                    out.push(Tok {
                        k: Tk::Label(nm),
                        lo: st,
                        hi: i as u32,
                    });
                }
            }
        } else if c == '"' {
            i = i + 1;
            let mut s = String::from("");
            let mut bad = false;
            while i < cs.len() && cs[i] != '"' {
                if cs[i] == '\\' {
                    let v = esc_val(&cs, i);
                    if v == 0 - 1 {
                        bad = true;
                    } else if v == 0 {
                        // STR-NUL (W7) closed by V1a LEN-STRING: a str carries its length, so an embedded `\0` / `\x00` / `\u{0}` is a byte
                        s.push(0 as char);
                    } else if v > 127 {
                        match char::from_u32(v as u32) {
                            Some(ch) => s.push(ch),
                            None => {
                                bad = true;
                            }
                        }
                    } else if v >= 0 {
                        s.push(v as u8 as char);
                    } // STR-UESC: a `\u{..}` above ASCII is the char itself (a literal non-ASCII char already lexes this way)
                    i = i + esc_len(&cs, i);
                    continue;
                }
                s.push(cs[i]);
                i = i + 1;
            }
            if i >= cs.len() || bad {
                out.push(Tok {
                    k: Tk::LexErr,
                    lo: st,
                    hi: i as u32,
                });
                if i < cs.len() {
                    i = i + 1;
                }
                continue;
            }
            i = i + 1;
            out.push(Tok {
                k: Tk::StrL(s),
                lo: st,
                hi: i as u32,
            });
        } else if is_d(c) {
            let mut n: i64 = 0;
            // a FLOAT literal: digits `.` digits, or an exponent. `1..2` is a range, not a float, so a
            // second `.` immediately after the first stops the read. A radix literal can match neither
            // shape, so this read is radix-independent and comes first.
            {
                let mut j = i;
                let mut isf = false;
                while j < cs.len() && (is_d(cs[j]) || cs[j] == '_') {
                    j = j + 1;
                }
                if j < cs.len() && cs[j] == '.' && !(j + 1 < cs.len() && cs[j + 1] == '.') {
                    if j + 1 >= cs.len() || !is_a(cs[j + 1]) {
                        isf = true;
                    }
                } // LEX-INT-DOT-E: `5.eq(..)` / `5.e3` is a method/field access on 5, never a float (rustc agrees)
                if j < cs.len()
                    && (cs[j] == 'e' || cs[j] == 'E')
                    && j + 1 < cs.len()
                    && (is_d(cs[j + 1]) || cs[j + 1] == '-' || cs[j + 1] == '+')
                {
                    isf = true;
                }
                if isf {
                    let mut txt = String::from("");
                    let mut k = i;
                    while k < cs.len() && (is_d(cs[k]) || cs[k] == '_') {
                        if cs[k] != '_' {
                            txt.push(cs[k]);
                        }
                        k = k + 1;
                    }
                    if k < cs.len() && cs[k] == '.' {
                        txt.push('.');
                        k = k + 1;
                        while k < cs.len() && (is_d(cs[k]) || cs[k] == '_') {
                            if cs[k] != '_' {
                                txt.push(cs[k]);
                            }
                            k = k + 1;
                        }
                    }
                    if k < cs.len() && (cs[k] == 'e' || cs[k] == 'E') {
                        txt.push('e');
                        k = k + 1;
                        if k < cs.len() && (cs[k] == '-' || cs[k] == '+') {
                            txt.push(cs[k]);
                            k = k + 1;
                        }
                        while k < cs.len() && (is_d(cs[k]) || cs[k] == '_') {
                            if cs[k] != '_' {
                                txt.push(cs[k]);
                            }
                            k = k + 1;
                        }
                    } /* FLOAT-EXP-SEP: `9.3e18_f64` -- a separator after the exponent (it stopped there, so _f64 lexed as a name) */
                    // a width suffix is consumed with the literal; f32 is a separate rung
                    if k + 2 < cs.len() && cs[k] == 'f' && cs[k + 1] == '6' && cs[k + 2] == '4' {
                        k = k + 3;
                    }
                    // an f32 suffix is deliberately NOT consumed: f32 is its own rung (F-3), and a
                    // silently widened f32 would print 16 digits where rustc prints 8

                    i = k;
                    out.push(Tok {
                        k: Tk::FloatL(txt),
                        lo: st,
                        hi: i as u32,
                    });
                    continue;
                }
            }
            // a RADIX literal `0x..` / `0o..` / `0b..` (digit separators allowed), before the decimal read
            let mut radix: i64 = 10;
            if c == '0' && i + 1 < cs.len() {
                let r = cs[i + 1];
                if r == 'x' || r == 'X' {
                    radix = 16;
                } else if r == 'o' || r == 'O' {
                    radix = 8;
                } else if r == 'b' || r == 'B' {
                    radix = 2;
                }
                if radix != 10 {
                    i = i + 2;
                }
            }
            if radix != 10 {
                loop {
                    if i >= cs.len() {
                        break;
                    }
                    let d = cs[i];
                    if d == '_' {
                        i = i + 1;
                        continue;
                    }
                    let v: i64 = if is_d(d) {
                        ((d as u32) - 48) as i64
                    } else if d >= 'a' && d <= 'f' {
                        (((d as u32) - 97) + 10) as i64
                    } else if d >= 'A' && d <= 'F' {
                        (((d as u32) - 65) + 10) as i64
                    } else {
                        0 - 1
                    };
                    if v < 0 || v >= radix {
                        break;
                    }
                    n = n * radix + v;
                    i = i + 1;
                }
            } else {
                while i < cs.len() && (is_d(cs[i]) || cs[i] == '_') {
                    if cs[i] != '_' {
                        n = n * 10 + ((cs[i] as u32) - 48) as i64;
                    }
                    i = i + 1;
                }
            }
            // a TYPE SUFFIX on the literal (`1u32`, `42i64`, `0usize`) is consumed with it
            if i < cs.len() && (cs[i] == 'u' || cs[i] == 'i' || cs[i] == 'f') {
                let sv = i;
                let mut j = i + 1;
                while j < cs.len() && (is_a(cs[j]) || is_d(cs[j])) {
                    j = j + 1;
                }
                let mut suf = String::from("");
                let mut q = sv;
                while q < j {
                    suf.push(cs[q]);
                    q = q + 1;
                }
                if oast_is_num_suffix(&suf) {
                    i = j;
                    // LIT-SUFFIX: the suffix IS the literal's type (`!0u8` == 255, `200u8 + 100` overflows u8). It was
                    // dropped; it is now lexed as the cast `(n as suf)` so the width reaches typeck/lowering.
                    if suf.as_str() != "i64" {
                        let hi = i as u32;
                        out.push(Tok {
                            k: Tk::LParen,
                            lo: st,
                            hi: st,
                        });
                        out.push(Tok {
                            k: Tk::Num(n),
                            lo: st,
                            hi: hi,
                        });
                        out.push(Tok {
                            k: Tk::AsT,
                            lo: hi,
                            hi: hi,
                        });
                        out.push(Tok {
                            k: Tk::Ident(suf),
                            lo: hi,
                            hi: hi,
                        });
                        out.push(Tok {
                            k: Tk::RParen,
                            lo: hi,
                            hi: hi,
                        });
                        continue;
                    }
                }
            }
            out.push(Tok {
                k: Tk::Num(n),
                lo: st,
                hi: i as u32,
            });
        } else if is_a(c) {
            let mut s = String::from("");
            while i < cs.len() && (is_a(cs[i]) || is_d(cs[i])) {
                s.push(cs[i]);
                i = i + 1;
            }
            // VIS-PUB: `pub` is dropped EXCEPT before a function (`pub fn`, `pub(crate) fn`), where it becomes PubT so the
            // item reader can record which module functions are private (rustc_privacy E0603)
            if s.as_str() == "pub" {
                let mut j = i;
                while j < cs.len() && (cs[j] == ' ' || cs[j] == '\n' || cs[j] == '\t') {
                    j = j + 1;
                }
                let mut pubcrate = false;
                if j < cs.len() && cs[j] == '(' {
                    let gs = j + 1;
                    while j < cs.len() && cs[j] != ')' {
                        j = j + 1;
                    }
                    let g: String = cs[gs..j].iter().collect();
                    pubcrate = g.trim() == "crate"; // PUB-CRATE (R2a P6): `pub(crate) fn` -- crate-private across crates
                    j = j + 1;
                    i = j; // VIS-PUB: `pub(crate)` / `pub(super)` -- the group is consumed with `pub` (was left as tokens: rejected)
                    while j < cs.len() && (cs[j] == ' ' || cs[j] == '\n' || cs[j] == '\t') {
                        j = j + 1;
                    }
                }
                // VIS-FIELD (R2a P6): also before a named struct FIELD (`pub x: T` -- an identifier then ONE `:`), so the struct
                // reader can tell public from private fields (rustc E0451)
                let field = {
                    let mut k = j;
                    while k < cs.len() && (is_a(cs[k]) || is_d(cs[k])) {
                        k = k + 1;
                    }
                    let had_id = k > j;
                    while k < cs.len() && (cs[k] == ' ' || cs[k] == '\t' || cs[k] == '\n') {
                        k = k + 1;
                    }
                    had_id
                        && k < cs.len()
                        && cs[k] == ':'
                        && !(k + 1 < cs.len() && cs[k + 1] == ':')
                };
                // PUB-QUAL-FN (R2a P5): `pub const fn` / `pub unsafe fn` / `pub async fn` / `pub extern "C" fn` -- the qualifiers
                // between `pub` and `fn` are skipped (the `pub` was dropped, so the fn read as private: E0603 on a public fn)
                let mut jf = j;
                loop {
                    let mut k = jf;
                    while k < cs.len() && (is_a(cs[k]) || is_d(cs[k])) {
                        k = k + 1;
                    }
                    let w: String = cs[jf..k].iter().collect();
                    if w.as_str() == "const"
                        || w.as_str() == "unsafe"
                        || w.as_str() == "async"
                        || w.as_str() == "extern"
                    {
                        jf = k;
                        while jf < cs.len() && (cs[jf] == ' ' || cs[jf] == '\n' || cs[jf] == '\t') {
                            jf = jf + 1;
                        }
                        if w.as_str() == "extern" && jf < cs.len() && cs[jf] == '"' {
                            jf = jf + 1;
                            while jf < cs.len() && cs[jf] != '"' {
                                jf = jf + 1;
                            }
                            jf = jf + 1;
                            while jf < cs.len()
                                && (cs[jf] == ' ' || cs[jf] == '\n' || cs[jf] == '\t')
                            {
                                jf = jf + 1;
                            }
                        }
                    } else {
                        break;
                    }
                }
                if field
                    || (jf + 2 < cs.len()
                        && cs[jf] == 'f'
                        && cs[jf + 1] == 'n'
                        && !(is_a(cs[jf + 2]) || is_d(cs[jf + 2])))
                {
                    out.push(Tok {
                        k: if pubcrate && !field {
                            Tk::Ident(String::from("__pubcrate__"))
                        } else {
                            Tk::PubT
                        },
                        lo: st,
                        hi: i as u32,
                    });
                }
                continue;
            }
            let k = if s.as_str() == "fn" {
                Tk::Fn
            } else if s.as_str() == "struct" {
                Tk::Struct
            } else if s.as_str() == "enum" {
                Tk::Enum
            } else if s.as_str() == "impl" {
                Tk::Impl
            } else if s.as_str() == "trait" {
                Tk::Trait
            } else if s.as_str() == "return" {
                Tk::Ret
            } else if s.as_str() == "dyn" {
                Tk::DynT
            } else if s.as_str() == "in" {
                Tk::InT
            } else if s.as_str() == "mod" {
                Tk::ModT
            } else if s.as_str() == "break" {
                Tk::BreakT
            } else if s.as_str() == "continue" {
                Tk::ContT
            } else if s.as_str() == "const" {
                Tk::ConstT
            } else if s.as_str() == "static" {
                Tk::StaticT
            } else if s.as_str() == "as" {
                Tk::AsT
            } else if s.as_str() == "use" {
                Tk::UseT
            } else if s.as_str() == "extern" {
                Tk::ExternT
            } else if s.as_str() == "crate" {
                Tk::CrateT
            } else if s.as_str() == "unsafe" {
                Tk::UnsafeT
            } else if s.as_str() == "where" {
                Tk::WhereT
            } else if s.as_str() == "type" {
                Tk::TypeT
            } else if s.as_str() == "loop" {
                Tk::LoopT
            } else if s.as_str() == "for" {
                Tk::ForT
            } else if s.as_str() == "match" {
                Tk::Match
            } else if s.as_str() == "while" {
                Tk::While
            } else if s.as_str() == "mut" {
                Tk::Mut
            } else if s.as_str() == "let" {
                Tk::Let
            } else if s.as_str() == "if" {
                Tk::If
            } else if s.as_str() == "else" {
                Tk::Else
            } else if s.as_str() == "true" {
                Tk::TrueT
            } else if s.as_str() == "false" {
                Tk::FalseT
            } else {
                Tk::Ident(s)
            };
            out.push(Tok {
                k: k,
                lo: st,
                hi: i as u32,
            });
        } else {
            let k2 = if c == '+' {
                Tk::Plus
            } else if c == '-' {
                Tk::Minus
            } else if c == '*' {
                Tk::Star
            } else if c == '/' {
                Tk::Slash
            } else if c == '%' {
                Tk::Percent
            } else if c == '<' {
                Tk::Lt
            } else if c == '>' {
                Tk::Gt
            } else if c == '(' {
                Tk::LParen
            } else if c == ')' {
                Tk::RParen
            } else if c == '{' {
                Tk::LBrace
            } else if c == '}' {
                Tk::RBrace
            } else if c == '[' {
                Tk::LBrack
            } else if c == ']' {
                Tk::RBrack
            } else if c == ';' {
                Tk::Semi
            } else if c == ',' {
                Tk::Comma
            } else if c == '.' {
                Tk::Dot
            } else if c == ':' {
                Tk::Colon
            } else if c == '=' {
                Tk::Eq
            } else {
                Tk::Eof
            };
            if c == '=' && i + 1 < cs.len() && cs[i + 1] == '>' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::FatArrow,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == ':' && i + 1 < cs.len() && cs[i + 1] == ':' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::PathSep,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '=' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::EqEq,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '<' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::Le,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '>' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::Ge,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '!' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::Ne,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '&' && i + 1 < cs.len() && cs[i + 1] == '&' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::AndAnd,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '$' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Dollar,
                    lo: st,
                    hi: i as u32,
                });
            } else if (c == '%' || c == '^' || c == '|' || c == '&')
                && i + 1 < cs.len()
                && cs[i + 1] == '='
            {
                let mut o = String::from("");
                o.push(c);
                i = i + 2;
                out.push(Tok {
                    k: Tk::OpEq(o),
                    lo: st,
                    hi: i as u32,
                });
            }
            /* COMPOUND-OPS: %= ^= |= &= <<= >>= are ONE token each, as += is */
            else if (c == '<' || c == '>')
                && i + 2 < cs.len()
                && cs[i + 1] == c
                && cs[i + 2] == '='
            {
                let mut o = String::from("");
                o.push(c);
                o.push(c);
                i = i + 3;
                out.push(Tok {
                    k: Tk::OpEq(o),
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '^' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Caret,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '<' && i + 1 < cs.len() && cs[i + 1] == '<' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::Shl,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '>' && i + 1 < cs.len() && cs[i + 1] == '>' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::Shr,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '&' && (i + 1 >= cs.len() || cs[i + 1] != '&') {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Amp,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '+' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::PlusEq,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '-' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::MinusEq,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '*' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::StarEq,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '/' && i + 1 < cs.len() && cs[i + 1] == '=' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::SlashEq,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '@' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::At,
                    lo: st,
                    hi: i as u32,
                });
            }
            // LEX-AT: `@` was not in the table and lexed as Eof -- a pattern `rest @ ..` silently ENDED the token stream (accepted, then segfaulted)
            else if c == '?' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Quest,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '.' && i + 1 < cs.len() && cs[i + 1] == '.' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::DotDot,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '|' && i + 1 < cs.len() && cs[i + 1] == '|' {
                i = i + 2;
                out.push(Tok {
                    k: Tk::OrOr,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '|' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Bar,
                    lo: st,
                    hi: i as u32,
                });
            } else if c == '!' {
                i = i + 1;
                out.push(Tok {
                    k: Tk::Bang,
                    lo: st,
                    hi: i as u32,
                });
            } else {
                i = i + 1;
                out.push(Tok {
                    k: k2,
                    lo: st,
                    hi: i as u32,
                });
            }
        }
    }
    let e = cs.len() as u32;
    out.push(Tok {
        k: Tk::Eof,
        lo: e,
        hi: e,
    });
    out
}
// CFG-ITEM: evaluate a cfg predicate against the ORACLE's configuration (rustc, no -O, x86_64 Linux): 1 true, 0 false,
// -1 not understood (the item is then KEPT, as before). `all(..)`, `any(..)`, `not(..)`, a name, `key = "value"`.
// Byte loops only: `str::contains(char)` / slicing / trim miscompile under SEM0 (it segfaulted on `#[cfg(unix)]`).
pub fn cfg_sub(b: &[u8], from: usize, to: usize) -> String {
    let mut r = String::new();
    let mut j = from;
    while j < to && j < b.len() {
        r.push(b[j] as char);
        j = j + 1;
    }
    r
}
pub fn cfg_pre(b: &[u8], p: &str) -> bool {
    let pb = p.as_bytes();
    if b.len() < pb.len() {
        return false;
    }
    let mut j = 0;
    while j < pb.len() {
        if b[j] != pb[j] {
            return false;
        }
        j = j + 1;
    }
    true
}
pub fn cfg_eval(s0: &str) -> i32 {
    let ob = s0.as_bytes();
    let mut lo = 0;
    let mut hi = ob.len();
    while lo < hi && ob[lo] == 32 {
        lo = lo + 1;
    }
    while hi > lo && ob[hi - 1] == 32 {
        hi = hi - 1;
    }
    let s = cfg_sub(ob, lo, hi);
    let b = s.as_bytes();
    let n = b.len();
    let closes = n > 0 && b[n - 1] == 41;
    if cfg_pre(b, "not(") && closes {
        let r = cfg_eval(cfg_sub(b, 4, n - 1).as_str());
        if r < 0 {
            return 0 - 1;
        }
        return 1 - r;
    }
    let isall = cfg_pre(b, "all(");
    let isany = cfg_pre(b, "any(");
    if (isall || isany) && closes {
        let mut parts: Vec<String> = Vec::new();
        let mut cur = String::new();
        let mut d = 0;
        let mut j = 4;
        while j < n - 1 {
            if b[j] == 40 {
                d = d + 1;
            }
            if b[j] == 41 {
                d = d - 1;
            }
            if b[j] == 44 && d == 0 {
                parts.push(cur);
                cur = String::new();
            } else {
                cur.push(b[j] as char);
            }
            j = j + 1;
        }
        if cur.len() > 0 {
            parts.push(cur);
        }
        let mut acc = if isall { 1 } else { 0 };
        let mut k = 0;
        while k < parts.len() {
            let r = cfg_eval(parts[k].as_str());
            if r < 0 {
                return 0 - 1;
            }
            if isall && r == 0 {
                acc = 0;
            }
            if isany && r == 1 {
                acc = 1;
            }
            k = k + 1;
        }
        return acc;
    }
    let mut norm = String::new();
    let mut q = 0;
    while q < n {
        if b[q] == 40 {
            return 0 - 1;
        }
        if b[q] != 32 {
            norm.push(b[q] as char);
        }
        q = q + 1;
    }
    if norm.as_str() == "unix"
        || norm.as_str() == "debug_assertions"
        || norm.as_str() == "target_os=\"linux\""
        || norm.as_str() == "target_family=\"unix\""
        || norm.as_str() == "target_arch=\"x86_64\""
        || norm.as_str() == "target_pointer_width=\"64\""
        || norm.as_str() == "target_endian=\"little\""
        || norm.as_str() == "target_env=\"gnu\""
        || norm.as_str() == "panic=\"unwind\""
        || norm.as_str() == "true"
    {
        return 1;
    }
    if norm.len() == 0 {
        return 0 - 1;
    }
    0
}
// R13 DERIVE-ORD: does a `#[derive(..)]` attribute list name `Ord` itself (not only `PartialOrd`)?
fn has_derive_ord(t: &str) -> bool {
    if !t.contains("derive") {
        return false;
    }
    let b = t.as_bytes();
    let mut i = 0;
    while i + 3 <= b.len() {
        if b[i] == 79 && b[i + 1] == 114 && b[i + 2] == 100 {
            let pre_ok = i == 0 || !(b[i - 1] as char).is_ascii_alphanumeric();
            let post_ok = i + 3 == b.len() || !(b[i + 3] as char).is_ascii_alphanumeric();
            if pre_ok && post_ok {
                return true;
            }
        }
        i = i + 1;
    }
    false
}
fn lt_ch(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}
// REGRESS-5 E0582: in `for<'a, ..> BOUND`, a bound lifetime that occurs in the bound's output part (after `->`, or after
// the first `=` of an associated-type binding) but never before it is unconstrained -- rustc E0582. The bound ends at a
// depth-0 `,` `;` `{` `)` or `>`.
fn hr_lt_output_only(cs: &Vec<char>) -> bool {
    let mut i = 0;
    while i + 4 < cs.len() {
        if cs[i] == 'f'
            && cs[i + 1] == 'o'
            && cs[i + 2] == 'r'
            && cs[i + 3] == '<'
            && cs[i + 4] == '\''
            && (i == 0 || !lt_ch(cs[i - 1]))
        {
            let mut lts: Vec<String> = Vec::new();
            let mut j = i + 4;
            while j < cs.len() && cs[j] != '>' {
                if cs[j] == '\'' {
                    let mut s = String::new();
                    j = j + 1;
                    while j < cs.len() && lt_ch(cs[j]) {
                        s.push(cs[j]);
                        j = j + 1;
                    }
                    lts.push(s);
                } else {
                    j = j + 1;
                }
            }
            let bs = j + 1;
            // REGRESS-E: a `for<'a> fn(..) -> R` POINTER type: its output is only a `->` outside the parens (an `Assoc = T`
            // inside a parameter type is not the output)
            let mut fb = bs;
            while fb < cs.len() && cs[fb] == ' ' {
                fb = fb + 1;
            }
            let fnptr =
                fb + 2 < cs.len() && cs[fb] == 'f' && cs[fb + 1] == 'n' && !lt_ch(cs[fb + 2]);
            let mut d = 0;
            let mut ob = 0;
            let mut e = bs;
            while e < cs.len() {
                let c = cs[e];
                if c == '-' && e + 1 < cs.len() && cs[e + 1] == '>' {
                    if ob == 0 && (!fnptr || d == 0) {
                        ob = e;
                    }
                    e = e + 2;
                    continue;
                }
                if c == '=' && ob == 0 && !fnptr && !(e + 1 < cs.len() && cs[e + 1] == '=') {
                    ob = e;
                }
                if c == '(' || c == '<' || c == '[' {
                    d = d + 1;
                } else if c == ')' || c == '>' || c == ']' {
                    if d == 0 {
                        break;
                    }
                    d = d - 1;
                } else if d == 0 && (c == ',' || c == ';' || c == '{') {
                    break;
                }
                e = e + 1;
            }
            if ob > 0 {
                let mut k = 0;
                while k < lts.len() {
                    let mut inp = false;
                    let mut out = false;
                    let mut q = bs;
                    while q < e {
                        if cs[q] == '\'' {
                            let mut s = String::new();
                            let mut r = q + 1;
                            while r < e && lt_ch(cs[r]) {
                                s.push(cs[r]);
                                r = r + 1;
                            }
                            if s.as_str() == lts[k].as_str() {
                                if q < ob {
                                    inp = true;
                                } else {
                                    out = true;
                                }
                            }
                        }
                        q = q + 1;
                    }
                    if out && !inp {
                        return true;
                    }
                    k = k + 1;
                }
            }
            i = e;
        } else {
            i = i + 1;
        }
    }
    false
}
// REGRESS-5: `#[derive(..)]` names other than the builtin derives (last path segment) are proc macros, which ourc
// cannot run -- the derive's output was silently dropped and the program ACCEPTED (issue-104884).
fn derive_unknown(t: &str) -> bool {
    let b = t.as_bytes();
    let mut i = 0;
    while i < b.len() && (b[i] == 91 || b[i] == 32) {
        i = i + 1;
    }
    if !(i + 7 <= b.len() && &t[i..i + 7] == "derive(") {
        return false;
    }
    i = i + 7;
    let mut w = String::new();
    let mut pathq = false;
    while i < b.len() {
        let c = b[i] as char;
        if c.is_ascii_alphanumeric() || c == '_' {
            w.push(c);
        } else if c == ':' {
            w = String::new();
            pathq = true; // REGRESS-E: a path-qualified derive (`aux::Embed`) names an external macro: not judged
        } else if c == ',' || c == ')' {
            if w.len() > 0 && !pathq {
                let ok = w == "Clone"
                    || w == "Copy"
                    || w == "Debug"
                    || w == "Default"
                    || w == "PartialEq"
                    || w == "Eq"
                    || w == "PartialOrd"
                    || w == "Ord"
                    || w == "Hash";
                if !ok {
                    return true;
                }
            }
            w = String::new();
            pathq = false;
            if c == ')' {
                return false;
            }
        }
        i = i + 1;
    }
    false
}
fn has_word(t: &str, w: &str) -> bool {
    let b = t.as_bytes();
    let n = w.len();
    let mut i = 0;
    while i + n <= b.len() {
        if &t[i..i + n] == w
            && (i == 0 || !lt_ch(b[i - 1] as char))
            && (i + n == b.len() || !lt_ch(b[i + n] as char))
        {
            return true;
        }
        i = i + 1;
    }
    false
}
// the integer width named in a `[repr(..)]` attribute text (`repr(u8)`, `repr(C, i32)`), or "" (self-host-safe: no arrays)
fn repr_int_width(t: &str) -> String {
    let all = "u8 u16 u32 u64 u128 usize i8 i16 i32 i64 i128 isize ";
    let ab = all.as_bytes();
    let mut w = String::new();
    let mut i = 0;
    while i < ab.len() {
        if ab[i] == 32 {
            if has_word(t, w.as_str()) {
                return w;
            }
            w = String::new();
        } else {
            w.push(ab[i] as char);
        }
        i = i + 1;
    }
    String::new()
}
// A8a SIG-REGION: `fn f<'a, 'b>(x: &'a T, y: &'b T) -> &'a T { .. y .. }` -- a body that RETURNS a parameter whose
// reference lifetime is not the declared return lifetime (no outlives bound) is rustc's "lifetime may not live long
// enough". Lifetimes are dropped by this lexer, so the check runs on the text. Only a block whose whole tail is the
// parameter (`{ y }`, `return y;`) is judged, and only lifetimes the fn itself declares; outlives bounds or a where clause skip it.
fn srm_idc(c: char) -> bool {
    (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || (c >= '0' && c <= '9') || c == '_'
}
fn srm_ws(cs: &Vec<char>, i0: usize) -> usize {
    let mut i = i0;
    while i < cs.len() && (cs[i] == ' ' || cs[i] == '\n' || cs[i] == '\t' || cs[i] == '\r') {
        i = i + 1;
    }
    i
}
fn srm_word_end(cs: &Vec<char>, i0: usize) -> usize {
    let mut i = i0;
    while i < cs.len() && srm_idc(cs[i]) {
        i = i + 1;
    }
    i
}
fn srm_word(cs: &Vec<char>, a: usize, b: usize) -> String {
    let mut s = String::new();
    let mut i = a;
    while i < b {
        s.push(cs[i]);
        i = i + 1;
    }
    s
}
// the lifetime of a `&'L` at i (after whitespace), or "" when there is none
fn srm_ref_lt(cs: &Vec<char>, i0: usize) -> String {
    let i = srm_ws(cs, i0);
    if i + 2 < cs.len() && cs[i] == '&' && cs[i + 1] == '\'' {
        let e = srm_word_end(cs, i + 2);
        return srm_word(cs, i + 2, e);
    }
    String::new()
}
pub fn sig_region_mismatch(cs: &Vec<char>) -> usize {
    let n = cs.len();
    let mut i = 0;
    while i + 3 < n {
        if cs[i] == 'f' && cs[i + 1] == 'n' && cs[i + 2] == ' ' && (i == 0 || !srm_idc(cs[i - 1])) {
            let r = srm_fn(cs, i + 3);
            if r < n {
                return r;
            }
        }
        i = i + 1;
    }
    n
}
fn srm_fn(cs: &Vec<char>, j0: usize) -> usize {
    let n = cs.len();
    let mut j = srm_ws(cs, j0);
    j = srm_word_end(cs, j);
    let mut gl: Vec<String> = Vec::new();
    j = srm_ws(cs, j);
    if j < n && cs[j] == '<' {
        let mut d = 0;
        while j < n {
            if cs[j] == '\'' {
                let e = srm_word_end(cs, j + 1);
                gl.push(srm_word(cs, j + 1, e));
            }
            if cs[j] == '<' {
                d = d + 1;
            } else if cs[j] == '>' {
                d = d - 1;
                if d == 0 {
                    break;
                }
            } else if cs[j] == ':' {
                return n;
            }
            j = j + 1;
        }
        j = j + 1;
    }
    j = srm_ws(cs, j);
    if j >= n || cs[j] != '(' {
        return n;
    }
    j = j + 1;
    let mut names: Vec<String> = Vec::new();
    let mut lts: Vec<String> = Vec::new();
    let mut d = 0;
    let mut seg = true;
    while j < n {
        let c = cs[j];
        if seg && d == 0 {
            let mut a = srm_ws(cs, j);
            let e0 = srm_word_end(cs, a);
            if srm_word(cs, a, e0).as_str() == "mut" {
                a = srm_ws(cs, e0);
            }
            let e = srm_word_end(cs, a);
            let k = srm_ws(cs, e);
            if e > a && k < n && cs[k] == ':' && !(k + 1 < n && cs[k + 1] == ':') {
                names.push(srm_word(cs, a, e));
                lts.push(srm_ref_lt(cs, k + 1));
            }
            seg = false;
        }
        if c == '(' || c == '[' || c == '<' {
            d = d + 1;
        } else if c == ']' || c == '>' {
            d = d - 1;
        } else if c == ')' {
            if d == 0 {
                break;
            }
            d = d - 1;
        } else if c == ',' && d == 0 {
            seg = true;
        }
        j = j + 1;
    }
    j = srm_ws(cs, j + 1);
    if !(j + 1 < n && cs[j] == '-' && cs[j + 1] == '>') {
        return n;
    }
    let rl = srm_ref_lt(cs, j + 2);
    if rl.len() == 0 || !srm_has(&gl, &rl) {
        return n;
    }
    while j < n && cs[j] != '{' {
        if cs[j] == ';' || (cs[j] == 'w' && j + 5 < n && srm_word(cs, j, j + 5).as_str() == "where")
        {
            return n;
        }
        j = j + 1;
    }
    let b0 = j;
    let mut bd = 0;
    let mut q = b0;
    while q < n {
        if cs[q] == '{' {
            bd = bd + 1;
            let a = srm_ws(cs, q + 1);
            let e = srm_word_end(cs, a);
            let z = srm_ws(cs, e);
            if e > a && z < n && cs[z] == '}' && !srm_upper_before(cs, b0, q) {
                let hit = srm_bad(&gl, &names, &lts, &rl, &srm_word(cs, a, e));
                if hit {
                    return a;
                }
            }
        } else if cs[q] == '}' {
            bd = bd - 1;
            if bd == 0 {
                return n;
            }
        } else if cs[q] == 'r'
            && q + 7 < n
            && !srm_idc(cs[q - 1])
            && srm_word(cs, q, q + 7).as_str() == "return "
        {
            let a = srm_ws(cs, q + 7);
            let e = srm_word_end(cs, a);
            let z = srm_ws(cs, e);
            if e > a
                && z < n
                && cs[z] == ';'
                && srm_bad(&gl, &names, &lts, &rl, &srm_word(cs, a, e))
            {
                return a;
            }
        } else if cs[q] == '"' || cs[q] == '|' {
            // a string or a closure: not judged
            return n;
        }
        q = q + 1;
    }
    n
}
// `Name { x }` is a struct literal, not a block
fn srm_upper_before(cs: &Vec<char>, lo: usize, q: usize) -> bool {
    let mut k = q;
    while k > lo && (cs[k - 1] == ' ' || cs[k - 1] == '\n' || cs[k - 1] == '\t') {
        k = k - 1;
    }
    let mut a = k;
    while a > lo && srm_idc(cs[a - 1]) {
        a = a - 1;
    }
    a < k && cs[a] >= 'A' && cs[a] <= 'Z'
}
fn srm_bad(
    gl: &Vec<String>,
    names: &Vec<String>,
    lts: &Vec<String>,
    rl: &String,
    w: &String,
) -> bool {
    let mut i = 0;
    while i < names.len() {
        if names[i].as_str() == w.as_str() {
            let l = &lts[i];
            return l.len() > 0
                && l.as_str() != rl.as_str()
                && l.as_str() != "static"
                && srm_has(gl, l);
        }
        i = i + 1;
    }
    false
}
// both lifetimes must be the fn's OWN parameters: an impl header's lifetimes carry implied bounds not judged here
fn srm_has(gl: &Vec<String>, l: &String) -> bool {
    let mut i = 0;
    while i < gl.len() {
        if gl[i].as_str() == l.as_str() {
            return true;
        }
        i = i + 1;
    }
    false
}
// R2a NONEXH: does an `enum` / `struct` item (optionally `pub` / `pub(..)`) follow position i (past whitespace and further
// attributes)? A `#[non_exhaustive]` on a VARIANT is followed by the variant's name instead.
fn nonexh_on_item(cs: &Vec<char>, i0: usize) -> bool {
    let mut i = i0;
    loop {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == '\n' || cs[i] == '\t' || cs[i] == '\r') {
            i = i + 1;
        }
        if i < cs.len() && cs[i] == '#' {
            while i < cs.len() && cs[i] != ']' {
                i = i + 1;
            }
            i = i + 1;
            continue;
        }
        break;
    }
    let mut w = String::new();
    let mut k = i;
    while k < cs.len() && w.len() < 24 && cs[k] != '{' && cs[k] != '(' && cs[k] != '<' {
        w.push(cs[k]);
        k = k + 1;
    }
    w.starts_with("enum ")
        || w.starts_with("struct ")
        || w.starts_with("pub enum ")
        || w.starts_with("pub struct ")
        || w.starts_with("pub")
}
