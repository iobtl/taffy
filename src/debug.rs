use core::fmt::{Debug, Display, Write};
use slotmap::Key;
use std::sync::Mutex;

use crate::node::Node;
use crate::tree::LayoutTree;

/// Prints a debug representation of the computed layout for a tree of nodes, starting with the passed root node.
pub fn print_tree(tree: &impl LayoutTree, root: Node) {
    println!("TREE");
    print_node(tree, root, false, String::new());
}

fn print_node(tree: &impl LayoutTree, node: Node, has_sibling: bool, lines_string: String) {
    let layout = tree.layout(node);

    let num_children = tree.child_count(node);

    let fork_string = if has_sibling { "├── " } else { "└── " };
    println!(
        "{lines}{fork}[x: {x}, y: {y}, width: {width}, height: {height}]",
        lines = lines_string,
        fork = fork_string,
        x = layout.location.x,
        y = layout.location.y,
        width = layout.size.width,
        height = layout.size.height,
    );
    let bar = if has_sibling { "│   " } else { "    " };
    let new_string = lines_string + bar;

    // Recurse into children
    for (index, child) in tree.children(node).enumerate() {
        let has_sibling = index < num_children - 1;
        print_node(tree, *child, has_sibling, new_string.clone());
    }
}

#[doc(hidden)]
pub struct DebugLogger {
    stack: Mutex<Vec<String>>,
}

static EMPTY_STRING: String = String::new();
impl DebugLogger {
    pub const fn new() -> Self {
        Self { stack: Mutex::new(Vec::new()) }
    }

    pub fn push_node(&self, new_key: impl Key) {
        let mut stack = self.stack.lock().unwrap();
        let mut key_string = String::new();
        write!(&mut key_string, "{:?}", new_key.data()).unwrap();
        stack.push(key_string);
    }

    pub fn pop_node(&self) {
        let mut stack = self.stack.lock().unwrap();
        stack.pop();
    }

    pub fn log(&self, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message}");
    }

    pub fn labelled_log(&self, label: &str, message: impl Display) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message}");
    }

    pub fn debug_log(&self, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {message:?}");
    }

    pub fn labelled_debug_log(&self, label: &str, message: impl Debug) {
        let stack = self.stack.lock().unwrap();
        let key = stack.last().unwrap_or(&EMPTY_STRING);
        let level = stack.len() * 4;
        let space = " ";
        println!("{space:level$}{key}: {label} {message:?}");
    }
}

#[cfg(feature = "debug")]
pub(crate) static NODE_LOGGER: DebugLogger = DebugLogger::new();
