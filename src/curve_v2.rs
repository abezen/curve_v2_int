extern crate num;
use num::{abs, pow};
use std::cmp;

use cosmwasm_std::Decimal256;

pub struct CurveValue {
    pos: bool,
    value: Decimal256
}


const A: i128 = 1000;
const PRECISION: i128 = 100;
const BETA: i128 = 1000;
const DIV: i128 = 100000;

const GAMMA: Decimal256 = Decimal256::raw(1_000_000_000_000_000u128); // GAMMA = 0.001
const A_256: Decimal256 = Decimal256::raw(100_000_000_000_000_000_000u128); // A = 100
const FOUR: Decimal256 = Decimal256::raw(4_000_000_000_000_000_000u128); // 4
const TWO: Decimal256 = Decimal256::raw(2_000_000_000_000_000_000u128); // 2
const GAMMA1: Decimal256 = Decimal256::raw(1001_000_000_000_000_000u128); // GAMMA + 1
const FOUR1: Decimal256 = Decimal256::raw(250_000_000_000_000_000u128); // 4
// const TEN_5: i128 = 100000;
const PRECISION_256: Decimal256 = Decimal256::raw(100_000_000_000_000_000_000_000u128); // precision = 100000



pub fn get_sqrt(n: i128) -> i128 {
    let mut x: i128 = n;
    let mut y: i128 = 1;
    while x > y {
        x = (x + y)/2;
        y = n /x;
    }

    return x;
}

pub fn get_order(n: i128) -> i128 {
    let mut order = 1;
    let mut m: i128 = n /10;

    while m > 10 {
        m /= 10;
        order += 1;
    }

    return order;
}

pub fn get_step(n: i128) -> i128 {
    let mut num: i128 = 10;

    for _i in 1..n+1 {
        num *= 10;
    }

    return num;
}

/* 
pub fn get_initial_values_d_256(x0: Decimal256, x1: Decimal256 ) -> (CurveValue, CurveValue) {

}
*/

pub fn get_initial_values_d_256(x0_i: i128, x1_i: i128) -> (Decimal256, Decimal256) {
    let d0_i: i128 = 2 * get_sqrt(x0_i * x1_i);

    let x0: Decimal256 = Decimal256::from_atomics(x0_i as u128, 0).unwrap();
    let x1: Decimal256 = Decimal256::from_atomics(x1_i as u128, 0).unwrap();
    let power: i128;

    if d0_i < 100 {
        power = 0;
    } 
    else {
       let a: i128 = get_order(d0_i);
       power = a /3-1;
    }
    let d_step_u: u128 = get_step(power) as u128;

    

    let d_step: Decimal256 = Decimal256::from_atomics(d_step_u, 0).unwrap();
   
   

    let d_0: Decimal256 = Decimal256::from_atomics(d0_i as u128,  0).unwrap();

    let mut d_left: Decimal256 = d_0 - d_step;
    let mut d_right: Decimal256 = d_0 + d_step;

   

    let mut curve_value_left: CurveValue = get_function_value_256(d_left, x0, x1);
    let mut curve_value_right: CurveValue = get_function_value_256(d_right, x0, x1);

   
    while curve_value_left.pos == true && curve_value_right.pos == true {

        if curve_value_left.value == curve_value_right.value {
            d_left -= d_step;
            d_right += d_step;

            curve_value_left = get_function_value_256(d_left, x0, x1);
            curve_value_right = get_function_value_256(d_right, x0, x1);

          
        } else if curve_value_left.value > curve_value_right.value {
            d_right += d_step;
            curve_value_right = get_function_value_256(d_right, x0, x1);
         
        } else {
            d_left -= d_step;
            curve_value_left = get_function_value_256(d_left, x0, x1);
          
        }
       
    }

    while  curve_value_left.pos == false && curve_value_right.pos == false {
        if curve_value_left.value == curve_value_right.value {
            d_left -= d_step;
            d_right += d_step;

            curve_value_left = get_function_value_256(d_left, x0, x1);
            curve_value_right = get_function_value_256(d_right, x0, x1);

         
        } else if curve_value_left.value < curve_value_right.value {
            d_left -= d_step;
            curve_value_left = get_function_value_256(d_left, x0, x1);
          
        } else {
            d_right += d_step;
            curve_value_right = get_function_value_256(d_right, x0, x1);
        
        }
      
    }

    

    return (d_left, d_right);

}


