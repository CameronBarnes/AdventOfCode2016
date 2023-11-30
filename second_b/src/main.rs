fn limit_move(start: (i32, i32), buttons: &Vec<Vec<char>>) -> ((i32, i32), (i32, i32)) {

    let x_limit = buttons.get(start.1 as usize).unwrap().len() as i32;
    let y_limit = buttons.get(start.0 as usize).unwrap().len() as i32;

    let x_limit = (5 - x_limit) / 2;
    let y_limit = (5 - y_limit) / 2;

    // println!("Start: {:?}, x_limit: {}, y_limit: {}", start, x_limit, y_limit);

    ((x_limit, 4 - x_limit), (y_limit, 4 - y_limit))

}

fn solve_for_line(str: &str, mut out: String, mut start: (i32, i32)) -> (String, (i32, i32)) {

    let mut buttons = Vec::new();
    buttons.push(          vec!('1'));
    buttons.push(     vec!('2', '3', '4'));
    buttons.push(vec!('5', '6', '7', '8', '9'));
    buttons.push(     vec!('A', 'B', 'C'));
    buttons.push(          vec!('D'));

    str.chars().for_each(|c| {
        let act = match c {
            'U' => (0, -1),
            'D' => (0, 1),
            'L' => (-1, 0),
            'R' => (1, 0),
            _ => panic!("Invalid input"),
        };
        let limits = limit_move(start, &buttons);
        start.0 = i32::min(limits.0.1, i32::max(limits.0.0, start.0 + act.0));
        start.1 = i32::min(limits.1.1, i32::max(limits.1.0, start.1 + act.1));
    });

    out.push_str(&buttons[start.1 as usize][start.0 as usize].to_string());
    (out, start)

}

fn main() {
    let input = include_str!("./input.txt");
    let (result, _) = input.lines()
        .fold((String::new(), (2, 2)), |(out, start), str| {
        solve_for_line(str, out, start)
    });
    println!("Answer is: {result}");
}
