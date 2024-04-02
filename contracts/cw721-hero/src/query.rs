use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Empty};
use cw721_base::QueryMsg as Cw721QueryMsg;
use cw721::{OwnerOfResponse, ApprovalResponse, ApprovalsResponse, OperatorResponse, NumTokensResponse, ContractInfoResponse, NftInfoResponse, AllNftInfoResponse, TokensResponse};
use cw721_base::msg::MinterResponse;

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(MintersResponse)]
    Minters {},

    #[returns(OwnerOfResponse)]
    OwnerOf {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(ApprovalResponse)]
    Approval {
        token_id: String,
        spender: String,
        include_expired: Option<bool>,
    },
    #[returns(ApprovalsResponse)]
    Approvals {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(OperatorResponse)]
    AllOperators {
        owner: String,
        include_expired: Option<bool>,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(NumTokensResponse)]
    NumTokens {},
    #[returns(ContractInfoResponse)]
    ContractInfo {},
    #[returns(NftInfoResponse<Empty>)]
    NftInfo {
        token_id: String,
    },
    #[returns(AllNftInfoResponse<Empty>)]
    AllNftInfo {
        token_id: String,
        include_expired: Option<bool>,
    },
    #[returns(TokensResponse)]
    Tokens {
        owner: String,
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(TokensResponse)]
    AllTokens {
        start_after: Option<String>,
        limit: Option<u32>,
    },
    #[returns(MinterResponse)]
    Minter {},
}

impl From<QueryMsg> for Cw721QueryMsg<Empty> {
    fn from(msg: QueryMsg) -> Cw721QueryMsg<Empty> {
        match msg {
            QueryMsg::OwnerOf { token_id, include_expired } => Cw721QueryMsg::OwnerOf { token_id, include_expired },
            QueryMsg::Approval { token_id, spender, include_expired } => Cw721QueryMsg::Approval { token_id, spender, include_expired },
            QueryMsg::Approvals { token_id, include_expired } => Cw721QueryMsg::Approvals { token_id, include_expired },
            QueryMsg::AllOperators { owner, include_expired, start_after, limit } => Cw721QueryMsg::AllOperators { owner, include_expired, start_after, limit },
            QueryMsg::NumTokens {} => Cw721QueryMsg::NumTokens {},
            QueryMsg::ContractInfo {} => Cw721QueryMsg::ContractInfo {},
            QueryMsg::NftInfo { token_id } => Cw721QueryMsg::NftInfo { token_id },
            QueryMsg::AllNftInfo { token_id, include_expired } => Cw721QueryMsg::AllNftInfo { token_id, include_expired },
            QueryMsg::Tokens { owner, start_after, limit } => Cw721QueryMsg::Tokens { owner, start_after, limit },
            QueryMsg::AllTokens { start_after, limit } => Cw721QueryMsg::AllTokens { start_after, limit },
            _ => panic!("Unsupported query variant"),
        }
    
    }
}
#[cw_serde]
pub struct MintersResponse {
    pub minters: Vec<Addr>,
}