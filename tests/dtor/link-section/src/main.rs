use dtor::dtor;

/// This will not be called in all cases.
#[dtor(method = link_section)]
unsafe fn _dtor_no_default_features() {
    println!("dtor-link-section:dtor");
}

fn main() {
    println!("dtor-link-section:main");
}