pub fn get_initial_bisection_values_d(x0: i128, x1: i128) -> (i128, i128) {

    let d0: i128 = 2 * get_sqrt(x0 * x1);
    let order_d: i128 = find_number_order(d0);
    let order_x0: i128 = find_number_order(x0);
    let order_x1: i128 = find_number_order(x1);
    let order_A: i128 = find_number_order(A);
    let order_xd: i128 = find_number_order(x0 + x1 - d0);
    let total_order: i128 = order_A + order_d + order_x0 + order_x1 + order_xd;

    let order: i128 = cmp::max((total_order - 36) / 5 + 1, 0);
  
    let powr: i128 = num::pow(10, order as usize);

    let power: i128;

    if d0 < 100 {
        power = 0;
    } 
    else {
       let a: i128 = get_order(d0);
       power = a /3-1;
    }

    let d_step: i128 = get_step(power);
    let mut d_left: i128 = d0 - d_step;
    let mut d_right: i128 = d0 + d_step;
    let mut f_left: i128 = get_function_value_3(d_left, x0, x1, powr);
    let mut f_right: i128 = get_function_value_3(d_right, x0, x1, powr);

    
    while f_left > 0 && f_right > 0 {
        if f_left == f_right {
            d_left -= d_step;
            d_right += d_step;
            f_left = get_function_value_3(d_left, x0, x1, powr);
            f_right = get_function_value_3(d_right, x0, x1, powr);
        } else if f_left > f_right {
            d_right += d_step;
            f_right = get_function_value_3(d_right, x0, x1, powr);
        } else {
            d_left -= d_step;
            f_left = get_function_value_3(d_left, x0, x1, powr);
        }
    }

    while f_left < 0 && f_right < 0 {
        if f_left == f_right {
            d_left -= d_step;
            d_right += d_step;
            f_left = get_function_value_3(d_left, x0, x1, powr);
            f_right = get_function_value_3(d_right, x0, x1, powr);
        } else if f_left > f_right {
            d_left -= d_step;
            f_left = get_function_value_3(d_right, x0, x1, powr);
        } else {
            d_right += d_step;
            f_right = get_function_value_3(d_left, x0, x1, powr);
        }
    }

    
    return (d_left, d_right);
}



pub fn get_function_zero_d_256(x0i: i128, x1i: i128) -> Decimal256 {
     
    let (mut d_left_i, mut d_right_i) = get_initial_bisection_values_d(x0i, x1i);
    
    
    let x0: Decimal256 = Decimal256::from_atomics(x0i as u128, 0).unwrap();
    let x1: Decimal256 = Decimal256::from_atomics(x1i as u128, 0).unwrap();

    let mut curve_value_left: CurveValue;
    let mut curve_value_right: CurveValue;

    let mut f_mid: Decimal256;

    let mut d_left: Decimal256;
    let mut d_right: Decimal256;

   
   
    d_left = Decimal256::from_atomics(d_left_i as u128, 0).unwrap();
    d_right = Decimal256::from_atomics(d_right_i as u128, 0).unwrap();

    curve_value_left = get_function_value_256(d_left, x0, x1);

   
    curve_value_right = get_function_value_256(d_right, x0, x1);
   
    let mut d_mid: Decimal256 = (d_left + d_right)/TWO;
   
    let mut curve_value_mid: CurveValue = get_function_value_256(d_mid, x0, x1);

    
     
    while curve_value_mid.value > PRECISION_256 {
        if curve_value_mid.value <= PRECISION_256 {
            return d_mid;
        }
        else if  curve_value_left.value <= PRECISION_256 {
            return d_left;
        } else if curve_value_right.value <= PRECISION_256 {
            return d_right;
        } else if  curve_value_left.pos == true && curve_value_mid.pos == true && curve_value_right.pos == false {
            d_left = d_mid;
        } else if curve_value_left.pos == false && curve_value_mid.pos == true && curve_value_right.pos == true {
            d_right = d_mid;
        } else if curve_value_left.pos == true && curve_value_mid.pos == false && curve_value_right.pos == false {
            d_right = d_mid;
        } else if curve_value_left.pos == false && curve_value_mid.pos == false && curve_value_right.pos == true {
            d_left = d_mid;
        }

        curve_value_left = get_function_value_256(d_left, x0, x1);
        curve_value_right = get_function_value_256(d_right, x0, x1);
        d_mid = (d_left + d_right) / TWO;
        curve_value_mid = get_function_value_256(d_mid, x0, x1);

    }
    

    return d_mid;

}



