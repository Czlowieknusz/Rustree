use tree::Node;

fn main() {
    let mut node = Node::new(3);
    println!(
        "Before adding Node is {:?} and it's depth is {}.",
        node,
        node.get_depth()
    );
    node.add_node(5);
    println!(
        "After adding Node is {:?} and it's depth is {}.",
        node,
        node.get_depth()
    );
}
