use cosmwasm_std::Empty;
use cw_multi_test::{App, Contract, ContractWrapper, Executor};
use multi_custom::converter_contract::{execute, instantiate, query};

fn converter_contract() -> Box<dyn Contract<Empty>> {
    Box::new(ContractWrapper::new(execute, instantiate, query))
}

#[test]
fn instantiate_should_work() {
    // prepare the simulator
    let mut app = App::default();

    // prepare address of the contract creator
    let creator_addr = app.api().addr_make("creator");

    // store the contract's code on the chain
    let code_id = app.store_code_with_creator(creator_addr, converter_contract());

    // prepare address of the contract owner
    let owner_addr = app.api().addr_make("owner");

    // invoke the `instantiate` entry-point of the contract
    let contract_addr = app.instantiate_contract(code_id, owner_addr, &Empty {}, &[], "converter-contract", None).unwrap();

    // contract is instantiated, we have a valid contract address
    assert_eq!(contract_addr.as_str(), "cosmwasm1mzdhwvvh22wrt07w59wxyd58822qavwkx5lcej7aqfkpqqlhaqfsgn6fq2");
}
