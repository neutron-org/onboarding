use std::str::FromStr;

use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Decimal;
use cosmwasm_std::{
    entry_point, from_json, to_json_binary, Addr, Binary, CosmosMsg, DecimalRangeExceeded, Deps,
    DepsMut, Env, MessageInfo, OverflowError, Reply, Response, StdError, StdResult, SubMsg,
    Uint128, WasmMsg,
};
use cw_storage_plus::Item;
use neutron_sdk::bindings::msg::NeutronMsg;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use minimal_contract::contract::CurrentValueResponse;
use minimal_contract::contract::ExecuteMsg as MinimalContractExecuteMsg;
use minimal_contract::contract::QueryMsg as MinimalContractQueryMsg;

use neutron_std::types::cosmos::bank::v1beta1::MsgSend;
use neutron_std::types::cosmos::base::v1beta1::Coin as SDKCoin;

use neutron_std::types::slinky::{self, oracle};

// A config structure for our contract that holds an address of a minimal contract instance
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Config {
    pub minimal_contract_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");

/// -------------------------- REPLY_IDS  -------------------------------------

/// Used to identify a reply from the minimal contract.
pub const INCREASE_COUNT_REPLY_ID: u64 = 0;

/// Used to identify a reply from the bank module.
pub const BANK_SEND_REPLY_ID: u64 = 1;

/// ------------------------ INSTANTIATION ------------------------------------

/// Any data that is necessary to set up your new contract should be added
/// here.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct InstantiateMsg {
    // an address of the minimal contract instance
    pub minimal_contract_address: Addr,
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response<NeutronMsg>, ContractError> {
    // save minimal contract address to config
    CONFIG.save(
        deps.storage,
        &Config {
            minimal_contract_address: msg.minimal_contract_address,
        },
    )?;

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
    /// This message allows the user to specify a Uint128 amount,
    /// to be added to the COUNTER storage item value.
    IncreaseCount { amount: Uint128 },

    /// This message allows the user to specify how much USD converted to NTRN,
    /// to be sent to the recepient address.
    SendNtrn {
        to_address: Addr,
        usd_amount: Uint128,
    },
}

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    /// Keep access to the StdError, just in case.
    #[error(transparent)]
    Std(#[from] StdError),
    #[error(transparent)]
    DecimalErr(#[from] DecimalRangeExceeded),
    #[error(transparent)]
    OverflowErr(#[from] OverflowError),
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env, // We don't use Env in uor implementation, hence the underscore
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    // The `match` here tries to parse msg into any of the known variants of ExecuteMsg.
    // If it's not possible, the user will get an error.
    match msg {
        ExecuteMsg::IncreaseCount { amount } => send_message_to_contract(deps, amount),
        ExecuteMsg::SendNtrn {
            to_address,
            usd_amount,
        } => send_tokens(deps, env, to_address.to_string(), usd_amount),
    }
}

pub fn send_message_to_contract(deps: DepsMut, amount: Uint128) -> Result<Response, ContractError> {
    let config = CONFIG.load(deps.storage)?;

    // here we compose a message to a minimal contract instance to increase a counter there by specified amount
    let message = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: config.minimal_contract_address.into_string(),
        msg: to_json_binary(&MinimalContractExecuteMsg::IncreaseCount { amount })?,
        funds: vec![], // Optionally, you can send funds along with the message.
    });

    Ok(Response::new()
        .add_submessage(SubMsg::reply_on_success(message, INCREASE_COUNT_REPLY_ID)) // we create a submessage to catch the successfull response
        .add_attribute("action", "send_message_to_contract"))
}

pub fn send_tokens(
    deps: DepsMut,
    env: Env,
    to_address: String,
    usd_amount: Uint128,
) -> Result<Response, ContractError> {
    // get NTRN price from Slinky
    let slinky_querier = oracle::v1::OracleQuerier::new(&deps.querier);
    let ntrn_price = slinky_querier.get_price(Some(slinky::types::v1::CurrencyPair {
        base: "NTRN".to_string(),
        quote: "USD".to_string(),
    }))?;

    if ntrn_price.price.is_none() {
        return Err(ContractError::Std(StdError::generic_err(
            "no price for NTRN/USD pair",
        )));
    }

    // normalize the price
    let normalized_price = Decimal::from_atomics(
        Uint128::from_str(&ntrn_price.price.unwrap().price)?,
        ntrn_price.decimals as u32,
    )?;

    // convert usd amount to ntrn amount
    let ntrn_amount = Decimal::from_str(&usd_amount.to_string())?
        .checked_mul(normalized_price)?
        .to_uint_floor();

    // compose bank send message
    let msg = MsgSend {
        from_address: env.contract.address.into_string(),
        to_address,
        amount: vec![SDKCoin {
            denom: "untrn".to_string(),
            amount: ntrn_amount.to_string(),
        }],
    };

    let sub_msg = SubMsg::reply_on_success(Into::<CosmosMsg>::into(msg), BANK_SEND_REPLY_ID); // ReplyOn::Success will capture the response only if send message succeeded.

    Ok(Response::new()
        .add_submessage(sub_msg)
        .add_attribute("action", "send_tokens"))
}

/// ----------------------------- REPLY HANDLER ------------------------------------
pub fn reply(deps: DepsMut, _env: Env, msg: Reply) -> StdResult<Response> {
    let config = CONFIG.load(deps.storage)?;

    // Handle the response message here
    if msg.id == INCREASE_COUNT_REPLY_ID {
        // parse data field from minimal contract execution respons to get counter value
        let counter: Uint128 = from_json(msg.payload)?;

        // make a query to a minimal contract to get current counter value
        let current_counter_value_via_query: CurrentValueResponse = deps.querier.query_wasm_smart(
            config.minimal_contract_address,
            &MinimalContractQueryMsg::CurrentValue {},
        )?;

        // check if counter value from a query does not equal to a counter value from response
        if current_counter_value_via_query.current_value != counter {
            return Err(StdError::generic_err(
                "couter from response does not equal to a counter from query",
            ));
        }

        Ok(Response::new()
            .add_attribute("reply", "success")
            .add_attribute("new_counter", counter))
    } else if msg.id == BANK_SEND_REPLY_ID {
        // Handle the bank response message here
        Ok(Response::new().add_attribute("bank_reply", "success"))
    } else {
        Err(StdError::generic_err("unknown reply id"))
    }
}

/// ----------------------------- QUERIES ------------------------------------

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, _msg: QueryMsg) -> StdResult<Binary> {
    Ok(to_json_binary("").unwrap())
}
