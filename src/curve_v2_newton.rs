extern crate num;
use std::cmp;

use cosmwasm_std::Decimal256;

pub struct CurveValue {
    pos: bool,
    value: Decimal256
}


const A: i128 = 1000;
// const PRECISION: i128 = 100;
const BETA: i128 = 1000;
// const DIV: i128 = 100000;

const GAMMA: Decimal256 = Decimal256::raw(1_000_000_000_000_000u128); // GAMMA = 0.001
const A_256: Decimal256 = Decimal256::raw(100_000_000_000_000_000_000u128); // A = 100
const FOUR: Decimal256 = Decimal256::raw(4_000_000_000_000_000_000u128); // 4
const THREE: Decimal256 = Decimal256::raw(3_000_000_000_000_000_000u128); // 3 
const TWO: Decimal256 = Decimal256::raw(2_000_000_000_000_000_000u128); // 2
const GAMMA1: Decimal256 = Decimal256::raw(1001_000_000_000_000_000u128); // GAMMA + 1
const FOUR1: Decimal256 = Decimal256::raw(250_000_000_000_000_000u128); // 0.25
const ONE: Decimal256 = Decimal256::raw(1_000_000_000_000_000_000u128); //  1
// const TWO: Decimal256 = Decimal256::raw(2_000_000_000_000_000_000u128); //  2
// const TEN_5: i128 = 100000;
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
    let k0: Decimal256 = FOUR * x0 * x1 / (d * d);
    let k: Decimal256 = A_256 * k0 * GAMMA * GAMMA / (GAMMA1 * GAMMA1 + k0 * k0 - TWO * k0 * GAMMA1);
    let fn_left: Decimal256 = k * d * (x0 + x1) + x0 * x1;
    let fn_right: Decimal256 = d * d * (k + FOUR1);
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

pub fn get_deriv_value(d: Decimal256, x0: Decimal256, x1: Decimal256) -> Decimal256 {
    return get_deriv_x1_value_256(d, x0, x1).value;
}

pub fn get_deriv_x1_value_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    let a: Decimal256 = FOUR * A_256 * x0 * GAMMA * GAMMA / d;
   
    let c: Decimal256 = GAMMA + ONE;
    let e: Decimal256 = FOUR * x0 / (d * d);
    let k: Decimal256 = e * x1;
 
    let numer1: CurveValue = CurveValue { pos: true, value: TWO * a * c * x1 };

    let mut n: Decimal256 = a * (e * x1 +c);

    let mut numer2: CurveValue = CurveValue { pos: true, value: ONE };

    if x0 >= d {
        n *= x0 - d;
        numer2.pos = true;
    } else {
        n *= d - x0;
        numer2.pos = false;
    }

    numer2.value = n;

    let mut numer: CurveValue = CurveValue { pos: true, value: ONE };

    if numer2.pos == true {
        numer.pos = true;
        numer.value = numer1.value + numer2.value;
    } else if numer1.value >= numer2.value {
        numer.value = numer1.value - numer2.value;
        numer.pos = true;
    } else {
        numer.value = numer2.value - numer1.value;
        numer.pos = false;
    }

    let l: Decimal256;
    let sgn: bool;

    if c > k {
        l = c - k;   
        sgn = true; 
    } else {
        l = k - c;
        sgn = false;
    }
   
    let der_value: Decimal256 = numer.value / (l * l * l);

    let mut der_struct: CurveValue = CurveValue { pos: true, value: ONE };
    
    if (numer.pos == true && sgn == true) || (numer.pos == false && sgn == false) {
        der_struct.pos = true;
    } else {
        der_struct.pos = false;
    }
    der_struct.value = der_value;

    

    return der_struct;
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
    
    let d: Decimal256 = get_function_zero_d_256(op, ap);
    println!("v2 get ask amount d = {}", d);
    let sum: u128 = op as u128 + of as u128;

    let x0: Decimal256 =  Decimal256::from_atomics(sum, 0).unwrap();
    let x1: Decimal256 = get_next_newton_x1(d, x0);
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128, 0).unwrap(); 
    return ap_dec - x1;
}


pub fn get_offer_amount(op: i128, aa: i128, ap: i128) -> Decimal256 {
    let d: Decimal256 = get_function_zero_d_256(op, ap);
    println!("v2 get offer amount d = {}", d);
    let op_dec: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let aa_dec: Decimal256 = Decimal256::from_atomics(aa as u128, 0).unwrap();
    let ap_dec: Decimal256 = Decimal256::from_atomics(ap as u128,0).unwrap();

    let x0: Decimal256 = get_next_newton_x1(d, ap_dec - aa_dec);
    
    return x0 - op_dec;
}


