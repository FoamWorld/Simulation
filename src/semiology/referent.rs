/// Trait for conceptional binging with real objects.
/// Used to compute shadows.
pub struct Reference {
    expr: Option<String>,
}
impl Reference {
    fn shadow(_: &Referent) -> Self {
        Reference { expr: None }
    }
}

/// Trait for simulating real objects.
/// Tools for management are not included.
pub trait Referent {
    fn is_static(&self) -> bool {
        return true;
    }
    fn is_interactive(&self) -> bool {
        return false;
    }
}
