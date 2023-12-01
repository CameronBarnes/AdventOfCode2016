fn is_possible(str: &str) -> bool {

    let mut iter = str.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();

    let total = a + b + c;
    let biggest = i32::max(a, i32::max(b, c));
    let remainder = total - biggest;
    remainder > biggest

}

fn main() {

    let input = include_str!("./input.txt");
    let result = input.lines().filter(|line| is_possible(line)).count();

    println!("{}", result);

}
