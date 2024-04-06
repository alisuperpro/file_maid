pub mod replaces_text {
    pub fn replace_several(
        old_text: String,
        start: usize,
        end: usize,
        mark: String,
    ) -> Vec<String> {
        let mut count = start;

        let mut array_text: Vec<String> = Vec::with_capacity(end);

        if start > end {
            println!("Error ")
        }

        while count <= end {
            let result = replace_with_mark(old_text.clone(), count.to_string(), mark.clone());
            count += 1;
            array_text.push(result);
        }

        return array_text;
    }

    pub fn replace_with_mark(old_text: String, to_replace: String, mark: String) -> String {
        let new_text = old_text.replace(&mark, &to_replace);

        return new_text;
    }
}
