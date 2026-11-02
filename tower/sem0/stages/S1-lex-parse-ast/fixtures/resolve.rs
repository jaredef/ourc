mod resolve {
    use std::collections::HashMap;
    use std::fmt::{self, Debug};

    pub const MAX_DEPTH: usize = 64;

    pub enum Res {
        Def(usize, String),
        PrimTy,
        Local { id: u32, name: String },
        Err,
        SelfTy = 7,
        ToolMod = 9,
    }

    pub struct Resolver {
        pub bindings: HashMap<String, Res>,
        pub scopes: Vec<u32>,
        pub depth: usize,
    }

    impl Resolver {
        pub const ROOT: u32 = 0;

        pub fn resolve(&mut self, path: &[String]) -> Result<u32, String> {
            let mut cur = Self::ROOT;
            for (i, seg) in path.iter().enumerate() {
                match self.bindings.get(seg) {
                    Some(Res::Local { id, .. }) => cur = *id,
                    Some(Res::Def(d, name)) => {
                        if name.is_empty() {
                            return Err(format!("empty at {}", i));
                        }
                        cur = *d as u32;
                    }
                    Some(_) => continue,
                    None => {
                        let parent = self.lookup(seg)?;
                        while let Some(top) = self.scopes.pop() {
                            if top == parent {
                                break;
                            }
                        }
                        if let Some(&s) = self.scopes.last() {
                            cur = s;
                        } else {
                            return Err("no scope".to_string());
                        }
                    }
                }
            }
            Ok(cur)
        }

        fn lookup(&self, name: &str) -> Result<u32, String> {
            self.bindings
                .get(name)
                .map(|_| 1u32)
                .ok_or_else(|| format!("unresolved: {}", name))
        }
    }

    pub fn merge<T>(a: Vec<T>, b: Vec<T>) -> Vec<T>
    where
        T: Clone + Debug,
    {
        let mut out = a;
        out.extend(b);
        out
    }
}
