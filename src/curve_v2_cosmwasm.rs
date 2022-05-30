// use cosmwasm_std::{Decimal, Uint128, Uint256};

extern crate num;
use num::abs;
extern crate num256;

use num256::Int256;


const A: u64 = 1000u64;
const PRECISION: u64 = 1000u64;
const BETA: u64 = 100000u64;
const DIV: u64 = 10000u64;



pub fn get_sqrt(n: Int256) -> Int256 {
    let nc: Int256 = n;
    let mut x: Int256 = nc.clone();
    let two: Int256 = Int256::from(2u32);
    
    let mut y: Int256 = Int256::from(1u32);
    
   let mut b: bool = false;
  
    while !b {
        x = (x + y.clone())/ two.clone();
        y = nc.clone() / x.clone();


        
        if x <= y  {
            b = true;
        }
        
     }

    return x;
}

pub fn get_order(n: Int256) -> Int256 {
    let mut order: Int256 = Int256::from(1u32);
    let ten: Int256 = Int256::from(10u32);
    let one: Int256 = Int256::from(1u32);
    let mut m: Int256 = n.clone() / ten.clone();

    while m > ten.clone() {
        m /= ten.clone();
        order += one.clone();
    }

    return order;
}

pub fn get_step(n: Int256) -> Int256 {
    let ten: Int256 = Int256::from(10u32);
    let one: Int256 = Int256::from(1u32);
    let mut num: Int256 = ten.clone();
    
    let count: Int256 = n.clone() + Int256::from(1u32);

    let mut i: Int256 = one.clone();

    while i < count {
        num *= ten.clone();
        i += one.clone();

    }


    return num;
}


pub fn get_initial_bisection_values_d(x0: Int256, x1: Int256) -> (Int256, Int256) {
    let two: Int256 = Int256::from(2u32);
    let hundred: Int256 = Int256::from(100u32);
    let null: Int256 = Int256::from(0u32);
    let three: Int256 = Int256::from(3u32);
    let one: Int256 = Int256::from(1u32);

    let d0: Int256 = two.clone() * get_sqrt(x0.clone() * x1.clone());

    let power: Int256;

    if d0.clone() < hundred.clone() {
        power = null.clone();
    } 
    else {
       let a: Int256 = get_order(d0.clone());
       power = a.clone() /three.clone() - one.clone();
    }

    println!("power = {}", power.clone());

    let d_step: Int256 = get_step(power.clone());
    let mut d_left: Int256 = d0.clone() - d_step.clone();
    let mut d_right: Int256 = d0.clone() + d_step.clone();
    let mut f_left: Int256 = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
    let mut f_right: Int256 = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
    
    while f_left.clone() > null.clone() && f_right.clone() > null.clone() {
        if f_left.clone() == f_right.clone() {
            d_left -= d_step.clone();
            d_right += d_step.clone();
            f_left = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
            f_right = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
        } else if f_left.clone() > f_right.clone() {
            d_right += d_step.clone();
            f_right = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
        } else {
            d_left -= d_step.clone();
            f_left = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
        }
    }

    while f_left.clone() < null.clone() && f_right.clone() < null.clone() {
        if f_left.clone() == f_right.clone() {
            d_left -= d_step.clone();
            d_right += d_step.clone();
            f_left = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
            f_right = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
        } else if f_left.clone() > f_right.clone() {
            d_left -= d_step.clone();
            f_left = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
        } else {
            d_right += d_step.clone();
            f_right = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
        }
    }

    
    return (d_left, d_right);
}





pub fn get_function_bisection_zero_d(x0: Int256, x1: Int256) -> Int256 {
    let two: Int256 = Int256::from(2u32);
    let hundred: Int256 = Int256::from(100u32);
    let null: Int256 = Int256::from(0u32);
    let three: Int256 = Int256::from(3u32);
    let one: Int256 = Int256::from(1u32);
    let mut f_mid: Int256;
    let (mut d_left, mut d_right) = get_initial_bisection_values_d(x0.clone(), x1.clone());
    let mut d_mid: Int256 = (d_left.clone() + d_right.clone())/two.clone();
    let mut f_left: Int256 = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
    let mut f_right: Int256 = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
    f_mid = get_function_value_3(d_mid.clone(), x0.clone(), x1.clone());
    let prec: Int256 = Int256::from(PRECISION);
    let mut d_mid_last: Int256 = null.clone();

    while abs(d_mid.clone() - d_mid_last.clone()) > prec.clone() {
        d_mid_last = d_mid.clone();
        if f_mid.clone() == null.clone()  {
            return d_mid;
        }

        if f_left.clone() == null.clone() {
            return d_left;
        }

        if f_right.clone() == null.clone() {
            return d_right;
        }


    
         if f_left.clone() > null.clone() && f_mid.clone() > null.clone() && f_right.clone() < null.clone() && f_mid.clone() >= f_left.clone() {
            d_left = d_mid.clone();
        }

        else if f_left.clone() < null.clone() && f_mid.clone() > null.clone() && f_right.clone() > null.clone() {
            d_right = d_mid.clone();
        }

        else if f_left.clone() > null.clone() && f_mid.clone() < null.clone() && f_right.clone() < null.clone() {
            d_right = d_mid.clone();
        }


     
        else if f_left.clone() < null.clone() && f_mid.clone() < null.clone() && f_right.clone() > null.clone() {
            d_left = d_mid.clone();
        }

        
        f_left = get_function_value_3(d_left.clone(), x0.clone(), x1.clone());
        f_right = get_function_value_3(d_right.clone(), x0.clone(), x1.clone());
        d_mid = (d_left.clone() + d_right.clone())/ two.clone();
        f_mid = get_function_value_3(d_mid.clone(), x0.clone(), x1.clone());


        
    }

    

    return d_mid;
}



