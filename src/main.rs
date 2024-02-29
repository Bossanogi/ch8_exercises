pub mod median_ex;
pub mod pig_latin;
use std::io;


fn main() {

    //Take lenght wanted
    println!("How much lenght of list you want?");
    let lenght_num = input_length();

    //First exercise: from list of integers take median and mode
    let numbers_vec = median_ex::generate_vec(lenght_num);
    let numbers2 = median_ex::generate();
    let (med, mode, errc) = median_ex::done(&numbers2);
        
    if errc {
        println!("Median is: {med}, mode is: {mode}");
    }    

    println!["{:?}", numbers2];
    println!("{:?}", numbers_vec);

    //Second exercise: from string of text take pig latin string
    println!("Say your words!");
    let text = input_string();
    let pig_text = pig_latin::convert(&text);

    println!("{text} to pig latin: {pig_text}");
}

fn input_length() -> usize {
    let mut lenght_string = String::new();
    loop {
        io::stdin().read_line(&mut lenght_string).expect("Failed to read line");

        let lenght_num = match lenght_string.trim_end().parse::<usize>() {
            Ok(num) => num,
            Err(e) => {
                println!("{e}"); 
                lenght_string.clear(); 
                continue;
            },
        };
        return lenght_num
    }

}

fn input_string() -> String {
    "ABCDE. This is last warning!111".to_string()
}