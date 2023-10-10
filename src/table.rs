fn get_longest_word(ingredients: Vec<&str>) -> &str {
    let all_singular_words: Vec<&str> = ingredients
        .iter()
        .flat_map(|x| x.split_whitespace())
        .collect();

    let longest_word = all_singular_words
        .iter()
        .max_by_key(|&&word| word.len())
        .unwrap_or(&"");

    longest_word
}

fn get_longest_ingredient_height(ingredients: Vec<&str>) -> usize {
    let most_number_of_words: usize = ingredients
        .iter()
        .map(|ingredient| ingredient.split_whitespace().count())
        .max()
        .unwrap_or(0);

    most_number_of_words
}

fn get_seperator_string(column_cell_size: usize) -> String {
    let sep = "-".repeat(column_cell_size);

    format!("+{}+{}+{}+", sep, sep, sep)
}

fn add_next_table_row(table: String, row: String) -> String {
    return format!("{}\n{}", table, row);
}

#[derive(Eq, PartialEq, Debug)]
struct RowIngredient {
    words: Vec<String>,
    padding: Vec<bool>,
}

fn add_padding(row_string: String, column_cell_size: usize) -> String {
    return format!("{} {}", row_string, " ".repeat(column_cell_size));
}

fn add_centered_ingredient(
    row_string: String,
    ingredient: String,
    column_cell_size: usize,
) -> String {
    let total_padding = column_cell_size - ingredient.len();

    let padding_per_side = (total_padding / 2) as f32;
    let front_padding = padding_per_side.floor() as usize;
    let back_padding = total_padding - front_padding;

    return format!(
        "{}{}{}{}",
        row_string,
        " ".repeat(front_padding),
        ingredient,
        " ".repeat(back_padding)
    );
}

fn create_row_ingredients(ingredients: Vec<&str>, row_cell_size: i32) -> Vec<RowIngredient> {
    let mut row: Vec<RowIngredient> = Vec::new();

    for ingredient in ingredients {
        let mut words: Vec<&str> = ingredient.split(' ').collect();
        let mut padding: Vec<bool> = Vec::new();
        let mut padded_words: Vec<String> = Vec::new();

        // Deze padding functie kan nog gerefactoord worden
        // Het moet gecentreerd dus
        let total_padding = row_cell_size - words.len() as i32;
        let boundary_padding = (total_padding / 2) as f32;
        let top_padding = boundary_padding.floor() as i32;
        let bottom_padding = total_padding - top_padding;

        for row_num in 0..row_cell_size as i32 {
            if top_padding - row_num > 0 {
                padded_words.push(String::from(""));
                padding.push(true);
            } else if row_cell_size - bottom_padding <= row_num {
                padded_words.push(String::from(""));
                padding.push(true)
            } else {
                padded_words.push(String::from(words.remove(0)));
                padding.push(false)
            }
        }

        let row_ingredient = RowIngredient {
            words: padded_words,
            padding,
        };

        row.push(row_ingredient)
    }

    return row;
}

fn build_rows(
    row_ingredients: Vec<RowIngredient>,
    row_cell_size: usize,
    column_cell_size: usize,
) -> Vec<String> {
    let mut rows: Vec<String> = Vec::new();

    for row_number in 0..row_cell_size {
        let mut row_string = String::from("|");

        for ingredient in row_ingredients.iter() {
            let padding = ingredient.padding[row_number];
            if padding == true {
                row_string = add_padding(row_string.clone(), column_cell_size - 1);
            } else {
                row_string = add_centered_ingredient(
                    row_string.clone(),
                    ingredient.words[row_number].clone(),
                    column_cell_size,
                );
            }

            row_string = format!("{}|", row_string);
        }
        rows.push(row_string);
    }

    return rows;
}

fn create_ingredient_row(
    column_cell_size: usize,
    row_cell_size: usize,
    ingredients: Vec<&str>,
) -> String {
    let row_ingredients = create_row_ingredients(ingredients, row_cell_size as i32);

    let rows = build_rows(row_ingredients, row_cell_size, column_cell_size);

    return rows.join("\n");
}

const PADDING: usize = 2; // 1 padding at both sides
const NUM_INGREDIENT_ROWS: usize = 3;
const ROW_SEPERATORS: usize = 4; // first row, after first ingredient, after second ingredient and last row

// Test voor maken
fn create_table(
    ingredients: Vec<&str>,
    longest_word_length: usize,
    longest_ingredient_height: usize,
) -> String {
    let column_cell_size = longest_word_length + PADDING;
    let row_cell_size = longest_ingredient_height + PADDING;

    let num_rows = row_cell_size * NUM_INGREDIENT_ROWS + ROW_SEPERATORS;

    let seperator = get_seperator_string(column_cell_size);

    let mut table = String::from("```\n");

    let mut num_padding = 0;

    for row_number in 0..num_rows {
        let ingredient_row = (row_number as f32 / row_cell_size as f32).floor() as usize;

        if row_cell_size * num_padding + ingredient_row == row_number {
            table = add_next_table_row(table.clone(), seperator.clone());
            num_padding += 1;
        }

        if row_number % row_cell_size == 0 && row_number != 0 {
            let row_ingredients: Vec<&str> =
                ingredients[(ingredient_row - 1) * 3..(ingredient_row - 1) * 3 + 3].to_vec();

            let row = create_ingredient_row(column_cell_size, row_cell_size, row_ingredients);

            table = format!("{}\n{}", table.clone(), row);
        }
    }

    table = format!("{}\n```", table.clone());

    return table;
}

