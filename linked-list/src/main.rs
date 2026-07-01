use linked_list::LinkedList;

fn main() {
    let mut list = LinkedList::new();

    list.push_back(10);
    list.push_back(20);
    list.push_back(30);

    println!("{:?}", list.pop_front()); // Some(10)
    println!("{:?}", list.pop_front()); // Some(20)
    println!("{:?}", list.pop_front()); // Some(30)
    println!("{:?}", list.pop_front()); // None

    println!("{:#?}", list);

}