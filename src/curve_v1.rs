extern crate num;
use std::cmp;

use cosmwasm_std::Decimal256;

pub struct CurveValue {
    pos: bool,
    value: Decimal256
}

const A_256: Decimal256 = Decimal256::raw(100_000_000_000_000_000_000u128); // A = 100
const FOUR: Decimal256 = Decimal256::raw(4_000_000_000_000_000_000u128); // 4
const THREE: Decimal256 = Decimal256::raw(3_000_000_000_000_000_000u128); // 3 
const TWO: Decimal256 = Decimal256::raw(2_000_000_000_000_000_000u128); // 2
const ONE: Decimal256 = Decimal256::raw(1_000_000_000_000_000_000u128); //  1
const PRECISION_256: Decimal256 = Decimal256::raw(100_000_000_000_000_000_000_000u128); // precision = 100000
const PRECISION_256_X: Decimal256 = Decimal256::raw(1_000_000_000_000_000_000u128); // precision = 10000


pub fn get_sqrt(n: i128) -> i128 {
    let mut x: i128 = n;
    let mut y: i128 = 1;
    while x > y {
        x = (x + y)/2;
        y = n /x;
    }

    return x;
}

pub fn get_sqrt_256(n: Decimal256) -> Decimal256 {
    let mut x: Decimal256 = n;
    let mut y: Decimal256 = ONE;

    while x > y {
        x = (x + y) / TWO;
        y = n / x;
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



pub fn get_function_value_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    
    let fn_left: Decimal256 = FOUR * A_256 * (x0 + x1) + d;
    let fn_right: Decimal256 = FOUR * A_256 * d + d * d * d / (FOUR * x0 * x1);
    let fn_value: Decimal256;
    let fn_pos: bool;
    if fn_right >= fn_left {
        fn_pos = false;
        fn_value = fn_right - fn_left;
    } else {
        fn_pos = true;
        fn_value = fn_left - fn_right;
    }
    let fn_struct= CurveValue {
        pos: fn_pos,
        value: fn_value
    };

    return fn_struct;
}

pub fn deriv_d_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    let der_left: Decimal256 = ONE;
    let der_right: Decimal256 = FOUR * A_256 + THREE * d * d / (FOUR * x0 * x1);
    let der_pos: bool;
    let der_value: Decimal256;

    if der_right >= der_left {
        der_pos = false;
        der_value = der_right - der_left;
    } else {
        der_pos = true;
        der_value = der_left - der_right;
    }

    let der_struct = CurveValue {
        pos: der_pos,
        value: der_value
    };

    return der_struct;
}

pub fn get_deriv_value(d: Decimal256, x0: Decimal256, x1: Decimal256) -> Decimal256 {
    return get_deriv_x1_value_256(d, x0, x1).value;
}

pub fn get_deriv_x1_value_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    
   return CurveValue {
        pos: true,
        value: FOUR * A_256 + d * d* d / (FOUR * x0 * x1 * x1)
   };
   
}

pub fn get_newton_step(func: CurveValue, der: CurveValue, x: Decimal256) -> Decimal256 {
    let frac: Decimal256 = func.value / der.value;
    let x1: Decimal256;

    if (func.pos  == true && der.pos == false) || (func.pos == false && der.pos == true) {
        x1 = x + frac;
    } else {   
        x1 = x - frac;
    }
    return x1;
}

pub fn get_next_newton_d(x0: Decimal256, x1: Decimal256) -> Decimal256 {
    let mut d_next: Decimal256;
    let mut d_prev = TWO * get_sqrt_256(x0 *x1);
    let mut func_next: CurveValue = get_function_value_256(d_prev, x0, x1);
    let mut der_next: CurveValue = deriv_d_256(d_prev, x0, x1);

    d_next = get_newton_step(func_next, der_next, d_prev);
    let mut delta: Decimal256 = get_delta(d_prev, d_next);

    while delta > PRECISION_256_X {
        d_prev = d_next;

        func_next = get_function_value_256(d_prev, x0, x1);
        der_next = deriv_d_256(d_prev, x0, x1);
        d_next = get_newton_step(func_next, der_next, d_prev);
        delta = get_delta(d_prev, d_next);
    }
        
    return d_next;

}


pub fn get_next_newton_x1(d: Decimal256, x0: Decimal256) -> Decimal256 {

    let mut x1_next: Decimal256;
    let mut x1_prev: Decimal256 = d * d /( FOUR * x0);
    let mut f_next: CurveValue = get_function_value_256(d, x0, x1_prev);
    let mut d_next: CurveValue = get_deriv_x1_value_256(d, x0, x1_prev);

    x1_next = get_newton_step(f_next, d_next, x1_prev);

    let mut delta: Decimal256;

    if x1_next >= x1_prev {
        delta = x1_next - x1_prev;
    } else {
        delta = x1_prev - x1_next;
    }

    while delta > PRECISION_256_X {
        x1_prev = x1_next;

        f_next = get_function_value_256(d, x0, x1_prev);
        d_next = get_deriv_x1_value_256(d, x0, x1_prev);

        x1_next = get_newton_step(f_next, d_next, x1_prev);

        if x1_next >= x1_prev {
            delta = x1_next - x1_prev;
        } else {
            delta = x1_prev - x1_next;
        }
    }
        
    return x1_next;

    
}
    

pub fn get_delta(x_prev: Decimal256, x_next: Decimal256) -> Decimal256 {
    if x_next >= x_prev {
        return x_next - x_prev;
    } else {
        return x_prev - x_next;
    }
}


pub fn get_ask_amount_256(op: i128, of: i128, ap: i128) -> Decimal256 {
    
    let offer_pool: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let ask_pool: Decimal256 = Decimal256::from_atomics(ap as u128, 0).unwrap();
    let offer: Decimal256 = Decimal256::from_atomics(of as u128, 0).unwrap();

    let d: Decimal256 = get_next_newton_d(offer_pool, ask_pool);

    println!("v1 get ask amount d = {}", d);

    let sum: u128 = op as u128 + of as u128;

    let x0: Decimal256 =  Decimal256::from_atomics(sum, 0).unwrap();

    let x1: Decimal256 = get_next_newton_x1(d, x0);
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128, 0).unwrap(); 
    return ap_dec - x1;
}

pub fn get_offer_amount(op: i128, aa: i128, ap: i128) -> Decimal256 {
    let op_dec: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let aa_dec: Decimal256 = Decimal256::from_atomics(aa as u128, 0).unwrap();
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128,0).unwrap();
    let d: Decimal256 = get_next_newton_d(op_dec, ap_dec);

    println!("v1 get offer amount d = {}", d);
    let x0: Decimal256 = get_next_newton_x1(d, ap_dec - aa_dec);
    return x0 - op_dec;
}


