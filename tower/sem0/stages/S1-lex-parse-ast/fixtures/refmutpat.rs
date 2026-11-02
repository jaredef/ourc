pub struct Foo { head: usize, len: usize }
impl Foo {
    pub fn f(&mut self) {
        let &mut Foo { head, len } = self;
        unsafe { g(head, len); }
    }
    pub fn h(&self) {
        let &Foo { head, len } = self;
        unsafe { g(head, len); }
    }
}
