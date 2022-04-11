extern crate num;
use num::abs;


const A: i128 = 100;
const PRECISION: i128 = 100000;
const BETA: i128 = 1000;
const DIV: i128 = 10;

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

pub fn get_initial_bisection_values_d(x0: i128, x1: i128) -> (i128, i128) {
    let d0: i128 = 2 * get_sqrt(x0 * x1);

    let power: i128;

    if d0 < 100 {
        power = 0;
    } 
    else {
       let a: i128 = get_order(d0);
       power = a /3;
    }

    let d_step: i128 = get_step(power);
    let mut d_left: i128 = d0 - d_step;
    let mut d_right: i128 = d0 + d_step;
    let mut f_left: i128 = get_function_value_1(d_left, x0, x1);
    let mut f_right: i128 = get_function_value_1(d_right, x0, x1);

    
    while f_left * f_right > 0 {
        d_left -= d_step;
        d_right += d_step;
        f_left = get_function_value_1(d_left, x0, x1);
        f_right = get_function_value_1(d_right, x0, x1);
    }
    
    return (d_left, d_right);
}

pub fn get_b(d: i128, x0: i128, x1: i128) -> i128 {
    let bk0: i128 = ( 4 * BETA * x0 * x1 ) / (d * d);
    let b: i128 = 1 + BETA - bk0;
    return b * b;  
}

pub fn get_function_value(d: i128, x0: i128, x1: i128) -> i128 {
    let b: i128 = get_b(d, x0, x1);
    let x01: i128 = x0 * x1;
    let t1: i128 = (4 * x01 * (x0 + x1 -d))/d - b * (x01 - d * d);
    return t1 / b;
    
}

pub fn get_function_bisection_zero_d(x0: i128, x1: i128) -> i128 {
    let mut f_mid: i128;
    let (mut d_left, mut d_right) = get_initial_bisection_values_d(x0, x1);
    let mut d_mid: i128 = (d_left + d_right)/2;
    let mut f_left: i128 = get_function_value_1(d_left, x0, x1);
    let mut f_right: i128 = get_function_value_1(d_right, x0, x1);

    while abs(d_left - d_right) > PRECISION {
        d_mid = (d_left + d_right)/2;
        f_mid = get_function_value_1(d_mid, x0, x1);

        if f_left * f_mid < 0 {
            d_right = d_mid;
        } 
        else if f_mid * f_right < 0 {
            d_left = d_mid;
        }
        

        f_left = get_function_value_1(d_left, x0, x1);
        f_right = get_function_value_1(d_right, x0, x1);
        
    }

    return d_mid;
}

pub fn get_k0(d: i128, x0: i128, x1: i128) -> i128 {
    return (4 * x0 * x1)/(d * d);
}

pub fn get_function_value_1(d: i128, x0: i128, x1: i128) -> i128 {
    let b: i128 = get_b(d, x0, x1);
    let k0: i128 = get_k0(d, x0, x1);

    let x01: i128 = x0 * x1;

    let t1: i128 = A * k0 * d * (x0 + x1) + b * x01 ;
    let t2: i128 = ((4 * A * k0 + b) * d * d) / 4 ;
    return t1 - t2;
    
}

pub fn get_ask_amount_bisection(op1: i128, of1: i128, ap1: i128) -> i128 {
    let op: i128 = op1/DIV;
    let ap: i128 = ap1/DIV;
    let of: i128 = of1 / DIV;
    let d: i128 = get_function_bisection_zero_d(op, ap);

    let ask_amnt: i128 = get_function_bisection_zero_x(d, op+of, ap);

    return ask_amnt * DIV;

}

pub fn get_function_bisection_zero_x(d: i128, x0: i128, x1: i128) -> i128 {
    
       let mut f_mid: i128;
   
       let (mut x1_left, mut x1_right) = get_initial_bisection_values_d(x0, x1);
       let mut x1_mid: i128 = (x1_left + x1_right)/2;
   
       let mut f_left: i128 = get_function_value_1(d, x0, x1_left);
       let mut f_right: i128 = get_function_value_1(d, x0, x1_right);
   
   
       while abs(x1_left - x1_right) > PRECISION {
           x1_mid = (x1_left + x1_right)/2;
           f_mid = get_function_value_1(d, x0, x1_mid);
   
           if f_left * f_mid < 0 {
               x1_right = x1_mid;
           } 
           else if f_mid * f_right < 0 {
               x1_left = x1_mid;
           }
           
           f_left = get_function_value_1(d, x0, x1_left);
           f_right = get_function_value_1(d, x0, x1_right);
           
       }
   
       return x1_mid;
   }



