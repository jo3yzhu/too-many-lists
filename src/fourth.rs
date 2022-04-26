use std::cell::RefCell;
use std::rc::Rc;

// Rc<RefCell<T>> or Arc<RefCell<T>> is a very common pattern because:
// Rc or Arc provide containers that can be shared, yet they can be only borrowed as shraed
// references, not mutable references. Mutablility would be avaliable if we put a RefCell<T> inside shared pointer
type Link<T> = Option<Rc<RefCell<Node<T>>>>;

struct Node<T> {
    val: T,
    next: Link<T>,
    prev: Link<T>,
}

struct List<T> {
    head: Link<T>,
    tail: Link<T>,
}

impl<T> Node<T> {
    fn new(val: T) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            val,
            prev: None,
            next: None,
        }))
    }
}

impl<T> List<T> {
    fn new() -> List<T> {
        List::<T> {
            head: None,
            tail: None,
        }
    }

    fn push_front(&mut self, val: T) {
        let new_head = Node::new(val);
        match self.head.take() {
            None => {
                self.head = Some(new_head.clone());
                self.tail = Some(new_head);
            }
            Some(old_head) => {
                old_head.borrow_mut().prev = Some(new_head.clone());
                new_head.borrow_mut().next = Some(old_head);
                self.head = Some(new_head);
            }
        }
    }

    fn pop_front(&mut self) -> Option<T> {
        self.head.take().map(|old_head| {
            match old_head.borrow_mut().next.take() {
                Some(new_head) => {
                    new_head.borrow_mut().prev.take();
                    self.head = Some(new_head);
                }
                None => {
                    // when there's only one node in the list, the head
                    // and tail points the same node
                    // so extra removal is required when there's only one node
                    self.tail.take();
                }
            }
            Rc::try_unwrap(old_head).ok().unwrap().into_inner().val
        })
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basic_test() {
        let mut list = List::<i32>::new();
        list.push_front(1);
        list.push_front(2);
        list.push_front(3);
        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_front(), Some(2));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_front(), None);
    }
}
