mod os_commands;
mod error_handling;
mod generics;
mod traits;
mod api;
mod compress;
mod decompress;

fn main() {
    // os_commands::commands();
    // error_handling::error_handling();
    // let sum = generics::sum_of_numbers(1, 3);
    // println!("{}", sum);
    // traits::traits_example();
    // api::miner::hello();
    // compress::compress();
    decompress::decompress();
}
