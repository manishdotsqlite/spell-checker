use bloomfilter::Bloom_Filter;

mod bloomfilter;

#[tokio::main]
async fn main() {
    let mut filter = Bloom_Filter::new();
    let _ = match filter.load_filter() {
        Ok(_) => (),
        Err(_) => println!("Bloom filter couldn't be loaded.")
    };    
    let _ = filter.check_file("test-file.txt").await;
    let _ = filter.save_filter();
}