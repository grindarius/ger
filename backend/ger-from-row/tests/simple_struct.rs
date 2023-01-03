use try_from_row_derive::*;

#[derive(FromRow)]
#[allow(dead_code)]
struct SimpleStruct {
    user_username: String,
    user_name: String,
    email: String,
}

fn main() {}
