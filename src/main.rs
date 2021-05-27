mod crypto;

fn main()
{
    let key_len = crypto::Crypto::get_key_length();
    println! ("len: {}", key_len);
}
