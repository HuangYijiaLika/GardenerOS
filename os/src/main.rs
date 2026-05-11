// 这是操作系统内核的主入口文件

// 声明不使用 Rust 标准库（std），我们要从零开始构建操作系统
#![no_std]
// 声明不使用 Rust 默认的 main 函数作为入口，我们自己定义入口点
#![no_main]
// 允许使用宏导出（用于定义 print! 和 println! 宏）
#[macro_use]

// 导入我们自己实现的模块
mod console;   // 控制台输出模块（提供打印功能）
mod lang_items; // 语言项模块（处理程序崩溃）
mod sbi;       // SBI 调用模块（与硬件交互）

// 导入汇编相关的功能
use core::arch::global_asm;

// 包含汇编入口文件 entry.asm
// 这会在编译时将汇编代码嵌入到最终的二进制文件中
global_asm!(include_str!("entry.asm"));

/// 清空 .bss 段的函数
/// 
/// .bss 段是存放未初始化全局变量的区域
/// 操作系统启动时，这些变量需要被初始化为 0
fn clear_bss() {
    // 声明外部符号，这些符号在 linker.ld 中定义
    extern "C" {
        fn sbss();  // .bss 段的起始地址
        fn ebss();  // .bss 段的结束地址
    }
    // 将 sbss 和 ebss 转换为整数地址
    let sbss_ptr = sbss as *const () as usize;
    let ebss_ptr = ebss as *const () as usize;
    
    // 遍历 .bss 段的每一个字节，将其设置为 0
    // write_volatile 确保编译器不会优化掉这个写入操作
    (sbss_ptr..ebss_ptr).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}

/// Rust 主函数（操作系统的核心入口）
/// 
/// #[no_mangle] 表示不混淆函数名，让链接器能找到这个函数
/// -> ! 表示这个函数永远不会返回（操作系统不会退出）
#[no_mangle]
pub fn rust_main() -> ! {
    // 声明外部符号，这些符号在 linker.ld 中定义，用于获取各个段的地址
    extern "C" {
        fn stext();          // .text 段（代码段）起始
        fn etext();          // .text 段结束
        fn srodata();        // .rodata 段（只读数据）起始
        fn erodata();        // .rodata 段结束
        fn sdata();          // .data 段（已初始化数据）起始
        fn edata();          // .data 段结束
        fn sbss();           // .bss 段（未初始化数据）起始
        fn ebss();           // .bss 段结束
        fn boot_stack();     // 栈空间起始
        fn boot_stack_top(); // 栈空间结束
    }
    
    // 第一步：清空 .bss 段，确保未初始化变量都是 0
    clear_bss();
    
    // 第二步：打印欢迎信息
    println!("Hello, world!");
    
    // 第三步：打印各个内存段的地址范围，帮助我们了解内存布局
    println!(".text [{:#x}, {:#x})", stext as *const () as usize, etext as *const () as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as *const () as usize, erodata as *const () as usize);
    println!(".data [{:#x}, {:#x})", sdata as *const () as usize, edata as *const () as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as *const () as usize, boot_stack_top as *const () as usize
    );
    println!(".bss [{:#x}, {:#x})", sbss as *const () as usize, ebss as *const () as usize);
    
    // 打印学号信息
    println!("Hello, OS!--23301036");
    
    // 第四步：触发 panic，测试异常处理功能并关闭虚拟机
    panic!("Shutdown machine!");
}