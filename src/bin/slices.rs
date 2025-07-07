// write a func that takes a string as an input and returns the first word from it

fn first_word(str: String) -> String {
    let mut ans = String::from("");
    for i in str.chars() {
        if i == ' ' {
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}
fn main() {
    let name = String::from("dundun chicken");
    let ans = first_word(name);
    println!("ans is: {}", ans);
}