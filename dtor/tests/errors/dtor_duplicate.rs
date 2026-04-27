use dtor::dtor;

#[dtor(method = term, method = unload)]
fn foo() {
}

#[dtor(unsafe, unsafe)]
fn foo() {
}

fn main() {
}
