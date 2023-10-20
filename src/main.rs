use std::collections::HashMap;
use std::collections::HashSet;
use std::fs;

use fluence_keypair::key_pair::{KeyPair, KeyFormat};
use bs58;

extern crate serde;
extern crate serde_json;
extern crate serde_derive;

#[derive(Debug, serde::Deserialize)]
struct Library {
    data: HashMap<String, String>,
}

fn key_maker() {
    
    let talis_1 = "☯"; // <- Change Me from Talis.
    let talis_2 = "🜄"; // <- Change Me from Talis.

    let mut matches = 0;
    let mut keys = KeyPair::generate_ed25519();
    let mut values = Vec::new();

    while matches < 2 {
        // Perform some operations here...
        keys = KeyPair::generate_ed25519();
        let peer_id = keys.get_peer_id().to_string();

        let mut result: [u32; 4] = [0; 4];

        for i in 0..4 {
            let chunk = &peer_id[i * 13..(i + 1) * 13];
            result[i] = chunk.chars().map(|c| c as u32).sum::<u32>() % 255;
        }

        let content = fs::read_to_string("talis.json")
            .expect("Could not read the file");

        // Parse the JSON content into the Rust data structure
        let library: Library = serde_json::from_str(&content)
            .expect("Error parsing the JSON");

        values = Vec::new();

        for i in 0..4 {
            let key = result[i].to_string();
            if let Some(value) = library.data.get(&key) {
                values.push(value.clone());
            }
        }

        println!("{:?}", values);
        
        let mut counted_matches = HashSet::new();
        let mut unique_match_count = 0;
        for value in values.iter() {
            if (value == talis_1 || value == talis_2) && !counted_matches.contains(&value) {
                counted_matches.insert(value);
                unique_match_count += 1;
            }
        }

        matches = unique_match_count
    }

    println!("Exited the loop because matches = {}", matches);

    let kp_str = bs58::encode(&keys.to_vec()).into_string();
    println!("keypair as base58 string: {}", kp_str);

    // you've successfully found
    println!("you've successfully found: {:?}", values);
}

fn main() {
    key_maker();
}