mod stack_mod;
use stack_mod::stack::Stack;

mod queue_mod;
use queue_mod::queue::Queue;

fn main() {
    test_stack();
    test_queue();
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
pub fn test_queue() {
    let mut q = Queue::new();

    q.enqueue(1);
    q.enqueue(2);
    println!("{:?}", q);

    q.dequeue();
    println!("{:?}", q);
}
