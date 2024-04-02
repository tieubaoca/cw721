use cosmwasm_schema::write_api;
use cw721_hero::msg::InstantiateMsg;
use cw721_hero::ExecuteMsg;
use cw721_hero::query::QueryMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}