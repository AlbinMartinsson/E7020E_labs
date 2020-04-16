//! bare1.rs
//!
//! Inspecting the generated assembly
//!
//! What it covers
//! - Rust panic tracing using ITM
//! - assembly calls and inline assembly
//! - more on arithmetics

#![no_main]
#![no_std]

use panic_itm as _;

use cortex_m_rt::entry;

#[entry]
#[inline(never)]
fn main() -> ! {
    let mut x = core::u32::MAX - 1;
    loop {
        cortex_m::asm::bkpt();
        x += 1;
        cortex_m::asm::bkpt();

        // prevent optimization by read-volatile (unsafe)
        unsafe {
            core::ptr::read_volatile(&x);
        }
    }
}

// 0. Setup
//    For this example we will use the `nightly` compiler
//    to get inline assembly.
//    (Inline assembly is currently not stabilized.)
//
//    > rustup override set nightly
//
//    In the `Cargo.toml` file, uncomment
//    # features = ["inline-asm"] # <- currently requires nightly compiler
//
//    The first time you use the new toolchain you may need to install the target.
//    > rustup target add thumbv7em-none-eabihf
//
//    You may need/want to install additional components also.
//    To that end look at the install section in the README.md.
//    (If you change toolchain, you may need to exit and re-start `vscode`.)
//
// 1. Build and run the application
//
//    > cargo run --example bare1
//    (or use the `itm fifo (debug)` or the `itm internal (debug)` launch configuration.)
//
//    Make sure you have followed the instructions for fifo `ITM` tracing accordingly.
//
//    When debugging the application it should hit the `bkpt` instruction.
//    What happens when you continue (second iteration of the loop)?
//    (passing 3 breakpoints)
//
//    It panics because it tries to add 1 to a u32 max, causing an overflow.
//
//    What is the `ITM` output.
//
//    panicked at 'attempt to add with overflow', examples/bare1.rs:23:9
//
//    Commit your answer (bare1_1)
//
// 2. Inspecting the generated assembly code.
//    Close and re-start the debug session. Run till you hit the `bkpt` instruction.
//
//    Under DEBUG CONSOLE you find the `gdb` interface.
//
//    What is the output of:
//    > disassemble
//
//    0x0800144c <+0>:	push	{r7, lr}
//    0x0800144e <+2>:	mov	r7, sp
//    0x08001450 <+4>:	sub	sp, #80	; 0x50
//    0x08001452 <+6>:	str	r0, [sp, #24]
//    0x08001454 <+8>:	bl	0x80021c2 <cortex_m::interrupt::disable>
//    0x08001458 <+12>:	b.n	0x800145a <rust_begin_unwind+14>
//    0x0800145a <+14>:	b.n	0x800145c <rust_begin_unwind+16>
//    0x0800145c <+16>:	mov.w	r0, #3758096384	; 0xe0000000
//    0x08001460 <+20>:	str	r0, [sp, #68]	; 0x44
//    0x08001462 <+22>:	mov	r1, r0
//    0x08001464 <+24>:	str	r0, [sp, #72]	; 0x48
//    0x08001466 <+26>:	movw	r0, #10484	; 0x28f4
// 0x08001470 <+36>:	add	r2, sp, #24
// 0x08001472 <+38>:	str	r2, [sp, #60]	; 0x3c
// 0x08001474 <+40>:	ldr	r2, [sp, #60]	; 0x3c
// 0x08001476 <+42>:	str	r2, [sp, #76]	; 0x4c
// 0x08001478 <+44>:	movw	r3, #5325	; 0x14cd
// 0x0800147c <+48>:	movt	r3, #2048	; 0x800
// 0x08001480 <+52>:	str	r0, [sp, #20]
// 0x08001482 <+54>:	mov	r0, r2
// 0x08001484 <+56>:	str	r1, [sp, #16]
// 0x08001486 <+58>:	mov	r1, r3
// 0x08001488 <+60>:	bl	0x80014ea <core::fmt::ArgumentV1::new>
// 0x0800148c <+64>:	str	r0, [sp, #12]
// 0x0800148e <+66>:	str	r1, [sp, #8]
// 0x08001490 <+68>:	b.n	0x8001492 <rust_begin_unwind+70>
 // 0x08001492 <+70>:	ldr	r0, [sp, #12]
