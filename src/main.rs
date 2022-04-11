
pub mod curve_v2;

fn main(){
    let a1: i128 = 20000000000;
    let a2: i128 = 20000000000;
    let a3: i128 = 1500000000;
   // let d: i128 = 24321000000 * 2;
    
    //let d: i128 = curve_v2_1::get_function_bisection_zero_d(a1, a2);
   // let (l, r) =curve_v2_1::get_initial_bisection_values_d(a1/10, a2/10);
  //  println!("l, r = {}, {}", l, r);

    //let s: i128 = curve_v2::get_function_bisection_zero_d(a1, a2/10);

   // println!("zero = {}", s);

    let sol: i128 = curve_v2::get_ask_amount_bisection(a1, a3, a2);
    println!("sol = {}", sol);

}