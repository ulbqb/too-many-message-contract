use callee::types::Data as CalleeData;
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{StdResult, Storage};
use cw_storage_plus::Item;

use crate::msg::HogeHogeMsg;

pub const ADDRESSES: Item<Addresses> = Item::new("addresses");

#[cw_serde]
pub struct Addresses {
    pub contract_1: String,
    pub contract_2: String,
    pub contract_3: String,
    pub contract_4: String,
    pub contract_5: String,
}

pub fn set_addresses(storage: &mut dyn Storage, addresses: Addresses) {
    let _ = ADDRESSES.save(storage, &addresses);
}

pub fn get_addresses(storage: &mut dyn Storage) -> StdResult<Addresses> {
    ADDRESSES.load(storage)
}

pub const CONTEXT: Item<Context> = Item::new("context");

#[cw_serde]
pub struct Context {
    pub req: HogeHogeMsg,
    pub c1_result: Data,
    pub loop_index: u8,
}

#[cw_serde]
pub struct Data {
    pub alpha: u8,
    pub beta: u8,
    pub delta: u8,
    pub gamma: u8,
    pub is_ok: bool,
}

pub fn init_context(storage: &mut dyn Storage) {
    let _ = CONTEXT.save(
        storage,
        &Context {
            req: HogeHogeMsg { value: 0 },
            c1_result: Data {
                alpha: 0,
                beta: 0,
                delta: 0,
                gamma: 0,
                is_ok: false,
            },
            loop_index: 0,
        },
    );
}

pub fn set_req(storage: &mut dyn Storage, req: HogeHogeMsg) -> StdResult<()> {
    let mut ctx = CONTEXT.load(storage)?;
    ctx.req = req;
    CONTEXT.save(storage, &ctx)
}

pub fn get_req(storage: &mut dyn Storage) -> StdResult<HogeHogeMsg> {
    let ctx = CONTEXT.load(storage)?;
    Ok(ctx.req)
}

pub fn set_c1_result(storage: &mut dyn Storage, result: CalleeData) -> StdResult<()> {
    let mut ctx = CONTEXT.load(storage)?;
    ctx.c1_result = Data {
        alpha: result.alpha,
        beta: result.beta,
        delta: result.delta,
        gamma: result.gamma,
        is_ok: result.is_ok,
    };
    CONTEXT.save(storage, &ctx)
}

pub fn get_c1_result(storage: &mut dyn Storage) -> StdResult<CalleeData> {
    let ctx = CONTEXT.load(storage)?;
    Ok(CalleeData {
        alpha: ctx.c1_result.alpha,
        beta: ctx.c1_result.beta,
        delta: ctx.c1_result.delta,
        gamma: ctx.c1_result.gamma,
        is_ok: ctx.c1_result.is_ok,
    })
}

pub fn increment_loop_index(storage: &mut dyn Storage) -> StdResult<()> {
    let mut ctx = CONTEXT.load(storage)?;
    ctx.loop_index += 1;
    CONTEXT.save(storage, &ctx)
}

pub fn get_loop_index(storage: &mut dyn Storage) -> StdResult<u8> {
    let ctx = CONTEXT.load(storage)?;
    Ok(ctx.loop_index)
}
