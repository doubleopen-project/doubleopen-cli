use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq)]
pub enum Algorithm {
    SHA1,
    SHA224,
    SHA256,
    SHA384,
    SHA512,
    MD2,
    MD4,
    MD5,
    MD6,
}
