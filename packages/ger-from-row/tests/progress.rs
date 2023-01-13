#[test]
fn tests() {
    let t = trybuild::TestCases::new();
    t.pass("tests/simple_enum.rs");
    t.pass("tests/simple_struct.rs");
    t.pass("tests/simple_struct_with_rename.rs");
    t.pass("tests/simple_struct_with_enum.rs");
    t.pass("tests/simple_struct_with_enum_without_rename.rs");
}