pub fn get_newton_d(op: i128, ap: i128) -> Decimal256 {
    let d0_i: i128 = 2 * get_sqrt(ap * op);

    let x0: Decimal256 = Decimal256::from_atomics(op as u128, 0).unwrap();
    let x1: Decimal256 = Decimal256::from_atomics(ap as u128, 0 ).unwrap();

    println!("get_newton_d 1");
    let mut d_prev: Decimal256 = Decimal256::from_atomics(d0_i as u128, 0).unwrap();
    let mut d_next: Decimal256;

    

    let mut fn_next: CurveValue = get_function_value_256(d_prev, x0, x1);
    println!("get_newton_d 2");
    let mut der_next: CurveValue = get_deriv_d_value_256(d_prev, x0, x1);
    println!("get_newton_d 3");
    d_next = get_newton_step(fn_next, der_next, d_prev);

    println!("get_newton_d 4");

    let mut delta: Decimal256 = get_delta(d_prev, d_next);

    while delta > PRECISION_256_X {
        d_prev = d_next;

        fn_next = get_function_value_256(d_prev, x0, x1);
        der_next = get_deriv_d_value_256(d_prev, x0, x1);

        d_next = get_newton_step(fn_next, der_next, d_prev);

        delta = get_delta(d_prev, d_next);
    }

    return d_next;


}



pub fn get_function_zero_d_256(x0i: i128, x1i: i128) -> Decimal256 {
     
    let (d_left_i, d_right_i) = get_initial_bisection_values_d(x0i, x1i);
    
    
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


pub fn get_initial_bisection_values_d(x0: i128, x1: i128) -> (i128, i128) {

    let d0: i128 = 2 * get_sqrt(x0 * x1);
    let order_d: i128 = find_number_order(d0);
    let order_x0: i128 = find_number_order(x0);
    let order_x1: i128 = find_number_order(x1);
    let order_a: i128 = find_number_order(A);
    let order_xd: i128 = find_number_order(x0 + x1 - d0);
    let total_order: i128 = order_a + order_d + order_x0 + order_x1 + order_xd;

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

//----------------------------------------
pub fn get_deriv_d_value_256(d: Decimal256, x0: Decimal256, x1: Decimal256) -> CurveValue {
    let a: Decimal256 = FOUR * x0 * x1;
    let b: Decimal256 = GAMMA1;
    let c: Decimal256 = a * A_256 * GAMMA * GAMMA;
    let s: Decimal256 = x0 + x1;
    let n: Decimal256 = c * s;
    let d2: Decimal256 = d * d;
    let e: Decimal256 = b * n * d2 + THREE * a * n;
    let m: Decimal256 = FOUR * c * a * d;

    let numer_value: Decimal256;
    let numer_pos: bool;

    println!("get_newton_d deriv 11");

    if m >= e {
        numer_pos = true;
        numer_value = d2 * (m - e);
    } else {
        numer_pos = false;
        numer_value = d2 * (e - m);
    }

    println!("get_newton_d deriv 12");

    //let curve1: CurveValue = CurveValue { pos: numer_pos, value: numer_value };

    println!("get_newton_d deriv 1");

    let denom_pos: bool;
    let denom_value: Decimal256;
    let bd: Decimal256 = b * d2;

    if bd >= a {
        denom_pos = true;
        denom_value = bd - a;
    } else {
        denom_pos = false;
        denom_value = a - bd; 
    }

    println!("get_newton_d deriv 2");

    let frac_pos: bool;
    let frac_value: Decimal256 = numer_value / denom_value;

    println!("get_newton_d deriv 3");

    if (numer_pos == true && denom_pos == true) || (numer_pos == false && denom_pos == false) {
        frac_pos = true;
    } 
    else {
        frac_pos = false;
    }

    let d_by_2: Decimal256 = d / TWO;

    let deriv_pos: bool;
    let deriv_value: Decimal256;

    println!("get_newton_d deriv 4");

    if frac_pos == true && frac_value >= d_by_2 {
        deriv_value = frac_value - d_by_2;
        deriv_pos = true;
    } else if frac_pos == false && frac_value >= d_by_2 {
        deriv_value = frac_value + d_by_2;
        deriv_pos = false;
    } else if frac_pos == true && frac_value < d_by_2 {
        deriv_value = d_by_2 - frac_value;
        deriv_pos = false;
    } else {
        deriv_value = frac_value + d_by_2;
        deriv_pos = false;
    }
    println!("get_newton_d deriv 5");

    let deriv: CurveValue = CurveValue { pos: deriv_pos, value: deriv_value };

    return deriv;


}


pub fn get_deriv_d_value(d: Decimal256, x0: Decimal256, x1: Decimal256) -> Decimal256 {
    return get_deriv_d_value_256(d, x0, x1).value;
}




  

   





