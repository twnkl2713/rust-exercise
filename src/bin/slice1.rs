fn main(){
    let word = String::from("Dundun Chicken");
    let word2 = find_first_word(&word); // borrow the variable
    println!("{}", word);
    println!("{}", word2);
}

// now takes a reference and returns a reference
fn find_first_word(word: &String) -> &str {
    let mut index = 0;
    for(_, i) in word.chars().enumerate() {
        if i == ' ' {
            break;
        }
        index = index + 1;
    }
    return &word[0..index];
}