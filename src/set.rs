use std::collections::{HashSet};
use std::fmt;
use std::fmt::Formatter;
use std::hash::Hash;

pub struct Set<T> {
    elements: HashSet<T>,
}

// format set, to_string
impl fmt::Display for Set<i32> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.elements)
    }
}


impl<T: Hash + Eq + Clone > Set<T> {
    // empty set
    pub fn empty() -> Self {
        Set {
            elements: HashSet::new(),
        }
    }
    pub fn new(elements: Vec<T>) -> Self {
        Set{
            elements: elements.into_iter().collect(),
        }
    }
    // using the hashset, use the function insert to add and element to set
    pub fn add(&mut self, element: T) {
        self.elements.insert(element);
    }

    // use contains in hashset
    pub fn contains(&self, element: &T) -> bool{
        self.elements.contains(element)
    }

    pub fn remove(&mut self, element: &T){
        self.elements.remove(element);
    }

    pub fn intersection(&mut self,other: &Set<T>) -> Set<T>{
        // functional to
        let res: HashSet<_> = self
            .elements
            .iter()
            .filter(|&element|other.contains(element))
            .cloned()
            .collect();

        Set{
            elements: res,
        }
    }
}