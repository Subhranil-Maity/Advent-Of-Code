
fn main() {
    let lines = include_str!("../../input/day1.txt").split('\n');
    let mut num_sum = 0 as usize;
    
    for line in lines{
      cheack(line, &mut num_sum);
    }
    println!("{}", num_sum);
}

fn cheack(line: &str, num_sum: &mut usize){
    let mut tem = 0 as usize;
    for chars in line.chars(){
       if chars.is_ascii_digit(){
            tem += chars.to_digit(10).unwrap() as usize;
            break;
       }
    }

    for chars in line.chars().rev(){
        if chars.is_ascii_digit(){
            tem = (tem*10) + chars.to_digit(10).unwrap() as usize;
            break;
       }
    }
    *num_sum += tem;
}