// Test voor maken
pub fn print_recipe_table(ingredients: Vec<&str>) -> String {
    let longest_word = get_longest_word(ingredients.clone());
    let longest_ingredient_height = get_longest_ingredient_height(ingredients.clone());

    let table = create_table(ingredients, longest_word.len(), longest_ingredient_height);

    return table;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_word() {
        let words = vec!["short", "longer", "longest"];
        let expectation = "longest";

        let result = get_longest_word(words);

        assert_eq!(result, expectation)
    }

    #[test]
    fn longest_multiple_words() {
        let words = vec!["short", "longer longest", "short"];
        let expectation = "longest";

        let result = get_longest_word(words);

        assert_eq!(result, expectation)
    }

    #[test]
    fn ingredient_height_one_word() {
        let words = vec!["one", "one", "one"];
        let expectation = 1;

        let result = get_longest_ingredient_height(words);

        assert_eq!(result, expectation)
    }

    #[test]
    fn ingredient_height_multiple_words() {
        let words = vec!["one two", "one two three", "one"];
        let expectation = 3;

        let result = get_longest_ingredient_height(words);

        assert_eq!(result, expectation)
    }

    #[test]
    fn centered_ingredient() {
        let expected_string = String::from("  test  ");

        let result = add_centered_ingredient(String::from(""), String::from("test"), 8);

        assert_eq!(result, expected_string);
    }

    #[test]
    fn row_ingredients() {
        let expected_row_ingredients = vec![
            RowIngredient {
                words: vec![String::from(""), String::from("test"), String::from("")],
                padding: vec![true, false, true],
            },
            RowIngredient {
                words: vec![
                    String::from("test"),
                    String::from("test"),
                    String::from("test"),
                ],
                padding: vec![false, false, false],
            },
            RowIngredient {
                words: vec![String::from("test"), String::from("test"), String::from("")],
                padding: vec![false, false, true],
            },
        ];

        let result = create_row_ingredients(vec!["test", "test test test", "test test"], 3);

        assert_eq!(result, expected_row_ingredients)
    }

    #[test]
    fn building_rows_with_one_word() {
        let expected_row = vec![String::from("| test | test |")];

        let result = build_rows(
            vec![
                RowIngredient {
                    words: vec![String::from("test")],
                    padding: vec![false],
                },
                RowIngredient {
                    words: vec![String::from("test")],
                    padding: vec![false],
                },
            ],
            1,
            6,
        );

        assert_eq!(result, expected_row);
    }

    #[test]
    fn building_rows_with_multiple_words() {
        let expected_row = vec![
            String::from("|      | test |      |"),
            String::from("| test | test | test |"),
            String::from("|      | test | test |"),
        ];

        let result = build_rows(
            vec![
                RowIngredient {
                    words: vec![String::from(""), String::from("test"), String::from("")],
                    padding: vec![true, false, true],
                },
                RowIngredient {
                    words: vec![
                        String::from("test"),
                        String::from("test"),
                        String::from("test"),
                    ],
                    padding: vec![false, false, false],
                },
                RowIngredient {
                    words: vec![String::from(""), String::from("test"), String::from("test")],
                    padding: vec![true, false, false],
                },
            ],
            3,
            6,
        );

        assert_eq!(result, expected_row);
    }

    #[test]
    fn test_build_rows() {
        let expected = vec![
            "|    |    |    |".to_string(),
            "|    |test|test|".to_string(),
            "|test|test|test|".to_string(),
            "|    |test|    |".to_string(),
            "|    |    |    |".to_string(),
        ];

        let row_ingredients = vec![
            RowIngredient {
                words: vec![
                    "".to_string(),
                    "".to_string(),
                    "test".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
                padding: vec![true, true, false, true, true],
            },
            RowIngredient {
                words: vec![
                    "".to_string(),
                    "test".to_string(),
                    "test".to_string(),
                    "test".to_string(),
                    "".to_string(),
                ],
                padding: vec![true, false, false, false, true],
            },
            RowIngredient {
                words: vec![
                    "".to_string(),
                    "test".to_string(),
                    "test".to_string(),
                    "".to_string(),
                    "".to_string(),
                ],
                padding: vec![true, false, false, true, true],
            },
        ];

        let result = build_rows(row_ingredients, 5, 4);

        assert_eq!(result, expected);
    }

    #[test]
    fn create_ingredient_row_empty() {
        let ingredients: Vec<&str> = vec![];
        let result = create_ingredient_row(3, 4, ingredients);
        assert_eq!(result, "|\n|\n|\n|");
    }

    #[test]
    fn create_ingredient_row_ingredients() {
        let expected = "|      |      |      |\n|      | test | test |\n| test | test | test |\n|      | test |      |\n|      |      |      |".to_string();
        let ingredients = vec!["test", "test test test", "test test"];

        let result = create_ingredient_row(6, 5, ingredients);
        assert_eq!(result, expected);
    }
}
