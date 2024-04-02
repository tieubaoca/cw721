
use cosmwasm_std::{DepsMut, MessageInfo, Response, StdError};
use cw721_base::{ContractError, state::TokenInfo};
use super::state::MINTERS_STORAGE;
use super::{Cw721OneChainContract, Extension};


pub fn mint(
    deps: DepsMut,
    info: MessageInfo,
    token_id: String,
    owner: String,
    token_uri: Option<String>,
    extension: Extension,
) -> Result<Response, ContractError> {

    let minters = MINTERS_STORAGE.load(deps.storage)?;
    if minters.iter().find(|minter| minter == &info.sender).is_none() {
        return Err(ContractError::Std(StdError::GenericErr { msg: "Unauthorized".to_string() }))
    }
    // create the token
    let token = TokenInfo {
        owner: deps.api.addr_validate(&owner)?,
        approvals: vec![],
        token_uri,
        extension,
    };
    Cw721OneChainContract::default().tokens
        .update(deps.storage, &token_id, |old| match old {
            Some(_) => Err(ContractError::Claimed {}),
            None => Ok(token),
        })?;

        Cw721OneChainContract::default().increment_tokens(deps.storage)?;

    Ok(Response::new()
        .add_attribute("action", "mint")
        .add_attribute("minter", info.sender)
        .add_attribute("owner", owner)
        .add_attribute("token_id", token_id))
}