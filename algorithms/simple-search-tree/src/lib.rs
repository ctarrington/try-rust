pub struct SimpleSearchTree<T> {
    root: Link<T>,
}

impl<T: std::cmp::PartialOrd> SimpleSearchTree<T> {
    pub fn new() -> Self {
        SimpleSearchTree { root: Link::new() }
    }

    pub fn insert(&mut self, new_element: T) {
        self.root.insert(new_element);
    }

    pub fn contains(&self, element: T) -> bool {
        self.root.contains(element)
    }
}

struct Link<T> {
    path: Option<Box<Node<T>>>,
}

impl<T: std::cmp::PartialOrd> Link<T> {
    fn new() -> Self {
        Link { path: None }
    }

    fn insert(&mut self, new_element: T) {
        if let Some(boxed_node) = self.path.as_mut() {
            boxed_node.insert(new_element);
        } else {
            self.path = Some(Box::new(Node::new(new_element)));
        }
    }

    fn contains(&self, element: T) -> bool {
        match &self.path {
            None => false,
            Some(boxed_node) => boxed_node.contains(element),
        }
    }
}

struct Node<T> {
    left: Link<T>,
    right: Link<T>,
    element: T,
}

impl<T: std::cmp::PartialOrd> Node<T> {
    fn new(element: T) -> Self {
        Node {
            left: Link::new(),
            right: Link::new(),
            element,
        }
    }

    fn insert(&mut self, new_element: T) {
        if new_element <= self.element {
            self.left.insert(new_element);
        } else {
            self.right.insert(new_element);
        }
    }

    fn contains(&self, element: T) -> bool {
        if element == self.element {
            true
        } else {
            if element <= self.element {
                self.left.contains(element)
            } else {
                self.right.contains(element)
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
        assert!(tree.contains(10));
        assert!(!tree.contains(11));
        tree.insert(11);
        assert!(tree.contains(11));
    }
}
