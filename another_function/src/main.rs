fn main() {
    another_function(4);
    let test = String::from("wows dit is een test");
    let res = first_word(&test);
    println!("test {}", res)

}

fn another_function(x: i32) {
    println!("The value of x is: {}", x);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }
    s.len()
}
