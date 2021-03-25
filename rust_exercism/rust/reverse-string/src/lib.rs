pub fn reverse(input: &str) -> String {
    //unimplemented!("Write a function to reverse {}", input);
    let mut s:String = String::new();
    s.extend(input.chars().rev());
    s
}
