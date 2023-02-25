// 递归
fn recursion_fibon(x: i32) -> i32 {
    if x <= 2 {
        return 1;
    }
    recursion_fibon(x - 1) + recursion_fibon(x - 2)
}

// 循环
fn loop_fibon(x: u8) {
    let (mut a, mut b) = (1, 1);
    print!("{a}\t");
    for _ in 1..=x {
        (a, b) = (a + b, a);
        print!("{b}\t");
    }
}

fn main() {
    println!("递归:");
    for i in 1..=30 {
        print!("{}\t", recursion_fibon(i));
    }

    println!("\n\n循环:");
    loop_fibon(30);
}
