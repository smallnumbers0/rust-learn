fn main() {
    let mut phrase = "hello";
    phrase = phrase.chars().rev();
    println!("{phrase}");
    
    

}


pub fn reverse_string(s: &mut Vec<char>) {
    s.reverse()
}