pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut result: Vec<Vec<char>> = Vec::new();
    for line in minefield {
        let mut vector: Vec<char> = Vec::new();
        for point in line.chars() {
            vector.push(point);
        }
        result.push(vector);
    }
    {
        let mut row = 0;
        for line in minefield {
            let mut col = 0;
            for point in line.chars() {
                if point == '*' {
                    raise_count_nearby(&mut result, row, col);
                }
                col += 1;
            }
            row += 1;
        }
    }
    let mut output: Vec<String> = Vec::new();
    for line in result {
        let mut string = String::new();
        for point in line {
            string.push(point);
        }
        output.push(string);
    }
    return output;
}

pub fn raise_count_nearby(field: &mut Vec<Vec<char>>, row: usize, col: usize) {
    let height = field.len();
    let width = match field.last() {
        Some(string) => string.len(),
        _ => 0,
    };
    let directions: Vec<(i32, i32)> = vec![(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];
    for direction in directions {
        let target_row = row as i32 + direction.0;
        let is_row_valid = (0 <= target_row) && (target_row < height as i32);
        let target_col = col as i32 + direction.1;
        let is_col_valid = (0 <= target_col) && (target_col < width as i32);
        if is_row_valid && is_col_valid {
            let point = match field[target_row as usize][target_col as usize] {
                '*' => '*',
                ' ' => '1',
                etc => {
                    let digit: i32 = etc.to_string().parse().unwrap();
                    let character = (digit + 1).to_string().chars().nth(0).unwrap();
                    character
                },
                _ => ' ',
            };
            field[target_row as usize][target_col as usize] = point;
        }
    }
}

#[cfg(test)]
fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

#[cfg(test)]
fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

#[cfg(test)]
fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);
    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();
    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();
    assert_eq!(expected, annotate(&cleaned_strs));
}

#[test]
fn no_rows() {
    #[rustfmt::skip]
    run_test(&[
    ]);
}

#[test]
fn no_columns() {
    #[rustfmt::skip]
    run_test(&[
        "",
    ]);
}

#[test]
fn no_mines() {
    #[rustfmt::skip]
    run_test(&[
        "   ",
        "   ",
        "   ",
    ]);
}

#[test]
fn board_with_only_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "***",
        "***",
    ]);
}

#[test]
fn mine_surrounded_by_spaces() {
    #[rustfmt::skip]
    run_test(&[
        "111",
        "1*1",
        "111",
    ]);
}

#[test]
fn space_surrounded_by_mines() {
    #[rustfmt::skip]
    run_test(&[
        "***",
        "*8*",
        "***",
    ]);
}

#[test]
fn horizontal_line() {
    #[rustfmt::skip]
    run_test(&[
        "1*2*1",
    ]);
}

#[test]
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*1 1*",
    ]);
}

#[test]
fn vertical_line() {
    #[rustfmt::skip]
    run_test(&[
        "1",
        "*",
        "2",
        "*",
        "1",
    ]);
}

#[test]
fn vertical_line_mines_at_edges() {
    #[rustfmt::skip]
    run_test(&[
        "*",
        "1",
        " ",
        "1",
        "*",
    ]);
}

#[test]
fn cross() {
    #[rustfmt::skip]
    run_test(&[
        " 2*2 ",
        "25*52",
        "*****",
        "25*52",
        " 2*2 ",
    ]);
}

#[test]
fn large_board() {
    #[rustfmt::skip]
    run_test(&[
        "1*22*1",
        "12*322",
        " 123*2",
        "112*4*",
        "1*22*2",
        "111111",
    ]);
}
