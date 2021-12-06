#[inline]
fn get_precedence(c: char) -> u8 {
    match c {
        '^' => 3,
        '*' | '/' | '%' => 2,
        '+' | '-' => 1,
        '(' => 0,
        _ => unreachable!(),
    }
}

fn calc_expr(line: String) -> f64 {
    let mut stack_op = vec!['('];
    let mut stack_num = vec![];
    let mut s = String::new();
    let mut pre_c = '('; // only for - (unary) and (nothing).digit
    for c in line.chars().chain([')'].into_iter()) {
        match c {
            '(' => stack_op.push('('),
            '0'..='9' => s.push(c),
            '.' => s.push_str(if let '0'..='9' = pre_c { "." } else { "0." }),
            _ => {
                if s != "" {
                    stack_num.push(s.parse().unwrap());
                    s = String::new();
                } else if c == '-' && pre_c == '(' {
                    s.push(c);
                    pre_c = '-';
                    continue;
                }
                match c {
                    ')' => {
                        while stack_op.last().unwrap() != &'(' {
                            calc_binary(&mut stack_num, &mut stack_op);
                        }
                        stack_op.pop();
                    }
                    '+' | '-' | '*' | '/' | '%' | '^' => {
                        while get_precedence(c) <= get_precedence(*stack_op.last().unwrap()) {
                            calc_binary(&mut stack_num, &mut stack_op);
                        }
                        stack_op.push(c)
                    }
                    _ => (), // space
                }
            }
        }
        if c != ' ' {
            pre_c = c;
        }
    }
    stack_num[0]
}

fn calc_binary(stack_num: &mut Vec<f64>, stack_op: &mut Vec<char>) {
    let x2 = stack_num.pop().unwrap();
    let x1 = stack_num.pop().unwrap();
    match stack_op.pop().unwrap() {
        '+' => stack_num.push(x1 + x2),
        '-' => stack_num.push(x1 - x2),
        '*' => stack_num.push(x1 * x2),
        '/' => stack_num.push(x1 / x2),
        '%' => stack_num.push(x1 % x2),
        '^' => stack_num.push(x1.powf(x2)),
        _ => unreachable!(),
    }
}

#[test]
fn test_18_2() {
    assert_eq!(39f64, calc_expr("1 + (-2 * 3) + (4 * (5 + 6))".to_owned()));
    assert_eq!(6973568802f64, calc_expr("2 * 3 ^ (4 * 5)".to_owned()));
    assert_eq!(74f64, calc_expr("5 + (8 * 3 + 9 + 3 * 4 * 3)".to_owned()));
    assert_eq!(
        47604f64,
        (100. * calc_expr("5 - 9.2 * (-7.1 * 3 * 3 + 0.9 * 3 + (8 + 6 % 4))".to_owned())).ceil()
    );
    assert_eq!(
        10184f64,
        (100. * calc_expr("((.2 + 4 * 9) * (6 + 9 * .8 + 6) / 6) + 2 - 4 ^ 2".to_owned())).floor()
    );
}
