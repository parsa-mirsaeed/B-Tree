#![allow(non_snake_case)]

use dioxus::prelude::*;
use std::fmt::{Debug, Display};
use std::cmp::Ordering;

// ==============================================================================
// 1. SMART KEY (Persian & Natural Sort)
//    Handles numbers logically (10 > 2) AND Persian Alphabet (Pe before Te)
// ==============================================================================
#[derive(Clone, Debug, PartialEq, Eq)]
struct NaturalString(String);

impl NaturalString {
    // Helper to assign correct alphabetical weight to Persian characters
    fn get_char_weight(c: char) -> u32 {
        match c {
            'آ' => 1, 'ا' => 2, 'ب' => 3, 
            'پ' => 4, // Pe comes strictly after Be
            'ت' => 5, // Te comes strictly after Pe
            'ث' => 6, 'ج' => 7, 
            'چ' => 8, // Che comes after Jim
            'ح' => 9, 'خ' => 10, 'د' => 11, 'ذ' => 12, 'ر' => 13, 'ز' => 14, 
            'ژ' => 15, // Zhe comes after Ze
            'س' => 16, 'ش' => 17, 'ص' => 18, 'ض' => 19, 'ط' => 20, 'ظ' => 21, 
            'ع' => 22, 'غ' => 23, 'ف' => 24, 'ق' => 25, 
            'ک' => 26, 
            'گ' => 27, // Gaf comes after Kaf
            'ل' => 28, 'م' => 29, 'ن' => 30, 'و' => 31, 'ه' => 32, 'ی' => 33,
            _ => c as u32 + 1000, // Non-Persian chars go to the end (keep Unicode order)
        }
    }

    // Compare two strings based on Persian weights
    fn compare_persian(s1: &str, s2: &str) -> Ordering {
        let chars1: Vec<char> = s1.chars().collect();
        let chars2: Vec<char> = s2.chars().collect();
        let len = std::cmp::min(chars1.len(), chars2.len());

        for i in 0..len {
            let w1 = Self::get_char_weight(chars1[i]);
            let w2 = Self::get_char_weight(chars2[i]);
            match w1.cmp(&w2) {
                Ordering::Equal => continue,
                other => return other,
            }
        }
        chars1.len().cmp(&chars2.len())
    }
}

impl PartialOrd for NaturalString {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for NaturalString {
    fn cmp(&self, other: &Self) -> Ordering {
        // 1. Try parsing as numbers first (Math Sort)
        let a_int = self.0.parse::<i64>();
        let b_int = other.0.parse::<i64>();

        match (a_int, b_int) {
            (Ok(a), Ok(b)) => a.cmp(&b),
            // 2. If text, use Custom Persian Sort instead of default Unicode
            _ => Self::compare_persian(&self.0, &other.0),
        }
    }
}

impl Display for NaturalString {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// ==============================================================================
// 2. CORE B-TREE LOGIC
// ==============================================================================

const MAX_KEYS: usize = 3;

#[derive(Clone, Debug, PartialEq)]
struct Node<K: Ord + Clone + Debug + PartialEq + 'static> {
    keys: Vec<K>,
    children: Vec<Node<K>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct BTree<K: Ord + Clone + Debug + PartialEq + 'static> {
    root: Node<K>,
}

impl<K: Ord + Clone + Debug + PartialEq> Node<K> {
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

impl<K: Ord + Clone + Debug + PartialEq> BTree<K> {
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
}

// ==============================================================================
// 3. CONTROLLER
// ==============================================================================

fn handle_insert(mut tree: Signal<BTree<NaturalString>>, mut input: Signal<String>) {
    let current_val = input.read().clone();
    if !current_val.trim().is_empty() {
        tree.write().insert(NaturalString(current_val));
        input.set(String::new());
    }
}

// ==============================================================================
// 4. VIEW
// ==============================================================================

fn main() {
    dioxus_logger::init(dioxus_logger::tracing::Level::INFO).expect("failed to init logger");
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    let mut tree = use_signal(|| BTree::<NaturalString>::new());
    let mut input_val = use_signal(|| String::new());

    let css = asset!("/assets/main.css");

    rsx! {
        document::Link { rel: "stylesheet", href: css }
        
        div {
            class: "app-container",
            
            h1 { "سیستم ذخیره‌سازی Academic" }
            p { class: "subtitle", "پیاده‌سازی درخت B-Tree استاندارد (Order 4) با مرتب‌سازی فارسی" }

            div {
                class: "input-group",
                
                input {
                    value: "{input_val}",
                    oninput: move |evt| input_val.set(evt.value()),
                    onkeydown: move |evt: KeyboardEvent| {
                        if evt.key() == Key::Enter {
                            handle_insert(tree, input_val);
                        }
                    },
                    placeholder: "نام دانشجو یا عدد...",
                }
                
                button {
                    onclick: move |_| handle_insert(tree, input_val),
                    "درج (Insert)"
                }
            }

            // Expanded Viewport
            div { class: "tree-viewport-unlimited",
                div { class: "tree",
                    // Root has no incoming edge label
                    RecursiveNode { node: tree.read().root.clone(), incoming_label: String::new() }
                }
            }
            div { class: "footer", "Powered by Rust by Parsa MirSaeed" }
        }
    }
}

#[component]
fn RecursiveNode(node: Node<NaturalString>, incoming_label: String) -> Element {
    if node.keys.is_empty() { return rsx! {}; }

    rsx! {
        div { class: "tree-branch",
            
            // The Label on the incoming line (only if not root)
            if !incoming_label.is_empty() {
                div { class: "connector-label", "{incoming_label}" }
            }

            div { class: "node-content",
                for key in node.keys.iter() {
                    span { class: "key-item", "{key.0}" }
                }
            }

            if !node.is_leaf() {
                div { class: "children-container",
                    // Iterate through children and Calculate Range Labels
                    {node.children.iter().enumerate().map(|(i, child)| {
                        // Logic for a-hu, hy-m, etc.
                        let label = if i == 0 {
                            // First child: < Key[0]
                            format!("< {}", node.keys[0].0)
                        } else if i == node.keys.len() {
                            // Last child: > Key[last]
                            format!("> {}", node.keys[i-1].0)
                        } else {
                            // Middle: Key[i-1] ... Key[i]
                            // NOTE: Reversed order to compensate for RTL text direction
                            format!("{} - {}", node.keys[i].0, node.keys[i-1].0)
                        };

                        rsx! { RecursiveNode { node: child.clone(), incoming_label: label } }
                    })}
                }
            }
        }
    }
}