#[derive(Debug)]
pub struct BST<T> {
    root:Link<T>
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug)]
struct Node<T> {
    elem: T,
    left: Link<T>,
    right: Link<T>
}

pub struct IntoIter<T>(BST<T>);

pub struct Iter<'a, T:'a> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> where T:Ord {
    type Item = &'a T;
    fn next(& mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.right.as_ref().map(|node| &**node);
            &node.elem
        })
    }
}

impl<T> Iterator for IntoIter<T> where T:Ord {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.get_rightmost()
    }
}

impl<T> IntoIterator for BST<T> where T:Ord {
    type Item = T;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter(self)
    }
}

impl<T> BST<T> where T:Ord {
    pub fn new() -> Self {
        BST{root: None}
    }

    pub fn insert(&mut self, data: T) -> bool{
        match self.root {
            None => {
                self.root = Some(Box::new(Node{
                    elem: data,
                    left: None,
                    right: None
                }));
                true
            },
            Some(ref mut node) => {
                BST::insert_helper(node, data)
            }
        }
    }

    fn insert_helper(cur_node:&mut Node<T>, data:T) -> bool {
        if cur_node.elem == data {
            false
        } else if cur_node.elem > data {
            if let Some(ref mut next_node) = cur_node.left {
                BST::insert_helper(next_node, data)
            } else {
                cur_node.left = Some(Box::new(Node {
                    elem: data,
                    left: None,
                    right: None
                }));
                true
            }
        } else {
            if let Some(ref mut next_node) = cur_node.right {
                BST::insert_helper(next_node, data)
            } else {
                cur_node.right = Some(Box::new(Node {
                    elem: data,
                    left: None,
                    right: None
                }));
                true
            }
        }
    }

    pub fn search(&mut self, data: T) -> bool {
        match self.root {
            None => false,
            Some(ref mut node) => {
                BST::search_helper(node, data)
            }
        }
    }

    fn search_helper(cur_node: &mut Node<T>, data:T) -> bool {
        if cur_node.elem == data {
            true
        } else if data < cur_node.elem {
            if let Some(ref mut next_node) = cur_node.left {
                BST::search_helper(next_node, data)
            } else {
                false
            }
        } else {
            if let Some(ref mut next_node) = cur_node.right {
                BST::search_helper(next_node, data)
            } else {
                false
            }
        }
    }

    pub fn get_rightmost(&mut self) -> Option<T>{
        match self.root {
            None => None,
            ref mut cur_link => {
                BST::get_rightmost_helper(cur_link)
            }
        }
    }

    fn get_rightmost_helper(cur_link: &mut Link<T>) -> Option<T>{
        if cur_link.as_ref().unwrap().right.is_some() {
            BST::get_rightmost_helper(&mut cur_link.as_mut().unwrap().right)
        } else {
            cur_link.take().map(|boxed_node| {
                let node = *boxed_node;
                node.elem
            })
        }
    }

    pub fn iter<'a>(&'a self) -> Iter<'a, T> {
        Iter {next: self.root.as_ref().map(|node| &**node)}
    }
}

#[cfg(test)]
mod test {
    use super::BST;

    #[test]
    fn insert_basic() {
        let mut bst = BST::new();
        let mut result = bst.insert(5);
        assert!(result == true);
        result = bst.insert(3);
        assert!(result == true);
        result = bst.insert(10);
        assert!(result == true);
        result = bst.insert(3);
        assert!(result == false);
        result = bst.insert(5);
        assert!(result == false);
    }

    #[test]
    fn search_basic() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(-1);
        bst.insert(100);
        let mut result = bst.search(20);
        assert!(result == false);
        result = bst.search(100);
        assert!(result == true);
        result = bst.search(-1);
        assert!(result == true);
    }

    #[test]
    fn rightmost_basic() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(10);
        bst.insert(15);
        let result = bst.get_rightmost();
        assert!(result == Some(15));
        let result = bst.get_rightmost();
        assert!(result == Some(10));
        let result = bst.get_rightmost();
        assert!(result == Some(5));
        let result = bst.get_rightmost();
        assert!(result == None);
    }

    #[test]
    fn into_iter_basic() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(10);
        bst.insert(15);
        let mut iter = bst.into_iter();
        assert_eq!(iter.next(), Some(15));
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn into_iter_for_loop() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(10);
        bst.insert(15);
        let mut results = vec![];
        for elt in bst {
            results.push(elt);
        }
        assert_eq!(results, vec![15,10,5]);
    }

    #[test]
    fn iter_basic() {
        let mut bst = BST::new();
        bst.insert(5);
        bst.insert(3);
        bst.insert(10);
        bst.insert(15);
        let mut iter = bst.iter();
        assert_eq!(iter.next(), Some(&5));
        assert_eq!(iter.next(), Some(&10));
        assert_eq!(iter.next(), Some(&15));
        assert_eq!(iter.next(), None);
    }
}
