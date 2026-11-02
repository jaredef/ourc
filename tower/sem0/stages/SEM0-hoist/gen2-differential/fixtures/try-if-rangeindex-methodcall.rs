// A SECOND manifestation of the SAME defect as paren-type-one-element.rs, kept as a guard because it is the
// shape found in real source (minimized from rustc 1.83.0 compiler/rustc_expand/src/module.rs, 286 lines).
//
// METHOD NOTE, worth more than the fixture. This was first characterised as a FOUR-WAY CONJUNCTION: it needed
// a `try` block AND an `if` AND a range index AND a method call with an argument, and removing any one of the
// four made gen2 succeed. All of that is true and none of it was the cause. The cause is one line in the TYPE
// parser, and this file contains no parenthesized type in type position at all -- the four elements together
// merely route the parser into a state where it parses `(i)` from `a.extend(i)` as a type.
//
// So: a conjunction can be the ROUTE to a defect rather than its cause. That is the inverse of the recorded
// hazard that a non-discriminating probe invents a conjunction which is not there -- here the conjunction was
// real, reproducible, and still not the thing to fix.
fn f() {
    let r = try {
        if c {
            return s[pos..];
        }
        a.extend(i);
    };
}
