// SBI（Supervisor Binary Interface）模块
// 
// SBI 是 RISC-V 架构中，监督者模式（S-mode）程序（如操作系统内核）
// 与机器模式（M-mode）程序（如 OpenSBI）之间的接口
// 
// 简单来说，操作系统通过 SBI 调用向硬件层请求服务
// 就像用户程序通过系统调用向操作系统请求服务一样

// 允许未使用的变量（避免编译警告）
#![allow(unused)]

// 导入汇编功能
use core::arch::asm;

// ========== SBI 调用号定义 ==========
// 每个调用号对应一个特定的硬件操作

const SBI_SET_TIMER: usize = 0;                    // 设置定时器
const SBI_CONSOLE_PUTCHAR: usize = 1;              // 输出一个字符到控制台
const SBI_CONSOLE_GETCHAR: usize = 2;              // 从控制台读取一个字符
const SBI_CLEAR_IPI: usize = 3;                    // 清除中断处理器间中断
const SBI_SEND_IPI: usize = 4;                     // 发送处理器间中断
const SBI_REMOTE_FENCE_I: usize = 5;               // 远程指令栅栏
const SBI_REMOTE_SFENCE_VMA: usize = 6;           // 远程虚拟内存栅栏
const SBI_REMOTE_SFENCE_VMA_ASID: usize = 7;      // 带 ASID 的远程虚拟内存栅栏
const SBI_SHUTDOWN: usize = 8;                     // 关闭系统

/// 通用 SBI 调用函数
/// 
/// 在 RISC-V 中，通过 ecall 指令发起 SBI 调用
/// 参数传递约定：
/// - a0(x10): 第一个参数
/// - a1(x11): 第二个参数  
/// - a2(x12): 第三个参数
/// - a7(x17): SBI 调用号
/// - 返回值放在 a0(x10)
#[inline(always)]  // 总是内联，减少函数调用开销
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;  // 存储返回值
    
    // 使用内联汇编执行 ecall 指令
    unsafe {
        asm!(
            "ecall",           // 发起 SBI 调用
            in("x10") arg0,    // 第一个参数放入 x10
            in("x11") arg1,    // 第二个参数放入 x11
            in("x12") arg2,    // 第三个参数放入 x12
            in("x17") which,   // 调用号放入 x17
            lateout("x10") ret // 返回值从 x10 取出
        );
    }
    ret  // 返回结果
}

/// 向控制台输出一个字符
pub fn console_putchar(c: usize) {
    sbi_call(SBI_CONSOLE_PUTCHAR, c, 0, 0);
}

/// 从控制台读取一个字符
pub fn console_getchar() -> usize {
    sbi_call(SBI_CONSOLE_GETCHAR, 0, 0, 0)
}

/// 关闭系统（关机）
/// 
/// -> ! 表示这个函数永远不会返回
pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");  // 如果返回了，说明出错了
}

/* SBI 调用流程说明：
 * 
 * 1. 操作系统内核（S-mode）执行 ecall 指令
 * 2. CPU 切换到 M-mode（机器模式）
 * 3. OpenSBI（M-mode 固件）根据 a7 中的调用号执行相应操作
 * 4. OpenSBI 将结果放入 a0，然后返回到 S-mode
 * 5. 操作系统内核从 a0 获取返回值，继续执行
 * 
 * 这个机制类似于用户程序调用系统调用，只是层次更深
 */