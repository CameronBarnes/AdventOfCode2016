fn is_possible(a: i32, b: i32, c: i32) -> bool {
    let total = a + b + c;
    let max = i32::max(a, i32::max(b, c));
    let remainder = total - max;
    remainder > max
}

fn parse_line(str: &str) -> (i32, i32, i32) {
    let mut iter = str.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();

    (a, b, c)
}

fn main() {
    
    let input = include_str!("./input.txt");
    let mut iter = input.lines();

    let mut counter = 0;

    while let (Some(a), Some(b), Some(c)) = (iter.next(), iter.next(), iter.next()) {

        let a = parse_line(a);
        let b = parse_line(b);
        let c = parse_line(c);

        if is_possible(a.0, b.0, c.0) {
            counter += 1;
        }
        if is_possible(a.1, b.1, c.1) {
            counter += 1;
        }
        if is_possible(a.2, b.2, c.2) {
            counter += 1;
        }

    }

    println!("{counter}")

}
