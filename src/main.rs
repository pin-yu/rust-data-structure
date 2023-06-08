mod linked_list;
use linked_list::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_linked_list() {
        let mut my_list = List::<i32>::new();
        my_list.append(5);
        my_list.append(10);
        my_list.append(15);
        my_list.print();
    }
}

fn main() {}
