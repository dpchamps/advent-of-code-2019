use adventofcode::int_code_computer::IntCodeMachine;

fn part_one() -> Vec<i64>{
    let program = IntCodeMachine::read_file_into_program("day-9-part-1-input");
    let input = vec![1];
    let mut machine = IntCodeMachine::new(&program, Some(&input));

    machine.run();

    machine.output.clone()
}

fn part_two() -> Vec<i64>{
    let program = IntCodeMachine::read_file_into_program("day-9-part-1-input");
    let input = vec![2];
    let mut machine = IntCodeMachine::new(&program, Some(&input));

    machine.run();

    machine.output.clone()
}

fn main(){
    let part_one_output = part_one();

    println!("Part one: {:?}", part_one_output);

    let part_two_output = part_two();

    println!("Part two: {:?}", part_two_output);
}

#[cfg(test)]
mod day_9_tests{
    use crate::{part_one, part_two};

    #[test]
    fn part_one_answer(){
        let output = part_one();

        assert_eq!(
            output,
            [2494485073]
        )
    }
}