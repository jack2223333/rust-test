mod case1;
mod case2;

fn main() {
    //println!("{}",case1::str_replace("abc", "a", "b"));//bbc
    //println!("{}",case1::str_replace("ababcc", "ab", "x"));//xxcc
    //println!("{}",case1::str_replace("语言", "", "x")); 中文会报错
    //println!("{}",case2::copy(1,4)); //调用样例2的函数，正常输出
    //let a = case2::Cat{height : 5, width: 10, weight: 100,name:String::from("aaa")};
    //case2::copy(a,1); //开辟空间大小不足，报错
    println!("--{}++",String::default());
}

