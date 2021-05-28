extern crate rand;
use rand::Rng;

pub struct Encoder
{
    raw_pass: String,
    pub encoded_pass: String
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

    pub fn generate_encryption(&self)
    {

    }

    pub fn new (password: &String) -> Encoder
    {
        Encoder
        {
            raw_pass: password.to_string(),
            encoded_pass: "".to_string()
        }
    }

    fn get_key_length() -> i32
    {
        rand::thread_rng().gen_range(20..50)
    }

}
