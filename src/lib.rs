use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};
use near_sdk::collections::{ UnorderedMap};
//use near_sdk::json_types::{U128};
use serde::Serialize;
use serde::Deserialize;
use std::collections::HashMap;
// use near_sdk::json_types::ValidAccountId;
//use near_sdk::env::is_valid_account_id;


near_sdk::setup_alloc!();

#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//structs for Stores
pub struct StoreObject {
    name: String,
    address: String,
    phone: String,
    wallet: String,
    logo: String,
}
#[derive(Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct StoreJson {
    user_id: AccountId,
    name: String,
    address: String,
    phone: String,
    wallet: String,
    logo: String,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//structs for Menu
pub struct MenuObject {
    id: i128,
    user_id:AccountId,
    name: String,
    description: String,
    category:String,
    price: Balance,
    img: String,

}
//structs for categories
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesObject {
	name: String,
}
#[derive(Serialize, Deserialize, BorshDeserialize, BorshSerialize, Clone)]
#[serde(crate = "near_sdk::serde")]
pub struct CategoriesJson {
    id: i128,
	name: String,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    stores: UnorderedMap<AccountId, StoreObject>,
    menus: UnorderedMap<AccountId, MenuObject>,
    categories: Vec<CategoriesJson>,
    menu_id: i128,
}

/// Initializing deafult impl
/// We are using default inizialization for the structs
impl Default for Contract {
    fn default() -> Self {
        Self {
            stores: UnorderedMap::new(b"s".to_vec()),
            categories: Vec::new(),
            menus: UnorderedMap:: new(b"s".to_vec()),
            menu_id: 0,
        }
    }
}


#[near_bindgen]
impl Contract {
//functions for stores
pub fn set_store(&mut self, 
    name: String,
    address: String,
    phone: String,
    wallet: String,
    logo: String,
) -> StoreObject {
    let store = self.stores.get(&env::signer_account_id());
    if store.is_some() {
        env::panic(b"store already exists");
    }
    let data = StoreObject {
        name: name,
        address: address,
        phone: phone,
        wallet: wallet,
        logo: logo,
    };
    self.stores.insert(&env::signer_account_id(), &data);
    env::log(b"store Created");
    data
}

pub fn put_store(&mut self,
    name: String,
    address: String,
    phone: String,
    wallet: String,
    logo: String,
) -> StoreObject {
    let return_data = StoreObject {
        name: name.clone(),
        address: address.clone(),
        phone: phone.clone(),
        wallet: wallet.clone(),
        logo: logo.clone(),
    };
    let mut store = self.stores.get(&env::signer_account_id()).expect("Store does not exist");
    store.name = name;
    store.address = address;
    store.phone = phone;
    store.wallet = wallet;
    store.logo = logo;
    self.stores.insert(&env::signer_account_id(), &store);
    env::log(b"store Update");
    return_data
}
pub fn get_store(&self, user_id: AccountId) -> StoreObject {
    let store = self.stores.get(&user_id).expect("Store does not exist");
    StoreObject {
        name: store.name,
        address: store.address,
        phone: store.phone,
        wallet: store.wallet,
        logo: store.logo,
    }
}

// funtions for get all stores
pub fn get_all_stores() -> StoreObject {
    let store = self.stores.iter().map(|(_key, value)| value.clone()).collect();
}

// funtions for menus
pub fn set_menu(&mut self,
    user_id:AccountId,
    name: String,
    description: String,
    category: String,
    price: Balance,
    img: String,
) -> MenuObject {
    self.menu_id += 1;
    let data = MenuObject {
        id: self.menu_id,
        user_id: user_id.to_string(),
        name: name.to_string(),
        description: description.to_string(),
        category: category.to_string(),
        price: price,
        img: img.to_string(),
    };
    self.menus.insert(&env::signer_account_id(), &data);
    env::log(b"Menu Created");
    data
}

// functions for categories
pub fn set_category(&mut self, name: String) -> CategoriesJson {      
    let category_id: i128 = (self.categories.len() + 1) as i128;
    let data = CategoriesJson {
        id: category_id,
        name: name.to_string(),
    };
    self.categories.push(data.clone());
    env::log(b"category Created");
    data
}
pub fn put_category(&mut self, category_id: i128, name: String) -> CategoriesJson {
    let index = self.categories.iter().position(|x| x.id == category_id).expect("Category does not exist");
    self.categories[index].name = name.to_string();
    env::log(b"Category Update");
    CategoriesJson {
        id: category_id,
        name: name.to_string(),
    }
}
pub fn get_category(&self, category_id: Option<i128>) -> Vec<CategoriesJson> {
    let mut categories = self.categories.clone();
    if category_id.is_some() {
        categories = self.categories.iter().filter(|x| x.id == category_id.unwrap()).map(|x| CategoriesJson {
            id: x.id,
            name: x.name.to_string(),
        }).collect();
    }
    categories
}
}
