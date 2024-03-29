use cosmwasm_schema::{cw_serde, QueryResponses};

/// Message type for `instantiate` entry_point
#[cw_serde]
pub struct InstantiateMsg {
    pub contract_1: String,
    pub contract_2: String,
    pub contract_3: String,
    pub contract_4: String,
    pub contract_5: String,
}

/// Message type for `execute` entry_point
#[cw_serde]
pub enum ExecuteMsg {
    HogeHoge(HogeHogeMsg),
}

#[cw_serde]
pub struct HogeHogeMsg {
    pub value: u8,
}

/// Message type for `migrate` entry_point
#[cw_serde]
pub enum MigrateMsg {}

/// Message type for `query` entry_point
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // This example query variant indicates that any client can query the contract
    // using `YourQuery` and it will return `YourQueryResponse`
    // This `returns` information will be included in contract's schema
    // which is used for client code generation.
    //
    // #[returns(YourQueryResponse)]
    // YourQuery {},
}

// We define a custom struct for each query response
// #[cw_serde]
// pub struct YourQueryResponse {}
