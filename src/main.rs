use tokio;
use web_scraping_2::web_scrap::{self, Quote};

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build();
    
    if let Ok(s) = runtime {
        ..=s.block_on(async{
            let quotes = web_scrap::scrape_quote().await;
            for quote in quotes{
                println!("{}\n", quote);
            }
        });
    }
}
