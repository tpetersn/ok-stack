use std::mem;

pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
} //making all of our types generic (List, Link and Node)

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });                                                 
        self.head = Some(new_node); //using Option<Box<ListNode>> instead of having an enum 
    }
    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| { //usuing a closure, they can refer to local variables outside of the closure
            self.head = node.next;
            node.elem
        })
    }
    
}

impl<T> Drop for List<T> {
    fn drop(&mut self) {
        let mut cur_link = self.head.take(); //again, instead of using mem::replace(self.head, None) 

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take(); //last time, instead of using mem::replace(boxed_node.next, None)
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;
    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}