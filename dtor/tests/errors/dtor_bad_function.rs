use dtor::dtor;

#[dtor]
pub const fn const_function() {
}

#[dtor]
pub default fn const_function() {
}

#[dtor]
pub fn default_function<T>() {
}

fn main() {
}
