use crate::error::ContractError;
use crate::storage;
use soroban_sdk::{contracttype, panic_with_error, Env, Map, String, Vec};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Did {
    pub did: String,
    pub is_revoked: bool,
}

pub fn set_initial_dids(e: &Env, dids: &Vec<String>) {
    if dids.is_empty() {
        panic_with_error!(e, ContractError::EmptyDIDs);
    }

    let mut dids_map: Map<String, Did> = Map::new(e);

    for did in dids.iter() {
        dids_map.set(
            did.clone(),
            Did {
                did: did.clone(),
                is_revoked: false,
            },
        )
    }

    storage::write_dids(e, &dids_map);
}

pub fn is_revoked(e: &Env, did: &String) -> Option<bool> {
    storage::read_dids(e)
        .get(did.clone())
        .map(|did_map| did_map.is_revoked)
}

pub fn is_registered(e: &Env, did: &String) -> bool {
    storage::read_dids(e).contains_key(did.clone())
}
