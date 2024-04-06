mod file_manager;
mod replaces;

#[cfg(test)]
mod tests {

    use crate::file_manager::rename_file::rename_file::rename_file;
    use crate::replaces::replaces_text::replaces_text::{replace_several, replace_with_mark};

    #[test]
    fn test_replace_with_mark() {
        let old_text: String = String::from("jjk T1 E__I__.txt");
        let to_replace: String = String::from("1");
        let mark: String = String::from("__I__");
        assert_eq!(
            replace_with_mark(old_text, to_replace, mark),
            "jjk T1 E1.txt"
        );
    }

    #[test]
    fn test_replace_several() {
        let result = replace_several(String::from("jjk T1 E__I__"), 1, 3, String::from("__I__"));
        assert_eq!(result, ["jjk T1 E1", "jjk T1 E2", "jjk T1 E3"])
    }

    #[test]
    fn test_rename_file() {
        let result = rename_file(String::from("ejemplo.txt"), String::from("example.txt"));

        assert!(result.is_ok())
    }
}
