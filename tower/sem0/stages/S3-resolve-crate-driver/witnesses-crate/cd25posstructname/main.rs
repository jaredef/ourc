pub struct DefId;
macro_rules! typed_def_id {
    ($Name:ident, $LocalName:ident) => {
        pub struct $Name(DefId);
        pub struct $LocalName(DefId);
    };
}
typed_def_id! { ModDefId, LocalModDefId }
fn main() {}
