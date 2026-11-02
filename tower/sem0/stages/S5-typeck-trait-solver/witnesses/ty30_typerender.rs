trait Tr {}
fn p(x: *const u8) {}
fn m(x: *mut u8) {}
fn d(x: &dyn Tr) {}
fn n() -> ! { loop {} }
