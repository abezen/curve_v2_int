
pub mod curve_v2;
pub mod curve_v2_price;
pub mod curve_v2_cosmwasm;


//use cosmwasm_std::Uint256;
extern crate num256;

use num256::Int256;

fn main(){
    let a1: Int256 = Int256::from(8000000000u64);
    let a2: Int256 = Int256::from(6000000000u64);
    let a3: Int256 = Int256::from(550000000u64);

    let num: Int256 = Int256::from(5000000u32);

   
    

     let ask_pool: Int256 = curve_v2_cosmwasm::get_ask_amount_bisection(a1.clone(), a3.clone(), a2.clone());

     println!("cosm wasm ask amount = {}, new ask pool amount = {} \n ---------------- \n", ask_pool.clone(), a2.clone() - ask_pool.clone());

     let offer_pool: Int256 = curve_v2_cosmwasm::get_offer_amount_bisection(a1, ask_pool, a2);
     println!("offer amount = {}", offer_pool);
     

     
  
}