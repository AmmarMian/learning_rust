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

### Trying to implement an Iterator and behold the difficulties

The problems I encountered when trying to implement an iterator are:
* To create `next` I need to to be able to create a mutable reference to the next part
* Not understanding still the intricacies of ownership..

Moreover, when I think about it, adding elements later before of after will be difficult for the same reasons. Maybe a better idea is to have a `struct` for the list and then a `Node` struct for the nodes. In `list` I can then store the reference to the `head` node. 

I should read more from:
https://rust-unofficial.github.io/too-many-lists/index.html

## Linked list new version

After some reading, I now implement the list like this:
```rust
    type Link<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: Link<T>
    }

    #[derive(Debug)]
    struct Node<T> {
        val: T,
        next: Link<T>
    }
```

This as a few advantages:
* The first node is allocated on the heap as for the remaining while in the case before, it was on the stack
* We don't allocate memory for nothing on the last node.
* It's more natural to handle the head for the list when we will do stuff like merging lists, etc

Full code with tests:
<details>
<summary>Click to see the full code</summary>

```rust
pub mod linked_list{
    use std::{fmt::Display, ops::Index};

    type Link<T> = Option<Box<Node<T>>>;

    pub struct List<T> {
        head: Link<T>
    }

    #[derive(Debug)]
    struct Node<T> {
        val: T,
        next: Link<T>
    }

    impl <T> List<T> {
        
        /// Create a new empty list
        ///
        /// # Example
        ///
        /// ```
        /// let list: List::<i32> = List::new()
        /// ```
        pub fn new() -> Self {
            List::<T> { head: None }
        }

        /// Check if list is empty
        ///
        /// # Example
        ///
        /// ```
        /// let list: List::<i32> = List::new()
        /// assert!(list.is_empty());
        /// ```
        pub fn is_empty(&self) -> bool {
            self.head.is_none()
        }


        /// Push a value at the beginning of list
        ///
        /// # Arguments
        /// val - Value to add to the beginning of list
        ///
        /// # Examples
        /// ```
        /// let list: List::<i32> = List::new()
        /// list.push(32);
        /// ```
        pub fn push(&mut self, val: T){

            let new_node = match self.head.take() {
                Some(node) => Node {val, next: Some(node)},
                None => Node {val, next: None}
            };
            self.head = Some(Box::new(new_node));
            
        }

        /// Append a value at the end of list
        ///
        /// # Arguments
        /// val - Value to add to the end of list
        ///
        /// # Examples
        ///
        /// ```
        /// let list: List::<i32> = List::new()
        /// list.append(32);
        /// ```
        pub fn append(&mut self, val: T) -> () {

            // Create a node for new val
            let new_node = Node {val, next: None};

            // Create a mutable reference to where we are in the list
            // We need &mut self in input to be able to create a mutable reference
            // and we need it to be mutable since we will modify what it points.
            let mut cursor = &mut self.head;
            while let Some(ref mut next_node) = cursor {
                cursor = &mut next_node.next;
            }
            *cursor = Some(Box::new(new_node));
            
        }


        /// Remove last element from list and returns it
        /// 
        /// # Returns
        /// Result<T, &str> - Ok(T) if list is not empty, Err(&str) if list is empty
        pub fn pop_last(&mut self) -> Result<T, &str> {

            // Dealing with empty list
            if self.is_empty(){
                return Err("List is empty!");
            }

            let mut cursor = &mut self.head;
            // Handle the single element list
            if cursor.as_ref().unwrap().next.is_none() {
                let node = cursor.take().unwrap();
                return Ok(node.val);
            }

            // Loop until we reach the last element
            loop {
                if cursor.as_ref().unwrap().next.is_none() {
                    let node = cursor.take().unwrap();
                    return Ok(node.val);
                }
                cursor = &mut cursor.as_mut().unwrap().next;
            }

        }

        /// Get the length of the list
        ///
        /// # Returns
        /// * usize - Length of the list
        pub fn len(&self) -> usize {
            let mut cursor = &self.head;
            let mut len = 0;
            while let Some(ref node) = cursor {
                len += 1;
                cursor = &node.next;
            }
            len
        }

        /// Get a value at a specific index
        ///
        /// # Arguments
        /// index (usize) - Index of the element to get
        ///
        /// # Returns
        /// Option<&T> - Some(&T) if index is valid, None if index is out of bounds
        pub fn get(&self, index: usize) -> Option<&T> {
            let mut cursor = &self.head;
            let mut i = 0;
            while let Some(ref node) = cursor {
                if i == index {
                    return Some(&node.val);
                }
                cursor = &node.next;
                i += 1;
            }
            None
        }


        /// Merge two lists: Append the second list to the first list.
        /// Will consume the second list.
        ///
        /// # Arguments
        /// other (List<T>) - List to merge with the current list
        ///
        /// # Examples
        /// ```
        /// let mut list1: List<i32> = List::new();
        /// list1.append(3);
        /// list1.append(-42);
        /// let mut list2: List<i32> = List::new();
        /// list2.push(32);
        /// list2.push(-4);
        /// list1.merge(list2);
        /// ```
        pub fn merge(&mut self, mut other: List<T>) {

            if !other.is_empty() && !self.is_empty(){
                // Go to last node of self
                let mut cursor = &mut self.head;
                while let Some(ref mut next_node) = cursor {
                    cursor = &mut next_node.next;
                }
                let other_first_node = other.head.take().unwrap();
                *cursor = Some(other_first_node);
            }
            
        }
            

    }

    impl <T> Index<usize> for List<T> {
        type Output = T;

        /// Get some value from the list at a specific index
        ///
        /// # Arguments
        /// * index (usize) - Index of the element to get
        ///
        /// # Returns
        /// * T - Value at the index
        ///
        /// # Panics
        /// Panics if index is out of bounds. Prefer using get() method if
        /// you want to handle out of bounds index.
        /// 
        /// # Example
        /// ```
        /// let list: List::<i32> = List::new()
        /// list.append(32);
        /// list.append(42);
        /// let val = list[1];
        /// assert_eq!(val, 42);
        /// ```
        fn index(&self, index: usize) -> &Self::Output {
            let val = self.get(index);
            match val {
                Some(v) => v,
                None => panic!("Index out of bounds")
            }
        }
            
    }

    impl <T:Display> Display for  List<T> {

        /// Display the list
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
             
            let mut s = String::from("[");
            let mut curr_element = &self.head;
            while let Some(ref node) = curr_element {
                match &node.next {
                    None => s.push_str(&format!("{}", node.val)),
                    _ => s.push_str(&format!("{}, ", node.val)),
                }
                curr_element = &node.next;
            }
            write!(f, "{}]", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::linked_list::List;


    #[test]
    fn new_i32_list() {
        let empty_list: List<i32> = List::new();
        assert!(empty_list.is_empty())
    }

    #[test]
    fn new_str_list() {
        let empty_list: List<&str> = List::new();
        assert!(empty_list.is_empty())
    }

    #[test]
    fn test_i32_len() {
        let mut list: List<i32> = List::new();
        assert_eq!(list.len(), 0);
        list.append(3);
        assert_eq!(list.len(), 1);
        list.append(3);
        assert_eq!(list.len(), 2);
        list.append(3);
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_i32_get() {
        let mut list: List<i32> = List::new();
        list.append(3);
        list.append(-42);
        assert_eq!(list.get(0).unwrap(), &3);
        assert_eq!(list.get(1).unwrap(), &-42);
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn test_i32_index() {
        let mut list: List<i32> = List::new();
        list.append(3);
        list.append(-42);
        assert_eq!(list[0], 3);
        assert_eq!(list[1], -42);
    }

    #[test]
    fn test_i32_append() {
        let mut list: List<i32> = List::new();
        list.append(3);
        list.append(-42);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_i32_pop_last() {
        let mut list: List<i32> = List::new();
        list.append(3);
        list.append(-42);
        let val = list.pop_last().unwrap();
        assert!(!list.is_empty());
        assert_eq!(val, -42);
        assert_eq!(list.len(), 1);
        let val = list.pop_last().unwrap();
        assert!(list.is_empty());
        assert_eq!(val, 3);
    }

    #[test]
    fn test_i32_push() {
        let mut list: List<i32> = List::new();
        list.push(3);
        list.push(-42);
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        let first = list[0];
        let second = list[1];
        assert_eq!(first, -42);
        assert_eq!(second, 3);
    }

    #[test]
    fn test_i32_merge() {
        let mut list1: List<i32> = List::new();
        list1.append(3);
        list1.append(-42);
        let mut list2: List<i32> = List::new();
        list2.push(32);
        list2.push(-4);
        list1.merge(list2);
        assert_eq!(list1.len(), 4);
        assert_eq!(list1[0], 3);
        assert_eq!(list1[1], -42);
        assert_eq!(list1[2], -4);
        assert_eq!(list1[3], 32);
    }

    #[test]
    fn test_string_len() {
        let mut list: List<String> = List::new();
        assert_eq!(list.len(), 0);
        list.append("a".to_string());
        assert_eq!(list.len(), 1);
        list.append("b".to_string());
        assert_eq!(list.len(), 2);
        list.append("c".to_string());
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_string_get() {
        let mut list: List<String> = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        assert_eq!(list.get(0).unwrap(), &"a".to_string());
        assert_eq!(list.get(1).unwrap(), &"b".to_string());
        assert_eq!(list.get(2), None);
    }

    #[test]
    fn test_string_index() {
        let mut list: List<String> = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        assert_eq!(list[0], "a");
        assert_eq!(list[1], "b");
    }

    #[test]
    fn test_string_append() {
        let mut list: List<String> = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);
    }

    #[test]
    fn test_string_pop_last() {
        let mut list: List<String> = List::new();
        list.append("a".to_string());
        list.append("b".to_string());
        let val = list.pop_last().unwrap();
        assert!(!list.is_empty());
        assert_eq!(val, "b");
        assert_eq!(list.len(), 1);
        let val = list.pop_last().unwrap();
        assert!(list.is_empty());
        assert_eq!(val, "a");
    }

    #[test]
    fn test_string_push() {
        let mut list: List<String> = List::new();
        list.push("a".to_string());
        list.push("b".to_string());
        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        let first = &list[0];
        let second = &list[1];
        assert_eq!(first, "b");
        assert_eq!(second, "a");
    }

}

```
</details>

</br>

Here notice:
* For push we used an approach that takes ownership of `cursor` but we can still write `*cursor=SOME_VALUE` because we used `ref`. Now if we break the loop before reassigning value, we still had a mutable reference in the `next_node` variable.
* That's why (Thanks Copilot), I had to use a cosntruction with `loop` so as to only take ownership of the node of `cursor` when I want to pop it.


### Implementing an iterator

We do something like:

```rust
    // Iterator patterns
    // -------------------------------

    /// Iterator over the list
    pub struct Iter<'a, T> {
        next: Option<&'a Node<T>>
    }
 
    impl <'a, T> Iterator for Iter<'a, T> {
        type Item = &'a T;

        /// Get the next element in the list
        fn next(&mut self) -> Option<Self::Item> {
            match self.next {
                Some(node) => {
                    self.next = node.next.as_deref();
                    Some(&node.val)
                },
                None => None
            }
        }
    }

    /// Mutable iterator over the list
    pub struct IterMut<'a, T> {
        next: Option<&'a mut Node<T>>
    }

    impl <'a, T> Iterator for IterMut<'a, T> {
        type Item = &'a mut T;

        /// Get the next element in the list
        fn next(&mut self) -> Option<Self::Item> {
            match self.next.take() {
                Some(node) => {
                    self.next = node.next.as_deref_mut();
                    Some(&mut node.val)
                },
                None => None
            }
        }
    }

    impl <T> List<T> {
        pub fn iter(&self) -> Iter<T> {
            Iter {next: self.head.as_deref()}
        }
        pub fn iter_mut(&mut self) -> IterMut<T> {
            IterMut {next: self.head.as_deref_mut()}
        }
    }
```

Here we defined both **consuming** and **non-consumming** iterators. The consuming one is used when we want to iterate over the list without caring about the list after. The non-consuming one is used when we want to modify the list while iterating over it. 

Key points are:
* We construct a `st` struct with a `next` field that is an `Option` of a reference to a `Node`. We then implement the `Iterator` trait for this struct. This is the idiomatic way rust does it.
* We use `as_deref` and `as_deref_mut` to get a reference to the value of the node. This is because `next` is an `Option` of a `Box` and we want to get a reference to the value of the `Box`.
* In this approach we then need lifetime annotations to specify that the reference is valid for the lifetime `'a` of the `Iter` struct.

{{<info>}}
They have a better approach with `peek` and other subtleties at [https://rust-unofficial.github.io/too-many-lists/second.html](https://rust-unofficial.github.io/too-many-lists/second.html). But I got bored of lists in this project so I stop here. I'll implement algorithms with `LinkedList` standard library.
{{</info>}}
