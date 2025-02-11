use cosmwasm_std::Addr;
use cw_storage_plus::Item;

pub const TOFROMAMOUNT: Item<(Addr, Addr, u128)> = Item::new("NEEDEDINFO");
pub const DEMON: Item<String> = Item::new("DEMON");
pub const SENDER: Item<String> = Item::new("SENDER");