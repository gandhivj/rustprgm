pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum=0;
    let mut n= num;
    let mut i=num;
    let mut m = 0;
    let mut count=0;
    while n>0{
     n=n/10;
     count =count+ 1;
}
     while i>0 {
        m=i%10

     }

     while i>0{
          m=i/10;
          sum =sum+ u32::pow(count,m);
          i/=10;

         }
println!("{:?} {} {} ",sum,m,count);

 if sum ==num {
    true

    }
    else{
    false
    }
