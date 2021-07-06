#![feature(string_remove_matches)]
use scraper::{Html, Selector};
use serde::{Deserialize, Serialize};
use std::io;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use warp::{filters::BoxedFilter, Filter, Reply};

#[derive(Clone, Serialize, Deserialize)]
struct Country {
    name: String,
}

impl Country {
    fn parse(&self) -> Self {
        if self.name.contains("_") {
            Self {
                name: String::from(&self.name.replace("_", " ")),
            }
        } else {
            Self {
                name: String::from(&self.name),
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let server = warp::serve(countries_filter().await);

    server
        .run(SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            8080,
        ))
        .await;
}

async fn countries_filter() -> BoxedFilter<(impl Reply,)> {
    let parsed_countries = parse_countries(get_countries().await.unwrap());
    warp::path("countries")
        .map(move || warp::reply::json(&parsed_countries))
        .boxed()
}

async fn get_countries() -> Result<Vec<Country>, Box<dyn std::error::Error>> {
    // Pull down list of sovereign states from Wikipedia
    let body = reqwest::get("https://en.wikipedia.org/wiki/List_of_sovereign_states")
        .await?
        .text()
        .await?;

    // Scrape the Wiki page content
    let fragment = Html::parse_fragment(&body);
    let selector = Selector::parse("span").unwrap();

    // Exclude some values
    let mut countries = Vec::<Country>::new();
    let exclusions = vec![
        "List_of_states",
        "Other_states",
        "Criteria_for_inclusion",
        "See_also",
        "Notes",
        "References",
        "Bibliography",
    ];

    for element in fragment.select(&selector) {
        let country = element.value().attr("id");
        match country {
            Some(c) => {
                // If we find an id match, check it against list of exclusions
                if !exclusions.iter().any(|&v| v == c) {
                    countries.push(Country {
                        name: String::from(c),
                    })
                }
            }
            None => {}
        }
    }

    Ok(countries)
}

fn parse_countries(countries: Vec<Country>) -> Vec<Country> {
    let mut countries_parsed = Vec::<Country>::new();

    for country in countries {
        countries_parsed.push(country.parse());
    }

    countries_parsed
}

// CLI verison of the app (swap warp server for this if running CLI-only)
#[allow(dead_code)]
fn start_test(countries: Vec<Country>) -> io::Result<()> {
    for (i, country) in countries.iter().enumerate() {
        let mut input = String::new();

        io::stdin().read_line(&mut input)?;

        if input.trim().to_lowercase() == country.name.to_lowercase() {
            println!("✅ Correct {}/{}", i + 1, countries.len() - 1);
        } else {
            print!("{}[2J", 27 as char);
            println!("❌ {:?} was the correct answer", country.name);
            break;
        }
    }

    Ok(())
}
