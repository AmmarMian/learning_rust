mod datastructures;
use datastructures::linked_list::List;

fn main() {

    println!("Int List\n--------");
    let mut list_int = List::new();
    list_int.append(1);
    list_int.append(2);
    list_int.append(3);
    list_int.append(-4);
    println!("List: {}", list_int);
    let _ = list_int.pop_last();
    println!("List after pop: {}", list_int);
    println!("Value at index 1: {}", list_int[1]);
    println!("Iterating over list");
    for value in list_int.iter_mut() {
        println!("{}", value);
    }


    println!("\nString List\n-----------");
    let mut list_string = List::new();
    list_string.append("One".to_string());
    list_string.append("Two".to_string());
    list_string.append("Three".to_string());
    list_string.append("Four".to_string());
    println!("List: {}", list_string);
    let _ = list_string.pop_last();
    println!("List after pop: {}", list_string);
    let _ = list_string.pop_last();
    println!("List after 2 pops: {}", list_string);
    let _ = list_string.pop_last();
    println!("List after 3 pops: {}", list_string);
    let _ = list_string.pop_last();
    println!("List after 4 pops: {}", list_string);
    let _ = list_string.pop_last();
    println!("List after 5 pops: {}", list_string);
    list_string.append("Five".to_string());
    println!("List after appends: {}", list_string);
    let mut new_list = List::new();
    new_list.append("Blabla".to_string());
    new_list.append("Bla".to_string());
    println!("New List: {}", new_list);
    list_string.merge(new_list);
    println!("List after merge: {}", list_string);
}

