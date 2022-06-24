
pub mod curve_v2;
pub mod curve_v2_price;
pub mod curve_v2_cosmwasm;


//use cosmwasm_std::Uint256;
// extern crate num256;
use cosmwasm_std::Decimal256;

use num256::Int256;

fn main(){
    /* 
    let a1: Int256 = Int256::from(8000000000u64);
    let a2: Int256 = Int256::from(6000000000u64);
    let a3: Int256 = Int256::from(550000000u64);

    let num: Int256 = Int256::from(5000000u32);

   
    

     let ask_pool: Int256 = curve_v2_cosmwasm::get_ask_amount_bisection(a1.clone(), a3.clone(), a2.clone());

     println!("cosm wasm ask amount = {}, new ask pool amount = {} \n ---------------- \n", ask_pool.clone(), a2.clone() - ask_pool.clone());

     let offer_pool: Int256 = curve_v2_cosmwasm::get_offer_amount_bisection(a1, ask_pool, a2);
     println!("offer amount = {}", offer_pool);
     */
/* 
     let n256: Decimal256 = Decimal256::from_atomics(12345u128, 2).unwrap();
     let n128: String = Decimal256::to_string(&n256);
     
     let (left, right) = n128.split_once(".").unwrap();

     println!("left = {}", left);
     */

     let op: i128 = 40000000000;
     let ap: i128 = 80000000000;
     let of: i128 = 1500000000;
    
    let ask: Decimal256 = curve_v2::get_ask_amount_256(op, of, ap);
    println!("ask amount = {}", ask);

    let ask_i: i128;

    let s1: String = Decimal256::to_string(&ask);
    if s1.contains(".") {
        let (left1, right1) = s1.split_once(".").unwrap();
        ask_i = left1.parse().unwrap();
    } else {
        ask_i = s1.parse().unwrap();
    }

   

    let offer: Decimal256 = curve_v2::get_offer_amount(op, ask_i, ap);
    println!("offer = {}", offer);

     
    
}