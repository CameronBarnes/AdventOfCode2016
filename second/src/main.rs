fn solve_for_line(str: &str, mut out: String, mut start: (i32, i32)) -> (String, (i32, i32)) {

    let buttons = [[1, 4, 7], [2, 5, 8], [3, 6, 9]];

    str.chars().for_each(|c| {
        let act = match c {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid input"),
        };
        start.0 = i32::min(2, i32::max(0, start.0 + act.0));
        start.1 = i32::min(2, i32::max(0, start.1 + act.1));
    });

    out.push_str(&buttons[start.0 as usize][start.1 as usize].to_string());
    (out, start)

}

fn main() {
    let input = include_str!("./input.txt");
    let (result, _) = input.lines()
        .fold((String::new(), (1, 1)), |(out, start), str| {
        solve_for_line(str, out, start)
    });
    println!("Answer is: {result}");
}
