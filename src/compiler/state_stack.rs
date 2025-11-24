use std::cell::RefCell;
use std::rc::Rc;

pub struct StateStack {
    top: Rc<RefCell<Layer>>,
}

impl StateStack {
    pub fn new() -> Self {
        Self { top: Layer::new_root() }
    }

    pub fn new_layer(&mut self) {
        let top = Rc::clone(&self.top);
        self.top = Layer::new_leaf(top)
    }

    pub fn drop_layer(&mut self) {
        let parent = self.top.borrow().parent.clone().expect("No parent exists");

        self.top = parent;
    }

    pub fn push(&mut self, name: &str) -> u8 {
        self.top.borrow_mut().push(name.to_string())
    }

    pub fn contains(&self, name: &str) -> bool {
        self.top.borrow().contains(name)
    }

    pub fn get(&self, name: &str) -> u8 {
        self.top.borrow().get(name)
    }
    
    pub fn count(&self) -> u16 {
        self.top.borrow().count()
    }
}

struct Layer {
    data: Vec<String>,
    parent: Option<Rc<RefCell<Layer>>>,
}
impl Layer {
    fn new_root() -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { data: vec![], parent: None }))
    }

    fn new_leaf(parent: Rc<RefCell<Layer>>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self {
            data: vec![],
            parent: Some(parent),
        }))
    }

    fn push(&mut self, name: String) -> u8 {
        self.data.push(name);
        self.data.len() as u8
    }

    fn contains(&self, name: &str) -> bool {
        // TODO: can we avoid .to_string()?
        if self.data.contains(&name.to_string()) {
            return true;
        }
        if let Some(parent) = &self.parent {
            parent.borrow().contains(name)
        } else {
            false
        }
    }

    fn get(&self, name: &str) -> u8 {
        // TODO: can we avoid .to_string()?
        if self.data.contains(&name.to_string()) {
            let position = self
                .data
                .iter()
                .position(|d| d.eq(name))
                .expect(format!("Could not find variable {} in stack", name).as_str());
            return (position + 1) as u8
        }
        if let Some(parent) = &self.parent {
            parent.borrow().get(name)
        } else {
            panic!("Could not find variable {} in stack", name)
        }
    }
    
    fn count(&self) -> u16 {
        self.data.len() as u16
    }
}