// 0x08001494 <+72>:	str	r0, [sp, #52]	; 0x34
// 0x08001496 <+74>:	ldr	r1, [sp, #8]
//    0x08001498 <+76>:	str	r1, [sp, #56]	; 0x38
//    0x0800149a <+78>:	mov	r2, sp
//    0x0800149c <+80>:	movs	r3, #1
//    0x0800149e <+82>:	str	r3, [r2, #0]
//    0x080014a0 <+84>:	add	r0, sp, #28
//    0x080014a2 <+86>:	movs	r2, #2
//    0x080014a4 <+88>:	add	r3, sp, #52	; 0x34
//    0x080014a6 <+90>:	ldr	r1, [sp, #20]
//    0x080014a8 <+92>:	bl	0x800153e <core::fmt::Arguments::new_v1>
//    0x080014ac <+96>:	b.n	0x80014ae <rust_begin_unwind+98>
//    0x080014ae <+98>:	add	r1, sp, #28
//    0x080014b0 <+100>:	ldr	r0, [sp, #16]
//    0x080014b2 <+102>:	bl	0x80016da <cortex_m::itm::write_fmt>
//    0x080014b6 <+106>:	b.n	0x80014b8 <rust_begin_unwind+108>
//    0x080014b8 <+108>:	b.n	0x80014ba <rust_begin_unwind+110>
//    0x080014ba <+110>:	movs	r0, #4
//    0x080014bc <+112>:	strb.w	r0, [r7, #-13]
//    0x080014c0 <+116>:	ldrb.w	r0, [r7, #-13]
//    0x080014c4 <+120>:	bl	0x8001578 <core::sync::atomic::compiler_fence>
// => 0x080014c8 <+124>:	b.n	0x80014ca <rust_begin_unwind+126>
//    0x080014ca <+126>:	b.n	0x80014ba <rust_begin_unwind+110>
//
//    How many instructions are in between the two `bkpt` instructions in the loop.
//    Notice, the generated code may not be exactly what you expect :)
//
//     18 insructions between the two breakpoints.
//
//    Which instruction stores the local variable on the stack.
//
//      0x08001484 <+56>:	str	r1, [sp, #16]
//
//    Commit your answers (bare1_2)
//
// 3. Release mode (optimized builds).
//    Rebuild `bare1.rs` in release (optimized mode).
//
//    > cargo build --example bare1 --release
//    (or using the vscode)
//
//    Compare the generated assembly for the loop
//    between the dev (un-optimized) and release (optimized) build.
//
//    What is the output of:
//    > disassemble
//    0x0800040a <+0>:	push	{r7, lr}
//    0x0800040c <+2>:	mov	r7, sp
//    0x0800040e <+4>:	sub	sp, #16
//    0x08000410 <+6>:	mvn.w	r0, #1
//    0x08000414 <+10>:	str	r0, [sp, #8]
//    0x08000416 <+12>:	movs	r0, #0
//    0x08000418 <+14>:	strb.w	r0, [sp, #12]
//    0x0800041c <+18>:	ldr	r0, [sp, #8]
//    0x0800041e <+20>:	str	r0, [sp, #4]
//    0x08000420 <+22>:	b.n	0x8000422 <bare1::__cortex_m_rt_main+24>
// => 0x08000422 <+24>:	bkpt	0x0000
// 0x08000428 <+30>:	adds	r1, r0, #1
// 0x0800042a <+32>:	mov	r2, r1
// 0x0800042c <+34>:	cmp	r1, r0
// 0x0800042e <+36>:	str	r2, [sp, #0]
// 0x08000430 <+38>:	bcc.n	0x8000446 <bare1::__cortex_m_rt_main+60>
// 0x08000432 <+40>:	b.n	0x8000434 <bare1::__cortex_m_rt_main+42>
// 0x08000434 <+42>:	ldr	r0, [sp, #0]
// 0x08000436 <+44>:	str	r0, [sp, #4]
// 0x08000438 <+46>:	bkpt	0x0000
// 0x0800043a <+48>:	b.n	0x800043c <bare1::__cortex_m_rt_main+50>
// 0x0800043c <+50>:	add	r0, sp, #4
// 0x0800043e <+52>:	bl	0x800045e <core::ptr::read_volatile>
// 0x08000442 <+56>:	b.n	0x8000444 <bare1::__cortex_m_rt_main+58>
// 0x08000444 <+58>:	b.n	0x8000422 <bare1::__cortex_m_rt_main+24>
// 0x08000446 <+60>:	movw	r0, #9888	; 0x26a0
// 0x0800044a <+64>:	movt	r0, #2048	; 0x800
// 0x0800044e <+68>:	movw	r2, #9860	; 0x2684
// 0x08000452 <+72>:	movt	r2, #2048	; 0x800
// 0x08000456 <+76>:	movs	r1, #28
// 0x08000458 <+78>:	bl	0x8000934 <core::panicking::panic>
// 0x0800045c <+82>:	udf	#254	; 0xfe
//
//  
//
//    How many instructions are in between the two `bkpt` instructions.
//
//    9
//
//    Where is the local variable stored?
//
//    0x08000436 <+44>:	str	r0, [sp, #4]
//
//    Is there now any reference to the panic handler?
//    If not, why is that the case?
//
//    No reference to the panic handler.
//
//    commit your answers (bare1_3)
//
//    Discussion:
//    In release (optimized) mode the addition is unchecked,
//    so there is a semantic difference here in between
//    the dev and release modes. This is motivated by:
//    1) efficiency, unchecked is faster
//    2) convenience, it would be inconvenient to explicitly use
//    wrapping arithmetics, and wrapping is what the programmer
//    typically would expect in any case. So the check
//    in dev/debug mode is just there for some extra safety
//    if your intention is NON-wrapping arithmetics.
//
//    The debug build should have additional code that checks if the addition
//    wraps (and in such case call panic). In the case of the optimized
//    build there should be no reference to the panic handler in the generated
//    binary. Recovering from a panic is in general very hard. Typically
//    the best we can do is to stop and report the error (and maybe restart).
//
//    Later we will demonstrate how we can get guarantees of panic free execution.
//    This is very important to improve reliability.
//
// 4. Now comment out the `read_volatile`.
//
//    > cargo build --example bare1 --release
//    (or using the vscode)
//
//    Compare the generated assembly for the loop
//    between the dev (un-optimized) and release (optimized) build.
//
//    What is the output of:
//    > disassemble
//
//    ** your answer here **
//
//    How many instructions are in between the two `bkpt` instructions.
//
//    ** your answer here **
//
//    Where is the local variable stored?
//    What happened, and why is Rust + LLVM allowed to do that?
//
//    ** your answer here **
//
//    commit your answers (bare1_4)
//
//
// 5. *Optional
//    You can pass additional flags to the Rust `rustc` compiler.
//
//    `-Z force-overflow-checks=off`
//
//    Under this flag, code is never generated for overflow checking even in
//    non optimized (debug/dev) builds.
//    You can enable this flag in the `.cargo/config` file.
//
//    What is now the disassembly of the loop (in debug/dev mode):
//
//    ** your answer here **
//
//    commit your answers (bare1_5)
//
//    Now restore the `.cargo/config` to its original state.
//
// 6. *Optional
//    There is another way to conveniently use wrapping arithmetics
//    without passing flags to the compiler.
//
//    https://doc.rust-lang.org/std/num/struct.Wrapping.html
//
//    Rewrite the code using this approach.
//
//    What is now the disassembly of the code in dev mode?
//
//    ** your answer here **
//
//    What is now the disassembly of the code in release mode?
//
//    ** your answer here **
//
//    commit your answers (bare1_6)
//
//    Final discussion:
//
//    Embedded code typically is performance sensitive, hence
//    it is important to understand how code is generated
//    to achieve efficient implementations.
//
//    Moreover, arithmetics are key to processing of data,
//    so its important that we are in control over the
//    computations. E.g. computing checksums, hashes, cryptos etc.
//    all require precise control over wrapping vs. overflow behavior.
//
//    If you write a library depending on wrapping arithmetics
//    do NOT rely on a compiler flag. (The end user might compile
//    it without this flag enabled, and thus get erroneous results.)
//
//    NOTICE:
//    ------
//    You are now on a `nightly` release of the compiler for good and bad.
//    You can chose to switch back to the stable channel. If so you must
//    restore the `Cargo.toml` (comment out the `features = ["inline-asm"]`)
//
//    Pros and cons of nightly:
//    + Access to new Rust features (such as inline assembly)
//    - No guarantee these features will work, they might change semantics,
//      or even be revoked.
//
//    The compiler itself is the same, the stable release is just a snapshot
//    of the nightly (released each 6 week). It is the latest nightly
//    that passed some additional regression test, not a different compiler.
//    And of course, the stable has the experimental features disabled.
//
//    So its up to you to decide if you want to use the stable or nightly.
