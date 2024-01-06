use tokio;
use web_scraping_2::web_scrap::{self};

fn main() {
    let runtime = tokio::runtime::Builder::new_current_thread()
    .enable_all()
    .build();
    
    if let Ok(s) = runtime {
        ..=s.block_on(async{
            let quotes = web_scrap::scrape_quote().await;
            println!("\n");
            let mut index: u16= 0;
            for quote in quotes{
                index+=1;
                println!("{}\n", "-".repeat(20));
                println!("Quote {}:\n{}\n", index, quote);
            }
        });
    }
}
