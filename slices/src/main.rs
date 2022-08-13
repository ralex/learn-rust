fn main() {
    let sentence = String::from("Hello world blabla");

    println!("First word end at position: {}", first_word(&sentence));
    println!("Sliced sentence: word1: '{}', word2: '{}'", &sentence[..5], &sentence[6..]);

    println!("First word: {}", first_word_new(&sentence));

    let my_string = String::from("hello world");

    // `first_word_new` works on slices of `String`s, whether partial or whole
    let _word = first_word_new(&my_string[0..6]);
    let _word = first_word_new(&my_string[..]);
    // `first_word_new` also works on references to `String`s, which are equivalent
    // to whole slices of `String`s
    let _word = first_word_new(&my_string);

    let my_string_literal = "hello world";

    // `first_word_new` works on slices of string literals, whether partial or whole
    let _word = first_word_new(&my_string_literal[0..6]);
    let _word = first_word_new(&my_string_literal[..]);

    // Because string literals *are* string slices already,
    // this works too, without the slice syntax!
    let _word = first_word_new(my_string_literal);
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

fn first_word_new(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
