use try_from_row_derive::*;

#[derive(FromRow)]
enum SimpleEnum {
    Here,
    There,
}

fn main() {}
