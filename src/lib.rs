
pub mod web_scrap{
    use reqwest;
    use select::{self, predicate::Class};
    
    pub async fn get_website_body(url: &str) -> Result<String, reqwest::Error>{
        let response = reqwest::get(url).await?;
        let body = response.bytes().await?;

        let body = String::from_utf8(body.to_vec()).expect("error converting body");
        Ok(body)
    }
    pub async fn scrape_quote() -> Vec<Quote>{
        let website_url_base: &str = "https://quotes.toscrape.com/";
        let mut website_url: String = website_url_base.to_string();
        let mut quote_list: Vec<Quote> = Vec::new();
        println!("Loading quotes page 1");
        loop {
            let website_body = get_website_body(website_url.as_str()).await.expect("Error getting website body");

            // next button class = next

            let document = select::document::Document::from(website_body.as_str());
            let next_button: Vec<_> = document.find(Class("next")).collect();

            let mut new_link: Option<String> = None;

            // get new button link
            if let Some(next_btn) = next_button.first() {
                let next_button_child: Vec<_> = next_btn.children().collect();
                let mut button = None;
                for child in next_button_child.iter(){
                    if let Some(name) = child.name()  {
                        if name=="a"{
                            button = Some(child);
                        }
                    }
                }
                if let Some(btn) =  button{
                    let href = btn.attr("href").unwrap();
                    new_link = Some(href.to_string());
                    let splitted_href: Vec<_> = href.split("/").collect();
                    println!("Loading quotes page {}", splitted_href[2]);
                }
            }


            let quotes: Vec<_> = document.find(select::predicate::Class("quote")).collect();
            
            

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
            if let Some(url) = new_link{
                website_url = format!("{}{}",website_url_base,url);
            }else{
                break;
            }
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