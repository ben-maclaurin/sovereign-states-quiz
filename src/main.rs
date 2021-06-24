#![feature(string_remove_matches)]
use scraper::{Html, Selector};
use std::io;

type Country = String;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    return match get_countries().await {
        Ok(v) => {
            start_test(parse_countries(v));
            Ok(())
        },
        Err(_) => Err("bad request")?
    }
}

async fn get_countries<'a>() -> Result<Vec<Country>, Box<dyn std::error::Error>> {

    // Pull down list of sovereign states from Wikipedia
    let body = reqwest::get("https://en.wikipedia.org/wiki/List_of_sovereign_states")
        .await?
        .text()
        .await?;

    // Scrape the Wiki page content
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("span").unwrap();

    // Exclude some values
    let mut countries = Vec::new();
    let exclusions = vec!["List_of_states", "Other_states", "Criteria_for_inclusion", "See_also", "Notes", "References", "Bibliography"];

    for element in fragment.select(&selector) {
        let country = element.value().attr("id");
        match country {
            Some(c) => {
                // If we find an id match, check it against list of exclusions
                if !exclusions.iter().any(|&v| v == c) {
                    countries.push(String::from(c))
                }
            },
            None => {},
        }
    }

    Ok(countries)
}

fn start_test(countries: Vec<Country>) -> io::Result<()> {
    for (i, country) in countries.iter().enumerate() {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == country.to_lowercase() {
            println!("✅ Correct {}/{}", i + 1, countries.len() - 1);
        } else {
            print!("{}[2J", 27 as char);
            println!("❌ {:?} was the correct answer", country);
            break;
        }
    }

    Ok(())
}

fn parse_countries(countries: Vec<Country>) -> Vec<Country> {
    let mut countries_parsed = Vec::new();

    for country in countries {
        if country.contains("_") {
            countries_parsed.push(country.replace("_", " "));
        } else {
            countries_parsed.push(String::from(country));
        }
    }

    countries_parsed
}

