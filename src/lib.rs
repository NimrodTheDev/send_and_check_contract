use cosmwasm_schema::schemars::JsonSchema;
use cosmwasm_std::{coins, entry_point, to_binary, to_json_binary, Addr, BankMsg, BankQuery, Binary, Deps, DepsMut, Env, Event, MessageInfo, QuerierWrapper, Response, StdError, StdResult};
use serde::{ Deserialize, Serialize};
use state::{DEMON, SENDER, TOFROMAMOUNT};
use cosmwasm_schema::schemars;

pub mod state;

// Define the message formats 
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
pub struct InstantiateMsg {
    pub demon: String,
}  

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
pub struct QueryMsg {
    pub value: u128, 
}

use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryEnum {
    #[returns(ValueResp)] // Specify the response type
    Value {addr: String},
}

// Define the response type for the `Value` query
#[cw_serde]
pub struct ValueResp {
    pub value: u128,
}




#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)] 
pub enum ExecuteMsg {
    SendTo {addr: String} 
}


#[entry_point] 
pub fn instantiate(
     deps: DepsMut,
     _env: Env,
     info: MessageInfo,
     msg: InstantiateMsg,
 ) -> StdResult<Response> {
    DEMON.save(deps.storage, &msg.demon).unwrap();
    SENDER.save(deps.storage, &info.sender.as_str().to_string()).unwrap();
     Ok(Response::new()) 
}
// Query function 
#[entry_point] 
pub fn query(
     deps: Deps,
     _env: Env,
     msg: QueryEnum,
 ) -> StdResult<Binary> {
    match msg {
        QueryEnum::Value { addr } =>{
            let coin_value = DEMON.load(deps.storage).unwrap();
            // let sender = SENDER.load(deps.storage).unwrap();
            // cw_utils::must_pay(info, denom)
            let value = deps.querier.query_balance(addr, coin_value).unwrap().amount.u128();
            // Ok(to_json_binary(&resp))
            Ok(to_json_binary(
                &QueryMsg {
                    value,
                }
            ).unwrap())
        }
    }
}  
// Execute function 
#[entry_point] 
pub fn execute(
     deps: DepsMut,
     env: Env,
     info: MessageInfo,
     msg: ExecuteMsg, 
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::SendTo { addr } =>{
            // let to_from_amount = TOFROMAMOUNT.load(deps.storage).unwrap();
            let demon = DEMON.load(deps.storage).unwrap();
            // if info.sender != Addr::unchecked(&to_from_amount.1) {
            //     return Err(StdError::generic_err("Unauthorized"));
            // }
            let sender_balance = deps.querier.query_balance(&info.sender, &demon)?.amount.u128();
            let amount = cw_utils::must_pay(&info, "usei").unwrap().u128();
            if sender_balance < amount {
                return Err(StdError::generic_err("Insufficient funds"));
            }
            let msg = BankMsg::Send {
                to_address: addr.clone(),
                amount: coins(amount, demon)
            };
            Ok(Response::new().add_message(msg).add_events(vec![Event::new("Success")]).add_attribute("Sent", format!("sei: {}, from: {}, to: {}", amount, info.sender, addr))) 
        }
    }
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{coins, Addr};
    use cw_multi_test::{App, AppBuilder, ContractWrapper, Executor, WasmKeeper};

    use crate::{execute, instantiate, query, ExecuteMsg, InstantiateMsg, QueryEnum, QueryMsg};

    #[test]
    fn TestQuery(){
        let mut app = App::new(|router, _, storage|{
            router.bank.init_balance(storage, &Addr::unchecked("cosmwasm1krqht4rynclgrqx04eezl2g49rentmsgmgvkud"), coins(10, "usei")).unwrap();
            router.bank.init_balance(storage, &Addr::unchecked("addr 2"), coins(10, "usei")).unwrap();
        });
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));

        let addr = app.instantiate_contract(code_id, Addr::unchecked("owner"), &InstantiateMsg{
            demon: "usei".to_owned()
        }, &[], "Contract", None).unwrap();
        println!("{}", addr.clone().into_string());
        
        let clone = addr.clone();
        let res: QueryMsg = app.wrap().query_wasm_smart(addr, &QueryEnum::Value { addr: "cosmwasm1krqht4rynclgrqx04eezl2g49rentmsgmgvkud".to_string() }).unwrap();
        println!("this is value: {}", res.value);
        assert_eq!(res.value, 10)

    }

    #[test]
    fn TestExecute(){
        let mut app = AppBuilder::new()
        .with_wasm(|wasm: &mut WasmKeeper| wasm.bech32_prefix("sei"))
        .build(|router, _, storage|{
            router.bank.init_balance(storage, &Addr::unchecked("sei1krqht4rynclgrqx04eezl2g49rentmsgmgvkud"), coins(10, "usei")).unwrap();
            router.bank.init_balance(storage, &Addr::unchecked("addr 2"), coins(10, "usei")).unwrap();
        });
        // let mut app = App::new(|router, _, storage|{
        //     router.bank.init_balance(storage, &Addr::unchecked("cosmwasm1krqht4rynclgrqx04eezl2g49rentmsgmgvkud"), coins(10, "usei")).unwrap();
        //     router.bank.init_balance(storage, &Addr::unchecked("cosmwasm1r5v5srda7xfth3hn2s26txvrcrntldjufdlpa4"), coins(10, "usei")).unwrap();
        // });
        let code = ContractWrapper::new(execute, instantiate, query);
        let code_id = app.store_code(Box::new(code));
        
        let addr = app.instantiate_contract(code_id, Addr::unchecked("owner"), &InstantiateMsg{
            demon: "usei".to_owned()
        }, &[], "Contract", None).unwrap();
        println!("{}", addr.clone().into_string());

        let resp = app.execute_contract(Addr::unchecked("cosmwasm1krqht4rynclgrqx04eezl2g49rentmsgmgvkud"), addr, &ExecuteMsg::SendTo { addr:  "cosmwasm1r5v5srda7xfth3hn2s26txvrcrntldjufdlpa4".to_string()}, &coins(5, "usei")).unwrap();

        let res = app.wrap().query_balance("cosmwasm1krqht4rynclgrqx04eezl2g49rentmsgmgvkud", "usei").unwrap().amount.u128();
        let res2 = app.wrap().query_balance("cosmwasm1r5v5srda7xfth3hn2s26txvrcrntldjufdlpa4", "usei").unwrap().amount.u128();
        assert_eq!(res, 5);
        assert_eq!(res2, 15);
    }
}