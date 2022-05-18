
pub mod curve_v2;
pub mod curve_v2_price;

fn main(){
    let a1: i128 = 80000000000;
    let a2: i128 = 60000000000;
    let a3: i128 = 5500000000;
   // let d: i128 = 24321000000 * 2;
    
   // let d: i128 = curve_v2_1::get_function_bisection_zero_d(a1, a2);
   // let (l, r) =curve_v2_1::get_initial_bisection_values_d(a1/10, a2/10);
  //  println!("l, r = {}, {}", l, r);

   // let d: i128 = curve_v2::get_function_bisection_zero_d(a1, a2);

   //  println!("d zero = {}", d);

     
   
   let taylor: i128 = curve_v2_price::get_taylor_int(100);
   println!("taylor = {}", taylor);

   let fee: i128 = curve_v2_price::fee(a1, a2);
   println!("fee = {}", fee);

   // Get the Oracle price
   let (p0, p1) = curve_v2_price::get_price_oracle(100, 10000, 20000, 10500, 21000);
   println!("p0 = {}, p1 = {}", p0, p1);
   

 // let (x1_left, x1_right) = curve_v2::get_initial_bisection_values_x(d, a1+a3 );

 // println!("x1_left = {}, x_right = {}", x1_left, x1_right);

 // let x1_zero: i128 = curve_v2::get_function_bisection_zero_x(d, a1 + a3);
 // println!("x1_zero = {}", x1_zero);

  let ask_amount: i128 = curve_v2::get_ask_amount_bisection(a1, a3, a2);
  println!("ask amount = {}", ask_amount);

  let offer_amount: i128 = curve_v2::get_offer_amount_bisection(a1, ask_amount, a2);

  println!("offer amount = {}", offer_amount);
}