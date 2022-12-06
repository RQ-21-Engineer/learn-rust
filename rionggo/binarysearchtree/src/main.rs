use std::cmp::Ordering;

// Define the binary search tree node
struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

// Define the binary search tree
struct BinarySearchTree {
    root: Option<Box<Node>>,
}

impl BinarySearchTree {
    // Create a new, empty binary search tree
    fn new() -> BinarySearchTree {
        BinarySearchTree { root: None }
    }

    // Insert a new value into the binary search tree
    fn insert(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            left: None,
            right: None,
        });

        let mut current_node = &mut self.root;
        loop {
            match current_node {
                None => {
                    *current_node = Some(new_node);
                    break;
                }
                Some(node) => {
                    current_node = match value.cmp(&node.value) {
                        Ordering::Less => &mut node.left,
                        Ordering::Greater => &mut node.right,
                        Ordering::Equal => break,
                    }
                }
            }
        }
    }

    // Search for a value in the binary search tree
    fn search(&self, value: i32) -> bool {
        let mut current_node = &self.root;
        loop {
            match current_node {
                None => return false,
                Some(node) => {
                    match value.cmp(&node.value) {
                        Ordering::Less => current_node = &node.left,
                        Ordering::Greater => current_node = &node.right,
                        Ordering::Equal => return true,
                    }
                }
            }
        }
    }
}

fn main() {
    // Create a new binary search tree
    let mut bst = BinarySearchTree::new();
    // Insert some values into the tree
    bst.insert(5);
    bst.insert(3);
    bst.insert(7);
    bst.insert(1);
    bst.insert(4);
    bst.insert(6);
    bst.insert(8);

    // print 
    println!("Search for 5: {}", bst.search(5));
    println!("Search for 3: {}", bst.search(3));
    println!("Search for 7: {}", bst.search(7));
    println!("Search for 1: {}", bst.search(1));
    println!("Search for 4: {}", bst.search(4));
    println!("Search for 6: {}", bst.search(6));
    println!("Search for 8: {}", bst.search(8));
    println!("Search for 9: {}", bst.search(9));


    // Search for some values in the tree
    assert!(bst.search(5));
    assert!(bst.search(3));
    assert!(bst.search(7));
    assert!(bst.search(1));
    assert!(!bst.search(2));
    assert!(!bst.search(9));

    println!("All tests passed!");
}