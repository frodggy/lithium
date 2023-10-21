mod key;
mod password;

use password::Password;
use serde::{Serialize, Deserialize};
use serde_encrypt::{traits::SerdeEncryptSharedKey, serialize::impls::BincodeSerializer};
use std::{collections::HashMap, fs};

type Website = String;
type Name = String;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PasswordManager {
    passwords: Vec<(String, HashMap<Website, Password>)>,
    keys: HashMap<Name, Vec<u8>>,
}

impl SerdeEncryptSharedKey for PasswordManager {
    type S = BincodeSerializer<Self>;
}

impl PasswordManager {
    pub fn add_email<S: Into<String>>(&mut self, email: S) {
        self.passwords.push((email.into(), HashMap::new()))
    }

    fn get_key<S: Into<String>>(&self, name: S) -> &Vec<u8> {
        let name: String = name.into();
        match self.keys.get(&name) {
            Some(k) => k,
            None => panic!("no key by the name of \"{}\" exists", name),
        }
    }

    pub fn create_key<S: Into<String>>(&mut self, name: S) {
        let key = key::generate_key();
        self.keys.insert(name.into(), key);
    }

    pub fn add_password<S>(&mut self, email: S, password: S, website: S, key: S)
    where
        S: Into<String> + Clone,
    {
        let self_clone = self.clone();
        let key = self_clone.get_key(key.clone());
        for e in &mut self.passwords {
            if e.0 == email.clone().into() {
                e.1.insert(
                    website.clone().into(),
                    Password::new(password.clone().into(), &key),
                );
            }
        }
    }

    pub fn save(&self) {
        let dest_path = home::home_dir().unwrap().join(".lithium");
        if !dest_path.exists() {
            fs::create_dir_all(dest_path).unwrap();
        }


    }

}

fn main() {
    
}
