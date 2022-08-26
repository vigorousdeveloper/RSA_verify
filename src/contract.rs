use cosmwasm_std::{
    entry_point, to_binary,   CosmosMsg, Deps, DepsMut,Binary,
    Env, MessageInfo, Response, StdResult, Uint128, BankMsg,Coin
};
use rsa::{BigUint,PublicKey,RsaPublicKey,PaddingScheme,Hash};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{
    CONFIG,State, PubKey
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = State {
       admin:msg.admin,
       denom:msg.denom,
       pubkey:msg.pubkey
    };
    CONFIG.save(deps.storage, &state)?;
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Transfer { message, signature, to_address, amount } => execute_transfer(deps,env,info,message,signature,to_address,amount),
        ExecuteMsg::ChangeAdmin { address } => execute_change_admin(deps, info, address),
        ExecuteMsg::SetPublicKey { pubkey } => execute_pub_key(deps,info,pubkey)
    }
}


fn execute_transfer(
    deps: DepsMut,
    _env:Env,
    info: MessageInfo,
    message: Vec<u8>,
    signature: Vec<u8>,
    to_address: String,
    amount: Uint128
) -> Result<Response, ContractError> {
    let state =CONFIG.load(deps.storage)?;
   
    if state.admin != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    let sign = signature;
    let n = BigUint::from_bytes_be(&state.pubkey.n);
    let e = BigUint::from_bytes_be(&state.pubkey.e);

    let digest = message;
  
    let pub_key = RsaPublicKey::new(n, e).unwrap();
    
    let result = pub_key.verify(
        PaddingScheme::new_pkcs1v15_sign(Some(Hash::SHA1)),
        &digest,
        &sign,
    );

    match  result {
        Ok(_result) =>{
            let msg:CosmosMsg = CosmosMsg::Bank(BankMsg::Send {
                to_address: to_address.clone(),
                amount: vec![Coin{
                    denom:state.denom,
                    amount:amount
                }]
            });

            Ok(Response::new()
                .add_message(msg)
                .add_attribute("action", "transfer")
                .add_attribute("coin_amount", amount)
                .add_attribute("to_address", to_address))
             }
        Err(_err) =>{
            return Err(ContractError::WrongSignature {  })
        }
    }
    
    
}




fn execute_change_admin(
    deps: DepsMut,
    info: MessageInfo,
    address: String,
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
   
   if state.admin != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG.update(deps.storage,|mut state|-> StdResult<_>{
            state.admin = address;
            Ok(state)
        }
    )?;
    Ok(Response::default())
}


fn execute_pub_key(
    deps: DepsMut,
    info: MessageInfo,
    pubkey:PubKey
) -> Result<Response, ContractError> {
   let state =CONFIG.load(deps.storage)?;
   
    if state.admin != info.sender.to_string() {
        return Err(ContractError::Unauthorized {});
    }

    CONFIG.update(deps.storage,
        |mut state|-> StdResult<_>{state.pubkey = pubkey;
            Ok(state)
        }
    )?;
    Ok(Response::new()
        .add_attribute("action", "set new public key"))
}


#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetStateInfo {} => to_binary(& query_get_info(deps)?),
    }
}


pub fn query_get_info(deps:Deps) -> StdResult<State>{
    let state = CONFIG.load(deps.storage)?;
    Ok(state)
}


