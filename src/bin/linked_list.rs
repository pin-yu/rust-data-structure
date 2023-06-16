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

    pub fn traverse(&self, callback: fn(&T)) {
        let mut cur = match self.head {
            Some(ref head) => {
                callback(&head.val);
                head
            }
            None => return,
        };

        while let Some(ref next) = cur.next {
            callback(&next.val);
            cur = next;
        }
    }
}

pub struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

fn main() {
    // let mut my_list = List::<i32>::new();
    // my_list.append(5);
    // my_list.append(10);
    // my_list.append(15);

    // my_list.traverse(|val| {println!("{}", *val);})
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut my_list = List::<i32>::new();
        
        my_list.append(5);
        my_list.append(10);
        my_list.append(15);

        my_list.traverse(|val| {println!("{}", *val);})
    }
}