fn main() {
    // 不可恢复的错误与panic!
    // 当panic!宏执行时，会打印一个错误信息，展开（unwind）、清理调用栈（Stack）
    // 推出程序
    // panic!("crash and burn!");
    let v = vec![1, 2, 3];
    v[99];
}