pub fn get_function_value_3(da: Int256, x0a: Int256, x1a: Int256) -> Int256 {
    let four: Int256 = Int256::from(4u32);
    let a256: Int256 = Int256::from(A);
    let beta256: Int256 = Int256::from(BETA);
    let pow: Int256 = Int256::from(1u32);
    let one: Int256 = Int256::from(1u32);
    let d: Int256 =  da.clone() / pow.clone();
    let x0: Int256 = x0a.clone() / pow.clone();
    let x1: Int256 = x1a.clone()  / pow.clone();
    let half_64: u64 = DIV /2u64;
    let half: Int256 = Int256::from(half_64);
    let null: Int256 = Int256::from(0i128);
    let num: Int256 = four.clone() * a256 * x0.clone() * x1.clone() * d.clone() * (x0.clone() + x1.clone() - d.clone());
    let denum1: Int256 = d.clone() * d.clone() + beta256.clone() * d.clone() * d.clone() - four.clone() * beta256.clone() * x0.clone() * x1.clone();
    let denum2: Int256;
    if four.clone() * x0.clone() * x1.clone() / (d.clone() * d.clone()) == one.clone() {
        denum2 = one.clone() }
    else {
        denum2 = one.clone() +  beta256.clone() - four.clone() * x0.clone() * x1.clone() * beta256.clone() / (d.clone() * d.clone()); // + half.clone();
    } 

    if denum1.clone() == null || denum2.clone() == null {
       // println!("denum1 = {}, denum2 = {}", denum1.clone(), denum2.clone());
    }
    let denum: Int256 = denum1.clone() * denum2.clone() ;

    let f: Int256 = num.clone()  / denum.clone()  + x0.clone() * x1.clone() - d.clone() * d.clone()  /four.clone() ;
    
    return f * pow;
    
}





pub fn get_ask_amount_bisection(op1: Int256, of1: Int256, ap1: Int256) -> Int256 {
    
    let div: Int256 = Int256::from(DIV);
    let op: Int256 = op1/div.clone();
    let ap: Int256 = ap1/div.clone();
    let of: Int256 = of1 / div.clone();

    let d: Int256 = get_function_bisection_zero_d(op.clone(), ap.clone());

    println!("d ask amount = {}", d.clone());

    let new_ask_pool: Int256 = get_function_bisection_zero_x(d.clone(), op.clone()+of.clone());

    println!("cosmwasm new ask amount = {}", new_ask_pool.clone());

    return (ap.clone() - new_ask_pool.clone()) * div.clone();

}




pub fn get_offer_amount_bisection(op1: Int256, aa1: Int256, ap1: Int256) -> Int256 {
    let div: Int256 = Int256::from(DIV);
    let op: Int256 = op1.clone()  / div.clone();
    let ap: Int256 = ap1.clone() / div.clone();
    let aa: Int256 = aa1.clone() / div.clone();

    let d: Int256 = get_function_bisection_zero_d(op.clone(), ap.clone());

    println!("offer d = {}", d.clone());

    let new_offer_pool: Int256 = get_function_bisection_zero_x(d.clone(), ap.clone() - aa.clone());
    return (new_offer_pool - op) * div;
}




