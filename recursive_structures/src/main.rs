mod datastructures;
use datastructures::custom_datastructures::LinkedList;

fn main() {

    println!("Int List\n--------");
    let a = LinkedList::List(3, Box::new(LinkedList::Nil));
    let b = LinkedList::List(4, Box::new(a));
    let c = LinkedList::List(5, Box::new(b));
    let list = LinkedList::List(6, Box::new(c));
    match list {
        LinkedList::List(val, _) => println!("First element is {}", val),
        LinkedList::Nil => println!("List is empty!"),
    }
    let idx = 2;
    let val = list[idx];
    println!("Element 2 is {}", val);
    println!("List display {}", list);
    println!("Iterating over list:");
    let max = list.into_iter()
                   .max();
    println!("Max is {}", max.unwrap());




    println!("\nString List\n-----------");
    let a = LinkedList::List("a", Box::new(LinkedList::Nil));
    let b = LinkedList::List("b", Box::new(a));
    let c = LinkedList::List("c", Box::new(b));
    let list = LinkedList::List("d", Box::new(c));
    match list {
        LinkedList::List(val, _) => println!("First element is {}", val),
        LinkedList::Nil => println!("List is empty!"),
    }
    let idx = 2;
    let val = list[idx];
    println!("Element 2 is {}", val);
    println!("List display {}", list);


}
