mod replaces;
use replaces::replaces_text::replaces_text;
use std::fs;
fn main() {
    let old_text: String = String::from("jjk T1 E__I__.txt");
    let to_replace: String = String::from("1");
    let mark: String = String::from("__I__");

    let cap: String = replaces_text::replace_with_mark(old_text, to_replace, mark);

    println!("{}", cap);

    let array_text = String::from("jjk E__I__");

    let result = replaces_text::replace_several(array_text, 1, 20, String::from("__I__"));

    println!("{:?}", result);

    rename_file(String::from("example.txt"), cap).expect("reason");
}

fn rename_file(path: String, new_name: String) -> std::io::Result<()> {
    fs::rename(path, new_name)?;
    Ok(())
}
