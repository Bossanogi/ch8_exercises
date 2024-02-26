pub mod median_ex;
use crate::median_ex::func::done;
use crate::median_ex::func::generate;

fn main() {
    let numbers2 = generate();
    let (med, mode, errc) = done(&numbers2);
        
    if errc {
        println!("Median is: {med}, mode is: {mode}");
    }    

    println!["{:?}", numbers2];

}
