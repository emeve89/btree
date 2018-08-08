#[derive(Debug, Default)]
pub struct Node {
    pub value: i32,
    pub left: Option<Box<Node>>,
    pub right: Option<Box<Node>>,
}

impl Node {
    pub fn add_value(&mut self, new_node: Node) {
        if new_node.value < self.value {
            match self.left {
                None => self.left = Some(Box::new(new_node)),
                Some(ref mut node) => node.add_value(new_node),
            }
        } else if new_node.value > self.value {
            match self.right {
                None => self.right = Some(Box::new(new_node)),
                Some(ref mut node) => node.add_value(new_node),
            }
        }
    }

    pub fn read(&mut self) {
        match self.left {
            Some(ref mut left) => {
                left.read();
            }
            None => {}
        }
        print!("{} ", self.value);
        match self.right {
            Some(ref mut right) => {
                right.read();
            }
            None => {}
        }
    }
}