use tokio;
use web_scraping_2::WebScrap;

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build();
    
    if let Ok(s) = runtime {
        ..=s.block_on(async{
            WebScrap::scrape_quote().await
        });
    }
}
