pub enum FunctionalTypes {
    Control,
    Concrete,
}

pub trait Referent {
    fn functional_type(&self) -> FunctionalTypes;
} // object

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
