use crate::read_input_file;
use std::io;
use std::io::Write;
use std::collections::HashMap;
use std::hash::Hash;

type MemoryMap = HashMap<(usize, usize), Vec<i64>>;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct OpcodeArg {
    parameter_mode : i64,
    value: i64,
    address : i64
}

impl OpcodeArg {
    fn new(parameter_mode : i64, value : i64, address : i64) -> Self{
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
    Equals,
    RBO
}

impl std::fmt::Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Opcode::Add => "ADD",
            Opcode::Mult => "MULT",
            Opcode::Equals => "EQUALS",
            Opcode::LessThan => "LESS_THAN",
            Opcode::JumpIfTrue => "JUMP_TRUE",
            Opcode::JumpIfFalse => "JUMP_FALSE",
            Opcode::ProgramEnd => "HALT",
            Opcode::Input => "INPUT",
            Opcode::Output => "OUTPUT",
            Opcode::RBO => "RBO",
        };

        write!(f, "{}", name)
    }
}

impl Opcode {
    pub fn new(input : i64) -> Result<(Self, i64), &'static str>{
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
            9 => Opcode::RBO,
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
            Opcode::RBO => 2,
            Opcode::ProgramEnd => 1
        }
    }

    pub fn disassemble(&self, args : &[OpcodeArg]) -> String {
        let arg_string = args.iter().fold(String::new(), |arg_string, arg| {
            format!("{}, ({}, (&{}, MODE:{}))", arg_string, arg.value, arg.address, arg.parameter_mode)
        });

        format!("{}{}", self, arg_string)
    }
}

pub struct IntCodeMachine{
    pub program : Vec<i64>,
    pub output : Vec<i64>,
    pub is_halted: bool,
    pub program_complete : bool,

    std_input: bool,
    input : Vec<i64>,
    program_counter : usize,
    relative_base_offset : usize,
    output_dasm : bool
}

impl IntCodeMachine {
    pub fn new(program : &Vec<i64>, input : Option<&Vec<i64>>) -> Self {
        let mut input = match input {
            Some(given_input) => given_input.clone(),
            _ => vec![]
        };

        // TODO: sick interpreter hack. Move this to a proper memory map.
        let mut program_space : Vec<i64> = vec![0; program.len()*10];
        let mut program = program.clone();

        program.append(&mut program_space);

        input.reverse();

        Self {
            program,
            input,
            output : vec![],
            program_counter : 0,
            std_input: false,
            is_halted : false,
            program_complete : false,
            relative_base_offset : 0,
            output_dasm : false
        }
    }

    pub fn set_debug_mode(&mut self){
        self.output_dasm = true;
    }

    pub fn read_file_into_program(program_name : &str) -> Vec<i64>{
        read_input_file(program_name)
            .split(",")
            .map( |x| x.parse::<i64>())
            .filter_map(|x| x.ok())
            .collect()
    }

