
pub mod curve_v2;
pub mod curve_v2_price;
pub mod curve_v2_newton;
pub mod curve_v1;
use cosmwasm_std::Decimal256;
pub mod curve_v2_newton2;

fn main(){
     let op: i128 = 80000000000;
     let ap: i128 = 80000000000;
     let of: i128 = 3500000000;
    
    let d: Decimal256 = curve_v2_newton:: get_function_zero_d_256(op, ap);
    let x0: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let x1: Decimal256 = Decimal256::from_atomics(ap as u128, 0).unwrap();
    let x0_of: Decimal256 = Decimal256::from_atomics(of as u128, 0).unwrap();

    let step: Decimal256 = curve_v2_newton::get_next_newton_x1(d, x0+x0_of);
    println!("step = {}, ask amount = {}", step, x1 - step);

    let ask_newton = curve_v2_newton::get_ask_amount_256(op, of, ap);
    println!("NEWTON ask amount = {}, ask pool new = {}", x1 - ask_newton, ask_newton);

    let ask_newton_v1 = curve_v1::get_ask_amount_256(op, of, ap);
    println!("NEWTON v1 ask amount = {}, ask pool new = {}", x1 - ask_newton_v1, ask_newton_v1);

    //let ask_bisection: Decimal256 = curve_v2::get_ask_amount_256(op, of, ap);
   // println!("ask_bisection = {}", ask_bisection);

   // let difference_ask_amount: Decimal256; 
   // if ask_bisection >= ask_newton {
   //     difference_ask_amount = ask_bisection - ask_newton;
   // } else {
   //     difference_ask_amount =  ask_newton - ask_bisection;
   // }
     
    //println!("difference between the methods = {}", difference_ask_amount);
    let ask_amount_i128: i128 = convert_decimal_to_u128(ask_newton);
    let ask_amount_i128_v1: i128 = convert_decimal_to_u128(ask_newton_v1);

    let offer_amount: Decimal256 = curve_v2_newton::get_offer_amount(op, ask_amount_i128, ap);
    println!("V2 offer amount = {}", offer_amount);
     
    let offer_amount_v1: Decimal256 = curve_v1::get_offer_amount(op, ask_amount_i128_v1, ap);
    println!("V1 offer amount = {}", offer_amount_v1);

    let (der, pos) = curve_v2_newton2:: get_deriv_d_value(d, x0, x1);
    println!("newton 2 deriv = {}, pos = {}", der, pos);    
}

pub fn convert_decimal_to_u128(n: Decimal256) -> i128 {
    let s: String = Decimal256::to_string(&n);
    let n_i: i128;
    if s.contains(".") {
    let (left, right) = s.split_once(".").unwrap();
     n_i = left.parse().unwrap();
    } else {
        n_i = s.parse().unwrap();
    }

    return n_i;
}