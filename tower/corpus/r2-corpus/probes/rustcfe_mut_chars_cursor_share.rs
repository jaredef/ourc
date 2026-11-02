// RUSTC-FE sub-arc 01 (rustc-lexer) / LEX: a `&mut Chars` cursor passed to a fn must SHARE the caller's
// cursor (a callee .next() advances the caller's `it`). Was: &mut on an SK=13 Chars local fell to the
// scalar eref(slot) path -> the callee got &slot, not the cursor -> the cursor never advanced across the
// call. rustc_lexer threads `chars: &mut Chars` through its unescape functions.
fn adv(cs: &mut std::str::Chars) { cs.next(); }
fn main() {
    let s = "hello";
    let mut it = s.chars();
    adv(&mut it);          // consume 'h'
    adv(&mut it);          // consume 'e'
    let c = it.next().unwrap();   // 'l'
    println!("{}", c as u32);     // 108
}
