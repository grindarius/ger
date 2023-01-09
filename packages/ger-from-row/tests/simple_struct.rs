use ger_from_row::*;

#[derive(FromRow)]
#[allow(dead_code)]
struct SimpleStruct {
    user_username: String,
    user_name: String,
    email: String,
}

fn main() {}
