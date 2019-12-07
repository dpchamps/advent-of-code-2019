use adventofcode::read_input_file;
use std::collections::HashMap;

#[derive(PartialEq, Copy, Clone, Debug)]
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

    fn disassemble(&self, args : Option<&[i32]>) -> String {
        match * self {
            Opcode::Add => format!("ADD &{},&{},&{}", args.unwrap()[0], args.unwrap()[1], args.unwrap()[2]),
            Opcode::Mult => format!("MULT &{},&{},&{}", args.unwrap()[0], args.unwrap()[1], args.unwrap()[2]),
            Opcode::ProgramEnd => format!("HALT"),
            Opcode::InvalidOpcode => format!("INVALID_OPCODE @{}:{}", args.unwrap()[0], args.unwrap()[1])
        }
    }

    fn introspect(&self, program_counter: usize, memory : &Vec<i32>) -> String{
        let self_size = self.get_size();

        match * self {
            Opcode::ProgramEnd|Opcode::InvalidOpcode => format!("()"),
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
        }
    }
}


fn get_opcode(opcode : &i32) -> Opcode {
    match opcode{
        1 => Opcode::Add,
        2 => Opcode::Mult,
        99 => Opcode::ProgramEnd,
        _ => Opcode::InvalidOpcode
    }
}

fn get_opcode_result(opcode : Opcode, args : Option<&[i32]> ) -> i32 {
    match opcode {
        Opcode::Add => args.unwrap()[0]+args.unwrap()[1],
        Opcode::Mult => args.unwrap()[0]*args.unwrap()[1],
        _ => 0
    }
}


fn run_opcode(opcode: Opcode, program_counter: usize, memory : &mut Vec<i32>){
    let op_size = opcode.get_size();
    let args : Vec<i32> = memory[program_counter +1..program_counter +op_size-1].iter()
        .map(|x| memory[*x as usize])
        .collect();

    println!("{:?}, {:?}", opcode, args);

    match opcode {
        Opcode::Add =>{
            let result_idx = memory[program_counter + op_size-1];
            memory[result_idx as usize] = get_opcode_result(opcode, Some(&args[..]))
        },
        Opcode::Mult => {
            let result_idx = memory[program_counter + op_size-1];

            memory[result_idx as usize] = get_opcode_result(opcode, Some(&args[..]))
        },
        _ => panic!()
    }
}

fn run_program(program : &mut Vec<i32>){
    let mut program_counter = 0;

    while program_counter < program.len(){
        let el = &program[program_counter];
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
                run_opcode(opcode, program_counter, program);

                program_counter += step;
            }
        }
    }
}

fn disassemble(program : &mut Vec<i32>) -> String {
    let mut program_counter = 0;
    let mut assembly= format!("");

    while program_counter < program.len(){
        let el = &program[program_counter];
        let opcode = get_opcode(el);
        let size = opcode.get_size();

        match opcode {
            Opcode::InvalidOpcode => {
                assembly = format!("{}{}", assembly, opcode.disassemble(Some(&[program_counter as i32, *el])));
                return assembly;
            }
            Opcode::ProgramEnd => {
                assembly = format!("{}{}:{}", assembly, program_counter, opcode.disassemble(None));
                return assembly;
            }
            _ => {
                let op_size = opcode.get_size();
                let args : Vec<i32> = program[program_counter +1..program_counter +op_size].iter().map(|x| *x).collect();

                assembly = format!(
                    "{}{}:{}={}",
                    assembly,
                    program_counter,
                    opcode.disassemble(Some(&args)),
                    opcode.introspect(program_counter, &program)
                );

                run_opcode(opcode, program_counter, program);
            }
        }

        assembly += "\n";
        program_counter += size;
    }

    assembly
}

/// Resolve an Address in memory
fn resolve(address: i32, address_resolutions : &HashMap<i32, (Opcode, i32, i32)>, program : &Vec<i32>) -> i32 {
    if address_resolutions.contains_key(&address){
        let (opcode, lhs, rhs) = address_resolutions.get(&address).unwrap();
        let resolved_lhs = resolve(*lhs, &address_resolutions, &program);
        let resolved_rhs = resolve(*rhs, &address_resolutions, &program);

        return get_opcode_result(
            *opcode,
            Some(&[resolved_lhs, resolved_rhs])
        );
    } else if address > 0 && (address as usize) < program.len() {
        return program[address as usize]
    }

    return 0;
}

