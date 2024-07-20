use super::referent::Referent;
use super::referent::FunctionalTypes;

pub struct Signifier {
	method: String,
	refer: Box<dyn Referent>
}
impl Referent for Signifier {
	fn functional_type(&self) -> FunctionalTypes {
		return FunctionalTypes::Concrete;
	}
}
