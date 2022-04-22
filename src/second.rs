type Link<T> = Option<Box<Node<T>>>;

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

    pub fn push(&mut self, i: T) {
        let new_node = Box::new(Node {
            val: i,
            next: self.head.take(),
        });
        self.head.replace(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.val
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.val)
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| &mut node.val)
    }
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();
        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

// create new type for into_iter that has ownership of original list
// IntoIter doesn't need a lifetime identifier because it simply owns entire list
pub struct IntoIter<T>(List<T>);

impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}

impl<T> List<T> {
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }
}

// Iter need a lifetime identifier because compiler needs to ensure that List lives longer that Iter
pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        // type of node here is &'a Node<T>, in closure passed to map, node is another copy of &'a Node<T>
        // &T has implemented Copy trait, so it can be copied with safety
        // Option<&'a Node<T>> is wrapper of &T, so it can also be copied
        self.next.map(|node| {
            // as_ref(&self) can also be helpful but it's kind of boring:
            // next.next.map(|x|{...}) cannot compile here because x would take ownership, and returning reference of variable in local scope is invalid
            // node.next.as_ref() might help to avoiding this, because it converts &Option<T> or Option<T> to Option<&T>
            // node.next.as_ref() convert Option<Box<Node<T>>> to Option<&Box<Node<T>>>
            // so, self.next = node.next.as_ref().map(|x|&**x) is ok:
            // 1. map function have token ownership of converted &Box<Node<T>> instead of Box<T> itself, check
            // 2. *x is borrowed Box<Node<T>>, **x is borrowed Node<T>, &**x is &Node<T>, check
            // self.next = node.next.as_ref().map(|x|&**x);

            // as_deref(&self) here returns Option<&<T as Deref>::Target>, reducing the boring ref and deref ops above:
            // type of node.next is Option<Box<Node<T>>> also known as Link<T>
            // so node.next.as_deref() act as below:
            // 1. if node.next is None, just return None;
            // 2. is node.next is Some(x) where type of x is Box<Node<T>>, return a shared reference of deref inner data
            self.next = node.next.as_deref();

            &node.val
        })
    }
}

impl<T> List<T> {
    fn iter(&self) -> Iter<T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

struct IterMut<'a, T> {
    next: Option<&'a mut Node<T>>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        // use take() to take the ownership of Option<&mut T> to avoid copy that, which is not allowed
        // implementation of take() is simple calling mem::replace(), replace it with None and return original wrapped value
        // after that, map() took its ownership and finally passed it to closure
        self.next.take().map(|node| {
            self.next = node.next.as_deref_mut();
            &mut node.val
        })
    }
}

impl<'a, T> List<T> {
    // TODO: why use '_ instead of 'a ??
    fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {
            next: self.head.as_deref_mut(),
        }
    }
}

mod tests {
    use super::List;

    fn plus_one(i: i32) -> i32 {
        i + 1
    }

    #[test]
    fn option_take_test() {
        // map method of Option<T>
        // 1. None -> None
        // 2. Some(val) -> Some(f(val))

        let mut x1 = Some(1);
        let mut x2 = Some(2);
        let x3 = x1.take();
        assert_eq!(x1, None);
        assert_eq!(x3.unwrap(), 1);
        assert_eq!(x1.take().map(plus_one), None);
        assert_eq!(x2.take().map(plus_one), Some(3));
    }

    #[test]
    fn basic_test() {
        let mut list = List::new();
        assert_eq!(None, list.peek());
        assert_eq!(None, list.peek_mut());

        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');
        assert_eq!(Some(&'d'), list.peek());
        assert_eq!(Some(&mut 'd'), list.peek_mut());

        // peek_mut() returns Option<&mut T>, so the type of val is &mut T
        list.peek_mut().map(|val| {
            *val = 'e';
        });

        assert_eq!(Some('e'), list.pop());
        assert_eq!(Some('c'), list.pop());
        assert_eq!(Some('b'), list.pop());

        list.push('b');
        assert_eq!(Some('b'), list.pop());
        assert_eq!(Some('a'), list.pop());
        assert_eq!(None, list.pop());
        assert_eq!(None, list.pop());
    }

    #[test]
    fn into_iter_test() {
        let mut list = List::new();

        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');

        let mut v = vec!['a', 'b', 'c', 'd'];
        v.reverse();

        // move occurred here, into_iter now has ownership of list
        // into_iter is kind of iterator that consumes original data source
        let into_iter = list.into_iter();
        let mut result = Vec::new();
        for val in into_iter {
            result.push(val);
        }
        assert_eq!(v, result);
    }

    #[test]
    fn iter_test() {
        let mut list = List::new();

        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');

        let mut v = vec!['a', 'b', 'c', 'd'];
        v.reverse();

        let mut result = Vec::new();
        for val in list.iter() {
            result.push(*val);
        }
        assert_eq!(v, result);
    }

    #[test]
    fn iter_mut_test() {
        let mut list = List::new();

        list.push('a');
        list.push('b');
        list.push('c');
        list.push('d');

        let mut iter_mut = list.iter_mut();
        assert_eq!(iter_mut.next(), Some(&mut 'd'));
        assert_eq!(iter_mut.next(), Some(&mut 'c'));
        assert_eq!(iter_mut.next(), Some(&mut 'b'));
        assert_eq!(iter_mut.next(), Some(&mut 'a'));
        assert_eq!(iter_mut.next(), None);
        assert_eq!(iter_mut.next(), None);
        assert_eq!(iter_mut.next(), None);
    }
}
