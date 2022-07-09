
#[allow(unused)]
mod anagram;
mod clock;
mod fibonacci;
mod forth;
mod forth_two;
mod gigasecond;
mod health_statistics;
mod low_power_game;
mod luhn;
mod macros_example;
mod magazine_cutout;
mod minesweeper;
mod parallel_frequency;
mod poker;
mod resistor_color;
mod reverse_string;
mod rpg;
mod rpn_calc;
mod space_age;
mod sublist;
mod sum_of_sqr;
mod prime_factors;
mod proverb;
mod raindrops;
mod sum_of_multiples;
mod high_scores;
mod matching_brackets;
mod acronym;
mod all_your_bases;

fn main() {
    let mut f = forth_two::Forth::new();

    f.eval(": fib over over + ;").unwrap();
    println!("{:?}", f.eval("1 1 loop 10 : fib ;"));
    println!("{:?}", f.stack());
    // println!("{:?}", resistor_color::color_to_value(resistor_color::ResistorColor::Green));
    // println!("Hello, world!");
}
