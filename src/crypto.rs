extern crate rand;
use rand::Rng;

pub struct Encoder
{
    pub raw_pass: String,
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
        // This is a temporary key, for testing
        // Eventually this will be replaced with pulling in an i32 vec from the key file
        let key_seq = vec![2,1,3];

        // Eventually, the string will be converted to b64 here
        // for testing though, it'll stay unencoded
        // let b64_pass = base64::encode(&self.raw_pass);

        // First, get the password as a vector of chars
        let mut start_vector: Vec<char> = self.raw_pass.chars().collect();
        let mut end_vector: Vec<char> = Vec::new();

        // Add meaningless characters to the end until it's a multiple of 12
        while start_vector.len() as i32 % 12 != 0 { start_vector.push(';'); }

        println! ("{:?}", start_vector);

        // let start_vec_slice = &start_vector;

        for i in 0..key_seq.len()
        {
            println! ("on item {} of key", i);
            let mut part_one: Vec<char> = Vec::new();
            let mut part_two: Vec<char> = Vec::new();
            let mut iteration = 0;
            {
                let start_vec_slice = &start_vector;
                for j in start_vec_slice.chunks(key_seq[i])
                {
                    if iteration % 2 == 0 { for k in j { part_one.push(*k); } }
                    else if iteration % 2 == 1 
                    {
                        for k in j { part_two.push(*k); }
                        end_vector.extend_from_slice(&part_two);
                        end_vector.extend_from_slice(&part_one);
                        part_one = Vec::new();
                        part_two = Vec::new();
                    }
                    iteration += 1;
                }
            }
            start_vector = end_vector.clone();
            end_vector = Vec::new();
        }
        println! ("{:?}", start_vector);
    }

    pub fn new (in_pass: &str) -> Encoder
    {
        Encoder
        {
            raw_pass: in_pass.to_string(),
            encoded_pass: "".to_string()
        }
    }

    fn get_key_length() -> i32
    {
        rand::thread_rng().gen_range(20..50)
    }

}
