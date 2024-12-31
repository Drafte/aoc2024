fn main() {
    let input = std::fs::read_to_string("C:\\s\\aoc\\day2\\src\\input.txt").unwrap();

    //for the input file, calculate the number of lines that are safe sequences
    let safe_sequences = input.lines().filter(|x| safe_sequence(x)).count();
    println!("Safe sequences: {}", safe_sequences);

    let safe_tolerant_sequences = input.lines().filter(|x| safe_tolerant_sequence(x)).count();
    println!("Safe tolerant sequences: {}", safe_tolerant_sequences);

}

fn safe_sequence(input: &str) -> bool {
    //tokenize the input into vector of integers
    let input: Vec<i32> = input.split(" ").map(|x| x.parse().unwrap()).collect();

    check_sequence_trend(input)
}
    
fn check_sequence_trend(input: Vec<i32>) -> bool {
    //get direction of the first two elements
    let first_diff = input[1] - input[0];
    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        return false;
    }
    for i in 1..input.len()-1 {
        //check the sign of the first_diff and ensure it matches the sign of the next difference
        if (first_diff > 0 && input[i+1] - input[i] < 0) || (first_diff < 0 && input[i+1] - input[i] > 0) {
            println!("{} {} {} {}", first_diff, input[i+1], input[i], input[i+1] - input[i]);
            return false;
        }
    
        if (input[i+1] - input[i]).abs() < 1 || (input[i+1] - input[i]).abs() > 3 {
            println!("{} {} {} {}", first_diff, input[i+1], input[i], input[i+1] - input[i]);
            return false;
        }
    }
    true
}    


