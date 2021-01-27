fn longest_common_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return String::new();
    }

    let mut iter = strs.iter();
    let mut longest_prefix = iter.next().unwrap().chars().collect::<Vec<_>>();

    for word in iter {
        if longest_prefix.is_empty() {
            break;
        }

        let mut i = 0;
        let length = longest_prefix.len();
        longest_prefix = word
            .chars()
            .take_while(|c| {
                if i != length {
                    let tf = c == &longest_prefix[i];
                    i += 1;
                    tf
                } else {
                    false
                }
            })
            .collect();
    }

    longest_prefix.iter().collect::<String>()
}
