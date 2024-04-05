fn main() {
    let old_text = String::from("jjk T1 E__I__.mp4");
    let to_replace = String::from("1");
    let mark = String::from("__I__");

    let cap = replace_with_mark(old_text, to_replace, mark);

    println!("{}", cap)
}

fn replace_with_mark(old_text: String, to_replace: String, mark: String) -> String {
    let new_text = old_text.replace(&mark, &to_replace);

    return new_text;
}
