use dtor::declarative::dtor;

dtor! {
    #[dtor]
    unsafe fn _dtor_no_default_features() {
        println!("dtor-no-default-features:dtor");
    }
}

fn main() {
    println!("dtor-no-default-features:main");
}
