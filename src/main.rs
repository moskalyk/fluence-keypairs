use fluence_keypair::key_pair::{KeyPair, KeyFormat};
use bs58;

fn key_maker() {
    let keys = KeyPair::generate_ed25519();
    println!("peer id                 : {}", keys.get_peer_id());
    let kp_str = bs58::encode(&keys.to_vec()).into_string();
    println!("keypair as base58 string: {}", kp_str);
}

fn main() {
    key_maker();
}