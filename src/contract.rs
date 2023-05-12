#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, to_binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Event};
// use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{NAME, SYMBOL, OWNER, TOTALSUPPLY, DECIMAL, BALANCES};

/*
// version info for migration info
const CONTRACT_NAME: &str = "crates.io:erc20";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");
*/

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    NAME.save(deps.storage, &msg.name)?;
    SYMBOL.save(deps.storage, &msg.symbol)?;
    OWNER.save(deps.storage, &deps.api.addr_validate(&msg.owner).unwrap())?;
    TOTALSUPPLY.save(deps.storage, &msg.initial_supply)?;
    DECIMAL.save(deps.storage, &msg.decimal)?;

    BALANCES.save(
        deps.storage,
        &deps.api.addr_validate(&msg.owner).unwrap(),
        &msg.initial_supply,
    )?;

    let event = Event::new("Transfer")
                .add_attribute("from",0.to_string())
                .add_attribute("to",msg.owner)
                .add_attribute("amount",msg.initial_supply.to_string());

    Ok(Response::new().add_event(event))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    use ExecuteMsg::*;

    match msg {
        Transfer { recipient, amount } => execute::transfer(_deps, _info, recipient, amount),
        TransferFrom { from, to, amount } => execute::transfer_from(_deps, _info, from, to, amount),
        Mint { recipient, amount } => execute::mint(_deps, _info, recipient, amount),
        Burn { amount } => execute::burn(_deps, _info, amount),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(_deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    use QueryMsg::*;
    match msg {
        Name {} => to_binary(&query::name(_deps)?),
        Symbol {} => to_binary(&query::symbol(_deps)?),
        Decimal {} => to_binary(&query::decimal(_deps)?),
        TotalSupply {} => to_binary(&query::total_supply(_deps)?),
        BalanceOf { address } => to_binary(&query::balance_of(_deps, address)?),
        Owner {} => to_binary(&query::owner(_deps)?),
    }
}

mod execute {
    use super::*;
    use cosmwasm_std::{Addr, DepsMut, MessageInfo, Response};
    use crate::state::{BALANCES, TOTALSUPPLY};

    pub fn transfer(deps:DepsMut, info:MessageInfo, recipient:String, amount:u128)-> Result<Response,ContractError>{
        let to_address = deps.api.addr_validate(&recipient).unwrap();
        _transfer(deps, &info.sender, &to_address, amount).expect("Issue with Transfer");
        Ok(Response::new().add_event(
            Event::new("Transfer")
                .add_attribute("from",&info.sender.to_string())
                .add_attribute("to",recipient)
                .add_attribute("amount",amount.to_string())
        ))
    }

    pub fn transfer_from(deps:DepsMut, _info:MessageInfo, from:String, to:String, amount:u128)-> Result<Response,ContractError>{
        let from_address = deps.api.addr_validate(&from).unwrap();
        let to_address = deps.api.addr_validate(&to).unwrap();
        _transfer(deps, &from_address, &to_address, amount).expect("Issue with Transfer");
        Ok(Response::new().add_event(
            Event::new("Transfer")
                .add_attribute("from",from)
                .add_attribute("to",to)
                .add_attribute("amount",amount.to_string())
        ))
    }

    pub fn mint(deps:DepsMut, _info:MessageInfo, recipient:String, amount:u128)-> Result<Response,ContractError>{
        let re = deps.api.addr_validate(&recipient).unwrap();
        let mut balances = BALANCES.load(deps.storage, &re)?;
        balances = balances.checked_add(amount).unwrap();
        BALANCES.save(deps.storage, &re, &balances)?;
        
        let mut total_supply = TOTALSUPPLY.load(deps.storage)?;
        total_supply = total_supply.checked_add(amount).unwrap();
        TOTALSUPPLY.save(deps.storage, &total_supply)?;

        Ok(Response::new().add_event(
            Event::new("Transfer")
                .add_attribute("from",0.to_string())
                .add_attribute("to",recipient)
                .add_attribute("amount",amount.to_string())
        ))
    }

    pub fn burn(deps:DepsMut, info:MessageInfo, amount:u128)-> Result<Response,ContractError>{
        let mut balances = BALANCES.load(deps.storage, &info.sender)?;
        balances = balances.checked_sub(amount).unwrap();
        BALANCES.save(deps.storage, &info.sender, &balances)?;

        let mut total_supply = TOTALSUPPLY.load(deps.storage)?;
        total_supply = total_supply.checked_sub(amount).unwrap();
        TOTALSUPPLY.save(deps.storage, &total_supply)?;

        Ok(Response::new().add_event(
            Event::new("Transfer")
                .add_attribute("from",info.sender.to_string())
                .add_attribute("to",0.to_string())
                .add_attribute("amount",amount.to_string())
        ))
    }

    fn _transfer(deps:DepsMut, from:&Addr, to:&Addr, amount:u128)-> Result<Response,ContractError>{
        let mut balances = BALANCES.load(deps.storage, from)?;
        balances = balances.checked_sub(amount).unwrap();
        BALANCES.save(deps.storage, from, &balances)?;

        let mut balances = BALANCES.load(deps.storage, to)?;
        balances = balances.checked_add(amount).unwrap();
        BALANCES.save(deps.storage, to, &balances)?;
        Ok(Response::new())
    }
}

mod query {
    use cosmwasm_std::{Addr, Deps, StdResult};
    use crate::state::{NAME, SYMBOL, OWNER, TOTALSUPPLY, DECIMAL, BALANCES};

    pub fn name(deps: Deps) -> StdResult<String> {
        let name = NAME.load(deps.storage)?;
        Ok(name)
    }

    pub fn symbol(deps: Deps) -> StdResult<String> {
        let symbol = SYMBOL.load(deps.storage)?;
        Ok(symbol)
    }

    pub fn decimal(deps: Deps) -> StdResult<u128> {
        let decimal = DECIMAL.load(deps.storage)?;
        Ok(decimal)
    }

    pub fn total_supply(deps: Deps) -> StdResult<u128> {
        let total_supply = TOTALSUPPLY.load(deps.storage)?;
        Ok(total_supply)
    }

    pub fn balance_of(deps: Deps, addr: Addr) -> StdResult<u128> {
        let balance = BALANCES.load(deps.storage, &addr)?;
        Ok(balance)
    }

    pub fn owner(deps: Deps) -> StdResult<Addr> {
        let owner = OWNER.load(deps.storage)?;
        Ok(owner)
    }
}

#[cfg(test)]
mod tests {


}
