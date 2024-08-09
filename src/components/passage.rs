use crate::semiology::referent::{Referent, Void};
use std::{borrow::Borrow, cell::RefCell, rc::Rc};

// trait FnGenerate = Fn(&mut BTreeMap<String, &dyn Any>) -> Rc<RefCell<dyn Referent>>;

pub struct Generator<T>
where
    T: Fn() -> Rc<RefCell<dyn Referent>>,
{
    pub generator: T,
}
impl<T> Generator<T>
where
    T: Fn() -> Rc<RefCell<dyn Referent>>,
{
    fn new(gen: T) -> Self {
        return Generator { generator: gen };
    }
    fn generate(&self) -> Rc<RefCell<dyn Referent>> {
        return (self.generator)();
    }
}
impl<T> Referent for Generator<T> where T: Fn() -> Rc<RefCell<dyn Referent>> {}

pub struct Passage<T>
where
    T: Fn() -> Rc<RefCell<dyn Referent>>,
{
    pub link_src: Rc<RefCell<dyn Referent>>,
    pub link_dest: Rc<RefCell<dyn Referent>>,
    pub gen_dest: Option<Generator<T>>,
}
impl<T> Passage<T>
where
    T: Fn() -> Rc<RefCell<dyn Referent>>,
{
    fn new(src: Rc<RefCell<dyn Referent>>, gen: Generator<T>) -> Self {
        Self {
            link_src: src,
            link_dest: Rc::new(RefCell::new(Void {})),
            gen_dest: Some(gen),
        }
    }
    fn portal(src: Rc<RefCell<dyn Referent>>, dest: Rc<RefCell<dyn Referent>>) -> Self {
        Self {
            link_src: src,
            link_dest: dest,
            gen_dest: None,
        }
    }
    fn pass(&mut self, uncached: bool) -> Rc<RefCell<dyn Referent>> {
        if uncached {
            match self.gen_dest.borrow() {
                Some(x) => {
                    self.link_dest = x.generate();
                }
                None => return Rc::new(RefCell::new(Void {})),
            }
        }
        return self.link_dest.clone();
    }
}
impl<T> Referent for Passage<T>
where
    T: Fn() -> Rc<RefCell<dyn Referent>>,
{
    fn is_interactive(&self) -> bool {
        return true;
    }
}
