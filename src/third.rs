use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
}

struct List<T> {
    head: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn perpend(&mut self, val: T) -> List<T> {
        List::<T> {
            head: {
                Some(Rc::new(Node {
                    val,
                    // Option<T> has implementated Copy traits, whose behavior is:
                    // Some(x) => Some(x.clone())
                    // None => None()
                    // so clone() here calling clone() of Rc<T>
                    next: self.head.clone(),
                }))
            },
        }
    }

    pub fn tail(&mut self) -> Option<List<T>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use std::rc::Rc;

    #[test]
    fn option_clone_test() {
        // Rc<T> support clone() for sure, there shall be counter inceasing in that implementation
        let a = Some(Rc::new(1));
        let copied_a = a.clone();
        assert_eq!(a.unwrap(), copied_a.unwrap());

        // it's reasonable that Box<T> support Clone trait
        // it would be useful if you want to clone the managed resource and wrapped it
        // so, Box<T> require that T shall implement Clone trait, too
        let b = Some(Box::new(1));
        let copied_b = b.clone();
        assert_eq!(b.unwrap(), copied_b.unwrap());
    }
}
