fn main() {
    //read file input into two vectors. each line has two integers, which need to be split between the 2 vectors
    let (mut x, mut y) = (Vec::new(), Vec::new());
    let input = std::fs::read_to_string("C:\\s\\aoc\\day1\\src\\input.txt").unwrap();
    for line in input.lines() {
        //split each line using whitespaces as delimiter
        let mut split = line.split_whitespace();
        x.push(split.next().unwrap().parse::<i32>().unwrap());
        y.push(split.next().unwrap().parse::<i32>().unwrap());
    }

    x.sort();
    y.sort();

    //calculate the manhattan distance between the two vectors
    let mut distance: i32 = 0;
    for i in 0..x.len() {
        distance += absdistance(x[i], y[i]);
    }
    println!("The Manhattan distance between the two vectors is: {}", distance);

    //calculate the similarity between the two vectors
    let mut similarity: i32 = 0;
    for i in 0..x.len() {
        //count how many times x[i] is present in y
        //nit: figure out how to break early from this loop
        let count = y.iter().filter(|&n| *n == x[i]).count();

        similarity += x[i] * count as i32;
    }
    println!("The similarity between the two vectors is: {}", similarity);

}

fn absdistance(a: i32, b: i32) -> i32 {
    (a - b).abs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distance() {
        assert_eq!(absdistance(1, 2), 1);
        assert_eq!(absdistance(2, 1), 1);
        assert_eq!(absdistance(1, 1), 0);
        assert_eq!(absdistance(0, 0), 0);
        assert_eq!(absdistance(-1, 1), 2);
        assert_eq!(absdistance(-1, -1), 0);
    }
}