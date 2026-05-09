#![no_std]
#![no_main]

//! 一个最小化的 `no_std`/`no_main` 入口程序。
//! - 通过 RISC-V `ecall` 发起系统调用（当前实现了 write/exit）。
//! - 将系统调用封装成 `core::fmt::Write`，以便复用 `write_fmt` 做格式化输出。
//! - 提供 `_start` 作为入口点，打印一行文字后退出。

use core::panic::PanicInfo;

/// RISC-V/Linux ABI：`write` 系统调用号。
const SYSCALL_WRITE: usize = 64;

/// 对应 `write(fd, buf, len)`：将 `buffer` 写入指定文件描述符 `fd`。
pub fn sys_write(fd: usize, buffer: &[u8]) -> isize {
  syscall(SYSCALL_WRITE, [fd, buffer.as_ptr() as usize, buffer.len()])
}

/// 标准输出适配器：把 `core::fmt` 的输出落到 `sys_write(1, ..)` 上。
struct Stdout;

impl Write for Stdout {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        sys_write(1, s.as_bytes());
        Ok(())
    }
}

/// 打印格式化参数到标准输出。
///
/// 在 `no_std` 环境下无法直接使用 `std::print!`，这里借助 `core::fmt` 完成格式化。
pub fn print(args: fmt::Arguments) {
    Stdout.write_fmt(args).unwrap();
}

use core::fmt::{self, Write};

#[macro_export]
/// 格式化输出宏（对齐 `print!` 的使用体验）。
///
/// 注意：该宏按 `$crate::console::print` 路径调用，通常用于将打印实现放入 `console` 模块；
/// 当前文件内直接调用的是上面的 `print` 函数（例如 `println!` 宏内部）。
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    }
}

#[macro_export]
/// 带换行的格式化输出宏。
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    }
}

#[panic_handler]
/// `no_std` 环境的 panic 处理：发生 panic 后停在死循环里。
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use core::arch::asm;

/// RISC-V/Linux ABI：`exit` 系统调用号。
const SYSCALL_EXIT: usize = 93;

/// 通用三参数系统调用封装。
///
/// 寄存器约定（RISC-V Linux）：
/// - a0(x10)、a1(x11)、a2(x12)：最多三个参数
/// - a7(x17)：系统调用号
/// - 返回值放在 a0(x10)
fn syscall(id: usize, args: [usize; 3]) -> isize {
    let mut ret: isize;
    unsafe {
        asm!("ecall",
             in("x10") args[0],
             in("x11") args[1],
             in("x12") args[2],
             in("x17") id,
             lateout("x10") ret
        );
    }
    ret
}

/// 对应 `exit(xstate)`：请求内核结束当前程序，并将 `xstate` 作为退出码。
pub fn sys_exit(xstate: i32) -> isize {
    syscall(SYSCALL_EXIT, [xstate as usize, 0, 0])
}

#[no_mangle]
/// 裸入口点：在 `no_main` 下由链接器/启动代码直接跳转到这里。
extern "C" fn _start() {
    println!("Hello, OS! --23301036");
    sys_exit(9);
}
