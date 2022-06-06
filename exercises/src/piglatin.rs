fn main() {
    // Convert strings to pig latin. The first consonant of each word is moved to
    // the end of the word and “ay” is added, so “first” becomes “irst-fay.”
    // Words that start with a vowel have “hay” added to the end instead (“apple”
    // becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!

    use std::io;

    let mut input = String::new();
    println!("Please input a word to convert it in pig latin:");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read word");
    let word = input.trim().to_lowercase();
    
    let bytes = word.as_bytes();

    match bytes.first() {
        // Vowels (a, e, i, o, u, y) in  bytes (u8)
        Some(97)|
        Some(101)|
        Some(105)|
        Some(111)|
        Some(117)|
        Some(121) => println!("{}-hay", word),
        None => println! ("There is no word to convert!"),
        _ => {
            let begin = match String::from_utf8(bytes.get(1..).unwrap().to_vec()) {
                Ok(b) => String::from(b),
                Err(_) => String::from(""),
            };
            let consonant = match String::from_utf8(bytes.get(0..1).unwrap().to_vec()){
                Ok(c) => String::from(c),
                Err(_) => String::from(""),
            };
            println!("{}-{}ay", begin, consonant);
        }
    };
}
