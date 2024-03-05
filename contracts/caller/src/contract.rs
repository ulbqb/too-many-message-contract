#[cfg(not(feature = "library"))]
use callee::msg::DoSomethingMsg;
use callee::types::Data;
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    wasm_execute, Binary, Deps, DepsMut, Env, MessageInfo, Reply, Response, StdError, StdResult,
    SubMsg, SubMsgResponse, SubMsgResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, HogeHogeMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::state::{
    get_addresses, get_c1_result, get_loop_index, get_req, increment_loop_index, init_context,
    set_addresses, set_c1_result, set_req, Addresses,
};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:caller";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

// callback id
pub const EXEC_HOGEHOGE_CALLBACK_ID_1: u64 = 1;
pub const EXEC_HOGEHOGE_CALLBACK_ID_2: u64 = 2;
pub const EXEC_HOGEHOGE_CALLBACK_ID_3: u64 = 3;
pub const EXEC_HOGEHOGE_CALLBACK_ID_4: u64 = 4;

/// Handling contract instantiation
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    set_addresses(
        deps.storage,
        Addresses {
            contract_1: msg.contract_1,
            contract_2: msg.contract_2,
            contract_3: msg.contract_3,
            contract_4: msg.contract_4,
            contract_5: msg.contract_5,
        },
    );

    // With `Response` type, it is possible to dispatch message to invoke external logic.
    // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

/// Handling contract migration
/// To make a contract migratable, you need
/// - this entry_point implemented
/// - only contract admin can migrate, so admin has to be set at contract initiation time
/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, msg: MigrateMsg) -> Result<Response, ContractError> {
    match msg {
        // Find matched incoming message variant and execute them with your custom logic.
        //
        // With `Response` type, it is possible to dispatch message to invoke external logic.
        // See: https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#dispatching-messages
    }
}

/// Handling contract execution
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::HogeHoge(msg) => exec_hogehoge(deps, env, info, msg),
    }
}

pub fn exec_hogehoge(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: HogeHogeMsg,
) -> Result<Response, ContractError> {
    init_context(deps.storage);
    set_req(deps.storage, msg.clone())?;

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        wasm_execute(
            get_addresses(deps.storage)?.contract_1,
            &DoSomethingMsg {
                value_1: Some(msg.value),
                value_2: None,
            },
            vec![],
        )?,
        EXEC_HOGEHOGE_CALLBACK_ID_1,
    )))
}

fn handle_hogehoge_callback_1(
    deps: DepsMut,
    _env: Env,
    result: SubMsgResponse,
) -> Result<Response, ContractError> {
    let data_bytes = result.data.ok_or(StdError::GenericErr {
        msg: "data is null".into(),
    })?;
    let data = match serde_json::from_slice::<Data>(data_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => return Err(StdError::generic_err(e.to_string()).into()),
    };
    set_c1_result(deps.storage, data.clone())?;

    let req = get_req(deps.storage)?;

    Ok(Response::new().add_submessage(SubMsg::reply_on_success(
        wasm_execute(
            get_addresses(deps.storage)?.contract_2,
            &DoSomethingMsg {
                value_1: Some(req.value),
                value_2: Some(data.alpha),
            },
            vec![],
        )?,
        EXEC_HOGEHOGE_CALLBACK_ID_2,
    )))
}

fn handle_hogehoge_callback_2(
    deps: DepsMut,
    _env: Env,
    result: SubMsgResponse,
) -> Result<Response, ContractError> {
    let data_bytes = result.data.ok_or(StdError::GenericErr {
        msg: "data is null".into(),
    })?;
    let data = match serde_json::from_slice::<Data>(data_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => return Err(StdError::generic_err(e.to_string()).into()),
    };

    let c1_result = get_c1_result(deps.storage)?;

    let mut res = Response::new();
    if data.is_ok && c1_result.is_ok {
        res = res.add_submessage(SubMsg::reply_on_success(
            wasm_execute(
                get_addresses(deps.storage)?.contract_3,
                &DoSomethingMsg {
                    value_1: Some(c1_result.alpha),
                    value_2: None,
                },
                vec![],
            )?,
            EXEC_HOGEHOGE_CALLBACK_ID_3,
        ))
    } else {
        res = res.add_submessage(SubMsg::reply_on_success(
            wasm_execute(
                get_addresses(deps.storage)?.contract_4,
                &DoSomethingMsg {
                    value_1: Some(data.alpha),
                    value_2: None,
                },
                vec![],
            )?,
            EXEC_HOGEHOGE_CALLBACK_ID_3,
        ))
    }

    Ok(res)
}

fn handle_hogehoge_callback_3(
    deps: DepsMut,
    _env: Env,
    result: SubMsgResponse,
) -> Result<Response, ContractError> {
    let data_bytes = result.data.ok_or(StdError::GenericErr {
        msg: "data is null".into(),
    })?;
    let data = match serde_json::from_slice::<Data>(data_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => return Err(StdError::generic_err(e.to_string()).into()),
    };

    let mut res = Response::new();
    for _ in 0..data.delta {
        res = res.add_submessage(SubMsg::reply_on_success(
            wasm_execute(
                get_addresses(deps.storage)?.contract_5,
                &DoSomethingMsg {
                    value_1: None,
                    value_2: None,
                },
                vec![],
            )?,
            EXEC_HOGEHOGE_CALLBACK_ID_4,
        ));
    }

    Ok(res)
}

fn handle_hogehoge_callback_4(
    deps: DepsMut,
    _env: Env,
    result: SubMsgResponse,
) -> Result<Response, ContractError> {
    let index = get_loop_index(deps.storage)?;
    let data_bytes = result.data.ok_or(StdError::GenericErr {
        msg: "data is null".into(),
    })?;
    let data = match serde_json::from_slice::<Data>(data_bytes.as_slice()) {
        Ok(v) => v,
        Err(e) => return Err(StdError::generic_err(e.to_string()).into()),
    };
    do_something(index, data.gamma);
    increment_loop_index(deps.storage)?;
    Ok(Response::new())
}

fn do_something(_index: u8, _gamma: u8) {}

/// Handling contract query
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        // Find matched incoming message variant and query them your custom logic
        // and then construct your query response with the type usually defined
        // `msg.rs` alongside with the query message itself.
        //
        // use `cosmwasm_std::to_binary` to serialize query response to json binary.
    }
}

/// Handling submessage reply.
/// For more info on submessage and reply, see https://github.com/CosmWasm/cosmwasm/blob/main/SEMANTICS.md#submessages
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn reply(deps: DepsMut, env: Env, msg: Reply) -> Result<Response, ContractError> {
    let result = match msg.result {
        SubMsgResult::Ok(res) => res,
        SubMsgResult::Err(e) => return Err(StdError::generic_err(e).into()),
    };
    match msg.id {
        EXEC_HOGEHOGE_CALLBACK_ID_1 => handle_hogehoge_callback_1(deps, env, result),
        EXEC_HOGEHOGE_CALLBACK_ID_2 => handle_hogehoge_callback_2(deps, env, result),
        EXEC_HOGEHOGE_CALLBACK_ID_3 => handle_hogehoge_callback_3(deps, env, result),
        EXEC_HOGEHOGE_CALLBACK_ID_4 => handle_hogehoge_callback_4(deps, env, result),
        id => Err(StdError::generic_err(format!("Unknown reply id: {}", id)).into()),
    }
}
