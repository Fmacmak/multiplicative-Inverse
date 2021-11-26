fn xgcd(a:i32, b:i32, s1:i32, s2:i32, t1:i32, t2:i32) -> (i32, i32, i32) {
  
   let s3:i32;
   let t3:i32;
   let q:i32;
   let r:i32;
   
     q = a.div_euclid(b); // quotient 
  
     r = a.rem_euclid(b);  // remainder 
  
   
   s3 = s1 - q * s2;
   t3 = t1 - q * t2;
   
  //when  r==0, b will be the gcd, s2 & t2 the Bezout coefficients

   if r == 0 {
     return (b.abs(), s2, t2)
     }
   else {
      xgcd(b, r, s2, s3, t2, t3)
   }
  }
  
 //////////////////////////////////////////////////
 
fn multinv(b:i32, n:i32) ->i32{

   let my_gcd;
   
    my_gcd = xgcd(n, b,1,0,0,1);
    
    let t:i32 = my_gcd.2; 
    println!("Value of  gcd is {}.\nThe Bezout coefficients s2 is {} and t2 is {}.\n ", my_gcd.0, my_gcd.1, my_gcd.2);
    
   if my_gcd.0 == 1{
      return t.rem_euclid(n);
             }
    else 
    {return 999;}  // 999 used as an error code
  
}

///////////////////////////////////////////////

   
fn main() {
  
  //  Multiplicative inverse of b in Zn i.e b MOD n
   let b = 11;
   let n = 26;
   
   
   println!("Multiplicative inverse of {} in Zn, where n is {}", b, n);
   
      let inverse;
      inverse= multinv(b, n);
      if inverse == 999{     //999 is used as an return error code in the multiplicative inverse function//
           println!(" There is no Multiplicative Inverse for {}mod{}", b, n);      
      }
      
      else{
      println!("The calculated multiplicative inverse of {}mod{} equals {} ", b, n, inverse);
     } 
    
} //end of main



