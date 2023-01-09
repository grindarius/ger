use ger_from_row::*;

#[derive(FromRow)]
#[allow(dead_code)]
struct SimpleStructWithRename {
    #[fromrow(field = "userUsername")]
    user_username: String,
    user_name: String,
    email: String,
}

fn main() {}
