use std::error::Error;
use colour::{dark_green, yellow};

use news_api::{Articles, get_article};

fn render_articles(articles: &Articles) {
    for a in &articles.articles {
        dark_green!("> {}\n", a.title);
        yellow!("- {}\n\n", a.url)
    }

}

fn main() -> Result<(), Box<dyn Error>>{

    let url: &str = "https://newsapi.org/v2/top-headlines?country=us&apiKey=4cf73e3474bb40da86cbba41fb3730c5";
    let _url_2: &str = "https://newsapi.org/v2/top-headlines?q=trump&apiKey=4cf73e3474bb40da86cbba41fb3730c5";

    let articles = get_article(url)?;

    render_articles(&articles);

    Ok(())
}