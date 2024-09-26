use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult,
};
use neutron_sdk::bindings::msg::NeutronMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// -------------------------- REPLY_IDS  -------------------------------------

/// Used to identify a reply to the .
pub const INCREASE_COUNT_REPLY_ID: u64 = 0;

/// ------------------------ INSTANTIATION ------------------------------------

/// Any data that is necessary to set up your new contract should be added
/// here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    _deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    Ok(Response::new()
        // We add some attributes to the response with information about the current call.
        // It's useful for debugging.
        .add_attribute("action", "instantiate")
        .add_attribute("contract_address", env.contract.address)
        .add_attribute("sender", info.sender.to_string()))
}

/// ---------------------------- EXECUTION ------------------------------------

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    ///
    Test {},
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    /// Keep access to the StdError, just in case.
    #[error(transparent)]
    Std(#[from] StdError),
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env, // We don't use Env in uor implementation, hence the underscore
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    // The `match` here tries to parse msg into any of the known variants of ExecuteMsg.
    // If it's not possible, the user will get an error.
    match msg {
        ExecuteMsg::Test {} => execute_test(deps, info),
    }
}

pub fn execute_test(
    _deps: DepsMut,
    _info: MessageInfo,
) -> Result<Response<NeutronMsg>, ContractError> {
    Ok(Response::default().add_attribute("action", "execute_add"))
}

/// ----------------------------- QUERIES ------------------------------------

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_json_binary("").unwrap())
}
