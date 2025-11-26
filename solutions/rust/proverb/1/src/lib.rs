pub fn build_proverb(list: &[&str]) -> String {
    let mut proverb = String::new();

    if list.is_empty() {
        return proverb;
    }

    list.windows(2).for_each(|words| {
        proverb
            .push_str(format!("For want of a {} the {} was lost.\n", words[0], words[1]).as_str());
    });

    proverb.push_str(format!("And all for the want of a {}.", list[0]).as_str());

    proverb
}
