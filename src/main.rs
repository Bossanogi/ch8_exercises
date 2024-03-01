
use std::io::stdin;

pub mod median_ex;
pub mod pig_latin;


fn main() {
    //First exercise: from list of integers take median and mode
    let numbers2 = median_ex::generate();
    let (med, mode, errc) = median_ex::done(&numbers2);
        
    if errc {
        println!("Median is: {med}, mode is: {mode}");
    }    

    println!["{:?}", numbers2];

    //Second exercise: from string of text take pig latin string
    let text = take_string();
    let pig_text = pig_latin::convert(&text);

    println!("{text} to pig latin: {pig_text}");
}

fn take_string() -> String {
    println!("Say your words");
    let mut buffer = String::new();
    let stdin = match stdin().read_line(&mut buffer) {
        Ok(i) => i,
        Err(e) => {println!("{e}"); 0},
    };

    if stdin != 0 {
        buffer.trim_end().to_string()
    } else {
        "".to_string()
    }
}