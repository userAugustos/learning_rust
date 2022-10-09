type ChildNode<Y> = Option<Box<BTNode<T>>>;

enum Op<T> {
    Add,
    Sub,
    Mul,
    Id(T),
}

struct BTNode<T> {
    left: ChildNode<T>,
    right: ChildNode<T>,
    op: Op<T>,
}

impl BTNode<i32> {
    pub fn new(op: Op<i32>, l: BTNode<i32>, r: BTNode<i32>) -> Self {
        BTNode::<i32> {
            op: op,
            left: Some(Box::new(l)),
            right: Some(Box::new(r)),
        }
    }
}

fn AddNode(l: BTNode<i32>, r: BTNode<i32>) -> BTNode<i32> {
    BTNode::new(Op: Add, l, r)
}
