extern crate rand;
use rand::Rng;

pub struct Encoder
{
    key_len: i32,
    steps: Vec<i32>,
    encoded_data: String,
    decoded_data: String
}

impl Encoder
{

    pub fn generate_new_key() -> Vec<i32>
    {
        let len = Encoder::get_key_length();
        let mut key: Vec<i32> = Vec::new();

        for i in 0..len
        {
            key.push(rand::thread_rng().gen_range(1..6));
        }

        key
    }

    fn get_key_length() -> i32
    {
        rand::thread_rng().gen_range(20..50)
    }

}
