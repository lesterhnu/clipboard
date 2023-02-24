use md5::{Md5, Digest};

pub fn md5(s:&str)->String{
    let mut hasher = Md5::new();
    hasher.update(s);
    let result = format!("{:x}", hasher.finalize());
    return result;
}