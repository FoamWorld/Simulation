pub enum FunctionalTypes {
    Control,
    Concrete,
}

/// Trait for simulating real objects.
/// Tools for management are not included.
pub trait Referent {
    fn functional_type(&self) -> FunctionalTypes;
}

pub struct Void {}
impl Referent for Void {
    fn functional_type(&self) -> FunctionalTypes {
        return FunctionalTypes::Concrete;
    }
}

pub struct Container {
    refer: Box<dyn Referent>
}
impl Referent for Container {
    fn functional_type(&self) -> FunctionalTypes {
        return FunctionalTypes::Control;
    }
}
