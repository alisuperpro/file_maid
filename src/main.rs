use file_maid::replaces::replace_several;
use file_maid::replaces::replace_with_mark;
fn main() {
    let old_text: String = String::from("jjk T1 E__I__.mp4");
    let to_replace: String = String::from("1");
    let mark: String = String::from("__I__");

    let cap: String = replace_with_mark(old_text, to_replace, mark);

    println!("{}", cap);

    let array_text = String::from("jjk E__I__");

    let result = replace_several(array_text, 1, 20, String::from("__I__"));

    println!("{:?}", result)
}
