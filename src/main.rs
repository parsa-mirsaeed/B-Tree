use std::fmt::Debug;
use std::str::FromStr;
use std::io::{self, Write};

// ==============================================================================
// CONFIGURATION
// ==============================================================================
const MAX_KEYS: usize = 3;

// ==============================================================================
// GENERIC NODE (The Engine)
// K: Ord + Clone + Debug means "This node works with Any Type that 
// can be Ordered, Cloned, and Printed"
// ==============================================================================
#[derive(Clone, Debug)]
struct Node<K: Ord + Clone + Debug> {
    keys: Vec<K>,
    children: Vec<Node<K>>,
}

#[derive(Debug)]
pub struct BTree<K: Ord + Clone + Debug> {
    root: Node<K>,
}

// ==============================================================================
// GENERIC IMPLEMENTATION
// ==============================================================================
impl<K: Ord + Clone + Debug> Node<K> {
    fn new(keys: Vec<K>, children: Vec<Node<K>>) -> Self {
        Node { keys, children }
    }

    fn is_leaf(&self) -> bool {
        self.children.is_empty()
    }

    fn insert(&mut self, key: K) -> Option<(K, Node<K>)> {
        let idx = match self.keys.binary_search(&key) {
            Ok(_) => return None, 
            Err(i) => i,
        };

        if self.is_leaf() {
            self.keys.insert(idx, key);
        } else {
            let child = &mut self.children[idx];
            if let Some((promoted_key, right_child)) = child.insert(key) {
                self.keys.insert(idx, promoted_key);
                self.children.insert(idx + 1, right_child);
            }
        }

        if self.keys.len() > MAX_KEYS {
            return Some(self.split_upper_median());
        }
        None
    }

    fn split_upper_median(&mut self) -> (K, Node<K>) {
        let mid_index = self.keys.len() / 2;
        let promoted_key = self.keys.remove(mid_index);
        let right_keys = self.keys.split_off(mid_index);
        
        let right_children = if self.is_leaf() {
            vec![]
        } else {
            self.children.split_off(mid_index + 1)
        };

        let right_node = Node::new(right_keys, right_children);
        (promoted_key, right_node)
    }
}

impl<K: Ord + Clone + Debug> BTree<K> {
    pub fn new() -> Self {
        BTree {
            root: Node::new(vec![], vec![]),
        }
    }

    pub fn insert(&mut self, key: K) {
        if let Some((promoted_key, right_node)) = self.root.insert(key) {
            let new_root = Node::new(
                vec![promoted_key],
                vec![self.root.clone(), right_node],
            );
            self.root = new_root;
        }
    }

    pub fn print_structure(&self) {
        println!("--- B-Tree Structure ---");
        self.print_recursive(&self.root, 0);
        println!("------------------------");
    }

    fn print_recursive(&self, node: &Node<K>, level: usize) {
        let indent = "    ".repeat(level);
        println!("{}Node Keys: {:?}", indent, node.keys);
        for child in &node.children {
            self.print_recursive(child, level + 1);
        }
    }
}

// ==============================================================================
// USER INTERFACE (The Recognizer)
// ==============================================================================
fn get_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
    buffer.trim().to_string()
}

fn main() {
    println!("Welcome to EduTalent B-Tree System");
    println!("Please enter your first data item (Number, English Name, or Farsi Name):");
    
    let first_input = get_input(">> ");

    // --- ARCHITECTURE DECISION: RECOGNIZER ---
    // Try to parse as unsigned integer (u32)
    if let Ok(num) = u32::from_str(&first_input) {
        println!("\n[System]: Detected NUMBER mode. Logic is Mathematical (10 > 4).");
        let mut db = BTree::<u32>::new();
        db.insert(num);
        db.print_structure();
        run_number_loop(db);
    } else {
        println!("\n[System]: Detected TEXT mode (English/Farsi). Logic is Lexicographical.");
        let mut db = BTree::<String>::new();
        db.insert(first_input);
        db.print_structure();
        run_string_loop(db);
    }
}

// Loop for Numbers
fn run_number_loop(mut db: BTree<u32>) {
    loop {
        let input = get_input(">> Add Number (or 'q' to quit): ");
        if input == "q" { break; }
        match u32::from_str(&input) {
            Ok(num) => {
                db.insert(num);
                db.print_structure();
            },
            Err(_) => println!("Error: Please enter a valid number in Number Mode."),
        }
    }
}

// Loop for Strings (English or Farsi)
fn run_string_loop(mut db: BTree<String>) {
    loop {
        let input = get_input(">> Add Name (or 'q' to quit): ");
        if input == "q" { break; }
        db.insert(input);
        db.print_structure();
    }
}
