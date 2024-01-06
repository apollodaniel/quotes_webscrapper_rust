use reqwest;
use select;

pub mod WebScrap{
    use reqwest::{Error, Response};

    const WEBSITE_URL_BASE: &str = "https://quotes.toscrape.com/";

    pub async fn get_website_body() -> Result<String, Error>{
        let response = reqwest::get(WEBSITE_URL_BASE).await?;
        let body = response.bytes().await?;
        Ok(String::from_utf8(body.to_vec()).unwrap())
    }
    pub async fn scrape_quote() {
        let website_body = get_website_body().await.expect("Error getting website body");
        println!("{}",website_body);
        //let document = select::document::Document::from("");
    }

    struct Quote<'a>{
        content: &'a str,
        author: &'a str
    }
}