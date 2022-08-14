use std::thread;
use std::time::Duration;

fn main() {
    let expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(1));
        num
    };

    println!("{:?}", expensive_closure(10));


    fn  add_one_v1   (x: u32) -> u32 { x + 1 }
    let add_one_v2 = |x: u32| -> u32 { x + 1 };
    let add_one_v3 = |x|             { x + 1 };
    let add_one_v4 = |x|               x + 1  ;

    /* Evaluating the closures is required for add_one_v3 and add_one_v4 to 
    be able to compile because the types will be inferred from their usage */
    println!("{:?}", add_one_v1(6));
    println!("{:?}", add_one_v2(7));
    println!("{:?}", add_one_v3(8));
    println!("{:?}", add_one_v4(9));


    let example_closure = |x| x;

    let _s = example_closure(String::from("hello"));
    // This would fail: String type as been inferred earlier
    // let n = example_closure(5);


    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list);


    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    list.sort_by_key(|r| r.width);
    println!("{:#?}", list);


    /* This would fail: The closure captures value then moves value out of the
    closure by transferring ownership of value to the sort_operations vector */
    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);


    let mut num_sort_operations = 0;
    list.sort_by_key(|r| {
        num_sort_operations += 1;
        r.width
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list);
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}
