use std::mem;

pub struct List {
    head: Link,
}

type Link = Option<Box<ListNode>>;

struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, None),
        });                                                 
        self.head = Some(new_node); //using Option<Box<ListNode>> instead of having an enum 
    }
    pub fn pop(&mut self) -> Option<i32> {
        match self.head.take() { //instead of using mem::replace(self.head, None)
            None => None,
            Some(node) => {
                
                self.head = node.next; 
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
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