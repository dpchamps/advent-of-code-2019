use adventofcode::int_code_computer::IntCodeMachine;


fn main(){
    let mut program = IntCodeMachine::read_file_into_program("day-5-part-1-input");

    IntCodeMachine::run_program(&mut program, None);
}