fn safe_tolerant_sequence(input: &str) -> bool {
    //tokenize the input into vector of integers
    let mut input: Vec<i32> = input.split(" ").map(|x| x.parse().unwrap()).collect();

    //get direction of the first two elements
    let first_diff = input[1] - input[0];
    if first_diff.abs() < 1 || first_diff.abs() > 3 {
        let mut input1 = input.clone();
        input.remove(1);
        if input.len() < 2 {
            return true;
        }
        input1.remove(0);
        return check_sequence_trend(input) || check_sequence_trend(input1);
    }

    for i in 1..input.len()-1 {
        //check the sign of the first_diff and ensure it matches the sign of the next difference
        if (first_diff > 0 && input[i+1] - input[i] < 0) || (first_diff < 0 && input[i+1] - input[i] > 0) {
            let mut input1 = input.clone();
            let mut input2 = input.clone();

            input2.remove(i-1);
            input1.remove(i+1);
            input.remove(i);
            return check_sequence_trend(input) || check_sequence_trend(input1) || check_sequence_trend(input2);
        }

        if (input[i+1] - input[i]).abs() < 1 || (input[i+1] - input[i]).abs() > 3 {
            let mut input1 = input.clone();
            let mut input2 = input.clone();

            input2.remove(i-1);
            input1.remove(i+1);
            input.remove(i);
            return check_sequence_trend(input) || check_sequence_trend(input1) || check_sequence_trend(input2);
        }
    }

    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn safe_tolerant_sequence_test() {
        assert_eq!(safe_tolerant_sequence("1 2 3 4 5"), true);
        assert_eq!(safe_tolerant_sequence("1 2 3 4 6"), true);
        assert_eq!(safe_tolerant_sequence("1 3 6 7 9"), true);
        assert_eq!(safe_tolerant_sequence("8 6 4 4 1"), true);
        assert_eq!(safe_tolerant_sequence("1 3 2 4 5"), true);
        assert_eq!(safe_tolerant_sequence("9 7 6 2 1"), false);
        assert_eq!(safe_tolerant_sequence("1 2 7 8 9"), false);
        assert_eq!(safe_tolerant_sequence("7 6 4 2 1"), true);
        assert_eq!(safe_tolerant_sequence("7 6 4 1 1"), true);
        assert_eq!(safe_tolerant_sequence("7 7 4 1 1"), false);
        assert_eq!(safe_tolerant_sequence("7 11"), true); //not within input
        assert_eq!(safe_tolerant_sequence("7 11 4"), true);
        assert_eq!(safe_tolerant_sequence("7 11 8"), true);
        assert_eq!(safe_tolerant_sequence("8 11 7"), true);
        assert_eq!(safe_tolerant_sequence("7 14 4 1 13"), false);
        assert_eq!(safe_tolerant_sequence("10 6 7 6 4"), true);
        assert_eq!(safe_tolerant_sequence("6 10 7 6 4"), true);
        assert_eq!(safe_tolerant_sequence("0 6 7 8 9"), true);
        assert_eq!(safe_tolerant_sequence("6 7 8 9 0"), true);
        assert_eq!(safe_tolerant_sequence("10 0 7 6 4"), true);
        assert_eq!(safe_tolerant_sequence("10 7 0 6 4"), true);
        assert_eq!(safe_tolerant_sequence("10 7 6 0 4"), true);
        assert_eq!(safe_tolerant_sequence("10 7 6 4 0"), true);
        assert_eq!(safe_tolerant_sequence("0 10 1 3 4"), true);
        
        //check for decreasing sequences
        assert_eq!(safe_tolerant_sequence("5 4 3 2 1"), true);

        //check for increasing sequences
        assert_eq!(safe_tolerant_sequence("1 2 3 4 5"), true);

        //check for repeated sequences
        assert_eq!(safe_tolerant_sequence("1 1 1 1 1"), false);

        //check for oscillating sequences
        assert_eq!(safe_tolerant_sequence("1 3 2 4 3"), false);

        //check for edge cases with 2 elements
        assert_eq!(safe_tolerant_sequence("1 3"), true);
        assert_eq!(safe_tolerant_sequence("3 1"), true);
        assert_eq!(safe_tolerant_sequence("1 9"), true);
        assert_eq!(safe_tolerant_sequence("9 1"), true);


        //check for repeating elements
        assert_eq!(safe_tolerant_sequence("1 1"), true);
        assert_eq!(safe_tolerant_sequence("3 3 4"), true);
        assert_eq!(safe_tolerant_sequence("3 3 3"), false);
        assert_eq!(safe_tolerant_sequence("3 4 4"), true);
        assert_eq!(safe_tolerant_sequence("3 4 4 5"), true);
        assert_eq!(safe_tolerant_sequence("3 3 4 5"), true);
        assert_eq!(safe_tolerant_sequence("3 4 5 5"), true);


        //check for edge cases with 3 elements
        assert_eq!(safe_tolerant_sequence("1 3 4"), true);
        assert_eq!(safe_tolerant_sequence("1 4 3"), true);
        assert_eq!(safe_tolerant_sequence("3 1 4"), true);
        assert_eq!(safe_tolerant_sequence("3 4 1"), true);
        assert_eq!(safe_tolerant_sequence("4 1 3"), true);
        assert_eq!(safe_tolerant_sequence("4 3 1"), true);
        assert_eq!(safe_tolerant_sequence("1 4 7"), true);
        assert_eq!(safe_tolerant_sequence("1 7 4"), true);
        assert_eq!(safe_tolerant_sequence("4 1 7"), true);
        assert_eq!(safe_tolerant_sequence("4 1 1"), true);
        assert_eq!(safe_tolerant_sequence("4 1 0"), true);
        assert_eq!(safe_tolerant_sequence("1 1 4"), true);
        assert_eq!(safe_tolerant_sequence("0 10 1"), true);
        assert_eq!(safe_tolerant_sequence("1 10 0"), true);
        assert_eq!(safe_tolerant_sequence("10 1 0"), true);
        assert_eq!(safe_tolerant_sequence("10 11 0"), true);
        assert_eq!(safe_tolerant_sequence("10 0 1"), true);
        assert_eq!(safe_tolerant_sequence("10 1 0"), true);
        assert_eq!(safe_tolerant_sequence("1 0 10"), true);
        assert_eq!(safe_tolerant_sequence("0 1 10"), true);


        //check for edge cases with 4 elements
        assert_eq!(safe_tolerant_sequence("1 3 4 5"), true);
        assert_eq!(safe_tolerant_sequence("1 4 3 5"), true);
        assert_eq!(safe_tolerant_sequence("1 4 5 3"), true);
        assert_eq!(safe_tolerant_sequence("3 1 4 5"), true);
        
        //check for value 5 being in 4 different spots in the sequence
        assert_eq!(safe_tolerant_sequence("3 2 5 1"), true);
        assert_eq!(safe_tolerant_sequence("3 5 2 1"), true);
        assert_eq!(safe_tolerant_sequence("5 3 2 1"), true);
        assert_eq!(safe_tolerant_sequence("3 2 1 5"), true);

        //check for repeated sequences at the edges
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4"), true);
        assert_eq!(safe_tolerant_sequence("1 2 3 4 4"), true);
        assert_eq!(safe_tolerant_sequence("1 2 2 3 4"), true);
        assert_eq!(safe_tolerant_sequence("1 2 3 3 4"), true);
        assert_eq!(safe_tolerant_sequence("1 2 3 4 4"), true);
        assert_eq!(safe_tolerant_sequence("5 2 2 4 3"), false);
        assert_eq!(safe_tolerant_sequence("5 2 3 4 5"), true);


        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4"), false);
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4 5"), false);
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4 5 5"), false);
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4 5 5 6"), false);
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4 5 5 6 6"), false);
        assert_eq!(safe_tolerant_sequence("1 1 2 3 4 4 5 5 6 6 7"), false);

        
}
}