pub fn get_function_value_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    let k0: Decimal256 = FOUR * x0 * x1 / (d * d);
    let k: Decimal256 = A_256 * k0 * GAMMA * GAMMA / (GAMMA1 * GAMMA1 + k0 * k0 - TWO * k0 * GAMMA1);
    let fn_left: Decimal256 = k * d * (x0 + x1) + x0 * x1;
    let fn_right: Decimal256 = d * d * (k + FOUR1);
    let fn_value: Decimal256;
    let fn_pos: bool;
    if fn_right >= fn_left {
        fn_pos = true;
        fn_value = fn_right - fn_left;
    } else {
        fn_pos = false;
        fn_value = fn_left - fn_right;
    }
    let fn_struct= CurveValue {
        pos: fn_pos,
        value: fn_value
    };

    return fn_struct;
}

pub fn find_number_order(n: i128) -> i128 {
    let mut order: i128 = 0;
    let mut tens: i128 = 10;
    let mut div: i128 = n / tens;

    while  div > 0  {
        order += 1;
        tens *= 10;
        div = n / tens;
    }
    
    return order;
}


pub fn get_function_value_3(da: i128, x0a: i128, x1a: i128, powr: i128) -> i128 {
   
    let d: i128 =  da / powr;
    let x0: i128 = x0a / powr;
    let x1: i128 = x1a  / powr;
    let num: i128 = 4 * A * x0 * x1 * d * (x0 + x1 - d);
    let denum1: i128 = d * d + BETA * d * d - 4 * BETA * x0 * x1;
    let denum2: i128;
    if 4 * x0 * x1 / (d * d) == 1{
        denum2 = 1}
    else {
        denum2 = 1 +  BETA - 4 * x0 * x1 * BETA / (d * d);
    } 

    if denum1 == 0 || denum2 == 0 {
     
    }
    let denum: i128 = denum1 * denum2 ;

    let f: i128 = num  / denum  + x0 * x1 - d * d  /4 ;
    
    return f * powr;
    
}

pub fn get_ask_amount_256(op: i128, of: i128, ap: i128) -> Decimal256 {
    let d: Decimal256 = get_function_zero_d_256(op, ap);
    let sum: u128 = op as u128 + of as u128;

    let x0: Decimal256 =  Decimal256::from_atomics(sum, 0).unwrap();
   


    let x1: Decimal256 = get_function_zero_x_256(d, x0);
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128, 0).unwrap(); 
    return ap_dec - x1;

}

