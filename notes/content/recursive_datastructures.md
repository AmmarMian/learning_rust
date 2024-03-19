---
layout: page
title: Recursive data structures
menu:
  main:
    weight: 2
mermaid: true
toc: true
---


In this small project, I try to implement a simple recursive data structure and some algorithms to manipulate it.

<!--more-->

## Linked List

A Linked list is a data structure where we can store an arbitrary number of values in a sequence. Each element in the list is a node that contains a value and a reference to the next node in the list. The last node in the list has a reference to nothing. In Rust though, we don't really use `NULL` to represent nothing, we can encapsulate the Linked list as an `enum` where we have either a `Node` or `None` which is not the same as null pointer in C language.

### Implementing it with a generic enum because why not

Since we want to be able to reuse code for any type, we can use a generic enum to represent the linked list.
```rust

mod custom_datastructures{
    pub enum LinkedList<T>{
        List(T, Box<LinkedList<T>>),
        Nil
    }
}
```

Here, we have an enum `LinkedList` with two variants `List` and `Nil`. `List` contains a value of type `T` and a reference to the next node in the list. `Nil` is the end of the list and contains nothing. Notice that `Box` is very important here, because we need to allocate memory on the heap for the next node in the list.

Creating a list then can be done like this:
```rust
let a = LinkedList::List(3, Box::new(LinkedList::Nil));
let b = LinkedList::List(4, Box::new(a));
let c = LinkedList::List(5, Box::new(b));
let list = LinkedList::List(6, Box::new(c));
```

Its a bit cumbersome but we'll try to define some methods to make it easier to use later.

{{<warning>}}
It is important to understand that accessing an element, like the first one cannot be done like `list.0` **because `list` is the enum and not the stored tuple.**
We should match againt it in this way:
```rust
match list {
    LinkedList::List(val, _) => println!("First element is {}", val),
    LinkedList::Nil => println!("List is empty!")
}
```
In this case `val` is tha value stored and `_` should be the next node but we don't need it.
{{</warning>}}

### Navigating to some index

In order to navigate to some index, we need to loop with a while loop and keep track of the index:
```rust
let index = 3;
let mut i = 0;
let mut list_ref = &list;
while let LinkedList::List(val, ref next) = *list_ref {
  if i == index {
      println!("Value at index {} is {}", index, val);
      break;
  }
  list_ref = next;
  i += 1;
}
```

{{<info>}}
Notice a few things here:
* We do not want to take ownership of list so we only take a reference to it.
* The reference is mutable because at some point, we want to change it to the next node.
* We use `ref` to borrow the next node instead of taking ownership of it. Also if we didn't put `ref` but tried to do something like `list_ref=next.to_ref()` or even `list_ref = &(*next)`, the problem would be that since we took ownership of `next`, it is dropped at this iteration of the loop, so trying to reference it at the following one doesn"t work.
* We use `*` to dereference the reference to the list.
{{</info>}}

### Implementing Index Trait

Some specific syntax is needed when implementing trait:
* First we need to implement the trait for generic `<T>` that we then pass to the enum.
* we need to precise the type of the output of the index, in this case `T`.
* `index` should match this use but the index can be something else than `usize` so we need to specify that it is the index of the list.
```rust
mod custom_datastructures{
    use std::ops::Index;


    pub enum LinkedList<T>{
        List(T, Box<LinkedList<T>>),
        Nil
    }

    impl <T> Index<usize> for LinkedList<T> {
        type Output = T;

        fn index(&self, index: usize) -> &Self::Output {
            let mut curr_element = self;
            let mut i = 0;
            while let LinkedList::List(val, next) = curr_element {
                if i == index {
                    return val;
                }
                i += 1;
                curr_element = next;
            }

            panic!("Index out of bounds!");

                
        }
    }

}
```
{{<info>}}
We do not need the `ref` keyword here because the scope is different: `next` is part of a data structure (`LinkedList`) that lives outside the method's scope, so it's not dropped when the method returns or when a loop iteration ends. The lifetime of next is tied to the lifetime of the list itself, not to the loop or method call. To use it now we can just write `list[3]` and it will return the value at index 3 or panic if the index is out of bounds.
{{</info>}}

### Implementing Display Trait

To implement the `Display` trait, we need to import the `fmt` module and implement the `fmt` method for the `LinkedList` enum. We can then use the `write!` macro to write to a buffer and return the result.
```rust
impl <T:Display> Display for  LinkedList<T> {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
         
        let mut s = String::from("[");
        let mut curr_element = self;
        while let LinkedList::List(val, next) = curr_element {
          s += & match **next {
              LinkedList::List(_, _) => format!("{}, ", val),
              LinkedList::Nil => format!("{}]", val)
          };
            curr_element = next;
        }
        write!(f, "{}", s)
    }
}
```

{{<info>}}
Notice a few things:
* We implement the Trait for types that implement the `Display` trait, so we need to specify that in the generic type. Otherwhise we don't know how to display the value.
* We use `&mut` to take a mutable reference to the formatter. We then write to the buffer `s` and return the result.
* We use the `match` statement to check if the next element is `Nil` or not. If it is, we don't want to add a comma and a space after the value.
* Regarding this, I need to use `**next` because `next` is a reference to a `Box` and `Box` is a smart pointer, so we need to dereference it twice to get the value. It seems the pattern `let LinkedList::List(val, next) = curr_element` works in way I did not anticipate. 
* On the match I have to add a `&` because as usual `+` expected an owned String on the left and a `&str` on the right. At first I did not understand the error while not putting it: `Value dropped before it is used`. That is because I sent back a `&str` on the match statement but the value to the `str` that we refer to is dropped at the end of the match statement. This is why the `&` is needed at the `match` level.
{{</info>}}

I'll rewrite this late to use an `iterator` and `collect` to make it more readable. 

## Trying to implement an Iterator and behold the difficulties

The problems I encountered when trying to implement an iterator are:
* To create `next` I need to to be able to create a mutable reference to the next part
* Not understanding still the intricacies of ownership..

Moreover, when I think about it, adding elements later before of after will be difficult for the same reasons. Maybe a better idea is to have a `struct` for the list and then a `Node` struct for the nodes. In `list` I can then store the reference to the `head` node. 

I should read more from:
https://rust-unofficial.github.io/too-many-lists/index.html
