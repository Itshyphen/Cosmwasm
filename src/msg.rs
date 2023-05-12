use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::Addr;

#[cw_serde]
pub struct InstantiateMsg {
    pub name: String,
    pub symbol: String,
    pub decimal: u128,
    pub initial_supply: u128,
    pub owner: String,
}

#[cw_serde]
pub struct Name {
    pub name: String,
}

#[cw_serde]
pub struct Symbol {
    pub symbol: String,
}

#[cw_serde]
pub struct Owner {
    pub owner: Addr,
}

#[cw_serde]
pub struct Decimals {
    pub decimal: u128,
}

#[cw_serde]
pub struct TotalSupply {
    pub decimal: u128,
}

#[cw_serde]
pub struct BalanceOf {
    pub decimal: u128,
}

#[cw_serde]
pub enum ExecuteMsg {
    Transfer {
        recipient: String,
        amount: u128,
    },
    TransferFrom {
        from: String,
        to: String,
        amount: u128,
    },
    Mint {
        recipient: String,
        amount: u128,
    },
    Burn {
        amount: u128,
    }
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(Name)]
    Name{},
    #[returns(Symbol)]
    Symbol{},
    #[returns(Owner)]
    Owner{},
    #[returns(Decimals)]
    Decimal{},
    #[returns(TotalSupply)]
    TotalSupply{},
    #[returns(BalanceOf)]
    BalanceOf{address: Addr},
}
