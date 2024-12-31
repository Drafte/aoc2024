use regex::Regex;

type Char2DArray = Vec<Vec<char>>;

type StringArray = Vec<String>;

fn main() {
    let input = std::fs::read_to_string("C:\\s\\aoc\\day4\\src\\input.txt").unwrap();
    
    let mut count = 0;
    let mut arr: Char2DArray = Vec::new();
    for line in input.lines() {
        //read each input line into a vector 
        arr.push(line.chars().collect());
    }

    //create StringArray from input.lines()
    let mut arr_result: StringArray = Vec::new();
    for i in 0..arr.len() {
        let s: String = arr[i].iter().collect();
        arr_result.push(s);
    }

    //count instances of XMAS or SAMX regex in arr_result straight
    count += get_xmas_samx_count(arr_result);

    // for i in 0..(arr.len() - 1) {
    //     for j in 0..(arr.len() - 1) {
    //         print!("{}", arr[i][j]);
    //     }
    //     println!();
    // }

    //count instances of XMAS or SAMX regex in arr diagonally
    count += read_diagonal_top_to_bottom(&arr);
    count += read_diagonal_bottom_to_top(&arr);

    //count instances of XMAS or SAMX regex in arr column-wise
    count += read_column_wise_top_to_bottom(&arr);

    println!("count: {}", count);

    //split the input into 3x3 grids
    let mut arr_3x3: Char2DArray = Vec::new();
    for i in 0..(arr.len() - 2) {
        for j in 0..(arr.len() - 2) {
            let mut arr_3x3_inner: Vec<char> = Vec::new();
            for k in 0..3 {
                for l in 0..3 {
                    arr_3x3_inner.push(arr[i + k][j + l]);
                }
            }
            arr_3x3.push(arr_3x3_inner);
        }
    }

    //count using regex how many 3x3 grids contain the following patterns:
    //M.S.A.M.S, S.S.A.M.M, M.M.A.S.S, S.M.A.S.M
    let mut count_3x3 = 0;
    for i in 0..arr_3x3.len() {
        let s: String = arr_3x3[i].iter().collect();
        let re = Regex::new(r"M.S.A.M.S|S.S.A.M.M|M.M.A.S.S|S.M.A.S.M").unwrap();
        if re.is_match(&s) {
            count_3x3 += 1;
        }
    } 

    println!("count_3x3: {}", count_3x3);

}

fn get_xmas_samx_count(arr_result: StringArray) -> u32 {
    //count instances of XMAS or SAMX regex in arr_result
    //match multiple instances of XMAS or SAMX in arr_result
    let re_xmas = Regex::new(r"XMAS").unwrap();
    let re_samx = Regex::new(r"SAMX").unwrap();
    let mut count = 0;
    for i in arr_result.iter() {
        println!("i: {}", i);
        count += re_xmas.find_iter(i).count();
        count += re_samx.find_iter(i).count();
    }

    count as u32
}

fn read_column_wise_top_to_bottom(arr: &Char2DArray) -> u32 {
    let mut arr_result: StringArray = Vec::new();
    for i in 0..arr.len() {
        let mut s = String::new();
        for j in 0..arr.len() {
            s.push_str(&arr[j][i].to_string());
        }
        arr_result.push(s);
    }

    get_xmas_samx_count(arr_result)
}

// this function reads 4 words diagonally from a 2D array, top to bottom
fn read_diagonal_top_to_bottom(arr: &Char2DArray) -> u32 {
    let mut arr_result: StringArray = Vec::new();
    
    //read in a string from all diagonals in arr, such as [0][0], [1][1], [2][2], etc., followed by [0][1], [1][2], [2][3], etc., followed by [1][0], [2][1], [3][2], etc. in sequence
    for i in 0..arr.len() - 1 {
        for j in 0..arr.len() - 1 {
            if i > 0 && j > 0 {
                continue;
            }
            let mut s = String::new();
            let mut k = i;
            let mut l = j;
            while k <= arr.len() - 1 && l <= arr.len() - 1 {
                // println!("arr[{}],[{}]", k, l);
                s.push_str(&arr[k][l].to_string());
                k += 1;
                l += 1;
            }
            if j == arr.len() - 3 {
                break;
            }
            arr_result.push(s);
        }

        if i == arr.len() - 4 {
            break;
        }
    }

    get_xmas_samx_count(arr_result)
}

//this function reads 4 words diagonally from a 2D array, bottom to top
fn read_diagonal_bottom_to_top(arr: &Char2DArray) -> u32
{
    let mut arr_result: StringArray = Vec::new();

    //read in a string from all diagonals in arr, such as [4][0], [3][1], [2][2], etc., followed by [4][1], [3][2], [2][3], etc., followed by [4][2], [3][3], [2][4], etc. in sequence
    for i in (0..arr.len()).rev() {
        for j in 0..arr.len() - 1 {
            if i < arr.len() - 1 && j > 0 {
                continue;
            }
            let mut s = String::new();
            let mut k = i;
            let mut l = j;
            while k >= 0 && l <= arr.len() - 1 {
                // println!("arr[{}],[{}]", k, l);
                s.push_str(&arr[k][l].to_string());
                if k == 0 {
                    break
                }
                k -= 1;
                l += 1;
            }
            if j == arr.len() - 3 {
                break;
            }
            arr_result.push(s);
        }

        if i == 3 {
            break;
        }
    }

    get_xmas_samx_count(arr_result)
}