pub fn get_offer_amount(op: i128, aa: i128, ap: i128) -> Decimal256 {
    let d: Decimal256 = get_function_zero_d_256(op, ap);
   

    let op_dec: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let aa_dec: Decimal256 = Decimal256::from_atomics(aa as u128, 0).unwrap();
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128,0).unwrap();

    let x0: Decimal256 = get_function_zero_x_256(d, ap_dec - aa_dec);
    return x0 - op_dec;
}



 
pub fn get_function_zero_x_256(d: Decimal256, x0: Decimal256) -> Decimal256 {
    let mut s: String = Decimal256::to_string(&d);
    let d_i: i128;
    if s.contains(".") {
    let (mut left, mut right) = s.split_once(".").unwrap();
     d_i = left.parse().unwrap();
    } else {
        d_i = s.parse().unwrap();
    }

    s = Decimal256::to_string(&x0);
   
  
    let x0_i: i128 = s.parse().unwrap();


    let (mut x1_left_i, mut x1_right_i) = get_initial_bisection_values_x(d_i, x0_i); 
    let mut x1_left = Decimal256::from_atomics(x1_left_i as u128, 0).unwrap();
    let mut x1_right = Decimal256::from_atomics(x1_right_i as u128, 0).unwrap();
    
    let mut curve_value_left: CurveValue = get_function_value_256(d, x0, x1_left);
    let mut curve_value_right: CurveValue = get_function_value_256(d, x0, x1_right);

    let mut x1_mid: Decimal256 = (x1_left + x1_right)/TWO;

    let mut curve_value_mid: CurveValue = get_function_value_256(d, x0, x1_mid);


    while curve_value_mid.value > PRECISION_256 {
        
        if  curve_value_left.pos == true && curve_value_mid.pos == true && curve_value_right.pos == false {
            x1_left = x1_mid;
            curve_value_left = curve_value_mid;
        } else if curve_value_left.pos == false && curve_value_mid.pos == true && curve_value_right.pos == true {
            x1_right = x1_mid;
            curve_value_right = curve_value_mid;
        } else if curve_value_left.pos == true && curve_value_mid.pos == false && curve_value_right.pos == false {
            x1_right = x1_mid;
            curve_value_right = curve_value_mid;
        } else if curve_value_left.pos == false && curve_value_mid.pos == false && curve_value_right.pos == true {
            x1_left = x1_mid;
            curve_value_left = curve_value_mid;
        }

       
        x1_mid = (x1_left + x1_right) / TWO;
        curve_value_mid = get_function_value_256(d, x0, x1_mid);

    }


    return x1_mid;
}




  

   pub fn get_initial_bisection_values_x(d: i128, x0: i128 ) -> (i128, i128) {
   
   
    let x1: i128 = d * d /( 4 * x0);
    
    let order_d: i128 = find_number_order(d);
    let order_x0: i128 = find_number_order(x0);
    let order_x1: i128 = find_number_order(x1);
    let order_A: i128 = find_number_order(A);
    let order_xd: i128 = find_number_order(x0 + x1 - d);
    let total_order: i128 = order_A + order_d + order_x0 + order_x1 + order_xd;
    let order: i128 = cmp::max((total_order - 36) / 4 + 1, 0);
    let powr: i128 = num::pow(10, order as usize);
   

    let power: i128;

    if x1 < 100 {
        power = 0;
    } 
    else {
       let a: i128 = get_order(x1);
       power = a /3 - 1;
    }

    let x1_step: i128 = get_step(power);
    let mut x1_left: i128 = x1 - x1_step;
    let mut x1_right: i128 = x1 + x1_step;

    let mut f1_left: i128 = get_function_value_3(d, x0, x1_left, powr);
    let mut f1_right: i128 = get_function_value_3(d, x0, x1_right, powr);


    while f1_left == f1_right {
        x1_left -= x1_step;
        x1_right += x1_step;
        f1_left = get_function_value_3(d, x0, x1_left, powr);
        f1_right = get_function_value_3(d, x0, x1_right, powr);
    }

    if f1_left > 0 && f1_right > 0  && f1_left > f1_right {
        
        while  f1_right > 0 {
            x1_right += x1_step;
            f1_right = get_function_value_3(d, x0, x1_right, powr);
        }
    }
    else if f1_left > 0 && f1_right > 0 && f1_left < f1_right  {
        
        while f1_left > 0 {
            x1_left -= x1_step;
            f1_left = get_function_value_3(d, x0, x1_left, powr);
        }
    }
    else if f1_left < 0 && f1_right < 0 && f1_left > f1_right {
        
        while f1_left < 0 {
            x1_left -= x1_step;
            f1_left = get_function_value_3(d, x0, x1_left, powr);
        }
    }
    else if f1_left < 0 && f1_right < 0 && f1_left < f1_right {
        
        while  f1_right < 0 {
            x1_right += x1_step;
            f1_right = get_function_value_3(d, x0, x1_right, powr);
        }
    }

   

    return (x1_left, x1_right);
}






