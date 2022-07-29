use utils::{
    binary_tree_mod::binary_tree::{BinarySearchTree, BinaryTree, Node},
    priority_queue_mod::priority_queue::PriorityQueue,
    queue_mod::queue::Queue,
    sort_mod::sort::Sort,
    stack_mod::stack::Stack,
};

fn main() {
    test_stack();
    test_queue();
    test_priority_queue();
    test_binary_tree();
    test_sort();
}

/**
 * @desc 栈
 */
fn test_stack() {
    #[derive(PartialEq, Debug)]
    struct TestStruct {
        num: i32,
    }

    let five = TestStruct { num: 5 };
    let nine = TestStruct { num: 9 };

    let mut s = Stack::<&TestStruct>::new();

    assert_eq!(s.pop(), None);

    s.push(&five);
    s.push(&nine);
    println!("{:?}", s);

    assert_eq!(s.pop(), Some(&nine));
    assert_eq!(s.pop(), Some(&five));
    assert_eq!(s.pop(), None);
}

/**
 * @desc 队列
 */
fn test_queue() {
    let mut q = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    println!("{:?}", q);

    q.dequeue();
    println!("{:?}", q);
}

/**
 * @desc 优先队列
 */
fn test_priority_queue() {
    let mut pq = PriorityQueue::new();

    pq.enqueue(2);
    pq.enqueue(1);
    pq.enqueue(3);
    println!("{:?}", pq);

    assert!(pq.min().unwrap() == 1);
    assert!(pq.max().unwrap() == 3);

    pq.delete_max();
    println!("{:?}", pq);

    pq.delete_min();
    println!("{:?}", pq);

    assert!(pq.len() == 1);

    pq.delete_min();
    assert!(pq.is_empty());
}

/**
 * @desc 二叉树
 */
fn test_binary_tree() {
    let mut root = Node::new("4", 4);
    root.insert("5", 5);
    root.insert("3", 3);
    root.insert("6", 6);
    root.insert("2", 2);
    root.insert("7", 7);
    println!("{:?}", root);

    if let Some(ref left) = root.left {
        assert!(left.value == 3);
    }
    if let Some(ref right) = root.right {
        assert_eq!(right.key, "5");
        if let Some(ref right) = right.right {
            assert_eq!(right.key, "6");
        }
    }

    println!("Pre Order traversal");
    root.pre_order();
    println!("In Order traversal");
    root.in_order();
    println!("Pos Order traversal");
    root.pos_order();
}

/**
 * @desc 排序
 */
fn test_sort() {
    let mut arr = vec!['5', '3', '4', '1', '2'];
    Sort::bubble_sort(&mut arr);

    let mut arr = vec!['5', '3', '4', '1', '2'];
    Sort::selection_sort(&mut arr);

    let mut arr = vec!['5', '3', '4', '1', '2'];
    Sort::insertion_sort(&mut arr);

    let mut arr = vec!['5', '3', '4', '1', '2'];
    Sort::shell_sort(&mut arr);
}
