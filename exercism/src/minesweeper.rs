#[allow(unused)]
#[derive(Clone, Copy)]
enum Field {
    Mine,
    Val(u32),
}

pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let mut solved: Vec<Vec<Field>> =
        vec![vec![Field::Val(0); minefield.get(0).unwrap_or(&"").len()]; minefield.len()];

    for (i, row) in minefield.into_iter().enumerate() {
        for (j, field) in row.bytes().enumerate() {
            if field == b'*' {
                annotate_mine(&mut solved, i, j);
            }
        }
    }

    solved
        .into_iter()
        .map(|row| {
            row.into_iter()
                .map(|field| match field {
                    Field::Val(n) => {
                        if n > 0 {
                            char::from_digit(n, 9).unwrap_or('8')
                        } else {
                            ' '
                        }
                    }
                    Field::Mine => '*',
                })
                .collect::<String>()
        })
        .collect()
}

fn annotate_mine(fields: &mut Vec<Vec<Field>>, x: usize, y: usize) {
    for i in -1..=1 {
        for j in -1..=1 {
            let pos_field = fields
                .get_mut((x as i32 + i) as usize)
                .and_then(|row| row.get_mut((y as i32 + j) as usize));

            if i == 0 && j == 0 {
                // let field_mine = fields.get_mut(x).and_then(|row| row.get_mut(y));
                if let Some(field) = pos_field {
                    *field = Field::Mine;
                }
            } else if let Some(Field::Val(n)) = pos_field {
                if (*n) < 8 {
                    *n = *n + 1;
                }
            };
        }
    }
}

fn remove_annotations(board: &[&str]) -> Vec<String> {
    board.iter().map(|r| remove_annotations_in_row(r)).collect()
}

fn remove_annotations_in_row(row: &str) -> String {
    row.chars()
        .map(|ch| match ch {
            '*' => '*',
            _ => ' ',
        })
        .collect()
}

fn run_test(test_case: &[&str]) {
    let cleaned = remove_annotations(test_case);

    let cleaned_strs = cleaned.iter().map(|r| &r[..]).collect::<Vec<_>>();

    let expected = test_case.iter().map(|&r| r.to_string()).collect::<Vec<_>>();

    assert_eq!(expected, annotate(&cleaned_strs));
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
fn horizontal_line_mines_at_edges() {
    #[rustfmt::skip]
        run_test(&[
            "*1 1*",
            ]);
}
