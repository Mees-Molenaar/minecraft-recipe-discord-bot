use std::collections::HashMap;
use std::fs::File;
use std::io::Seek;
use std::{thread, time};

fn get_item_paths() -> Vec<String>{
    let response = reqwest::blocking::get(
        "https://minecraft.fandom.com/wiki/Item",
    )
    .unwrap()
    .text()
    .unwrap();

    let document = scraper::Html::parse_document(&response);

    let div_selector = scraper::Selector::parse("div.div-col.columns.column-width").unwrap();
    let ul_selector = scraper::Selector::parse("ul").unwrap();
    let a_selector = scraper::Selector::parse("a").unwrap();

    let divs = document.select(&div_selector);

    let mut paths = Vec::new();

    for div in divs {
        let ul = div.select(&ul_selector).next().unwrap();

        for a in ul.select(&a_selector) {
            let url = a.value().attr("href");

            if url.is_some() {
                paths.push(url.unwrap().to_string());
            }
        }
    }

    return paths

}


fn main() {
    let planks = [
    "Oak",
    "Spruce",
    "Birch",
    "Jungle",
    "Acacia",
    "Dark Oak",
    "Mangrove",
    "Cherry"
];
    
    let  base_url = "https://minecraft.fandom.com";

    let paths = get_item_paths();

    println!("{}", paths.len());

    let mut recipes: HashMap<String, Vec<String>> = HashMap::new();

    for path in paths {

        let url = format!("{base_url}{path}");
        println!("{}", url);

        let response = reqwest::blocking::get(
            url,
        )
        .unwrap()
        .text()
        .unwrap();

        let recipe_selector = scraper::Selector::parse(".mcui-input").unwrap();
        let recipe_row_selector = scraper::Selector::parse(".mcui-row").unwrap();
        let row_item = scraper::Selector::parse(".invslot").unwrap();
        let not_empty_item = scraper::Selector::parse(".invslot-item").unwrap();
        let a_selector = scraper::Selector::parse("a").unwrap();
        
        let document = scraper::Html::parse_document(&response);
        let recipe = document.select(&recipe_selector).next();

        let mut ingredients: Vec<String> = Vec::with_capacity(9);

        // Moet eerst een check doen of er wel een echt recept is en niet als crafting ingredient
        let toc_points = scraper::Selector::parse(".toctext").unwrap();

        let mut craftable = false;

        for toc_point in document.select(&toc_points) {
            let toc_text = toc_point.inner_html();

            if toc_text == "Crafting" {
                craftable = true;
            }
        }

        if recipe.is_some() && craftable {

        for recipe_row in recipe.unwrap().select(&recipe_row_selector) {
            for item in recipe_row.select(&row_item) {
                let non_empty_item = item.select(&not_empty_item).next();

               if non_empty_item.is_some() {
                let first_link = non_empty_item.unwrap().select(&a_selector).next();

                if first_link.is_some() {
                    ingredients.push(first_link.unwrap().value().attr("title").unwrap().to_string());

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

        recipes.insert(path, ingredients);

        let two_seconds = time::Duration::from_secs(3);
        thread::sleep(two_seconds);
        
    }

    let mut file = File::options()
        .read(true)
        .write(true)
        .open("data/recipes.json")
        .unwrap();

    let _ = file.seek(std::io::SeekFrom::Start(0)).unwrap();

    serde_json::to_writer(file, &recipes).unwrap();

}