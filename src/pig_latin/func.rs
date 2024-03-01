use regex::Regex;

pub fn convert(text: &String) -> String {
    if text.len() == 0 {
        panic!("Nothing to convert!");
    };

    let mut pig_words: Vec<String> = Vec::new();
    let words = text.split_whitespace();
    
    for word in words {
        let first_ch = word.chars().nth(0);
        match first_ch {
            
        }
    }
    
    
    let result = "Pig latin text";
    result.to_string()
}
