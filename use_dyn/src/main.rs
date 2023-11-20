use std::{
    fs,
    io::{self, Stdin},
};

#[derive(Debug)]
struct A<T> {
    a: T,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = "-";
    let (mut stdin_read, mut file_read);

    // readable trait对象动态分发,运行时确定具体类型
    let readable: &mut dyn io::Read = if arg == "-" {
        stdin_read = io::stdin();
        &mut stdin_read
    } else {
        file_read = fs::File::open(arg)?;
        &mut file_read
    };

    let mut buf = vec![0; 10];
    readable.read(&mut buf)?;
    println!("动态分发 The bytes is :{:?}", String::from_utf8_lossy(&buf));

    // 默认代码单态化处理
    let a = A { a: 1u8 };
    println!("单态化: {:?}", a.a);

    Ok(())
}
