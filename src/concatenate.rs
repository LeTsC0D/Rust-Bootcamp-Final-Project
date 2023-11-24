fn main() {
    let string1 = "Hello, ";
    let string2 = "Simon!";
    
    // Call the concatenate_strings function with references to string1 and string2
    let concatenated_string = concatenate_strings(string1, string2);

    // Print the result
    println!("{}", concatenated_string);
}

fn concatenate_strings(str1: &str, str2: &str) -> String {
    // Create a new String to store the result
    let mut result = String::new();

    // Append the contents of the first input string slice
    result.push_str(str1);

    // Append the contents of the second input string slice
    result.push_str(str2);

    // Return the concatenated string
    result
}
