pub mod custom_datastructures {
    use std::{fmt::Display, ops::Index};

    pub enum LinkedList<T> {
        List(T, Box<LinkedList<T>>),
        Nil,
    }

    impl<T> Index<usize> for LinkedList<T> {
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

    // NOT HOW THIS WORKS
    // impl <T: Copy> Iterator for LinkedList<T> {
    //     type Item = T;

    //     fn next(&mut self) -> Option<Self::Item> {
    //         let data = self;
    //         match data {
    //             LinkedList::List(val, next) => {
    //                 self = *next;
    //                 Some(*val)
    //             },
    //             LinkedList::Nil => None
    //         }
    //     }

    // }

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
}

