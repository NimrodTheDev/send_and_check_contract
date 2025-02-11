use SendAndCheckSeiContract::{ExecuteMsg, InstantiateMsg, QueryMsg, QueryEnum};
use cosmwasm_schema::write_api;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryEnum,
    }
}
