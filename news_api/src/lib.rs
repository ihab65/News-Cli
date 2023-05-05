use serde::Deserialize;



#[derive(thiserror::Error, Debug)]
pub enum NewsApiError {
    #[error("Failed fetching articles")]
    RequestFailed,
    #[error("Failed converting to a string")]
    FailedResponseToString,
    #[error("Article parcing failed")]
    ArticleParceFailed,
}

#[derive(Deserialize, Debug)]
pub struct Articles {
    pub articles: Vec<Article>
}

#[derive(Deserialize, Debug)]
pub struct Article {
    pub title: String,
    pub url: String,
}

pub fn get_article(url: &str) -> Result<Articles, NewsApiError> {
    let response = ureq::get(url)
    .call()       .map_err(|_e| NewsApiError::RequestFailed)?
    .into_string().map_err(|_e| NewsApiError::FailedResponseToString)?;


    let articles: Articles = serde_json::from_str(&response)
        .map_err(|_e| NewsApiError::ArticleParceFailed)?;
    Ok(articles)
}
