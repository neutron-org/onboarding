use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{
    entry_point, to_json_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError,
    StdResult, Uint128,
};
use cw_storage_plus::Item;
use neutron_sdk::bindings::msg::NeutronMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// -------------------------- STORAGE ----------------------------------------

/// Do you want your contract to have persistent state? If you do, you need
/// storage items. This particular item stores a (potentially) very large
/// integer number: Uint128, but you can store any value that can be serialized
/// and deserialized, including the types that you defined yourself.
pub const COUNTER: Item<Uint128> = Item::new("counter");

/// ------------------------ INSTANTIATION ------------------------------------

/// Any data that is necessary to set up your new contract should be added
/// here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    /// In our exciting minimal contract, we set the initial value for a counter.
    initial_value: Uint128,
}

/// instantiate() gets executed when you instantiate your contract. It's one
/// of the 3 most important entry points in CosmWasm: instantiate(),
/// execute(), and query().
///
/// Any instantiate() entrypoint expects 4 arguments:
///     - deps: most importantly, gives you access to the storage
///     - env:  keeps information about the execution environment,
///             e.g., the address of the current contract
///     - info: keeps information about the message that is currently
///             executed, e.g., the address of the message sender
///     - msg:  the InstantiateMsg that you define yourself.
///
/// InstantiateMsg is defined below.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
    // The return type of this entrypoint is Result<Response<NeutronMsg>, ContractError>.
    // Without going too much into the details, it means that this entrypoint can either
    // return a valid Response, or a ContractError. The ContractError type is defined below.
) -> Result<Response<NeutronMsg>, ContractError> {
    // Here we save the initial value from the InstantiateMsg to the COUNTER
    // storage item. Saving data to storage consumes gas!
    //
    // This operation can return an error, which will be automatically returned to the user
    // because of the `?` operator.
    COUNTER.save(deps.storage, &msg.initial_value)?;

    Ok(Response::new()
        // We add some attributes to the response with information about the current call.
        // It's useful for debugging.
        .add_attribute("action", "instantiate")
        .add_attribute("initial_value", msg.initial_value)
        .add_attribute("contract_address", env.contract.address)
        .add_attribute("sender", info.sender.to_string()))
}

/// ---------------------------- EXECUTION ------------------------------------

/// ExecuteMsg is the enum that defines the messages that the user can
/// send to the contract.
///
/// You only need messages for actions that require
/// state modification; for read-only actions, you have the query() entry
/// point (see below)
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// This message allows the user to specify a Uint128 amount,
    /// to be added to the COUNTER storage item value.
    IncreaseCount { amount: Uint128 },
}

/// If this was really a **minimal** example, we would not define our own error type,
/// and would simply return Err(StdError::generic_err("error message")) in case of an
/// error.
///
/// However, in most cases you want to define contract-specific errors, which we do below.
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    /// Keep access to the StdError, just in case.
    #[error(transparent)]
    Std(#[from] StdError),

    /// We will return this error if the user tries to increment the counter by
    /// more than 100. For no particular reason.
    #[error("Can not increment by more than 100 (got {amount})")]
    InvalidIncreaseAmount { amount: Uint128 },
}

/// Don't use magic numbers in your code! Move them to constants instead.
pub const MAX_INCREASE_AMOUNT: Uint128 = Uint128::new(100u128);

/// This is the execute() entrypoint. Users that want to perform **actions**
/// that modify the contract state (as opposed to running queries, which do not
/// modify state) need to send one of the variants of the ExecuteMsg to this
/// entry point. The ExecuteMsg is en enum that is defined below.
///
/// This entrypoint expects the same arguments that instantiate() does, but instead
/// of InstantiateMsg, it needs the ExecuteMsg.
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
        ExecuteMsg::IncreaseCount { amount } => execute_increase_amount(deps, info, amount),
    }
}

pub fn execute_increase_amount(
    deps: DepsMut,
    info: MessageInfo,
    amount: Uint128,
) -> Result<Response<NeutronMsg>, ContractError> {
    // Return the InvalidIncreaseAmount error if the user tries to increase
    // by more than 100. We save this value in a well-named constant
    // MAX_INCREASE_AMOUNT because we are nice people.
    if amount.gt(&MAX_INCREASE_AMOUNT) {
        return Err(ContractError::InvalidIncreaseAmount { amount });
    }

    // We need to increase the counter. Step 1: load the current value.
    // This operation consumes gas!
    let mut counter = COUNTER.load(deps.storage)?;

    // Step 2: add the user value to the value loaded from the storage.
    counter += amount;

    // Step 3: save the increased amount to the storage.
    COUNTER.save(deps.storage, &counter)?;

    Ok(Response::default()
        .add_attribute("action", "execute_add")
        .add_attribute("amount", amount.to_string())
        .add_attribute("sender", info.sender))
}

/// ----------------------------- QUERIES ------------------------------------

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// A query message to get the current value of COUNTER. The `#[returns(Uint128)]`
    /// derive marco here is required to generate proper JSON schemas for our smart
    /// contract.
    #[returns(Uint128)]
    CurrentValue {},
}

/// We could simply read the Uint128 value from storage and return it as is,
/// but in general it's better to provide a custom response types for your
/// queries.
#[cw_serde]
pub struct CurrentValueResponse {
    pub current_value: Uint128,
}

/// This is the query() entrypoint. It allows the contract to define queries that
/// can be run by the user. Queries can not modify state.
///
/// The ExecuteMsg is en enum that is defined below.
///
/// This entrypoint expects the same arguments that execute() does, but instead
/// of InstantiateMsg, it needs the ExecuteMsg, and there is no MessageInfo.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    // Similar to execute(), we try to parse msg into any of the known variants of QueryMsg.
    match msg {
        QueryMsg::CurrentValue {} => query_current_value(deps),
    }
}

pub fn query_current_value(deps: Deps) -> StdResult<Binary> {
    let current_value = &COUNTER.load(deps.storage)?;
    // to_json_binary is a handy helper function from cosmwasm_std that allows you
    // to convert any properly defined Rust type to StdResult<Binary>.
    to_json_binary(&CurrentValueResponse {
        current_value: current_value.clone(),
    })
}
