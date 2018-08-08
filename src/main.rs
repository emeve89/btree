use binary_tree::tree::Tree;

mod binary_tree;

fn main() {
    let mut tree: Tree = Default::default();
    tree.add_node(10);
    tree.add_node(6);
    tree.add_node(3);
    tree.add_node(8);
    tree.add_node(1);
    tree.add_node(15);
    tree.add_node(9);
    tree.add_node(2);
    tree.add_node(56);
    tree.add_node(1);
    tree.add_node(-9);
    tree.add_node(4);
    tree.add_node(76);

    print!("{:#?}\n\n", tree);
    tree.read();
}