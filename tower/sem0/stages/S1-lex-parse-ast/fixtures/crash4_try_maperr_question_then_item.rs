// CRASH-4 regression guard: C-SEM0-TRY-MAPERR-QUESTION-THEN-ITEM (census R5).
impl S {
    fn a(&mut self) -> R {
        let e: P<_> = try {
            if p.t != x { }
            expr
        };
        let e = e.map_err(|err| { err.emit(); })?;
    }
    fn b() { }
}
