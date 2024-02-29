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
    let text = String::from("As expected! This is last warning!");
    let pig_text = pig_latin::convert(&text);

    println!("{text} to pig latin: {pig_text}");
}
