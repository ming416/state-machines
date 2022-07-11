//定义trait返回红绿灯时间
trait LightTime {
     fn get_time (&self) -> u8;     
 }
 //定义实现红绿
 struct Red {}
 struct Yellow {}
 struct Green {}
 
 impl LightTime for Red {
     fn get_time (&self) -> u8{
         15
     }
 }
 
  impl LightTime for Yellow {
     fn get_time (&self) -> u8{
         5
     }
 }
  
  impl LightTime for Green {
     fn get_time (&self) -> u8{
         10
     }
 }
 
 fn main() {
   //返回红灯的时长
   let red = Red{};
   println!("red light time is :{}",red.get_time());
}
