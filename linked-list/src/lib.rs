use std::fmt::Display;

#[derive(Debug)]
pub struct Node<T: Display + PartialOrd> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct LinkedList<T: Display + PartialOrd> {
    head: Option<Box<Node<T>>>,
}

impl<T: Display + PartialOrd> LinkedList<T> {
    pub fn new() -> Self {
        Self { head: None, }
    }

    pub fn is_empty(&self) -> bool {
        self.head.is_none()
    }

    pub fn front(&self) -> Option<&T> {
        if let Some(node) = self.head.as_ref() {
            return Some(&node.value)
        }
        None
    }

    pub fn push_front(&mut self, val: T) {
        //save pointer to head
        let old_head = self.head.take();
        //create the new node
        let new_node = Node {
            value: val,
            next: old_head,
        };
        //update the head
        self.head = Some(Box::new(new_node));
    }

    pub fn traverse_list(&self) {
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            println!("{}", node.value);

            current = node.next.as_ref();
        }
    }

    pub fn push_back(&mut self, value: T) {
        if self.head.is_none() {
            self.push_front(value);
            return;
        }

        let mut current = self.head.as_mut().unwrap();

        while current.next.is_some() {
            current = current.next.as_mut().unwrap();
        }

        let new_node = Node {
            value,
            next: None,
        };

        current.next = Some(Box::new(new_node));
    }

    pub fn push_back_2(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        let current = self.head.as_mut(); //for match, no need to unwrap as we will evalauet Some or None

        //matching pattern

        match current {
            None => self.head = Some(new_node),
            Some(mut current) => {
                while current.next.is_some() {
                    current = current.next.as_mut().unwrap();
                }
                current.next = Some(new_node);
            }
        }
    }

    pub fn pop_front(&mut self) -> Option<T> {
        let old_head = self.head.take();

        if let Some(node) = old_head {
            self.head = node.next;
            return Some(node.value)
        }
        None
    }

    pub fn back(&self) -> Option<&T> {
        //this will handle the None case as well
        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if node.next.is_none() {
                return Some(&node.value);    
            }

            current = node.next.as_ref();

        }
        None
    }

    pub fn len(&self) -> usize {
        let mut length = 0;

        let mut current = self.head.as_ref();

        while let Some(node) = current {
            length += 1;
            // if node.next.is_none() {
            //     return length;
            // let the loop end naturally}
            current = node.next.as_ref();
        }
        length
    }

    pub fn contains(&self, value: &T) -> bool
    where
        T: PartialEq {

        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if &node.value == value {
                return true;
            }
            current = node.next.as_ref();
        }
        false
    }

    //pop_back: Use of as_ref(), as_mut(), take(), unwrap()
    pub fn pop_back(&mut self) -> Option<T> {
        if self.head.is_none() {
            return None;
        }
        
        //case 2: only one node
        if self.head.as_ref().unwrap().next.is_none() {
            return self.pop_front();
        }

        //case 3: more than one node; we would like to check for current.next.next and if its Some then only we will move the current
        //else, we are on the second last node and need to get rid of the last_node

        //current will be taken as mut while current's next will be taken as ref; because we will only modifying current
        let mut current = self.head.as_mut().unwrap(); // we are unwrapping fearlessely as we know the value exists and also we need to traverse to next

        while current.next.as_ref().unwrap().next.as_ref().is_some() {
            current = current.next.as_mut().unwrap();
        }

        let last_node = current.next.take().unwrap();

        Some(last_node.value)

    }

    pub fn get(&self, index: usize) -> Option<&T> {

        let mut counter = 0;

        let mut current = self.head.as_ref();

        while let Some(node) = current {
            if counter == index {
                return Some(&node.value);
            }
            counter += 1;
            current = node.next.as_ref();
        }
        None
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        let mut counter = 0;

        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if counter == index {
                return Some(&mut node.value);
            }
            counter += 1;
            current = node.next.as_mut();
        }
        None
    }

    pub fn insert(&mut self, index: usize, val: T) -> bool {
        if index == 0 {
            self.push_front(val);
            return true;
        }

        if self.head.is_none() {
            return false;
        }

        //traverse to index - 1
        let mut counter = 0;
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if counter == index - 1 {
                //we reached the correct location
                // do the insert
                let next_node = node.next.take();
                let new_node = Node {
                    value: val,
                    next: next_node,
                };
                node.next = Some(Box::new(new_node));
                return true;
            }
            counter += 1;
            current = node.next.as_mut();
        }
        false
    }

    pub fn remove(&mut self, index: usize) -> Option<T> {
        if self.head.is_none() {
            return None;
        }

        if index == 0 {
            return self.pop_front();
        }

        let mut counter = 0;
        let mut current = self.head.as_mut();

        while let Some(node) = current {
            if counter == index - 1 {
                if node.next.as_ref().is_some() {
                    let mut removed_node = node.next.take().unwrap();
                    node.next = removed_node.next.take();
                    return Some(removed_node.value);
                }
            }
            /*alternate impl
                if let Some(mut removed_node) = node.next.take() {
                    node.next = removed_node.next.take();
                    return Some(removed_node.value);
                }
             */
            counter += 1;
            current = node.next.as_mut();

        }

        return None;
    }

    pub fn reverse(&mut self) {
        let mut prev = None;
        let mut current = self.head.take();

        while let Some(mut node) = current {
            let next = node.next.take();

            node.next = prev;
            prev = Some(node);

            current = next;
        }
        self.head = prev;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_list_is_empty() {
        let list: LinkedList<i32> = LinkedList::new();
        assert!(list.is_empty());
        assert_eq!(list.len(), 0);
        assert_eq!(list.front(), None);
        assert_eq!(list.back(), None);
    }

    #[test]
    fn push_front_works() {
        let mut list = LinkedList::new();

        list.push_front(10);
        list.push_front(20);
        list.push_front(30);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&30));
        assert_eq!(list.back(), Some(&10));
    }

    #[test]
    fn push_back_works() {
        let mut list = LinkedList::new();

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.len(), 3);
        assert_eq!(list.front(), Some(&10));
        assert_eq!(list.back(), Some(&30));
    }

    #[test]
    fn pop_front_works() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.pop_front(), Some(10));
        assert_eq!(list.pop_front(), Some(20));
        assert_eq!(list.pop_front(), Some(30));
        assert_eq!(list.pop_front(), None);
        assert!(list.is_empty());
    }

     #[test]
    fn pop_back_works() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.pop_back(), Some(30));
        assert_eq!(list.pop_back(), Some(20));
        assert_eq!(list.pop_back(), Some(10));
        assert_eq!(list.pop_back(), None);
        assert!(list.is_empty());
    }

    #[test]
    fn get_works() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&30));
        assert_eq!(list.get(3), None);
    }

     #[test]
    fn get_mut_works() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        if let Some(value) = list.get_mut(1) {
            *value = 99;
        }

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&99));
        assert_eq!(list.get(2), Some(&30));
        assert_eq!(list.get(3), None);
    }

     #[test]
    fn contains_works() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert!(list.contains(&20));
        assert!(list.contains(&30));
        assert!(!list.contains(&99));
    }

    #[test]
    fn insert_works_middle() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert!(list.insert(1, 15));

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&15));
        assert_eq!(list.get(2), Some(&20));

    }

    #[test]
    fn insert_works_front() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert!(list.insert(0, 5));

        assert_eq!(list.get(0), Some(&5));
        assert_eq!(list.get(1), Some(&10));
        assert_eq!(list.get(2), Some(&20));

    }

    #[test]
    fn insert_out_of_bound_fails() {
        let mut list = LinkedList::new();
        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert!(!list.insert(10, 5));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn remove_works_middle() {
        let mut list = LinkedList::new();

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        assert_eq!(list.remove(1), Some(20));

        assert_eq!(list.get(0), Some(&10));
        assert_eq!(list.get(1), Some(&30));
        assert_eq!(list.len(), 2);

    }

     #[test]
    fn remove_out_of_bounds_returns_none() {
        let mut list = LinkedList::new();

        list.push_back(10);

        assert_eq!(list.remove(5), None);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn reverse_works() {
        let mut list = LinkedList::new();

        list.push_back(10);
        list.push_back(20);
        list.push_back(30);

        list.reverse();

        assert_eq!(list.get(0), Some(&30));
        assert_eq!(list.get(1), Some(&20));
        assert_eq!(list.get(2), Some(&10));
    }

    #[test]
    fn reverse_empty() {
        let mut list: LinkedList<i32> = LinkedList::new();

        list.reverse();

        assert!(list.is_empty());
        assert_eq!(list.front(), None);
    }

}