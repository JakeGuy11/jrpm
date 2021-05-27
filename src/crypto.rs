extern crate rand;
use rand::Rng;

pub struct Crypto
{
    key_len: i32,
    steps: Vec<i32>,
    encoded_data: String,
    decoded_data: String
}

impl Crypto
{

    pub fn generate_new_key()
    {
        //TODO: generate a key file
    }

    pub fn get_key_length() -> i32
    {
        rand::thread_rng().gen_range(20..50)
    }

}
