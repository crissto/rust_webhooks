pub fn title_case(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f
            .to_uppercase()
            .chain(c.flat_map(|t| t.to_lowercase()))
            .collect(),
    }
}

pub fn split_by_commas(string: &str) -> Vec<u64> {
    string
        .split(',')
        .skip_while(|&x| x.is_empty())
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}
