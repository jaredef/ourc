#!/usr/bin/env bash
# R0 boundary map -- probe real-Rust features through the derived toolchain AND rustc (the oracle), and
# categorize each. Turns "subset" into a measured contract; the highest-value output is the MIS-ACCEPT set
# (derived compiles but disagrees with rustc, or accepts what rustc rejects) -- the dangerous silent cases.
#
#   TC=/path/to/assembled/toolchain  bash map.sh
#
# Categories per probe:
#   OK        both build+run, stdout+exit match rustc              (SUPPORTED)
#   GAP       derived rejects, rustc accepts                       (a clean missing feature)
#   MISCOMP   both build, outputs DIFFER                           (MIS-ACCEPT: silent miscompile)
#   UNSOUND   derived builds+runs, rustc REJECTS                   (MIS-ACCEPT: accepts invalid code)
#   REJECT-OK both reject                                          (correct rejection)
#   RUSTC?    rustc itself failed to build a valid probe (harness bug -- excluded)
set -uo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
TC="${TC:?set TC=/path/to/assembled/toolchain (rzc + stages)}"
W="$(mktemp -d)"; PROBES="$HERE/probes"; mkdir -p "$PROBES"
declare -a NAMES; declare -A CAT

probe(){ # $1 = name ; probe file already written
  local name="$1"; local src="$PROBES/$name.rs"
  NAMES+=("$name")
  # derived: rzc build -> run  (timeout the build too -- the frontend can loop on an unsupported construct)
  local dbuilt=1 dout="" drc=0
  if timeout 25 "$TC/rzc" build "$src" "$W/$name.elf" >/dev/null 2>"$W/$name.derr"; then
    dout="$(timeout 10 "$W/$name.elf" 2>/dev/null)"; drc=$?
  else dbuilt=0; fi
  # rustc -O -> run
  local rbuilt=1 rout="" rrc=0
  if timeout 30 rustc -O --edition 2021 "$src" -o "$W/$name.host" >/dev/null 2>"$W/$name.rerr"; then
    rout="$(timeout 10 "$W/$name.host" 2>/dev/null)"; rrc=$?
  else rbuilt=0; fi
  # categorize
  local c
  if [ "$rbuilt" = 0 ] && [ "$dbuilt" = 0 ]; then c="REJECT-OK"
  elif [ "$rbuilt" = 0 ] && [ "$dbuilt" = 1 ]; then c="UNSOUND"
  elif [ "$rbuilt" = 1 ] && [ "$dbuilt" = 0 ]; then c="GAP"
  elif [ "$dout" = "$rout" ] && [ "$drc" = "$rrc" ]; then c="OK"
  else c="MISCOMP  d=[$dout/$drc] r=[$rout/$rrc]"; fi
  CAT[$name]="$c"
  printf '  %-22s %s\n' "$name" "$c"    # incremental -- shows progress + survives a later hang
}

W2(){ cat > "$PROBES/$1.rs"; }   # write a probe file from stdin

# ============================ PROBES ============================
# ---- integers / arithmetic ----
W2 int_i64 <<'RS'
fn main(){ let a:i64=5_000_000_000; let b:i64=3; println!("{}", a*b - 1); }
RS
probe int_i64

W2 int_cast_trunc <<'RS'
fn main(){ let x:i32=300; let y=x as u8; println!("{}", y as i32); }
RS
probe int_cast_trunc

W2 bitwise <<'RS'
fn main(){ let a:u32=0xF0; let b:u32=0x0F; println!("{} {} {} {} {}", a|b, a&b, a^b, a<<2, a>>2); }
RS
probe bitwise

W2 unsigned_div <<'RS'
fn main(){ let a:u32=4000000000; let b:u32=7; println!("{} {}", a/b, a%b); }
RS
probe unsigned_div

W2 overflow_wrap <<'RS'
fn main(){ let a:u8=250; let b:u8=10; println!("{}", a.wrapping_add(b)); }
RS
probe overflow_wrap

# ---- inference / bindings ----
W2 infer_let <<'RS'
fn main(){ let x=40; let y=2; let z=x+y; println!("{}", z); }
RS
probe infer_let

W2 tuple <<'RS'
fn main(){ let t=(1,2,3); println!("{}", t.0+t.1+t.2); }
RS
probe tuple

W2 shadowing <<'RS'
fn main(){ let x=5; let x=x*2; let x=x+1; println!("{}", x); }
RS
probe shadowing

# ---- references / arrays / slices ----
W2 mut_ref_fn <<'RS'
fn inc(x:&mut i32){ *x += 5; }
fn main(){ let mut a=10; inc(&mut a); println!("{}", a); }
RS
probe mut_ref_fn

