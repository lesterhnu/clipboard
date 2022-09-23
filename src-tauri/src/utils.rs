use crypto::md5::Md5;
use crypto::digest::Digest;

pub fn md5(s:&str)->String{
    let mut hasher = Md5::new();
    hasher.input_str(s);
    hasher.result_str()
}