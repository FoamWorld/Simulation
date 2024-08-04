use super::referent::Referent;

pub struct Signifier {
	method: String,
	refer: Box<dyn Referent>
}
impl Referent for Signifier {
	fn get(&self, key: String) -> Option<&dyn std::any::Any> {
		return None;
	}
}
