use cw_storage_plus::Item;
use cosmwasm_std::Addr;

pub const MINTERS_STORAGE: Item<Vec<Addr>> = Item::new("minters");