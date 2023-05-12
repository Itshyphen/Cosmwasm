use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

pub const NAME: Item<String> = Item::new("name");
pub const SYMBOL: Item<String> = Item::new("symbol");
pub const DECIMAL: Item<u128> = Item::new("decimal");
pub const OWNER: Item<Addr> = Item::new("owner");
pub const TOTALSUPPLY: Item<u128> = Item::new("total_supply");

pub const BALANCES: Map<&Addr,u128> = Map::new("balances");
