use crate::read_input_file;
use crate::int_code_computer::Opcode::Output;
use std::io;
use std::io::Write;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct OpcodeArg {
    parameter_mode : i32,
    value: i32,
    address : i32
}

impl OpcodeArg {
    fn new(parameter_mode : i32, value : i32, address : i32) -> Self{
        Self {parameter_mode, value, address }
    }
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum Opcode{
    Add,
    Mult,
    ProgramEnd,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals
}

impl Opcode {
    pub fn new(input : i32) -> Result<(Self, i32), &'static str>{
        let parameter_mode = input/100;
        let opcode_input = input - (parameter_mode * 100);

        let opcode = match opcode_input {
            1 => Opcode::Add,
            2 => Opcode::Mult,
            3 => Opcode::Input,
            4 => Opcode::Output,
            5 => Opcode::JumpIfTrue,
            6 => Opcode::JumpIfFalse,
            7 => Opcode::LessThan,
            8 => Opcode::Equals,
            99 => Opcode::ProgramEnd,
            _ => return Err("Invalid Opcode Input")
        };

        Ok((opcode, parameter_mode))
    }

    pub fn get_size(&self) -> usize {
        match *self {
            Opcode::Add => 4,
            Opcode::Mult => 4,
            Opcode::Input => 2,
            Opcode::Output => 2,
            Opcode::JumpIfTrue => 3,
            Opcode::JumpIfFalse => 3,
            Opcode::LessThan => 4,
            Opcode::Equals => 4,
            Opcode::ProgramEnd => 1
        }
    }

    pub fn disassemble(&self, args : &Vec<OpcodeArg>) -> String {
        match * self {
            Opcode::Add => format!("ADD &{:?},&{:?},&{:?}", args[0], args[1], args[2]),
            Opcode::Mult => format!("MULT &{:?},&{:?},&{:?}", args[0], args[1], args[2]),
            Opcode::Input => format!("INPUT &{:?}", args[0]),
            Opcode::Output => format!("OUTPUT&{:?}", args[0]),
            Opcode::ProgramEnd => format!("HALT"),
            _ => format!("{:?}, {:?}", self, args)
        }
    }

    pub fn introspect(&self, program_counter: usize, memory : &Vec<i32>) -> String{
        let self_size = self.get_size();

        match * self {
            Opcode::ProgramEnd => format!("()"),
            Opcode::Add => {
                let arg1 = memory[memory[program_counter +1] as usize];
                let arg2 = memory[memory[program_counter +2] as usize];

                format!("({}+{}={})", arg1, arg2, arg1+arg2)
            }
            Opcode::Mult => {
                let arg1 = memory[memory[program_counter +1] as usize];
                let arg2 = memory[memory[program_counter +2] as usize] ;

                format!("({}*{}={})", arg1, arg2, arg1*arg2)
            }
            _ => "".to_string()
        }
    }
}

fn load_program(program_name : &str) -> Vec<i32>{
    read_input_file(program_name)
        .split(",")
        .map( |x| x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect()
}

fn compute_result(opcode : &Opcode, args : &Vec<OpcodeArg>) -> Option<i32> {
    match opcode {
        Opcode::Add => Some(args[0].value+args[1].value),
        Opcode::Mult => Some(args[0].value*args[1].value),
        _ => None
    }
}

fn extract_args(opcode : &Opcode, parameter_mode : &i32, program : &Vec<i32>, program_counter : usize) -> Result<Vec<OpcodeArg>, &'static str> {
    let mut args : Vec<OpcodeArg> = vec![];
    let mut mode : i32 = parameter_mode.clone();

    if opcode == &Opcode::ProgramEnd{
        return Ok(args);
    }

    for (idx, value) in program[program_counter+1..program_counter+opcode.get_size()].iter().enumerate(){
        let next_mode = mode % 10;
        let next_arg = match next_mode {
            0 => OpcodeArg::new(next_mode, program[*value as usize], *value),
            1 => OpcodeArg::new(next_mode, *value, *value),
            _ => return Err("Invalid parameter mode")
        };

        args.push(next_arg);
        mode = mode / 10;

    }

    return Ok(args);
}

pub fn run_program(program : &mut Vec<i32>) -> Result<(), &'static str>{
    let mut program_counter = 0;

    loop {
        let (opcode, parameter_mode) = Opcode::new(program[program_counter])?;
        let args = extract_args(&opcode, &parameter_mode, &program, program_counter)?;
        let result = compute_result(&opcode, &args);

        match opcode {
            Opcode::Add|Opcode::Mult => {
                program[args[2].address as usize] = result.unwrap();
            }
            Opcode::Input => {
                let mut input = String::with_capacity(1);
                print!("Input: ");
                io::stdout().flush();
                if let Err(_) = io::stdin().read_line(&mut input){
                    return Err("Failed to extract user input");
                }
                if let Ok(parse_input) = input.trim().parse::<i32>(){
                    program[args[0].address as usize] = parse_input;
                }else{
                    return Err("Failed to parse user input");
                }
            },
            Opcode::Output => {
                println!("{}", args[0].value);
            },
            Opcode::JumpIfTrue => {
                if args[0].value != 0 {
                    program_counter = args[1].value as usize;
                    continue;
                }
            },
            Opcode::JumpIfFalse => {
                if args[0].value == 0 {
                    program_counter = args[1].value as usize;
                    continue;
                }
            },
            Opcode::LessThan => {
                program[args[2].address as usize] = (args[0].value < args[1].value) as i32;
            },
            Opcode::Equals => {
                program[args[2].address as usize] = (args[0].value == args[1].value) as i32;
            },
            Opcode::ProgramEnd => return Ok(())
        }

        program_counter += opcode.get_size();

        if program_counter > program.len(){
            break;
        }
    }

    Ok(())
}

pub fn run(program_name : &str) -> Result<Vec<i32>, &'static str>{
    let mut program = load_program(program_name);

    run_program(&mut program)?;

    Ok(program)
}


#[cfg(test)]
mod intcode_tests{
    use crate::int_code_computer::*;

    #[test]
    fn process_add_opcode() {
        match  Opcode::new(1){
            Ok((Opcode::Add, 0)) => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn process_mult_opcode(){
        match  Opcode::new(2){
            Ok((Opcode::Mult, 0)) => assert!(true),
            _ => panic!()
        }
    }

    #[test]
    fn process_param_mode(){
        let (opcode, param_mode) = Opcode::new(10102).unwrap();

        assert_eq!(param_mode, 101);
        assert!(opcode == Opcode::Mult);
    }

//    #[test]
//    fn run_program_test_opcodes(){
//        let mut program = vec![1,2,3,0,2,4,5,1,99];
//        run_program(&mut program).unwrap_err();
//
//        assert_eq!(
//            program,
//            vec![3,8,3,0,2,4,5,1,99]
//        )
//    }

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

//    #[test]
//    fn test_user_input(){
//        let mut program = vec![3,0,4,0,99];
//
//        run_program(&mut program).unwrap();
//    }
}