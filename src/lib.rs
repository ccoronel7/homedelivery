use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, near_bindgen, AccountId, Balance};
use near_sdk::collections::{ UnorderedMap};
use near_sdk::json_types::{U128};
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
    owner_id: AccountId,
    name: String,
    address: String,
    location: String,
    schedule:String,
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
    id_tienda: AccountId,
    platillos: Vec<PlatilloObject>,
}
#[derive(BorshDeserialize, BorshSerialize, Serialize, Deserialize, Clone)]
#[serde(crate = "near_sdk::serde")]
//structs for Menu
pub struct PlatilloObject {
    id: i128,
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
    menus: Vec<MenuObject>,
    categories: Vec<CategoriesJson>,
    platillo_id: i128,
}

/// Initializing deafult impl
/// We are using default inizialization for the structs
impl Default for Contract {
    fn default() -> Self {
        Self {
            stores: UnorderedMap::new(b"s".to_vec()),
            categories: Vec::new(),
            menus: Vec::new(),
            platillo_id: 0,
        }
    }
}


#[near_bindgen]
impl Contract {
//functions for stores
pub fn set_store(&mut self, 
    owner_id: AccountId,
    name: String,
    address: String,
    location: String,
    schedule: String,
    phone: String,
    wallet: String,
    logo: String,
) -> StoreObject {
    let store = self.stores.get(&env::signer_account_id());
    if store.is_some() {
        env::panic(b"store already exists");
    }
    let data = StoreObject {
        owner_id: owner_id.to_string(),
        name: name,
        address: address,
        location: location,
        schedule:schedule,
        phone: phone,
        wallet: wallet,
        logo: logo,
    };
    self.stores.insert(&env::signer_account_id(), &data);

    let data_menu = MenuObject {
        id_tienda: env::signer_account_id().to_string(),
        platillos: Vec::new(),
    };
    self.menus.push(data_menu);
    env::log(b"store Created");
    data
}

pub fn put_store(&mut self,
    owner_id: AccountId,
    name: String,
    address: String,
    location: String,
    schedule: String,
    phone: String,
    wallet: String,
    logo: String,
) -> StoreObject {
    let return_data = StoreObject {
        owner_id: owner_id.clone(),
        name: name.clone(),
        address: address.clone(),
        location: location.clone(),
        schedule:schedule.clone(),
        phone: phone.clone(),
        wallet: wallet.clone(),
        logo: logo.clone(),
    };
    let mut store = self.stores.get(&env::signer_account_id()).expect("Store does not exist");
    store.owner_id = owner_id;
    store.name = name;
    store.address = address;
    strore.location = location;
    strore.schedule = schedule;
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
        owner_id: store.owner_id,
        name: store.name,
        address: store.address,
        phone: store.phone,
        wallet: store.wallet,
        logo: store.logo,
    }
}

// funtions for get all stores
pub fn get_all_stores(&self) -> Vec<StoreObject> {
    self.stores.iter().map(|(_key, value)| value.clone()).collect()
}

// funtions for get all stores
pub fn get_menu(&self, user_id: AccountId) -> MenuObject {
    let index = self.menus.iter().position(|x| x.id_tienda == user_id.to_string()).expect("Menu no exists");
    self.menus[index].clone()
}
// funtions for platillos
pub fn set_platillo(&mut self,
    name: String,
    description: String,
    category:String,
    price: U128,
    img: String,
) -> MenuObject {
    let index = self.menus.iter().position(|x| x.id_tienda == env::signer_account_id()).expect("Menu no exists");
    self.platillo_id += 1;
    self.menus[index].platillos.push(PlatilloObject {
        id: self.platillo_id,
        name: name,
        description: description,
        category: category,
        price: price.0,
        img: img,
    });
    env::log(b"Menu Created");
    self.menus[index].clone()
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