    fn get_input_from_stdin() -> Result<i64, &'static str> {
        let mut input = String::new();
        print!("Input: ");
        io::stdout().flush();
        if let Err(_) = io::stdin().read_line(&mut input){
            return Err("Failed to extract user input");
        }
        if let Ok(parse_input) = input.trim().parse::<i64>(){
            Ok(parse_input)
        }else{
            return Err("Failed to parse user input");
        }
    }


    fn is_valid_pc_state(&self) -> Result<(), &'static str>{
        if self.program_counter > 0 && self.program_counter < self.program.len(){
            return Ok(())
        }

        Err("Program in invalid state")
    }

    fn get_input(&mut self) -> Result<i64, &'static str>{
        match self.input.pop() {
            Some(result) => Ok(result),
            _ => {
                if self.std_input {
                    return IntCodeMachine::get_input_from_stdin()
                }

                // Hack: indicate wait needs to happen
                self.is_halted = true;
                Ok(0)
            }
        }
    }

    fn set_std_input(&mut self, receive_input : bool){
        self.std_input = receive_input;
    }

    fn compute(opcode : &Opcode, args : &Vec<OpcodeArg>) -> Option<i64> {
        match opcode {
            Opcode::Add => Some(args[0].value+args[1].value),
            Opcode::Mult => Some(args[0].value*args[1].value),
            _ => None
        }
    }

    fn extract_args(&mut self, opcode : &Opcode, parameter_mode : &i64) -> Result<Vec<OpcodeArg>, &'static str> {
        let mut args : Vec<OpcodeArg> = vec![];
        let mut mode : i64 = parameter_mode.clone();
        let arg_range = self.program_counter+1..self.program_counter+opcode.get_size();

        if opcode == &Opcode::ProgramEnd{
            return Ok(args);
        }

        for (idx, value) in self.program[arg_range].iter().enumerate(){
            let next_mode = mode % 10;
            let next_arg = match next_mode {
                0 => OpcodeArg::new(next_mode, self.program[*value as usize], *value),
                1 => OpcodeArg::new(next_mode, *value, *value),
                2 => {
                    let relative_offset = (*value + (self.relative_base_offset as (i64))) as usize;

                    OpcodeArg::new(next_mode, self.program[relative_offset], relative_offset as i64)
                },
                _ => return Err("Invalid parameter mode")
            };

            args.push(next_arg);
            mode = mode / 10;

        }

        return Ok(args);
    }

    fn run_cycle(&mut self) -> Result<(Opcode, Vec<OpcodeArg>), &'static str>{
        let (opcode, parameter_mode) = Opcode::new(self.program[self.program_counter])?;
        let args = self.extract_args(&opcode, &parameter_mode)?;
        let result = IntCodeMachine::compute(&opcode, &args);

        if self.output_dasm {
            println!("\t{}", opcode.disassemble(&args));
        }

        match opcode {
            Opcode::Add|Opcode::Mult => {
                self.program[args[2].address as usize] = result.unwrap();
            }
            Opcode::Input => {
                let input_result = self.get_input()?;
                // Hack
                if self.is_halted {
                    return Ok((opcode, args));
                }

                self.program[args[0].address as usize] = input_result;
            },
            Opcode::Output => {
                self.output.push(args[0].value);
            },
            Opcode::JumpIfTrue => {
                if args[0].value != 0 {
                    self.program_counter = args[1].value as usize;
                    return Ok((opcode, args));
                }
            },
            Opcode::JumpIfFalse => {
                if args[0].value == 0 {
                    self.program_counter = args[1].value as usize;
                    return Ok((opcode, args));
                }
            },
            Opcode::LessThan => {
                self.program[args[2].address as usize] = (args[0].value < args[1].value) as i64;
            },
            Opcode::Equals => {
                self.program[args[2].address as usize] = (args[0].value == args[1].value) as i64;
            },
            Opcode::RBO => {
                let next = self.relative_base_offset as i64 + args[0].value;
                self.relative_base_offset = next as usize;
            }
            Opcode::ProgramEnd => return Ok((opcode, args))
        }

        self.program_counter += opcode.get_size();

        Ok((opcode, args))
    }

    pub fn send_input(&mut self, input : i64) -> Result<(), &'static str>{
        if self.program_complete {
            return Err("Attempt to send input to program that is no longer running.");
        }

        self.input.push(input);
        self.is_halted = false;

        self.run()
    }

    pub fn run(&mut self) -> Result<(), &'static str>{

        loop{
            let (last_opcode,_) = self.run_cycle()?;

            self.is_valid_pc_state()?;

            if self.is_halted {
                break;
            }

            if last_opcode == Opcode::ProgramEnd{
                self.program_complete = true;
                self.is_halted = true;
                break;
            }
        }

        Ok(())
    }

    // Todo: Now that there's branching, need to sweep to cover all reachable program space.
    pub fn disassemble(&mut self) -> Result<String, &'static str>{
        let mut assembly = String::new();

        loop {
            let (last_opcode, args) = self.run_cycle()?;

            assembly = format!("{}\n{}", assembly, last_opcode.disassemble(&args));

            if last_opcode == Opcode::ProgramEnd {
                break;
            }
        }

        Ok(assembly)
    }

    // Run a program an mutate the input array
    pub fn run_program(program : &mut Vec<i64>, args : Option<&Vec<i64>>) -> Result<Vec<i64>, &'static str>{
        let mut int_machine = IntCodeMachine::new(program, args);

        int_machine.run();
        *program = int_machine.program;

        Ok(int_machine.output.clone())
    }
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


    #[test]
    fn run_program_test_cases(){
        let mut program = vec![1,0,0,0,99];

        IntCodeMachine::run_program(&mut program, None).unwrap();
        assert_eq!(
            program[..5],
            [2,0,0,0,99]
        );

        program = vec![2,3,0,3,99];

        IntCodeMachine::run_program(&mut program, None).unwrap();
        assert_eq!(
            program[..5],
            [2,3,0,6,99]
        );

        program = vec![1,1,1,4,99,5,6,0,99];

        IntCodeMachine::run_program(&mut program, None).unwrap();
        assert_eq!(
            program[..9],
            [30,1,1,4,2,5,6,0,99]
        );
    }

    #[test]
    fn test_user_input(){
        let mut program = vec![3,1,1,1,2,1,99];
        let input = vec![9];

        IntCodeMachine::run_program(&mut program, Some(&input)).unwrap();

        assert_eq!(
            program[..7],
            [3,10,1,1,2,1,99]
        )
    }

    #[test]
    fn test_wait_for_input(){
        let program = vec![3,1,1,1,2,1,99];
        let mut machine = IntCodeMachine::new(&program, None);

        machine.run().unwrap();

        assert!(!machine.program_complete);
        assert!(machine.is_halted);

        machine.send_input(9).unwrap();

        assert!(machine.program_complete);
        assert!(machine.is_halted);

        assert_eq!(
            machine.program[..7],
            [3,10,1,1,2,1,99]
        )
    }

    #[test]
    fn test_output(){
        let mut program = vec![3,1,4,1,99];
        let input = vec![1337];

        let output = IntCodeMachine::run_program(&mut program, Some(&input)).unwrap();

        assert_eq!(
            output,
            vec![1337]
        );
    }

    #[test]
    fn disassembly(){
        let program = vec![3, 1, 4, 1, 1101, 2, 2, 1, 1102, 5, 5, 2, 99];
        let input = vec![1337];

        let mut int_machine = IntCodeMachine::new(&program, Some(&input));

        let dasm = int_machine.disassemble().unwrap();

        assert_eq!(dasm,
"
INPUT, (1, (&1, MODE:0))
OUTPUT, (1337, (&1, MODE:0))
ADD, (2, (&2, MODE:1)), (2, (&2, MODE:1)), (1337, (&1, MODE:0))
MULT, (5, (&5, MODE:1)), (5, (&5, MODE:1)), (4, (&2, MODE:0))
HALT"
        )
    }

    #[test]
    fn large_number(){
        let program = vec![1102,34915192,34915192,7,4,7,99,0];
        let mut machine = IntCodeMachine::new(&program, None);

        machine.run().unwrap();

        assert_eq!(
            machine.output[0],
            1_219_070_632_396_864
        );

        let program = vec![104,1125899906842624,99];
        let mut machine = IntCodeMachine::new(&program, None);

        machine.run().unwrap();

        assert_eq!(
            machine.output[0],
            1_125_899_906_842_624
        );
    }

    #[test]
    fn quine(){
        let program = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut machine = IntCodeMachine::new(&program, None);

        machine.run().unwrap();

        assert_eq!(
            machine.output,
            vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99]
        )
    }
}