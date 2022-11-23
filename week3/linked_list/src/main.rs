use linked_list::LinkedList;
pub mod linked_list;
use crate::linked_list::ComputeNorm;

fn main() {
    let mut list1 = LinkedList::new();
    assert!(list1.is_empty());
    assert_eq!(list1.get_size(), 0);

    for i in 1..11 {
        list1.push_front(i);
    }

    println!("{}", list1);
    println!("list size: {}", list1.get_size());
    println!("top element: {}", list1.pop_front().unwrap());
    println!("{}", list1);
    println!("size: {}", list1.get_size());
    println!("{}", list1.to_string()); // ToString impl for anything impl Display

    let list2 = list1.clone();
    assert!(!list1.is_empty());
    assert_eq!(list1.get_size(), 9);
    println!("list1 == list2 : {}", list1 == list2);
    
    // If you implement iterator trait:
    for val in list1 {
        print!("{} ", val);
    }
    println!();
    for val in &list2 {
        print!("{} ", val);
    }
    println!();
    println!("original list : {}", list2);

    let mut list3 = LinkedList::new();
    for i in 1..11 {
        list3.push_front(i as f64);
    }

    println!("list2.compute_norm() = {}", list2.compute_norm());
    println!("list3.compute_norm() = {}", list3.compute_norm());
    
}
