mod crypto;

fn main()
{
    let key_len = crypto::Encoder::generate_new_key();
    println! ("len: {:?}", key_len);
}
