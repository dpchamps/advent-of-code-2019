use adventofcode::int_code_computer::IntCodeMachine;
use permutohedron::heap_recursive;
use std::borrow::BorrowMut;

fn get_phase_permutations(base : &mut [i32]) -> Vec<Vec<i32>>{
//    let mut base = [0,1,2,3,4];
    let mut permutations : Vec<Vec<i32>> = vec![];

    heap_recursive(base, |permutation| { permutations.push(permutation.to_vec())} );

    permutations
}

fn create_amp(phase : i32, input_signal : i32, program : &Vec<i32>) -> IntCodeMachine {
    let input = vec![phase, input_signal];

    let int_machine = IntCodeMachine::new(&program.clone(), Some(&input));

    int_machine
}

fn run_amplification_circuit(phase_sequence: &Vec<i32>, program : &Vec<i32>) -> Result<i32, &'static str>{
    let mut signal = 0;
    for phase in phase_sequence{
        let mut amp = create_amp(*phase, signal, program);


       amp.run()?;

        signal = amp.output[0];
    }

    Ok(signal)
}

fn run_feedback_loop(phase_sequence: &Vec<i32>, program : &Vec<i32>) -> Result<i32, &'static str> {
    let mut amps : Vec<IntCodeMachine> = vec![];

    for phase in phase_sequence{
//        println!("Initializing amp with phase: {}", phase);
        let mut machine = IntCodeMachine::new(program, None);

        machine.run()?;
        machine.send_input(*phase);

        amps.push(
            machine
        )
    }

//    println!("Total amps: {}", amps.len());

    let mut i = 0;
    let mut last_input = 0;
    let mut last_output = 0;

    loop {
        let amp = &mut amps[i];


        if !amp.program_complete {
            amp.send_input(last_input)?;
            if let Some(output) = amp.output.pop(){
                last_input = output
            }

        }else if i == 4{
//            println!("Amp {} complete {}", i, last_input);
            break
        }

        i = if i == amps.len()-1{
            0
        }else{
            i + 1
        }

    }

    Ok(last_input)
}

fn run_part_one_permutations(program : &Vec<i32>) -> (i32, Vec<i32>){
    let permutations = get_phase_permutations(&mut [0,1,2,3,4]);

    let mut max_signal = -std::i32::MAX;
    let mut best_permutation = vec![];

    for permutation in permutations {
        if let Ok(result) = run_amplification_circuit(&permutation, program){
            if result > max_signal{
                max_signal = result;
                best_permutation = permutation;
            }
        }
    }

    (max_signal, best_permutation)
}

fn run_part_two_permutations(program : &Vec<i32>) -> (i32, Vec<i32>){
    let permutations = get_phase_permutations(&mut [5,6,7,8,9]);

    let mut max_signal = -std::i32::MAX;
    let mut best_permutation = vec![];

    for permutation in permutations {
        if let Ok(result) = run_feedback_loop(&permutation, program){
            if result > max_signal{
                max_signal = result;
                best_permutation = permutation;
            }
        }
    }

    (max_signal, best_permutation)
}

fn part_one(){
    let mut program = IntCodeMachine::read_file_into_program("day-7-part-1-input");
    let (max_result, best_permutation) = run_part_one_permutations(&program);

    println!("Part one: {}, {:?}", max_result, best_permutation);
}

fn part_two(){
    let mut program = IntCodeMachine::read_file_into_program("day-7-part-1-input");
    let (max_result, best_permutation) = run_part_two_permutations(&program);

    println!("Part one: {}, {:?}", max_result, best_permutation);
}

fn main(){
    part_one();
    part_two();
}

#[cfg(test)]
mod day_7_tests{
    use crate::{run_part_one_permutations, run_feedback_loop, run_part_two_permutations};
    use adventofcode::int_code_computer::IntCodeMachine;

    #[test]
    fn example_one(){
        let mut program = vec![3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0];
        let (max_result, best_permutation) = run_part_one_permutations(&program);

        assert_eq!(max_result, 43210);
        assert_eq!(best_permutation, vec![4,3,2,1,0])
    }

    #[test]
    fn example_two(){
        let mut program = vec![3,23,3,24,1002,24,10,24,1002,23,-1,23, 101,5,23,23,1,24,23,23,4,23,99,0,0];
        let (max_result, best_permutation) = run_part_one_permutations(&program);

        assert_eq!(max_result, 54321);
        assert_eq!(best_permutation, vec![0,1,2,3,4])
    }

    #[test]
    fn example_three(){
        let mut program = vec![3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33, 1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0];
        let (max_result, best_permutation) = run_part_one_permutations(&program);

        assert_eq!(max_result, 65210);
        assert_eq!(best_permutation, vec![1,0,4,3,2])
    }

    #[test]
    fn example_four(){
        let mut program = vec![3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5];
        let (max_result, best_permutation) = run_part_two_permutations(&program);

        assert_eq!(max_result, 139629729);
        assert_eq!(best_permutation, vec![9,8,7,6,5])
    }

    #[test]
    fn example_five(){
        let mut program = vec![3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,
                               -5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,
                               53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10];
        let (max_result, best_permutation) = run_part_two_permutations(&program);

        assert_eq!(max_result, 18216);
        assert_eq!(best_permutation, vec![9,7,8,5,6])
    }
}