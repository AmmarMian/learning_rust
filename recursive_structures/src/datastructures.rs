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

    impl <T> Drop for List<T> {
        fn drop(&mut self) {
            let mut cursor = self.head.take();
            while let Some(mut node) = cursor {
                cursor = node.next.take();
            }
        }
    }

    // Non-consumming iterator patterns
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

