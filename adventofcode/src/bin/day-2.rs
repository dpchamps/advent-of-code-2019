use adventofcode::read_input_file;

#[derive(PartialEq)]
enum Opcode {
    Add,
    Mult,
    ProgramEnd,
    InvalidOpcode
}

impl Opcode {
    fn get_size(&self) -> usize {
        match *self {
            Opcode::Add => 4,
            Opcode::Mult => 4,
            _ => 0
        }
    }
}

fn opcode_add(lhs : i32, rhs : i32) -> i32 {
    lhs + rhs
}

fn opcode_mult(lhs : i32, rhs : i32) -> i32 {
    lhs * rhs
}

fn get_opcode(opcode : &i32) -> Opcode {
    match opcode{
        1 => Opcode::Add,
        2 => Opcode::Mult,
        99 => Opcode::ProgramEnd,
        _ => Opcode::InvalidOpcode
    }
}


fn run_opcode(opcode: Opcode, idx: usize, memory : &mut Vec<i32>){
    let op_size = opcode.get_size();
    let args : Vec<i32> = memory[idx+1..idx+op_size-1].iter()
        .map(|x| memory[*x as usize])
        .collect();

    match opcode {
        Opcode::Add =>{
            let result_idx = memory[idx + op_size-1];
            memory[result_idx as usize] = opcode_add(args[0], args[1])
        },
        Opcode::Mult => {
            let result_idx = memory[idx + op_size-1];

            memory[result_idx as usize] = opcode_mult(args[0], args[1])
        },
        _ => panic!()
    }
}

fn run_program(program : &mut Vec<i32>){
    let mut idx = 0;

    while idx < program.len(){
        let el = &program[idx];
        let opcode = get_opcode(el);

        match opcode{
            Opcode::ProgramEnd => {
                return;
            },
            Opcode::InvalidOpcode => {
                panic!(format!("Received invalid opcode {}", el))
            }
            _ => {
                let step = opcode.get_size();
                run_opcode(opcode, idx, program);

                idx += step;
            }
        }
    }
}

fn collect_input(file : &str) -> Vec<i32> {
    read_input_file(file)
        .split(",")
        .map( |x| x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect()
}

fn main(){
    let mut program_part_one = collect_input("day-2-part-1-input");

    program_part_one[1] = 12;
    program_part_one[2] = 2;

    run_program(&mut program_part_one);
    println!("{}", program_part_one[0])
}

#[cfg(test)]
mod day_2_tests{
    use crate::{get_opcode, Opcode, run_program, run_opcode};

    #[test]
    fn process_add_opcode(){
        match  get_opcode(&1){
           Opcode::Add => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn process_mult_opcode(){
        match  get_opcode(&2){
            Opcode::Mult => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn process_program_end_opcode(){
        match get_opcode(&99) {
            Opcode::ProgramEnd => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn process_invalid_opcode(){
        match get_opcode(&123) {
            Opcode::InvalidOpcode => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn run_program_test_opcodes(){
        let mut program = vec![1,2,3,0,2,4,5,1,99];
        run_program(&mut program);

        assert_eq!(
            program,
            vec![3,8,3,0,2,4,5,1,99]
        )
    }

    #[test]
    fn run_program_early_exit(){
        let mut program = vec![99,2,3,0,2,4,4,1,99];
        run_program(&mut program);

        assert_eq!(
            program,
            vec![99,2,3,0,2,4,4,1,99]
        )
    }

    #[test]
    #[should_panic]
    fn run_program_bad_opcode(){
        let mut program = vec![1,2,3,0,20,4,4,1,99,1,1,1];
        run_program(&mut program);
    }

    #[test]
    fn run_program_test_cases(){
        let mut program = vec![1,0,0,0,99];

        run_program(&mut program);
        assert_eq!(
            program,
            vec![2,0,0,0,99]
        );

        program = vec![2,3,0,3,99];

        run_program(&mut program);
        assert_eq!(
            program,
            vec![2,3,0,6,99]
        );

        program = vec![1,1,1,4,99,5,6,0,99];

        run_program(&mut program);
        assert_eq!(
            program,
            vec![30,1,1,4,2,5,6,0,99]
        );
    }
}