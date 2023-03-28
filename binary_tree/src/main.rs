use std::cell::RefCell;
use std::rc::Rc;

// 定义二叉树
#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline] // 使编译器内联该函数 以提高性能 但是会增加编译时间 一般用于热点函数
    fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

// BFS 广度优先搜索 用队列实现 从根节点开始 依次将左右子节点入队
fn bfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    let mut queue = vec![];
    if let Some(node) = root {
        queue.push(node);
    };
    while !queue.is_empty() {
        let node = queue.remove(0);
        res.push(node.borrow().val);
        if let Some(left) = node.borrow().left.clone() {
            queue.push(left);
        };
        if let Some(right) = node.borrow().right.clone() {
            queue.push(right);
        };
    }
    res
}

// DFS 深度优先搜索 用栈实现 从根节点开始 依次将左右子节点入栈 但是出栈顺序是先右后左 从而实现了先左后右的遍历
// 先序遍历
fn dfs(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = vec![];
    if let Some(node) = root {
        res.push(node.borrow().val);
        res.append(&mut dfs(node.borrow().left.clone()));
        res.append(&mut dfs(node.borrow().right.clone()));
    };
    res
}

fn main() {
    let root = Rc::new(RefCell::new(TreeNode::new(1)));
    let left = Rc::new(RefCell::new(TreeNode::new(2)));
    let right = Rc::new(RefCell::new(TreeNode::new(3)));
    left.borrow_mut().left = Some(Rc::new(RefCell::new(TreeNode::new(4))));
    left.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(5))));
    right.borrow_mut().right = Some(Rc::new(RefCell::new(TreeNode::new(6))));
    root.borrow_mut().left = Some(left);
    root.borrow_mut().right = Some(right);
    println!("BFS: {:?}", bfs(Some(root.clone())));
    println!("DFS: {:?}", dfs(Some(root.clone())));
}
