


const LN_2_6: [i128; 6] = [  1000000,   693147,   480453,  333025,  230835, 160003];
const COEFFS: [i128; 6] = [120, 120, 60, 20, 5, 1];
const T_1_2: i128 = 500; // moving average half-time
const TEN_6: i128 = 1000000;
const FACTORIAL: i128 = 120;
const GAMMA: i128 = 10000;
const F_MID: i128 = 400;
const F_OUT: i128 = 4000;


pub fn get_taylor_int(t: i128) -> i128 {
    let mut t_1_2_array: [i128; 6] = [0,0,0,0,0,1];
    let mut t_array: [i128; 6] = [1,0,0,0,0,0];
    let sings: [i128; 6] = [1,-1, 1,-1, 1,-1];
    let mut terms: [i128; 6] = [0,0,0,0,0,0];
    
    for i in (0..5).rev() {
        t_1_2_array[i] = t_1_2_array[i+1] * T_1_2; 
    }


    for i in 1..6 {
        t_array[i] = t_array[i-1] * t;
    }

    let mut sum: i128 = 0;

    for i in 0..6 {
        terms[i] = t_1_2_array[i] * COEFFS[i] * LN_2_6[i] * t_array[i] * sings[i];        
        sum += terms[i];
    }

    return sum / (FACTORIAL * t_1_2_array[0]);

    
}

// x0 - offer pool tokens, x1 -ask pool tokens
pub fn fee(x0: i128, x1: i128) -> i128 {
    let t1: i128 = (x0 + x1) * (x0 + x1);
    let t2: i128 = (x0 - x1) * (x0 - x1);
    

    return (F_MID - F_OUT ) * GAMMA * t1 / (GAMMA * t1 + t2) + F_OUT;
}


// oracle price calculated from previous oracle price and the last price where p0 - offer pool price, p1 - ask pool price
pub fn get_price_oracle(t: i128, p0_last: i128, p1_last: i128, p0_oracle_last: i128, p1_oracle_last: i128) -> (i128, i128) {
    let alpha: i128 = get_taylor_int(t);
    let alpha_1: i128 = TEN_6 - alpha;

    let p0_oracle_new: i128 = p0_last * alpha_1 + alpha * p0_oracle_last;
    let p1_oracle_new: i128 = p1_last * alpha_1 + alpha * p1_oracle_last;

    return (p0_oracle_new, p1_oracle_new);
}



