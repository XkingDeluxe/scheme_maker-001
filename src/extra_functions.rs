pub struct StringFunctions{}

impl StringFunctions{
    pub fn insert_replace(str_base: String, index_start: usize, index_stop: usize, str_insert: String) -> String{
        let mut hold = String::new();
        hold += &str_base[..index_start];
        hold += &str_insert;
        hold += &str_base[index_stop+1..];
        hold
    }
}