fn main() {
    let name = String::from("Dundun Chicken");
    let mut space_index = 0;
    for i in name.chars() {
        if i == ' ' {
            break;
        }
        space_index = space_index + 1;
    }
    let ans = &name[0..space_index];
    println!("ans is {}", ans);
}