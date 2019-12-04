use adventofcode::read_input_file;

#[derive(PartialEq)]
enum Operation{
    Add,
    Mult
}

#[derive(PartialEq)]
enum Opcode {
    Operation(Operation),
    ProgramEnd,
    InvalidOpcode
}

fn opcode_add(lhs : i32, rhs : i32) -> i32 {
    lhs + rhs
}

fn opcode_mult(lhs : i32, rhs : i32) -> i32 {
    lhs * rhs
}

fn get_opcode(opcode : &i32) -> Opcode {
    match opcode{
        1 => Opcode::Operation(Operation::Add),
        2 => Opcode::Operation(Operation::Mult),
        99 => Opcode::ProgramEnd,
        _ => Opcode::InvalidOpcode
    }
}

fn run_operation(operation : Operation, args: &[i32]) -> i32{
    match operation {
        Operation::Add => opcode_add(args[0], args[1]),
        Operation::Mult => opcode_mult(args[0], args[1])
    }
}

fn run_program(program : &mut Vec<i32>){
    for idx in (0..program.len()).step_by(4){
        let el = &program[idx];

        match get_opcode(el){
            Opcode::Operation(operation) => {
                let args : Vec<i32> = program[idx+1..idx+3].iter().map(|x| program[*x as usize]).collect();
                let result_index = program[idx+3];

                program[result_index as usize] = run_operation(operation, &args[..]);
            },
            Opcode::ProgramEnd => {
                return;
            },
            Opcode::InvalidOpcode => {
                panic!(format!("Received invalid opcode {}", el))
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
    use crate::{get_opcode, Opcode, Operation, run_program, run_operation};

    #[test]
    fn process_add_opcode(){
        match  get_opcode(&1){
            Opcode::Operation(operation) => assert!(operation == Operation::Add),
            _ => panic!()
        }

        assert_eq!(
            run_operation(Operation::Add, &vec![2,2]),
            4
        )
    }

    #[test]
    fn process_mult_opcode(){
        match  get_opcode(&2){
            Opcode::Operation(operation) => assert!(operation == Operation::Mult),
            _ => panic!()
        }

        assert_eq!(
            run_operation(Operation::Mult, &vec![6,5]),
            30
        )
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