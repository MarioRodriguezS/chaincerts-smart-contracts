#![cfg(test)]

use crate::{DeployerContract, DeployerContractClient};
use soroban_sdk::testutils::Address as _;
use soroban_sdk::{vec, Address, BytesN, Env, FromVal, String, Val, Vec};

use self::contract::{
    CapabilityInvocation, DIDDocument, Method, OptionMethodService, Service, VerificationMethod,
};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(file = "./did_contract.wasm");
}

extern crate std;

#[test]
fn test_from_contract() {
    let env = Env::default();
    let client = DeployerContractClient::new(&env, &env.register_contract(None, DeployerContract));

    // Install the WASM code to be deployed from the deployer contract.
    let wasm_id = env.deployer().upload_contract_wasm(contract::WASM);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let address = Address::generate(&env);
    let init_args = did_init_args(&env, &address);
    let (contract_id, init_result) = client.deploy(&client.address, &wasm_id, &salt, &init_args);
    assert!(init_result.is_void());

    let expected_did_document = DIDDocument {
        id: String::from_str(&env, "did:chaincerts:ABC123"),
        authentication: vec![&env, String::from_str(&env, "did:chaincerts:ABC123#key-1")],
        context: vec![
            &env,
            String::from_str(&env, "https://www.w3.org/ns/did/v1"),
            String::from_str(&env, "https://www.example.com/context/v1"),
        ],
        services: vec![
            &env,
            Service {
                type_: String::from_str(&env, "VerifiableCredential"),
                service_endpoint: String::from_str(&env, "https://did.chaincerts.co/ABC123"),
            },
        ],
        verification_method: vec![
            &env,
            VerificationMethod {
                id: String::from_str(&env, "did:chaincerts:ABC123#key-1"),
                type_: String::from_str(&env, "Ed25519VerificationKey2020"),
                controller: String::from_str(&env, "did:chaincerts:ABC123"),
                blockchain_account_id: address,
            },
        ],
    };
    // Invoke contract to check that it is initialized correctly.
    let client = contract::Client::new(&env, &contract_id);

    let did_document = client.public_did_document();
    assert_eq!(did_document, expected_did_document);
}

#[test]
fn test_deploy_from_address() {
    let env = Env::default();
    let deployer_client =
        DeployerContractClient::new(&env, &env.register_contract(None, DeployerContract));

    // Upload the Wasm to be deployed from the deployer contract.
    // This can also be called from within a contract if needed.
    let wasm_hash = env.deployer().upload_contract_wasm(contract::WASM);

    // Define a deployer address that needs to authorize the deployment.
    let deployer = Address::generate(&env);

    // Deploy contract using deployer, and include an init function to call.
    let salt = BytesN::from_array(&env, &[0; 32]);
    let address = Address::generate(&env);
    let init_fn_args = did_init_args(&env, &address);
    env.mock_all_auths();
    let (contract_id, init_result) =
        deployer_client.deploy(&deployer, &wasm_hash, &salt, &init_fn_args);

    assert!(init_result.is_void());

    let expected_did_document = DIDDocument {
        id: String::from_str(&env, "did:chaincerts:ABC123"),
        authentication: vec![&env, String::from_str(&env, "did:chaincerts:ABC123#key-1")],
        context: vec![
            &env,
            String::from_str(&env, "https://www.w3.org/ns/did/v1"),
            String::from_str(&env, "https://www.example.com/context/v1"),
        ],
        services: vec![
            &env,
            Service {
                type_: String::from_str(&env, "VerifiableCredential"),
                service_endpoint: String::from_str(&env, "https://did.chaincerts.co/ABC123"),
            },
        ],
        verification_method: vec![
            &env,
            VerificationMethod {
                id: String::from_str(&env, "did:chaincerts:ABC123#key-1"),
                type_: String::from_str(&env, "Ed25519VerificationKey2020"),
                controller: String::from_str(&env, "did:chaincerts:ABC123"),
                blockchain_account_id: address,
            },
        ],
    };
    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);

    let did_document = client.public_did_document();
    assert_eq!(did_document, expected_did_document);
}

fn did_init_args(env: &Env, address: &Address) -> Vec<Val> {
    let id = String::from_str(env, "did:chaincerts:ABC123");
    let authentication_params = (
        String::from_str(env, "did:chaincerts:ABC123#key-1"),
        address,
    );
    let context = vec![
        env,
        String::from_str(env, "https://www.w3.org/ns/did/v1"),
        String::from_str(env, "https://www.example.com/context/v1"),
    ];
    let method = Method {
        type_: String::from_str(env, "otp"),
        verified: true,
        timestamp: 1684872059,
        service: OptionMethodService::None,
    };
    let verification_processes = vec![env, method];
    let service = Service {
        type_: String::from_str(env, "VerifiableCredential"),
        service_endpoint: String::from_str(env, "https://did.chaincerts.co/ABC123"),
    };
    let services = vec![env, service];
    let public_add_cap: Option<CapabilityInvocation> = Option::None;

    vec![
        env,
        Val::from_val(env, &id),
        Val::from_val(env, &authentication_params),
        Val::from_val(env, &context),
        Val::from_val(env, &verification_processes),
        Val::from_val(env, &services),
        Val::from_val(env, &public_add_cap),
    ]
}
