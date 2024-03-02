use regex::Regex;

pub fn convert(text: &String) -> String {
    if text.len() == 0 {
        panic!("Nothing to convert!");
    };

    let mut pig_words: Vec<String> = Vec::new();
    let words = text.split_whitespace();
    let consonant_reg = Regex::new(r"[[:<:]][bcdfjghklmnprstvwxz]").unwrap();
    let vowel_reg = Regex::new("r[[:<:]][aeiouy]").unwrap();
    
    for word in words {
        let mut pig_word = String::new();

        if consonant_reg.is_match(word) {
            pig_word = pig_consonant(word);
        } else if  vowel_reg.is_match(word){
            pig_word = pig_vowel(word);
        }
        pig_words.push(pig_word);
    }
    
    pig_words.join(" ").to_string()
}

fn pig_consonant(word: &str) -> String {
    let mut buffer = String::new();
    let (first, last) = word.split_at(1);
    buffer.push_str(last);
    buffer.push_str("ay");
    buffer.push_str(first);
    buffer
}

fn pig_vowel(word: &str) -> String {
    let mut buffer = word.to_string();
    buffer.push_str("hay");
    buffer
}