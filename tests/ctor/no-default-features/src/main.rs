use ctor::declarative::ctor;

ctor! {
    #[ctor]
    unsafe fn _ctor_no_default_features() {
        println!("ctor-no-default-features:ctor");
    }
}

fn main() {
    println!("ctor-no-default-features:main");
}
