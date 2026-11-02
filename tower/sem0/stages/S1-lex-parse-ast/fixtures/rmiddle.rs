use std::marker::PhantomData;

#[derive(Clone, Copy)]
pub struct Ty<'tcx>(&'tcx TyKind<'tcx>);

pub enum TyKind<'tcx> {
    Bool,
    Int(IntTy),
    Ref(&'tcx TyKind<'tcx>, Mutability),
    Adt(DefId, &'tcx [Ty<'tcx>]),
    Tuple(&'tcx [Ty<'tcx>]),
    FnPtr(&'tcx [Ty<'tcx>], Ty<'tcx>),
}

pub enum IntTy { I8, I16, I32, I64, Isize }
pub enum Mutability { Not, Mut }
pub struct DefId { pub krate: u32, pub index: u32 }

pub struct Interner<'tcx> {
    types: Vec<TyKind<'tcx>>,
    _marker: PhantomData<&'tcx ()>,
}

pub trait TypeFoldable<'tcx>: Sized {
    type Output;
    fn fold_with<F: TyFolder<'tcx>>(self, folder: &mut F) -> Self::Output;
}

pub trait TyFolder<'tcx> {
    fn fold_ty(&mut self, t: Ty<'tcx>) -> Ty<'tcx>;
}

impl<'tcx> Ty<'tcx> {
    pub fn kind(self) -> &'tcx TyKind<'tcx> { self.0 }

    pub fn is_scalar(self) -> bool {
        matches!(self.kind(), TyKind::Bool | TyKind::Int(_))
    }
}

impl<'tcx> Interner<'tcx> {
    pub fn intern(&mut self, k: TyKind<'tcx>) -> usize {
        let idx = self.types.len();
        self.types.push(k);
        idx
    }
}

fn describe<'a>(t: &'a TyKind<'a>) -> &'static str {
    match t {
        TyKind::Bool => "bool",
        TyKind::Int(_) => "int",
        TyKind::Ref(..) => "ref",
        TyKind::Adt(_, args) if args.is_empty() => "adt0",
        TyKind::Adt(..) => "adt",
        TyKind::Tuple(elems) => if elems.is_empty() { "unit" } else { "tuple" },
        TyKind::FnPtr(..) => "fnptr",
    }
}
