use super::referent::Referent;

pub struct Signifier {
	method: String,
	refer: Box<dyn Referent>
}
