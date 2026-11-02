pub enum Abi {
    Rust,
    C { unwind: bool },
    System { unwind: bool },
    Cdecl,
    Stdcall,
    Fastcall,
    Aapcs,
    Win64,
    SysV64,
}

pub enum Endian {
    Little,
    Big,
}

pub struct TargetSpec {
    pub llvm_target: &'static str,
    pub pointer_width: u32,
    pub arch: &'static str,
    pub endian: Endian,
    pub is_like_windows: bool,
    pub has_atomic: bool,
}

pub union Reg {
    pub i: u64,
    pub f: f64,
}

impl Abi {
    pub fn name(&self) -> &'static str {
        match self {
            Abi::Rust => "Rust",
            Abi::C { .. } => "C",
            Abi::System { unwind } if *unwind => "system-unwind",
            Abi::System { .. } => "system",
            Abi::Cdecl => "cdecl",
            Abi::Stdcall | Abi::Fastcall => "x86-conv",
            _ => "other",
        }
    }

    pub fn supports_unwind(&self) -> bool {
        matches!(self, Abi::C { unwind: true } | Abi::System { unwind: true })
    }
}

pub fn pointer_bytes(spec: &TargetSpec) -> u32 {
    spec.pointer_width / 8
}
