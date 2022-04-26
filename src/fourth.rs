use std::cell::Ref;
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

    fn push_back(&mut self, val: T) {
        let new_tail = Node::new(val);
        match self.tail.take() {
            None => {
                self.tail = Some(new_tail.clone());
                self.head = Some(new_tail);
            }
            Some(old_tail) => {
                old_tail.borrow_mut().next = Some(new_tail.clone());
                new_tail.borrow_mut().prev = Some(old_tail);
                self.tail = Some(new_tail);
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

    fn pop_back(&mut self) -> Option<T> {
        self.tail.take().map(|old_tail| {
            match old_tail.borrow_mut().prev.take() {
                Some(new_tail) => {
                    new_tail.borrow_mut().next.take();
                    self.tail = Some(new_tail);
                }
                None => {
                    self.head.take();
                }
            }
            Rc::try_unwrap(old_tail).ok().unwrap().into_inner().val
        })
    }

    // peek function requires shared reference of the first element.
    // however, a trival shared reference &T cannot be retrieved via RefCell<T>
    // because &T cannot keep track of every reference generated from RefCell<T>
    // so if we want to get reference, use Ref<T> instead of &T
    fn peek_front(&self) -> Option<Ref<T>> {
        self.head  // Option<Rc<RefCell<Node<T>>>>
            .as_ref()  // Option<&Rc<RefCell<Node<T>>>>
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }

    fn peek_back(&self) -> Option<Ref<T>> {
        self.tail// Option<Rc<RefCell<Node<T>>>>
            .as_ref() // Option<&Rc<RefCell<Node<T>>>>
            .map(|node| Ref::map(node.borrow(), |node| &node.val))
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        while self.pop_front().is_some() {}
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    #[test]
    fn basic_test() {
        let mut list = List::<i32>::new();
        // 3 -> 1 -> 2 -> 4
        list.push_front(1);
        list.push_back(2);
        list.push_front(3);
        list.push_back(4);

        assert_eq!(list.pop_front(), Some(3));
        assert_eq!(list.pop_back(), Some(4));
        assert_eq!(list.pop_front(), Some(1));
        assert_eq!(list.pop_back(), Some(2));

        assert_eq!(list.pop_front(), None);
        assert_eq!(list.pop_back(), None);
    }

    #[test]
    fn peek_test() {
        let mut list = List::<i32>::new();
        // 2 -> 1
        list.push_front(1);
        list.push_front(2);
        assert_eq!(&*list.peek_front().unwrap(), &2);
        assert_eq!(&*list.peek_back().unwrap(), &1);
    }
}
