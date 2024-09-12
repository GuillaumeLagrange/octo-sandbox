#[derive(Debug)]
struct Foo {
    bar: i32,
}

fn main() {
    let a = 2;

    let b = Foo { bar: 42 };

    dbg!(&b);

    println!("This is {}", b.bar);
    println!("This is {a}");
}
