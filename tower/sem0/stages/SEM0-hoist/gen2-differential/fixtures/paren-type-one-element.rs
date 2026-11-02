// THE defect. gen2 (SEM0-built SEM0) SIGSEGVed on this 18-byte file; gen1 (L6-built) did not.
//
// `s1_parser.rs`'s type parser had `items.remove(0)` for the one-element parenthesized type, and `remove` is
// NOT in s9_method_kind's dispatch (push push_str len clone is_some is_none unwrap as_bytes pop). SEM0 lowered
// it to `call remove` -- a call to a function that does not exist -- so a SEM0-BUILT s1_parser carried a
// dangling call on that one line and died the moment it parsed `(T)`. An L6-built one did not, because L6
// lowers `.remove` correctly. Fixed by using `pop()`, which is the same operation when the length is 1.
//
// The boundary is exact: `()` and `(A,B)` and `(A,B,C)` are all fine, because only the ONE-element case reaches
// that branch. This shape is not exotic -- `Option<&(dyn Error + 'static)>` is idiomatic rustc, and the parens
// there are REQUIRED to bind `+ 'static` to the `dyn`.
//
// One line closed SEVEN of the nine gen1/gen2 divergences in the 192-file rustc sample.
fn s() -> (E) { }
