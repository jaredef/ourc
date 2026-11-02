# P-MONO-ELEM: generic-fn-over-element-struct is a TWO-BUG composite (root-caused)

Probes: generic_slice_struct_method (cleanest), census_trait_generic_crate, census_layer3 (the last two also bundle
derives/macros/closures on top). Symptom: total<T:Shape>(&[T]) and total_v<T>(&Vec<T>) both return 0 (orc 26).

## Bug 1 (SOLVED, fix in hand) - slice-param mono inference
total<T>(&[T]) was monomorphized as `total$_` (element type unresolved) because infer_gparam's unify_ret can't
bridge a `&[T]` param template against a `&Vec<Rect>` arg (Vec->slice Deref/Unsize coercion). FIX (verified to
flip total$_ -> total$Rect): in infer_gparam, when the param strips to `[G]` and the arg is a Vec<C>/&Vec<C>/[C],
bind G to the element C. (Helpers hir_strip_amp + hir_seq_elem; add before the unify_ret fallback.) NOTE: this
alone does NOT close the probe -- Bug 2 blocks it. Reverted from the tree (no partial commits); redo with Bug 2.
total_v<T>(&Vec<T>) already mono'd to total_v$Rect correctly and STILL returns 0 -> Bug 2 is the primary blocker.

## Bug 2 (PRIMARY, deep, unfixed) - lifted map-closure element method dispatch
Inside total_v$Rect: `v.iter().map(|x| x.area()).sum()`. The lifted closure __closure_* is mis-lowered:
  fn __closure(env, x): v6 = load8(x+8); v7 = load(x); if v7==0 {..}; v11 = call area v6; ret
It treats x (a &Rect element) as if it were an Option/enum (tag-check at offset 0) and calls `area` on x.h
(offset 8), NOT on x. So area() gets a wrong value -> 0. ROOT: the map-closure param x is not typed &Rect (the
iterator element type Rect doesn't flow into the lifted closure's param), so `x.area()` mis-dispatches. Connects
to [[rusteze-lifted-closure-param-typing]] (closure params typed by body heuristics). FIX DIRECTION: type the
lifted map/iter-adapter closure param from the receiver's element type (Vec<Rect>/&[Rect] -> x: &Rect) so
x.method() routes to Rect::area (call area x). Then Bug 1 + Bug 2 together close generic_slice_struct_method;
census_* additionally need their derive/macro/closure bundles.

## Status
Reverted to green (HEAD 63652a35). Both bugs precisely root-caused. Bug 2 is the real rung (closure-element
typing); Bug 1 is a trivial add once Bug 2 lands. Next SEM0 debt after this: layer9 tuple/deref pair,
macro_rules_repetition_stmt, rustcfe arena/dyn, impl_trait_return_lifetime_bound, grow9_closurefield_18.

## UPDATE 2026-09-25 — Bug2 bottoms out in CLOSURE-MONOMORPHIZATION (deep rung, not surgical)
Pursued the closure-typing rung. The map closure `|x| x.area()` is lifted ONCE by the pre-pass (s9_lift_expr,
before emit_s9_mono), so it is emitted GENERICALLY with x: &T (T unresolved). The shared __closure fn is called
by every monomorphization (total$Rect), so it cannot be typed &Rect. Attempted a `$clospelem` env hint threaded
from the map receiver's element type in s9_lift_expr -> s9_emit_closure_fn (both implemented, reverted): CORRECT
for NON-generic `vec![Rect].iter().map(|x| x.area())` (ety resolves to Ty Path Rect), but INERT for the generic
probe because in the pre-pass `s: &[T]` so s9_iter_elem_ty gives "T", not Rect.
ROOT (deepest): a closure INSIDE a generic fn that calls a trait method on the element needs to be MONOMORPHIZED
alongside the fn (a per-instantiation __closure$Rect typed x:&Rect), OR resolved via single-impl dispatch
(Shape::area has exactly 1 impl -> direct call) for the 1-impl case. generic_slice_struct_method is 1-impl
(Rect); census_trait_generic_crate is 2-impl (Rect+Square) and genuinely needs per-instantiation closure mono.
Also: the untyped-x `x.area()` currently mis-lowers as an Option-unwrap shape (load tag@0, payload@8, call area
on payload) -- a wrong fallback for an untyped receiver, worth its own look.
RECOMMENDATION: scope closure-mono-in-generics as its own arc (interlocks with emit_s9_mono + s9_lift). The
$clospelem hint is a correct building block (keep for when non-generic map-over-struct probes appear). For the
loop, move to more tractable pipelines first (layer9 tuple/deref, macro_rules_repetition_stmt).
