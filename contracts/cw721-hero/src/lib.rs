use cosmwasm_schema::cw_serde;
use cosmwasm_std::Empty;
use cw2::set_contract_version;
pub use cw721_base::{ContractError, InstantiateMsg as Cw721BaseInstantiateMsg, state::TokenInfo};

pub mod msg;
pub mod state;
pub mod query;
pub mod execute;
use query::{QueryMsg, MintersResponse};
// Version info for migration
const CONTRACT_NAME: &str = "crates.io:cw721-one-chain";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cw_serde]
pub struct Trait {
    pub display_type: Option<String>,
    pub trait_type: String,
    pub value: String,
}

// see: https://docs.opensea.io/docs/metadata-standards
#[cw_serde]
#[derive(Default)]
pub struct Metadata {
    pub image: Option<String>,
    pub image_data: Option<String>,
    pub external_url: Option<String>,
    pub description: Option<String>,
    pub name: Option<String>,
    pub attributes: Option<Vec<Trait>>,
    pub background_color: Option<String>,
    pub animation_url: Option<String>,
    pub youtube_url: Option<String>,
}

pub type Extension = Option<Metadata>;

pub type Cw721OneChainContract<'a> = cw721_base::Cw721Contract<'a, Extension, Empty, Empty, Empty>;
pub type ExecuteMsg = cw721_base::ExecuteMsg<Extension, Empty>;

#[cfg(not(feature = "library"))]
pub mod entry {
    use super::*;
    use msg::InstantiateMsg;
    use state::MINTERS_STORAGE;
    use execute::mint;
    use cosmwasm_std::{entry_point, to_json_binary, Addr};
    use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

    // This makes a conscious choice on the various generics used by the contract
    #[entry_point]
    pub fn instantiate(
        mut deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: InstantiateMsg,
    ) -> Result<Response, ContractError> {
        let cw721_base_instantiate_msg = Cw721BaseInstantiateMsg {
            name: msg.name,
            symbol: msg.symbol,
            minter: info.sender.clone().into(),
        };
        let res = Cw721OneChainContract::default().instantiate(deps.branch(), env, info, cw721_base_instantiate_msg)?;
        let minters: StdResult<Vec<Addr>> = msg
        .minters
        .into_iter()
        .map(|addr| deps.api.addr_validate(&addr))
        .collect();
        MINTERS_STORAGE.save(deps.storage, &minters?)?;
        set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
        Ok(res)
    }

    #[entry_point]
    pub fn execute(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        msg: ExecuteMsg,
    ) -> Result<Response, ContractError> {
        match msg {
            ExecuteMsg::Mint { owner, extension, token_id, token_uri } =>{
                mint(
                    deps,
                    info,
                    token_id,
                    owner,
                    token_uri,
                    extension
                )
            }
            _ => Cw721OneChainContract::default().execute(deps, env, info, msg)
        }
    }




    #[entry_point]
    pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
        match msg {
            QueryMsg::Minters {  } => {
                let minters = MINTERS_STORAGE.load(deps.storage)?;
                to_json_binary(&MintersResponse {minters: minters })
            }
            _ => Cw721OneChainContract::default().query(deps, env, msg.into())
        }
    }
}
