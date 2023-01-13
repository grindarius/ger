use ger_from_row::*;

#[derive(FromRow)]
#[allow(dead_code)]
enum Role {
    User,
    Admin,
}

#[derive(FromRow)]
#[allow(dead_code)]
struct SimpleStructWithRename {
    #[fromrow(field = "userUsername")]
    user_username: String,
    user_name: String,
    email: String,
    #[fromrow(num = "user_role")]
    role: Role,
}

fn main() {}
