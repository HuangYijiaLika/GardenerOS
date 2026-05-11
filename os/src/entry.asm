# 汇编入口文件
# 
# 这是操作系统启动时执行的第一段代码
# 它负责初始化栈空间，然后跳转到 Rust 主函数

    # 将这段代码放入 .text.entry 段（链接脚本中定义的入口代码段）
    .section .text.entry
    
    # 声明 _start 为全局符号，让链接器能找到它
    .globl _start

# 程序入口点
# OpenSBI 启动后会跳转到这个地址
_start:
    # 设置栈指针（sp 寄存器）
    # la = load address，将 boot_stack_top 的地址加载到 sp
    # 栈是从高地址向低地址增长的，所以栈顶是 boot_stack_top
    la sp, boot_stack_top
    
    # 调用 Rust 主函数
    call rust_main

    # ========== 栈空间定义 ==========
    
    # 将栈空间放入 .bss.stack 段（未初始化数据段的栈部分）
    .section .bss.stack
    
    # 声明 boot_stack 为全局符号
    .globl boot_stack
boot_stack:
    # 分配 4096 * 16 = 64KB 的栈空间
    # .space 指令会预留指定字节数的空间
    .space 4096 * 16
    
    # 声明 boot_stack_top 为全局符号
    .globl boot_stack_top
boot_stack_top:  # 栈顶地址（boot_stack + 64KB）

/* 栈的工作原理：
 * 
 * 栈是一块内存区域，用于存储函数调用时的临时数据（局部变量、返回地址等）
 * 
 * 在 RISC-V 中：
 * - sp 寄存器指向当前栈顶
 * - 栈从高地址向低地址增长
 * - 当调用函数时，sp 会减小（分配栈空间）
 * - 当函数返回时，sp 会增大（释放栈空间）
 * 
 * 本代码中：
 * - boot_stack 是栈的最低地址（栈底）
 * - boot_stack_top 是栈的最高地址（栈顶）
 * - 初始时 sp = boot_stack_top，栈为空
 */