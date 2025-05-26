use std::{fs::{self, File}, io::{BufRead, BufReader}};
use sqlx::{sqlite};

/// MAXIMUM BIT SIZE OF BIT MAP
const BIT_SIZE: usize = 200000;


/// Bloom Filter Struct that maps 3 bits off of a hashed word. 
pub struct Bloom_Filter {
        pub filename: String,
        pub bit_map: [u8; BIT_SIZE],
}


impl Bloom_Filter {
        pub fn new() -> Self {
                Bloom_Filter {
                        filename: "filter.txt".to_owned(),
                        bit_map: std::array::from_fn(|_| '0' as u8),
                }
        }

        pub fn load_filter(&mut self) -> Result<(), &'static str> {
                let filter_string = match fs::read_to_string(&self.filename) {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't read filter file.")
                };
                let bit_array = filter_string.as_bytes();
                let length = bit_array.len();
                if length <= 0 {
                        return Err("Filter file is empty.");
                }

                for i in 0..=length-1 {
                        self.bit_map[i] = bit_array[i];
                }
                Ok(())
        }

        fn hash_word(word: &str) -> i32 {
                let base: i32 = 31;
                let mod_val: i32 = 200000;

                word.bytes().fold(0, |hash, byte| (hash * base + byte as i32) % mod_val)

        }

        fn check_bloom_for_spelling(&self, word: &str) -> bool {

                let hash_position: i32 = Self::hash_word(word);
                let test_1 = self.bit_map[hash_position as usize] == '1' as u8;

                test_1

        }


        async fn check_db_for_spelling(word: &str) -> Result<bool, &'static str> {

                let sql = format!("SELECT EXISTS (SELECT 1 FROM words WHERE word = '{}')", word);
                let opt = sqlite::SqliteConnectOptions::new().filename("Dictionary.db");
                let conn = match sqlite::SqlitePool::connect_with(opt).await {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't connect to the dictionary.")
                };

                let check: bool =  match sqlx::query_scalar(&sql).fetch_one(&conn).await {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't check word in the dictionary.")
                };
                Ok(check)
        }

        pub async fn populate_bloom_filter_with_words(&mut self) -> Result<(), &'static str> {
                let sql = "SELECT word FROM words";
                let opt = sqlite::SqliteConnectOptions::new().filename("Dictionary.db");
                let conn = match sqlite::SqlitePool::connect_with(opt).await {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't connect to the dictionary.")
                };

                let rows: Vec<String> = match sqlx::query_scalar(sql).fetch_all(&conn).await {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't fetch words from the dictionary.")
                };

                for row in rows {
                        Self::populate_bloom_filter(self, &row);
                }

                Ok(())
        }

        fn populate_bloom_filter(&mut self, word: &str){

                let hash_position: i32 = Self::hash_word(word);
                self.bit_map[hash_position as usize] = '1' as u8;

        }

        pub fn print_bit_map(&self) {
                let bitmap_vec = self.bit_map.to_vec();
                match String::from_utf8(bitmap_vec) {
                        Ok(s) => println!("Bitmap: {}", s),
                        Err(e) => println!("Failed to convert bitmap to string: {:?}", e),
                }
        }


        pub async fn check_file_bf(&mut self, filename: &str) -> Result<(), &'static str> {
                let file = match File::open(filename) {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't read file.")
                };

                let reader = BufReader::new(file);

                for (index, line) in reader.lines().enumerate() {
                        match line {
                                Ok(line) => {
                                        let words = line.split_whitespace().map(|word| word.chars().filter(|c| c.is_alphanumeric()).collect::<String>()).collect::<Vec<String>>();
                                        for word in words {
                                                let word = word.to_lowercase();
                                                let is_in_bloom = self.check_bloom_for_spelling(&word);
                                                if !is_in_bloom {
                                                        println!("Spelling error: '{}' in line {}", word, index + 1);
                                                }

                                        }
                                },
                                Err(_) => ()
                        }
                }
                Ok(())
        }

        pub async fn check_file_db(&self, filename: &str) -> Result<(), &'static str> {
                let file = match File::open(filename) {
                        Ok(s) => s,
                        Err(_) => return Err("Couldn't read file.")
                };

                let reader = BufReader::new(file);

                for (index, line) in reader.lines().enumerate() {
                        match line {
                                Ok(line) => {
                                        let words = line.split_whitespace().map(|word| word.chars().filter(|c| c.is_alphanumeric()).collect::<String>()).collect::<Vec<String>>();
                                        for word in words {
                                                let word = word.to_lowercase();
                                                match Self::check_db_for_spelling(&word).await {
                                                       Ok(is_in_db) => {
                                                                if !is_in_db {
                                                                        println!("Spelling error: '{}' in line {}", word, index + 1);
                                                                }
                                                        },
                                                        Err(_) => ()
                                                };
                                        }
                                },
                                Err(_) => ()
                        }
                }
                Ok(())
        }


        pub fn save_filter(&self) {
                println!("Saving filter to file: {}", self.filename);
                let bitmap_string = self.bit_map.to_vec();
                let bitmap = match String::from_utf8(bitmap_string) {
                        Ok(s) => s,
                        Err(e) => {
                                println!("Failed to convert bitmap to string: {:?}", e);
                                return;
                        }
                };

                match fs::write(&self.filename, &bitmap) {
                        Ok(_) => (),
                        Err(_) => return
                }
        }
        

}