use collections::HashMap;
use std::hash::sip::SipHasher;

#[deriving(Decodable)]
pub type Hashes = HashMap<String, String, SipHasher>;
