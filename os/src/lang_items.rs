// 语言项模块
// 
// Rust 编译器需要一些特殊的函数来处理语言层面的操作
// 这些函数被称为 "lang items"（语言项）
// 
// 在裸机环境下（no_std），我们需要自己实现这些函数

// 导入关机函数
use crate::sbi::shutdown;
// 导入 panic 信息类型
use core::panic::PanicInfo;

/// Panic 处理函数
/// 
/// 当程序发生 unrecoverable error（不可恢复的错误）时，会调用这个函数
/// 例如：访问越界、断言失败、调用 panic! 宏等
/// 
/// #[panic_handler] 属性告诉编译器这是 panic 处理函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // 获取 panic 消息，如果没有消息则使用默认值
    let msg = info.message().as_str().unwrap_or("(no message)");

    // 检查是否有位置信息（文件名和行号）
    if let Some(location) = info.location() {
        // 打印详细的 panic 信息：文件名、行号、消息
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            msg
        );
    } else {
        // 如果没有位置信息，只打印消息
        println!("Panicked: {}", msg);
    }

    // 关机
    shutdown()
}

/* Panic 处理流程：
 * 
 * 1. 程序执行过程中发生错误（如 array[10] 但数组只有 5 个元素）
 * 2. Rust 运行时调用 panic! 宏
 * 3. panic! 宏调用我们定义的 panic 处理函数
 * 4. 我们的 panic 函数打印错误信息
 * 5. 最后调用 shutdown() 关闭系统
 * 
 * 在标准库环境中，panic 会导致程序崩溃并打印堆栈跟踪
 * 在裸机环境中，我们只能简单地打印信息并关机
 */