fn derive(
    value :i32,
    result : i32,
    address : i32,
    address_resolutions: &HashMap<i32, (Opcode, i32, i32)>,
    program : &Vec<i32>,
) -> Option<(i32, bool)>{
    if address_resolutions.contains_key(&address){
        let (opcode, lhs, rhs) = address_resolutions.get(&address).unwrap();

        if *lhs == value && *rhs == value{
            return None
        } else if *lhs == value{
            match(opcode){
                Opcode::Add => {
                    return Some((result - resolve(*rhs, &address_resolutions, &program), true));
                }
                Opcode::Mult => {
                    return Some((result / resolve(*rhs, &address_resolutions, &program), true));
                }
                _=>panic!()
            }

        } else if *rhs == value{
            match(opcode){
                Opcode::Add => {
                    return Some((result - resolve(*lhs, &address_resolutions, &program), true));
                }
                Opcode::Mult => {
                    return Some((result / resolve(*lhs, &address_resolutions, &program), true));
                }
                _=>panic!()
            }
        } else{
            let next_lhs_result = match opcode {
                Opcode::Add => result-resolve(*rhs, &address_resolutions, &program),
                Opcode::Mult=> result/resolve(*rhs, &address_resolutions, &program),
                _ => panic!()
            };

            let next_rhs_result = match opcode {
                Opcode::Add => result-resolve(*lhs, &address_resolutions, &program),
                Opcode::Mult=> result/resolve(*lhs, &address_resolutions, &program),
                _ => panic!()
            };

            let derived_lhs = derive(value, next_lhs_result, *lhs, &address_resolutions, &program);
            let derived_rhs = derive(value, next_rhs_result, *rhs, &address_resolutions, &program);

            if let Some((result, found)) = derived_lhs{
                if found {
                    return derived_lhs;
                }
            }

            if let Some((result, found)) = derived_rhs{
                if found {
                    return derived_rhs;
                }
            }

            return None;
        }
    }

    return Some((program[address as usize], false))
}

fn collect_input(file : &str) -> Vec<i32> {
    read_input_file(file)
        .split(",")
        .map( |x| x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect()
}

fn run_program_1(){
    let mut program_part_one = collect_input("day-2-part-1-input");

    program_part_one[1] = 12;
    program_part_one[2] = 2;

    run_program(&mut program_part_one);
    println!("Program 1 Result: {}", program_part_one[0])
}

fn run_program_2(){
    let program_result = 19690720;
    let mut program = collect_input("day-2-part-2-input");

    program[1] = 0;
    program[2] = 0;

    let mut address_resolutions : HashMap<i32, (Opcode, i32, i32)> = HashMap::new();
    let mut program_counter: usize = 0;

    loop{
        let instruction = program[program_counter];
        let opcode = get_opcode(&instruction);

        let step = match opcode{
            Opcode::Add => opcode.get_size(),
            Opcode::Mult => opcode.get_size(),
            Opcode::ProgramEnd => 0,
            Opcode::InvalidOpcode => panic!("")
        };

        if step == 0 {
            break;
        }

        let arg1 = program[program_counter +1];
        let arg2 = program[program_counter +2];
        let indexer = program[program_counter +3];

        address_resolutions.insert(indexer, (opcode, arg1, arg2));
        program_counter += step;
    }

    if let Some((noun,_)) = derive(1, program_result, 0, &address_resolutions, &program){
        program[1] = noun;

    }else{
        println!("Couldn't derive a noun");
        return
    }

    if let Some((verb,_)) = derive(2, program_result, 0, &address_resolutions, &program){
        program[2] = verb;
    } else {
        println!("Couldn't derive a verb");
        return;
    }

    println!("Noun: {}, Verb: {}", program[1], program[2]);
    run_program(&mut program);
    println!("Program 2 Result: {}", program[0]);
    println!("Answer: {}", 100* program[1]+program[2]);
    assert_eq!(program[0], program_result);
}

fn main(){
    run_program_1();
    run_program_2();
}

#[cfg(test)]
mod day_2_tests{
    use crate::{get_opcode, Opcode, run_program, run_opcode, disassemble};

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
        println!("Here");
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