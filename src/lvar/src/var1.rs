pub fn vstring() {
 const NAME: u32 = 123;
 println!("The value of NAME is:{}", NAME);
 let mut x = 5;
 println!("The value of x is:{}", x);
 x =  6;
 println!("The value of x is:{}", x)
}

pub fn vnumber() {
 let sumary = 5 + 10 ;
 println!("sum:{}", sumary);
 let sumary:f32 = 7.6 - 3.2;
 println!("sum:{}", sumary);
 let months = ["january", "february", "march"];
 println!("the first of month is {}", months[0]);
}

pub fn complextuple() {
 let tup:(i32, f64, u8) = (23, 23.5, 12);
 let (x, y, z) = tup ;
 println!("x:{}, y:{} , z:{}", x, y, z);
}

pub fn plus_one(x: i32) -> i32 {
   let y = x + 5 ;
   y + 100
}

pub fn check_if_loop(x:i32) {
 println!("start if 语句");

 if x % 2 == 0 {
   println!("x is oushu");
 } else if x >100 { println!("x 大于100") }
 else { println!("x is ok") }
 let mut counter = 0 ;
 loop {
   counter +=1 ;
   println!("counter :{}", counter);
  if counter ==3 {
    break
  }
 }
 let a = ["a", "b", "c", "d"];
 for element in a.iter() {
  println!("the value is {}", element)
 }
}