W2 array_slice <<'RS'
fn sum(s:&[i32])->i32{ let mut t=0; for &x in s { t+=x; } t }
fn main(){ let a=[1,2,3,4,5]; println!("{}", sum(&a[1..4])); }
RS
probe array_slice

# ---- structs / enums / methods / generics ----
W2 struct_method <<'RS'
struct P{x:i32,y:i32}
impl P{ fn sum(&self)->i32{ self.x+self.y } }
fn main(){ let p=P{x:3,y:4}; println!("{}", p.sum()); }
RS
probe struct_method

W2 enum_match <<'RS'
enum E{ A(i32), B(i32,i32), C }
fn v(e:E)->i32{ match e { E::A(x)=>x, E::B(a,b)=>a+b, E::C=>0 } }
fn main(){ println!("{}", v(E::A(5))+v(E::B(3,4))+v(E::C)); }
RS
probe enum_match

W2 generic_fn <<'RS'
fn maxg<T:PartialOrd>(a:T,b:T)->T{ if a>b {a} else {b} }
fn main(){ println!("{} {}", maxg(3,7), maxg(9,2)); }
RS
probe generic_fn

W2 generic_struct <<'RS'
struct W<T>{ v:T }
impl<T:Copy> W<T>{ fn get(&self)->T{ self.v } }
fn main(){ let w=W{v:42}; println!("{}", w.get()); }
RS
probe generic_struct

# ---- traits ----
W2 trait_default <<'RS'
trait Greet{ fn base(&self)->i32; fn full(&self)->i32{ self.base()+100 } }
struct S;
impl Greet for S{ fn base(&self)->i32{ 5 } }
fn main(){ let s=S; println!("{}", s.full()); }
RS
probe trait_default

W2 trait_object <<'RS'
trait Shape{ fn area(&self)->i32; }
struct Sq(i32); struct Rect(i32,i32);
impl Shape for Sq{ fn area(&self)->i32{ self.0*self.0 } }
impl Shape for Rect{ fn area(&self)->i32{ self.0*self.1 } }
fn main(){ let v:Vec<Box<dyn Shape>>=vec![Box::new(Sq(3)),Box::new(Rect(2,5))]; let mut t=0; for s in &v { t+=s.area(); } println!("{}",t); }
RS
probe trait_object

W2 operator_overload <<'RS'
use std::ops::Add;
#[derive(Clone,Copy)] struct V{x:i32,y:i32}
impl Add for V{ type Output=V; fn add(self,o:V)->V{ V{x:self.x+o.x,y:self.y+o.y} } }
fn main(){ let a=V{x:1,y:2}; let b=V{x:3,y:4}; let c=a+b; println!("{} {}", c.x, c.y); }
RS
probe operator_overload

W2 assoc_type <<'RS'
trait Container{ type Item; fn first(&self)->Self::Item; }
struct IntBox(i32);
impl Container for IntBox{ type Item=i32; fn first(&self)->i32{ self.0 } }
fn main(){ let b=IntBox(7); println!("{}", b.first()); }
RS
probe assoc_type

# ---- patterns ----
W2 match_guard_range <<'RS'
fn classify(n:i32)->i32{ match n { x if x<0 => -1, 0..=9 => 0, _ => 1 } }
fn main(){ println!("{} {} {}", classify(-3), classify(5), classify(50)); }
RS
probe match_guard_range

W2 or_pattern <<'RS'
fn f(n:i32)->i32{ match n { 1|2|3 => 10, 4|5 => 20, _ => 30 } }
fn main(){ println!("{} {} {}", f(2), f(5), f(9)); }
RS
probe or_pattern

W2 slice_pattern <<'RS'
fn f(s:&[i32])->i32{ match s { [] => 0, [a] => *a, [a,.., b] => a+b } }
fn main(){ let v=[1,2,3,4]; println!("{}", f(&v)); }
RS
probe slice_pattern

W2 binding_at <<'RS'
fn f(n:i32)->i32{ match n { x @ 1..=5 => x*10, _ => 0 } }
fn main(){ println!("{}", f(3)); }
RS
probe binding_at

W2 exhaustive_reject <<'RS'
enum E{ A, B }
fn f(e:E)->i32{ match e { E::A => 1 } }
fn main(){ println!("{}", f(E::A)); }
RS
probe exhaustive_reject

# ---- closures / iterators ----
W2 closure_capture <<'RS'
fn main(){ let base=100; let add=|x:i32| x+base; println!("{}", add(5)); }
RS
probe closure_capture

