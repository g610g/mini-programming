use mini_programming::{core, utils};
fn main() {
    foo("test");
}
fn foo(_my_str: &str) {
    let white_space_test: String = String::from("test this is a white space");

    let filtered_string: String = utils::clear_whitespaces(&white_space_test);
    println!("{}", filtered_string);
    core::print(&"println".to_string());
    //bar("sorround this with qoutest");
}
fn bar(my_str: &str) {
    println!("{my_str}");
}
