
pub mod web_scrap{
    use reqwest;
    use select;
    const WEBSITE_URL_BASE: &str = "https://quotes.toscrape.com/";

    pub async fn get_website_body() -> Result<String, reqwest::Error>{
        let response = reqwest::get(WEBSITE_URL_BASE).await?;
        let body = response.bytes().await?;

        let body = String::from_utf8(body.to_vec()).expect("error converting body");
        Ok(body)
    }
    pub async fn scrape_quote() -> Vec<Quote>{
        let website_body = get_website_body().await.expect("Error getting website body");
        let document = select::document::Document::from(website_body.as_str());
        let quotes: Vec<_> = document.find(select::predicate::Class("quote")).collect();
        
        let mut quote_list: Vec<Quote> = Vec::new();

        for quote in quotes{
            let content: Vec<_> = quote.find(select::predicate::Class("text")).collect();
            let content = content.first();
            
            let mut content_string: String = String::new();
            if let Some(ref _content) = content {
                content_string = _content.text();
            }

            let author: Vec<_> = quote.find(select::predicate::Class("author")).collect();
            let author = author.first();

            let mut author_string: String = String::new();
            if let Some(_author) = author {
                author_string = _author.text();
            }

            quote_list.push(
                Quote { content: content_string.trim().to_string(), author: author_string.trim().to_string() }
            );
        }
        return quote_list;
    }

    pub struct Quote{
        content: String,
        author: String
    }

    impl std::fmt::Display for Quote {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f,"Content: {}\nAuthor: {}", self.content, self.author)
        }
    }
}