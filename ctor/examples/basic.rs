use ctor::ctor;

#[ctor(unsafe)]
fn ctor() {
    println!("ctor");
}

fn main() {
    println!("main");
}
