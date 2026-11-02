use std::cell::RefCell;

pub struct TyVid(pub u32);

pub enum InferTy {
    TyVar(TyVid),
    IntVar(u32),
    FloatVar(u32),
    Fresh(u32),
}

pub enum Constraint<'tcx> {
    Eq(InferTy, InferTy),
    Subtype(InferTy, InferTy),
    Outlives(TyVid, &'tcx str),
}

pub struct InferCtxt<'tcx> {
    pub constraints: RefCell<Vec<Constraint<'tcx>>>,
    pub next_var: RefCell<u32>,
}

impl<'tcx> InferCtxt<'tcx> {
    pub fn new() -> Self {
        InferCtxt {
            constraints: RefCell::new(Vec::new()),
            next_var: RefCell::new(0),
        }
    }

    pub fn fresh_var(&self) -> TyVid {
        let mut n = self.next_var.borrow_mut();
        let id = *n;
        *n += 1;
        TyVid(id)
    }

    pub fn eq(&self, a: InferTy, b: InferTy) {
        self.constraints.borrow_mut().push(Constraint::Eq(a, b));
    }

    pub fn is_resolved(&self, t: &InferTy) -> bool {
        !matches!(t, InferTy::TyVar(_) | InferTy::Fresh(_))
    }
}

pub fn count_outlives(cs: &[Constraint<'_>]) -> usize {
    cs.iter().filter(|c| matches!(c, Constraint::Outlives(..))).count()
}
