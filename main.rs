use std::fs::File;
use std::io::{self, Write, Error, Read};
use std::{thread, time::{self, Duration}};

macro_rules! printf {
    /*
    标准输出中的这种行缓冲机制
    在遇到换行符之前，输出的内容并不会隐式的刷新
    这就导致 print! 宏和 println! 宏实际上并不完全相同 
     */
    ($($arg:tt)*) =>{
        print!($($arg)*);
        io::stdout().flush().unwrap();
    }
}

fn tnop(s:String, t: Duration){
    let bytes = s.as_bytes(); // 转换为字节数组
    /*
    使用 iter 方法创建了一个可以遍历字节数组的迭代器。
    iter 方法会依次返回集合中的每一个元素。
    之后的 enumerate 则将 iter 的每个输出作为元素逐一封装在对应的元组中返回。
    元组的第一个元素是索引，第二个元素是指向集合中字节的引用，
    使用 enumerate 可以较方便地获得迭代索引。
    */
    /*
    for (i, &item) in bytes.iter().enumerate() {
        println!("{} {}", i, item as char);
    }
    */
    for &item in bytes.iter() {
        printf!("{}", item as char);
        thread::sleep(t);
    }
}

fn main() -> Result<(), Error>  {
    let path = "./tno.txt";
    let mut input = File::open(path).expect("Failed to open.");
    //let buffered = BufReader::new(input);
    let mut contents = String::new();
    input.read_to_string(&mut contents).unwrap();
    let ten_millis = time::Duration::from_millis(50); // ms
    tnop(contents, ten_millis);
    Ok(())
}


