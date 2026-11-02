use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId {
    pub krate: u32,
    pub index: u32,
}

#[derive(Clone, Copy)]
pub struct NodeId(pub u32);

#[derive(Clone, Copy)]
pub enum Namespace {
    Type,
    Value,
    Macro,
}

#[derive(Clone)]
pub enum Res {
    Def(DefId, Namespace),
    Local(NodeId),
    SelfTy,
    Err,
}

pub enum ScopeKind {
    Module,
    Block,
    Closure,
}

pub struct Rib {
    kind: ScopeKind,
    bindings: HashMap<String, Res>,
}

impl Rib {
    pub fn new(kind: ScopeKind) -> Rib {
        Rib { kind: kind, bindings: HashMap::new() }
    }

    pub fn get(&self, name: &str) -> Option<Res> {
        match self.bindings.get(name) {
            Some(r) => Some(r.clone()),
            None => None,
        }
    }
}

pub struct Segment {
    pub ident: String,
    pub id: NodeId,
}

pub enum PathResult {
    Resolved(Res),
    Unresolved(String),
    Indeterminate,
}

mod prim {
    pub fn is_prim(name: &str) -> bool {
        match name {
            "u32" => true,
            "bool" => true,
            _ => false,
        }
    }

    pub mod tables {
        pub fn arity() -> u32 {
            5
        }
    }
}

pub struct Resolver<'a> {
    session: &'a str,
    ribs: Vec<Rib>,
    next: u32,
    definitions: HashMap<u32, DefId>,
}

impl<'a> Resolver<'a> {
    pub fn new(session: &'a str) -> Resolver<'a> {
        let ribs = vec![Rib::new(ScopeKind::Module)];
        Resolver { session: session, ribs: ribs, next: 0, definitions: HashMap::new() }
    }

    pub fn fresh(&mut self) -> NodeId {
        let id = self.next;
        self.next = self.next + 1;
        NodeId(id)
    }

    pub fn with_scope<F>(&mut self, kind: ScopeKind, body: F)
    where
        F: FnOnce(&mut Resolver<'a>),
    {
        self.ribs.push(Rib::new(kind));
        body(self);
        self.ribs.pop();
    }

    pub fn define(&mut self, name: impl Into<String>, index: u32) {
        let def = DefId { krate: 0, index: index };
        self.definitions.insert(index, def);
        let last = self.ribs.len() - 1;
        self.ribs[last].bindings.insert(name.into(), Res::Def(def, Namespace::Value));
    }

    pub fn resolve_ident(&self, name: &str) -> Option<Res> {
        let depth = self.ribs.iter().rev().position(|rib| rib.get(name).is_some());
        match depth {
            Some(d) => self.ribs[self.ribs.len() - 1 - d].get(name),
            None => None,
        }
    }

    pub fn resolve_path<P>(&self, segs: Vec<Segment>, pred: P) -> PathResult
    where
        P: FnMut(&Res) -> bool,
    {
        let _ = pred;
        for seg in segs {
            match self.resolve_ident(&seg.ident) {
                Some(res) => return PathResult::Resolved(res),
                None => return PathResult::Unresolved(seg.ident),
            }
        }
        PathResult::Indeterminate
    }
}
