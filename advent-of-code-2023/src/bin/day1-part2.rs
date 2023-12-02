fn main() {
    let lines = include_str!("../../input/day1.txt").split('\n');
    let mut num_sum = 0 as usize;
    for line in lines {
        cheack(line, &mut num_sum);
    }
    println!("{}", num_sum);
}

fn cheack(line: &str, num_sum: &mut usize) {
    let mut line = line.to_string();
    line = line.replace("one", "1ne");
    line = line.replace("two", "2wo");
    line = line.replace("three", "3hree");
    line = line.replace("four", "4our");
    line = line.replace("five", "5ive");
    line = line.replace("six", "6ix");
    line = line.replace("seven", "7even");
    line = line.replace("eight", "8ight");
    line = line.replace("nine", "9ine");

    line = line.replace("on8ight", "18");
    line = line.replace("nin8ight", "98");
    line = line.replace("thre8ight", "38");
    line = line.replace("fiv8ight", "58");

    line = line.replace("seve9ine", "79");

    line = line.replace("tw1ne", "21");
    line = line.replace("eigh2wo", "82");

    let mut tem = 0 as usize;
    for chars in line.chars() {
        if chars.is_ascii_digit() {
            tem += chars.to_digit(10).unwrap() as usize;
            break;
        }
    }

    for chars in line.chars().rev() {
        if chars.is_ascii_digit() {
            tem = (tem * 10) + chars.to_digit(10).unwrap() as usize;
            break;
        }
    }
    *num_sum += tem;
}
