use std::mem;

// zero cost abstraction here because no padding
pub struct List {
    head: Link,
}

// null pointer optimization here
enum Link {
    Empty,
    More(Box<Node>),
}

// all nodes are allocated on heap, all elements are uniformly allocated
struct Node {
    val: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List {
            head: Link::Empty,
        }
    }

    pub fn push(&mut self, elem: i32) {
        // mem::replace did follows
        // 1. set self.head to Empty
        // 2. give original self.head to new_node's next
        // it's definitely a ugly implementation
        let new_node = Box::new(Node {
            val: elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });
        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.val)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
    }
}

mod tests {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        list.push(1);
        list.push(2);
        list.push(3);
        list.push(4);

        assert_eq!(Some(4), list.pop());
        assert_eq!(Some(3), list.pop());
        assert_eq!(Some(2), list.pop());

        list.push(2);
        assert_eq!(Some(2), list.pop());
        assert_eq!(Some(1), list.pop());
        assert_eq!(None, list.pop());
        assert_eq!(None, list.pop());
    }
}


