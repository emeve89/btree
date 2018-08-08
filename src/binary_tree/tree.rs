use binary_tree::node::Node;

#[derive(Debug, Default)]
pub struct Tree {
    root: Option<Node>,
}

impl Tree {
    pub fn add_node(&mut self, value: i32) {
        let new_node = Node {
            value,
            ..Default::default()
        };
        match self.root {
            None => {
                self.root = Some(new_node);
            }
            Some(ref mut node) => {
                node.add_value(new_node);
            }
        }
    }

    pub fn read(self) {
        self.root.unwrap().read();
    }
}