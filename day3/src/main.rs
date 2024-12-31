use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("C:\\s\\aoc\\day3\\src\\input.txt").unwrap();
    
    //split the input using "dont()" as delimiter
    let parts: Vec<&str> = input.split("don't()").collect();

    let mut sum = find_mul_regex(&input);
    println!("Sum of all the results: {}", sum);

    //skip the first part
    //iterate over the parts and print them
    for (_, part) in parts.iter().skip(1).enumerate() {
        //split the part using "do()" as delimiter
        let sub_parts: Vec<&str> = part.split("do()").collect();
        let sum_parts = find_mul_regex(sub_parts[0]);
        sum -= sum_parts;
    }

    println!("Sum of results without don't()s: {}", sum);
}



fn find_mul_regex(input: &str) -> u64 {
    //reduce the sequence of characters into sequence of "mul(X, Y)" strings where X and Y are 1-3 digit numbers
    //use Regex to match mul(X, Y) and compute  the result of X * Y
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let mut results = vec![];
    for cap in re.captures_iter(input) {
        let x = cap[1].parse::<u64>().unwrap();
        let y = cap[2].parse::<u64>().unwrap();
        results.push(x * y);
    }

    //calculate the sum of all the results
    results.iter().sum()
}