W2 iter_chain <<'RS'
fn main(){ let s:i32=(1..=5).map(|x| x*x).filter(|x| x%2==1).sum(); println!("{}", s); }
RS
probe iter_chain

W2 iter_collect <<'RS'
fn main(){ let v:Vec<i32>=(1..4).map(|x| x*2).collect(); println!("{} {} {}", v[0], v[1], v[2]); }
RS
probe iter_collect

# ---- error handling ----
W2 option_q <<'RS'
fn half(n:i32)->Option<i32>{ if n%2==0 { Some(n/2) } else { None } }
fn go()->Option<i32>{ let a=half(8)?; let b=half(a)?; Some(a+b) }
fn main(){ println!("{}", go().unwrap()); }
RS
probe option_q

W2 result_q <<'RS'
fn parse(ok:bool)->Result<i32,i32>{ if ok { Ok(7) } else { Err(1) } }
fn go()->Result<i32,i32>{ let a=parse(true)?; Ok(a*2) }
fn main(){ println!("{}", go().unwrap()); }
RS
probe result_q

# ---- std ----
W2 string_ops <<'RS'
fn main(){ let mut s=String::from("ab"); s.push_str("cd"); s.push('e'); println!("{} {}", s, s.len()); }
RS
probe string_ops

W2 str_methods <<'RS'
fn main(){ let s="a,bb,ccc"; let n:usize=s.split(',').count(); println!("{}", n); }
RS
probe str_methods

W2 vec_methods <<'RS'
fn main(){ let mut v=vec![3,1,2]; v.push(0); v.sort(); println!("{} {} {} {}", v[0],v[1],v[2],v[3]); }
RS
probe vec_methods

W2 hashmap <<'RS'
use std::collections::HashMap;
fn main(){ let mut m:HashMap<i32,i32>=HashMap::new(); m.insert(1,10); m.insert(2,20); println!("{}", m.get(&1).unwrap()+m.get(&2).unwrap()); }
RS
probe hashmap

W2 format_specs <<'RS'
fn main(){ println!("{:04} {:>5} {:x}", 42, "hi", 255); }
RS
probe format_specs

W2 debug_derive <<'RS'
#[derive(Debug)] struct P{x:i32,y:i32}
fn main(){ let p=P{x:1,y:2}; println!("{:?}", p); }
RS
probe debug_derive

# ---- ownership (rustc REJECTS -- derived should too) ----
W2 use_after_move <<'RS'
struct S{v:i32}
fn main(){ let a=S{v:1}; let b=a; println!("{}", a.v); let _=b; }
RS
probe use_after_move

W2 double_mut_borrow <<'RS'
fn main(){ let mut x=5; let a=&mut x; let b=&mut x; *a+=1; *b+=1; println!("{}", x); }
RS
probe double_mut_borrow

# ---- macros / const / misc real-Rust ----
W2 macro_rules <<'RS'
macro_rules! sq { ($x:expr) => { $x*$x } }
fn main(){ println!("{}", sq!(6)); }
RS
probe macro_rules

W2 const_item <<'RS'
const N:i32=10;
fn main(){ let a=[0i32;3]; println!("{}", N + a.len() as i32); }
RS
probe const_item

W2 const_generic <<'RS'
fn sumn<const N:usize>(a:[i32;N])->i32{ let mut t=0; for x in a { t+=x; } t }
fn main(){ println!("{}", sumn([1,2,3,4])); }
RS
probe const_generic

W2 impl_trait_ret <<'RS'
fn evens()->impl Iterator<Item=i32>{ (0..10).filter(|x| x%2==0) }
fn main(){ let s:i32=evens().sum(); println!("{}", s); }
RS
probe impl_trait_ret

W2 box_recursive <<'RS'
enum List{ Cons(i32, Box<List>), Nil }
fn sum(l:&List)->i32{ match l { List::Cons(x,n)=>x+sum(n), List::Nil=>0 } }
fn main(){ let l=List::Cons(1,Box::new(List::Cons(2,Box::new(List::Nil)))); println!("{}", sum(&l)); }
RS
probe box_recursive

# ============================ REPORT ============================
declare -A COUNT
for n in "${NAMES[@]}"; do
  c="${CAT[$n]}"; key="${c%% *}"; COUNT[$key]=$(( ${COUNT[$key]:-0} + 1 ))
done
echo "----------------------------------------------------------------"
echo "totals: OK=${COUNT[OK]:-0}  GAP=${COUNT[GAP]:-0}  MISCOMP=${COUNT[MISCOMP]:-0}  UNSOUND=${COUNT[UNSOUND]:-0}  REJECT-OK=${COUNT[REJECT-OK]:-0}  (of ${#NAMES[@]})"
rm -rf "$W"
