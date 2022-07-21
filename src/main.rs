mod stack_mod;
use stack_mod::stack::Stack;

fn main() {
    test_stack();
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
