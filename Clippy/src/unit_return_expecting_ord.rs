pub fn sort_names(names: &mut [String]) {
    names.sort_by_key(|n| {
        println!("{n}");
    });
}
