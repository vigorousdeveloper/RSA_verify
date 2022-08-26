use cosmwasm_std::{ Uint128};
use schemars::{JsonSchema};
use serde::{Deserialize, Serialize};
use crate::state::PubKey;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
   pub admin: String,
   pub denom: String,
   pub pubkey:PubKey
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Transfer{
        message: Vec<u8>,
        signature: Vec<u8>,
        to_address: String,
        amount:Uint128
    },
    ChangeAdmin {
        address:String
    },
    SetPublicKey{
       pubkey:PubKey
    }
   
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
       GetStateInfo{}
    }
