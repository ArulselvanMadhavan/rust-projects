#[derive(Debug)]
pub struct BST {
    root:Link
}

type Link = Option<Box<Node>>;

#[derive(Debug)]
struct Node {
    elem: i32,
    left: Link,
    right: Link
}

impl BST {
    pub fn new() -> Self {
        BST{root: None}
    }

    pub fn insert(&mut self, data: i32) -> bool{
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

    fn insert_helper(cur_node:&mut Node, data:i32) -> bool {
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

    pub fn search(&mut self, data: i32) -> bool {
        match self.root {
            None => false,
            Some(ref mut node) => {
                BST::search_helper(node, data)
            }
        }
    }

    fn search_helper(cur_node: &mut Node, data:i32) -> bool {
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
}
