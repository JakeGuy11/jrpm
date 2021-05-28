extern crate rand;
use rand::Rng;

pub struct Encoder
{
    raw_pass: String,
    pass_site: String,
    encoded_pass: String
}

impl Encoder
{

    pub fn generate_new_key() -> Vec<i32>
    {
        let len = Encoder::get_key_length();
        let mut key: Vec<i32> = Vec::new();

        for _i in 0..len
        {
            key.push(rand::thread_rng().gen_range(1..6));
        }

        key
    }

    pub fn new (password: &str, site: &str) -> Encoder
    {
        Encoder
        {
            raw_pass: password.to_string(),
            pass_site: site.to_string(),
            encoded_pass: "".to_string()
        }
    }

    fn get_key_length() -> i32
    {
        rand::thread_rng().gen_range(20..50)
    }

}
