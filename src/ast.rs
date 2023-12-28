#[derive(Debug)]
pub enum Node {
    Add { left: Box<Node>, right: Box<Node> },
    Multiply { left: Box<Node>, right: Box<Node> },
    Divide { left: Box<Node>, right: Box<Node> },
    Assign { id: String, expr: Box<Node> },
    Number { value: i32 },
}
