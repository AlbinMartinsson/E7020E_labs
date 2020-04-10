//! bare0.rs
//!
//! Simple bare metal application
//! What it covers:
//! - constants
//! - global (static) variables
//! - checked vs. wrapping arithmetics
//! - safe and unsafe code
//! - making a safe API
//! - assertions
//! - panic handling

// build without the Rust standard library
#![no_std]
// no standard main, we declare main using [entry]
#![no_main]

// Panic handler, for textual output using semihosting
use panic_semihosting as _;

// import entry point
use cortex_m_rt::entry;

// a constant (cannot be changed at run-time)
//const X_INIT: u32 = 10;
const X_INIT: u32 = core::u32::MAX;

// global mutable variables (changed using unsafe code)
static mut X: u32 = X_INIT;
static mut Y: u32 = 0;

fn read_x()->u32{
    unsafe{X}
}

fn read_y()->u32{
    unsafe{Y}
}

fn write_x(x:u32){
    unsafe{X = x}
}

fn write_y(y:u32){
    unsafe{Y = y}
}


#[entry]
fn main() -> ! {
    // local mutable variable (changed in safe code)
    let mut x = read_x();

    loop {
        x.wrapping_add(1);
        //x += 1; // <- place breakpoint here (3)
        //Rewritten with safe code 
        write_x(read_x().wrapping_add(1));
        write_y(read_x());
        assert!(x == read_x() && read_x() == read_y() + 1);


      
        // unsafe {

        //     X.wrapping_add(1);
        //     Y = X; 
            //X += 1;
            //assert!(x == X && X == Y);
            //assert!(x == X && X == Y + 1);

        //}
    }
}

// Here we assume you are using `vscode` with `cortex-debug`.
//
// 0. Compile/build and run the example in debug (dev) mode.
//
//    > cargo run --example bare0
//    (or use vscode)
//
// 1. Run the program in the debugger, let the program run for a while and
//    then press pause.
//
//    Look under Variables/Local what do you find.
//
//    x: 3034454
//
//    In the Expressions (WATCH -vscode) view add X and Y
//    what do you find
//
//    X: 3034454
//    Y: 3034454
//
//
//    Step through one complete iteration of the loop
//    and see how the (Local) Variables are updated
//    can you foresee what will eventually happen?
//
//   
// 	   The local variable x started as 2004912, and after one iteration it had incremented by one. 
//     The new value for x is 2004913. 
//
//    Commit your answers (bare0_1)
//
// 2. Alter the constant X_INIT so that `x += 1` directly causes `x` to wrap.
// 	  What happens when `x` wraps
//    (Hint, look under OUTPUT/Adopter Output to see the `openocd` output.)
//
//   The program halts when reaching a panic, the following error was shown in adapter output:
//   "panicked at 'attempt to add with overflow', examples/bare0.rs:39:9". So it tried to add one more to the max
//    value of a U:32 and then it panics.
//
//    Commit your answers (bare0_2)
//
// 3. Place a breakpoint at `x += 1`
//
//    Change (both) += operations to use wrapping_add
//    load and run the program, what happens
//    x assumes the value of 4294967295, which is the max for u32. After the wrapping_add operation 
//    the value is still 4294967295 because wrapping_add takes the max value into account.
//
//    Now continue execution, what happens
//    The program runs without reaching a panic beacuse wrapping_add doesnt cause an overflow.
//
//    Commit your answers (bare0_3)
//
//    (If the program did not succeed back to the breakpoint
//    you have some fault in the program and go back to 3.)
//
// 4. Change the assertion to `assert!(x == X && X == Y + 1)`, what happens?
//
//    It panics beacuse of an overflow, Y = X = x = u32Max, and then if we try to add 1 we get an overflow.
//
//    Commit your answers (bare0_4)
//
// 5. Remove the assertion and implement "safe" functions for
//    reading and writing X and Y
//    e.g. read_x, read_y, write_x, write_y
//
//    Rewrite the program to use ONLY "safe" code besides the
//    read/write functions (which are internally "unsafe")
//
//    Commit your solution (bare0_5)
//
// 6. *Optional
//    Implement a read_u32/write_u32, taking a reference to a
//    "static" variable
//
//    Rewrite the program to use this abstraction instead of "read_x", etc.
//
//    Commit your solution (bare0_6)
//
