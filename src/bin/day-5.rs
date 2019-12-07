use adventofcode::{read_input_file, int_code_computer};

fn part_one(){
    int_code_computer::run("day-5-part-1-input");
}

fn part_two(){
    let mut prog = vec![3,9,8,9,10,9,4,9,99,-1,8];

    int_code_computer::run_program(&mut prog);
}

fn main(){
    int_code_computer::run("day-5-part-1-input");
}