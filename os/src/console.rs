// 控制台输出模块
// 
// 这个模块提供了类似标准库的 print! 和 println! 宏
// 在裸机环境下实现格式化输出

// 导入 SBI 模块中的字符输出函数
use crate::sbi::console_putchar;
// 导入格式化相关的 trait
use core::fmt::{self, Write};

// 定义一个空结构体，用于实现 Write trait
struct Stdout;

// 为 Stdout 实现 Write trait
// Write trait 提供了 write_str 方法，用于输出字符串
impl Write for Stdout {
    /// 输出一个字符串
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // 遍历字符串中的每个字符
        for c in s.chars() {
            // 通过 SBI 调用输出每个字符
            console_putchar(c as usize);
        }
        // 返回成功
        Ok(())
    }
}

/// 底层打印函数
/// 
/// 接收格式化参数，输出到控制台
pub fn print(args: fmt::Arguments) {
    // 创建 Stdout 实例并调用 write_fmt 方法
    // write_fmt 会自动处理格式化字符串
    Stdout.write_fmt(args).unwrap();
}

/// 定义 print! 宏
/// 
/// 用法：print!("Hello, {}!", name);
#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 调用底层 print 函数，传入格式化参数
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

/// 定义 println! 宏（带换行）
/// 
/// 用法：println!("Hello, {}!", name);
#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        // 在格式化字符串末尾添加换行符
        $crate::console::print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

/* 格式化输出原理：
 * 
 * 1. 当你调用 println!("Hello, {}!", "world") 时：
 *    - 宏会将其展开为 format_args!("Hello, {}!\n", "world")
 *    - format_args! 是 Rust 内置宏，用于创建格式化参数
 * 
 * 2. 然后调用 console::print(args)，传入格式化参数
 * 
 * 3. print 函数使用 Stdout.write_fmt(args) 处理格式化
 *    - write_fmt 是 Write trait 的方法
 *    - 它会解析格式化字符串，将参数插入到正确位置
 *    - 然后调用 write_str 输出结果
 * 
 * 4. write_str 遍历字符串，通过 SBI 调用逐个输出字符
 */