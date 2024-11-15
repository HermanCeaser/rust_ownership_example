fn main() {
    // println!("Hello, world!");

    let string1 = String::from("Hello ");
    let string2 = String::from("World!");

    let concatenated_string = concatenate_strings(&string1, &string2);

    println!("Concatenated String: {}", concatenated_string)
}

fn concatenate_strings(foo: &str, bar: &str) -> String {
    let mut result = String::from(foo);
    result.push_str(bar);
    result
}