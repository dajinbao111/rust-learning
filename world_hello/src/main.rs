fn main() {
    let mut x = 6;
    let y = &mut x;
    *y+=1;
    println!("{}", x)
}

fn foo(x: &str) -> &str {
    x
}

fn long_str<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}