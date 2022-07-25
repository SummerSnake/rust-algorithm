mod stack_mod;
use stack_mod::stack::Stack;

mod queue_mod;
use queue_mod::queue::Queue;

mod priority_queue_mod;
use priority_queue_mod::priority_queue::PriorityQueue;

fn main() {
    test_stack();
    test_queue();
    test_priority_queue();
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
