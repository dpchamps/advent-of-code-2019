use std::fs;
use std::io;

static INPUT_FILE : &str = "resources/day-1-input";

fn get_input() -> Result<Vec<i32>, io::Error> {
    Ok(
        fs::read_to_string(INPUT_FILE)?
            .split("\n")
            .map( |x| x.parse::<i32>())
            .filter_map(|x| x.ok())
            .collect()
    )
}

fn calculate_fuel_cost( mass : i32 ) -> i32 {
    (mass / 3) - 2
}

fn sum_fuel_cost_for_modules( modules : Vec<i32>) -> i32 {
    modules.into_iter().map(calculate_fuel_cost).sum()
}

fn main(){
    println!(
        "{}",
        sum_fuel_cost_for_modules(get_input().unwrap())
    )
}

#[cfg(test)]
mod tests{
    use crate::{calculate_fuel_cost, sum_fuel_cost_for_modules};

    #[test]
    fn fuel_cost(){
        assert_eq!(calculate_fuel_cost(12), 2);
        assert_eq!(calculate_fuel_cost(14), 2);
        assert_eq!(calculate_fuel_cost(1969), 654);
        assert_eq!(calculate_fuel_cost(100756), 33583);
    }

    #[test]
    fn sum_costs(){
        assert_eq!(
            sum_fuel_cost_for_modules(vec![12, 14, 1969, 100756]),
            34241
        )
    }
}