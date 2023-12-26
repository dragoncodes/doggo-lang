#[derive(Debug)]
pub enum Node {
    Add { left: Box<Node>, right: Box<Node> },
    Multiply { left: Box<Node>, right: Box<Node> },
    Divide { left: Box<Node>, right: Box<Node> },
    Number { value: i32 },
}