pub fn get_function_bisection_zero_x(d: Int256, x0: Int256) -> Int256 {
    let two: Int256 = Int256::from(2u32);
    let null: Int256 = Int256::from(0u32);
  
       let (mut x1_left, mut x1_right) = get_initial_bisection_values_x(d.clone(), x0.clone());
       let mut x1_mid: Int256 = (x1_left.clone() + x1_right.clone())/two.clone();
   
       let mut f_left: Int256 = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
       let mut f_right: Int256 = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());
       let mut f_mid: Int256 = get_function_value_3(d.clone(), x0.clone(), x1_mid.clone());
       let prec: Int256 = Int256::from(PRECISION);
       let mut x1_mid_last: Int256 = null.clone();
   
       while abs(x1_mid.clone() - x1_mid_last.clone()) > prec {
        x1_mid_last = x1_mid.clone();
           
          if f_mid.clone() == null.clone() {
            return x1_mid;
        }

        if f_left.clone() < null.clone() && f_mid.clone() < null.clone() && f_right.clone() > null.clone() {
            x1_left = x1_mid.clone();
        }
        else if f_left.clone() < null.clone() && f_mid.clone() > null.clone() && f_right.clone() > null.clone() {
            x1_right = x1_mid.clone();
        }
        else if f_left.clone() > null.clone() && f_mid.clone() > null.clone() && f_right.clone() < null.clone() {
            x1_left = x1_mid.clone();
        }
        else if f_left.clone() > null.clone() && f_mid.clone() < null.clone() && f_right.clone() < null.clone() {
            x1_right = x1_mid.clone();
        }

            x1_mid = (x1_left.clone() + x1_right.clone())/two.clone();
            f_mid = get_function_value_3(d.clone(), x0.clone(), x1_mid.clone());
           f_left = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
           f_right = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());
           
       }
   
       return x1_mid;
   }




   pub fn get_initial_bisection_values_x(d: Int256, x0: Int256 ) -> (Int256, Int256) {
    let two: Int256 = Int256::from(2u32);
    let hundred: Int256 = Int256::from(100u32);
    let null: Int256 = Int256::from(0u32);
    let three: Int256 = Int256::from(3u32);
    let one: Int256 = Int256::from(1u32);
    let x1: Int256 = d.clone()  / (two.clone() *x0.clone());

    let power: Int256;

    if x1.clone() < hundred.clone() {
        power = null.clone();
    } 
    else {
       let a: Int256 = get_order(x1.clone());
       power = a.clone() /three.clone() - one.clone();
    }

    let x1_step: Int256 = get_step(power.clone()); // * Int256::from(10u32);
    let mut x1_left: Int256 = x1.clone() - x1_step.clone();
    let mut x1_right: Int256 = x1.clone() + x1_step.clone();

    let mut f1_left: Int256 = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
    let mut f1_right: Int256 = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());


    while f1_left.clone() == f1_right.clone() {
        x1_left -= x1_step.clone();
        x1_right += x1_step.clone();
        f1_left = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
        f1_right = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());
    }

    if f1_left.clone() > null.clone() && f1_right.clone() > null.clone()  && f1_left.clone() > f1_right.clone() {
        println!("loop 1");
        while  f1_right.clone() > null.clone() {
            x1_right += x1_step.clone();
            f1_right = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());
        }
    }
    else if f1_left.clone() > null.clone() && f1_right.clone() > null.clone() && f1_left.clone() < f1_right.clone()  {
        println!("loop 2");
        while f1_left.clone() > null.clone() {
            x1_left -= x1_step.clone();
            f1_left = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
        }
    }
    else if f1_left.clone() < null.clone() && f1_right.clone() < null.clone() && f1_left.clone() > f1_right.clone() {
        println!("loop 3");
        while f1_left.clone() < null.clone() {
            x1_left -= x1_step.clone();
            f1_left = get_function_value_3(d.clone(), x0.clone(), x1_left.clone());
        }
    }
    else if f1_left.clone() < null.clone() && f1_right.clone() < null.clone() && f1_left.clone() < f1_right.clone() {
        println!("loop 4 x1_step = {}", x1_step.clone());
        while  f1_right.clone() < null.clone() {
           // x1_right = x1_right.clone() + x1_step.clone();
            x1_right += x1_step.clone();
            f1_right = get_function_value_3(d.clone(), x0.clone(), x1_right.clone());
        }
    }

    println!("x1 left = {}, x1 right = {}", x1_left.clone(), x1_right.clone());

    return (x1_left, x1_right);
}




/*
pub fn calculate_b_k0(d: i128, x0: i128, x1: i128) -> i128 {
    return 4 * BETA * x0 * x1 / (d * d);
}
*/

/* 
pub fn get_function_value_4(d: i128, x0: i128, x1: i128) -> i128 {
    let bk0: i128 = calculate_b_k0(d, x0, x1);
    let r: i128 = 1 + BETA - bk0;

    return 4 * A * bk0 * d * (x0 + x1 - d) +
        (4 * x0 * x1 - d * d) * BETA * r * r;

}
*/




