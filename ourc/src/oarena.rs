pub struct IdArena {
    pub ids: Vec<u32>,
    pub kinds: Vec<u32>,
}
pub fn ar_new() -> IdArena {
    IdArena {
        ids: Vec::new(),
        kinds: Vec::new(),
    }
}
pub fn ar_alloc(a: &mut IdArena, kind: u32) -> u32 {
    let i = a.ids.len() as u32;
    a.ids.push(i);
    a.kinds.push(kind);
    i
}
pub fn ar_kind(a: &IdArena, i: u32) -> u32 {
    a.kinds[i as usize]
}
pub fn ar_len(a: &IdArena) -> u32 {
    a.ids.len() as u32
}
