use adventofcode::read_input_file;


fn collect_input(file : &str) -> Vec<i32> {
    read_input_file(file)
        .split("\n")
        .map( |x| x.parse::<i32>())
        .filter_map(|x| x.ok())
        .collect()
}

fn calculate_fuel_cost( mass : i32 ) -> i32 {
    (mass / 3) - 2
}

fn calculate_fuel_cost_rec(mass : i32) -> i32{
    let cost = calculate_fuel_cost(mass);

    if cost > 0 {
        return cost + calculate_fuel_cost_rec(cost)
    }

    return 0
}

fn sum_fuel_cost_for_modules( modules : Vec<i32>, fuel_calculator : fn(i32) -> i32) -> i32 {
    modules.into_iter().map(fuel_calculator).sum()
}

fn main(){
    let part_1_fuel_cost =
        sum_fuel_cost_for_modules(
            collect_input("day-1-part-1-input"),
            calculate_fuel_cost
        );

    let part_2_fuel_cost =
        sum_fuel_cost_for_modules(
            collect_input("day-1-part-2-input"),
            calculate_fuel_cost_rec
        );


    println!(
        "Part 1 {}\nPart 2 {}",
        part_1_fuel_cost,
        part_2_fuel_cost
    )
}

#[cfg(test)]
mod day_1_tests{
    use crate::{calculate_fuel_cost, calculate_fuel_cost_rec, sum_fuel_cost_for_modules};

    #[test]
    fn fuel_cost(){
        assert_eq!(calculate_fuel_cost(12), 2);
        assert_eq!(calculate_fuel_cost(14), 2);
        assert_eq!(calculate_fuel_cost(1969), 654);
        assert_eq!(calculate_fuel_cost(100756), 33583);
    }

    #[test]
    fn fuel_cost_recursive(){
        assert_eq!(calculate_fuel_cost_rec(14), 2);
        assert_eq!(calculate_fuel_cost_rec(1969), 966);
        assert_eq!(calculate_fuel_cost_rec(100756), 50346);
    }

    #[test]
    fn sum_costs(){
        assert_eq!(
            sum_fuel_cost_for_modules(vec![12, 14, 1969, 100756], calculate_fuel_cost),
            34241
        )
    }
}