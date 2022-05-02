
pub mod curve_v2;
pub mod curve_v2_price;

fn main(){
    let a1: i128 = 20000000000;
    let a2: i128 = 80000000000;
    let a3: i128 = 1500000000;
   // let d: i128 = 24321000000 * 2;
    
    //let d: i128 = curve_v2_1::get_function_bisection_zero_d(a1, a2);
   // let (l, r) =curve_v2_1::get_initial_bisection_values_d(a1/10, a2/10);
  //  println!("l, r = {}, {}", l, r);

    //let s: i128 = curve_v2::get_function_bisection_zero_d(a1, a2/10);

   // println!("zero = {}", s);

  //  let sol: i128 = curve_v2::get_ask_amount_bisection(a1, a3, a2);
  //  println!("sol = {}", sol);

  //  let taylor: f64 = curve_v2::get_float_taylor(2.0);
  //   println!("taylor = {}", taylor);

   //  let taylor_int: i128 = curve_v2::get_int_taylor(2,1);
   //  println!("taylor int = {}", taylor_int);
   let taylor: i128 = curve_v2_price::get_taylor_int(100);
   println!("taylor = {}", taylor);

   let fee: i128 = curve_v2_price::fee(a1, a2);
   println!("fee = {}", fee);

   let (p0, p1) = curve_v2_price::get_price_oracle(100, 10000, 20000, 10500, 21000);
   println!("p0 = {}, p1 = {}", p0, p1);
}