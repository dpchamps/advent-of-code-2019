use std::borrow::Borrow;
use std::collections::HashMap;

struct PassCoderator{
    digits : Vec<i32>
}

impl PassCoderator {
    fn new(start : i32) -> Self {
        let digits : Vec<i32> = start.to_string().chars().map(|x| x.to_digit(10).unwrap() as i32).collect();

        Self {
            digits
        }
    }

    fn increment(&mut self){
        let mut place = self.digits.len()-1;

        loop{
            if self.digits[place] == 9 {
                self.digits[place] = 0;
                if place == 0{
                    let mut next = vec![1];
                    self.digits.iter().for_each(|x| next.push(*x));
                    self.digits = next;
                    break;
                }else{
                    place -= 1;
                }
            }else {
                self.digits[place] += 1;
                break;
            }
        }
    }

    fn to_number(&self) -> i32{
        let mut str : String = "".to_string();

        for digit in &self.digits{
            str = format!("{}{}", str, digit.clone());
        }

        str.parse::<i32>().unwrap()
    }

    fn is_valid_passcode(&self) -> bool {
        let mut has_double = false;
        let mut counts: (i32, i32) = (-1, 0);

        for (idx, digit) in self.digits.iter().enumerate(){

            if counts.0 == *digit{
                counts.1 += 1;
            }else{
                if counts.1 > 0  {
                    has_double = true;
                }
                counts.0 = *digit;
                counts.1 = 0;
            }


            if idx > 0 && self.digits[idx-1] > *digit {
                return false;
            }
        }

        if counts.1 > 0 {
            has_double = true;
        }

        has_double
    }

    fn is_valid_passcode_2(&self) -> bool {
        let mut has_double = false;
        let mut counts: (i32, i32) = (-1, 0);

        for (idx, digit) in self.digits.iter().enumerate(){

            if counts.0 == *digit{
                counts.1 += 1;
            }else{
                if counts.1 == 1  {
                    has_double = true;
                }
                counts.0 = *digit;
                counts.1 = 0;
            }


            if idx > 0 && self.digits[idx-1] > *digit {
                return false;
            }
        }

        if counts.1 == 1 {
            has_double = true;
        }

        has_double
    }
}

fn part_one(){
    // 240298-784956
    let mut passcoderator = PassCoderator::new(240298);
    let mut current = 240298;
    let mut valid_passcodes = 0;
    while current < 784956 {
        if passcoderator.is_valid_passcode() {
            valid_passcodes += 1;
        }

        current += 1;
        passcoderator.increment();
    }

    println!("Passcoderator is: {}", passcoderator.to_number());
    println!("Possible passcodes {}", valid_passcodes);
}

fn part_two(){
    // 240298-784956
    let mut passcoderator = PassCoderator::new(240298);
    let mut current = 240298;
    let mut valid_passcodes = 0;
    while current < 784956 {
        if passcoderator.is_valid_passcode_2() {
            valid_passcodes += 1;
        }

        current += 1;
        passcoderator.increment();
    }

    println!("Passcoderator is: {}", passcoderator.to_number());
    println!("Possible passcodes {}", valid_passcodes);
}

fn main(){
    part_one();
    part_two();
}

#[cfg(test)]
mod day_3_tests{
    use crate::*;

    #[test]
    fn pass_code_parses_input(){
        let passcoderator = PassCoderator::new(2000);

        assert_eq!(
            passcoderator.digits,
            vec![2,0,0,0]
        )
    }

    #[test]
    fn increments_number(){
        let mut passcoderator = PassCoderator::new(998);

        passcoderator.increment();

        assert_eq!(
            passcoderator.to_number(),
            999
        );

        passcoderator.increment();

        assert_eq!(
            passcoderator.to_number(),
            1000
        );
    }

    #[test]
    fn is_valid_passcode(){
        assert!(PassCoderator::new(122345).is_valid_passcode());
        assert!(PassCoderator::new(111122).is_valid_passcode());
        assert!(PassCoderator::new(223333).is_valid_passcode());
        assert!(PassCoderator::new(111111).is_valid_passcode());

        assert!(!PassCoderator::new(223450).is_valid_passcode());
        assert!(!PassCoderator::new(123789).is_valid_passcode());
    }
    #[test]
    fn is_valid_passcode_2(){
        assert!(PassCoderator::new(122345).is_valid_passcode_2());
        assert!(PassCoderator::new(111122).is_valid_passcode_2());
        assert!(PassCoderator::new(223333).is_valid_passcode_2());

        assert!(!PassCoderator::new(223450).is_valid_passcode_2());
        assert!(!PassCoderator::new(123444).is_valid_passcode_2());
        assert!(!PassCoderator::new(123789).is_valid_passcode_2());
    }

}