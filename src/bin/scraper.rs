use scraper::{Html, Selector};
use std::collections::HashMap;
use std::fs::File;
use std::{thread, time};

const BASE_URL: &str = "https://minecraft.fandom.com";
const RECIPE_SELECTOR: &str = ".mcui-input";
const RECIPE_ROW_SELECTOR: &str = ".mcui-row";
const ROW_ITEM_SELECTOR: &str = ".invslot";
const NOT_EMPTY_ITEM_SELECTOR: &str = ".invslot-item";
const A_SELECTOR: &str = "a";

fn get_item_paths() -> Vec<String> {
    let item_url = format!("{}/wiki/Item", BASE_URL);
    let response = reqwest::blocking::get(item_url).expect("Error while getting the item page");

    let text = response
        .text()
        .expect("Error while getting the text from the response.");
    let document = Html::parse_document(&text);

    let div_selector = scraper::Selector::parse("div.div-col.columns.column-width").unwrap();
    let ul_selector = scraper::Selector::parse("ul").unwrap();
    let a_selector = scraper::Selector::parse(A_SELECTOR).unwrap();

    let divs = document.select(&div_selector);

    let paths: Vec<String> = divs
        .flat_map(|div| {
            let ul = div.select(&ul_selector).next().unwrap();

            ul.select(&a_selector)
                .filter_map(|a| a.value().attr("href").map(String::from))
        })
        .collect();

    paths
}

fn main() {
    let _planks = [
        "Oak", "Spruce", "Birch", "Jungle", "Acacia", "Dark Oak", "Mangrove", "Cherry",
    ];

    let paths = get_item_paths();

    println!("Total paths: {}", paths.len());

    let mut recipes: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths {
        let url = format!("{}{}", BASE_URL, path);
        println!("Fetching URL: {}", url);

        let response = reqwest::blocking::get(&url)
            .expect(&format!("Error while getting recipe for {}", path));

        let text = response
            .text()
            .expect("Error while getting text from response.");

        let recipe_selector =
            Selector::parse(RECIPE_SELECTOR).expect("Error creating recipe selector.");
        let recipe_row_selector =
            Selector::parse(RECIPE_ROW_SELECTOR).expect("Error creating recipe row selector.");
        let row_item =
            Selector::parse(ROW_ITEM_SELECTOR).expect("Error creating recipe item selector.");
        let not_empty_item = Selector::parse(NOT_EMPTY_ITEM_SELECTOR)
            .expect("Error creating non empty item selector.");
        let a_selector = Selector::parse(A_SELECTOR).expect("Error creating a selector.");

        let document = Html::parse_document(&text);
        let recipe = document.select(&recipe_selector).next();

        if let Some(recipe) = recipe {
            let mut ingredients: Vec<String> = Vec::with_capacity(9);

            // Moet eerst een check doen of er wel een echt recept is en niet als crafting ingredient
            let toc_points = Selector::parse(".toctext").unwrap();

            let mut craftable = false;

            for toc_point in document.select(&toc_points) {
                let toc_text = toc_point.inner_html();

                if toc_text == "Crafting" {
                    craftable = true;
                }
            }

            if craftable {
                for recipe_row in recipe.select(&recipe_row_selector) {
                    for item in recipe_row.select(&row_item) {
                        if let Some(non_empty_item) = item.select(&not_empty_item).next() {
                            if let Some(first_link) = non_empty_item.select(&a_selector).next() {
                                ingredients
                                    .push(first_link.value().attr("title").unwrap().to_string());
                            } else {
                                ingredients.push("#".to_string());
                            }
                        } else {
                            ingredients.push("#".to_string());
                        }
                    }
                }
            } else {
                for _ in 0..9 {
                    ingredients.push("#".to_string());
                }
            }

            recipes.insert(
                path.split('/')
                    .last()
                    .expect("Error getting the recipe name.")
                    .to_string()
                    .replace('_', " ")
                    .to_lowercase(),
                ingredients,
            );

            let wait_time = time::Duration::from_secs(3);
            thread::sleep(wait_time);
        }
    }

    let file = File::create("data/recipes.json").expect("Error opening recipes file.");

    serde_json::to_writer(&file, &recipes).expect("Error writing recipes to json.");
}
