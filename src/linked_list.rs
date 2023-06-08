use std::fmt::Display;

pub struct List<T> {
    head: Option<Box<Node<T>>>,
}

impl<T> List<T>
where
    T: Display,
{
    pub fn new() -> List<T> {
        List { head: None }
    }

    pub fn append(&mut self, val: T) {
        let mut cur = match self.head {
            Some(ref mut head) => head,
            None => {
                self.head = Some(Box::new(Node { val, next: None }));
                return;
            }
        };

        while let Some(ref mut next) = cur.next {
            cur = next;
        }

        cur.next = Some(Box::new(Node { val, next: None }));
    }

    // TODO: pass a callback function that can do something for each node
    pub fn traverse(&self) {
        let mut cur = match self.head {
            Some(ref head) => {
                println!("{}", head.val);
                head
            }
            None => return,
        };

        while let Some(ref next) = cur.next {
            println!("{}", next.val);
            cur = next;
        }
    }
}

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}
