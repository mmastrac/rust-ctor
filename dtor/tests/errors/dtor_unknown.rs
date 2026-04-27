use dtor::dtor;

#[dtor(unknown)]
fn foo() {
}

#[dtor(unsafe, unknown, unknown2)]
fn foo() {
}

fn main() {
}
