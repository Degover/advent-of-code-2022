pub fn get_letter_priority(c: &char) -> Option<usize> {
    let priority_list = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    return priority_list.chars().position(|inn_c| inn_c == *c);
}
