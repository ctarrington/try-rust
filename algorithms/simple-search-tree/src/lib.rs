struct SimpleSearchTree<T> {
    root: Link<T>,
}

impl<T: std::cmp::PartialOrd + Copy> SimpleSearchTree<T> {
    fn new() -> Self {
        SimpleSearchTree {
            root: None,
        }
    }

    fn insert(mut self, new_element: T) {
        match self.root {
            None => self.root = Some(Box::new(Node::new(new_element))),
            Some(mut boxed_node) => boxed_node.insert(new_element),
        }
    }

    fn contains(&self, element: T) -> bool {
        match &self.root {
            None => false,
            Some(boxed_node)=> boxed_node.contains(element),
        }

    }
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    element: T,
}

impl<T: std::cmp::PartialOrd + Copy> Node<T> {
   fn new(element: T) -> Self {
       Node {
           left: None,
           right: None,
           element,
       }
   }

    fn insert(mut self, new_element: T) {
        if new_element <= self.element {
            match self.left {
                None=> self.left = Some(Box::new(Node::new(new_element))),
                Some(mut boxed_node)=> boxed_node.insert(new_element),
            }
        } else {
            match self.right {
                None=> self.right = Some(Box::new(Node::new(new_element))),
                Some(mut boxed_node)=> boxed_node.insert(new_element),
            }
        }
    }

    fn contains(&self, element: T) -> bool {
        if element == self.element {
            true
        } else {
            if element <= self.element {
                match &self.left {
                    None => false,
                    Some(boxed_node)=> boxed_node.contains(element),
                }
            } else {
                match &self.left {
                    None => false,
                    Some(boxed_node) => boxed_node.contains(element),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::SimpleSearchTree;

    #[test]
    fn insertion() {
        let mut tree = SimpleSearchTree::<u32>::new();
        tree.insert(10);
        // `assert!(tree.contains(10));
        //  assert!(!tree.contains(11));
        println!("hi");
    }
}
