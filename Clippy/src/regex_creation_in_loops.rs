use regex::Regex;

pub fn count_numbers(lines: &[&str]) -> usize {
    let mut count = 0;
    for line in lines {
        let re = Regex::new(r"^\d+$").unwrap();
        if re.is_match(line) {
            count += 1;
        }
    }
    count
}
