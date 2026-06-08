// Rust implementation of interrupts.d
// Provides interrupt and exception flag management

use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use crate::atomic::{AtomicField, compiler_barrier};

thread_local! {
    static INTERRUPT_SHIELD: AtomicBool = AtomicBool::new(false);
    static INTERRUPT_PENDING: AtomicBool = AtomicBool::new(false);
    static ALARMED_FLAG: AtomicBool = AtomicBool::new(false);
    static STEPPING_FLAG: AtomicBool = AtomicBool::new(false);
    static STEP_COUNT: AtomicI32 = AtomicI32::new(-1);
    static MICRO_STEP_COUNT: AtomicI32 = AtomicI32::new(-1);
}

// These will be imported from C code
extern "C" {
    static THREADLOCAL_interrupted_flag: *mut AtomicField;
    static THREADLOCAL_exception_flag: *mut AtomicField;
}

pub fn interrupt_shield() -> bool {
    INTERRUPT_SHIELD.with(|flag| flag.load(Ordering::SeqCst))
}

pub fn set_interrupt_shield(value: bool) {
    INTERRUPT_SHIELD.with(|flag| flag.store(value, Ordering::SeqCst));
}

pub fn interrupt_pending() -> bool {
    INTERRUPT_PENDING.with(|flag| flag.load(Ordering::SeqCst))
}

pub fn set_interrupt_pending(value: bool) {
    INTERRUPT_PENDING.with(|flag| flag.store(value, Ordering::SeqCst));
}

pub fn alarmed_flag() -> bool {
    ALARMED_FLAG.with(|flag| flag.load(Ordering::SeqCst))
}

pub fn set_alarmed_flag(value: bool) {
    ALARMED_FLAG.with(|flag| flag.store(value, Ordering::SeqCst));
}

pub fn stepping_flag() -> bool {
    STEPPING_FLAG.with(|flag| flag.load(Ordering::SeqCst))
}

pub fn set_stepping_flag(value: bool) {
    STEPPING_FLAG.with(|flag| flag.store(value, Ordering::SeqCst));
}

pub fn step_count() -> i32 {
    STEP_COUNT.with(|count| count.load(Ordering::SeqCst))
}

pub fn set_step_count(value: i32) {
    STEP_COUNT.with(|count| count.store(value, Ordering::SeqCst));
}

pub fn micro_step_count() -> i32 {
    MICRO_STEP_COUNT.with(|count| count.load(Ordering::SeqCst))
}

pub fn set_micro_step_count(value: i32) {
    MICRO_STEP_COUNT.with(|count| count.store(value, Ordering::SeqCst));
}

pub fn determine_exception_flag() {
    unsafe {
        let interrupted = (*THREADLOCAL_interrupted_flag).test();
        let stepping = stepping_flag();
        let alarmed = alarmed_flag();
        let flag = interrupted || stepping || alarmed;
        (*THREADLOCAL_exception_flag).store(if flag { 1 } else { 0 });
    }
}

pub fn alarm(seconds: u32) -> i32 {
    unsafe { libc::alarm(seconds) as i32 }
}

pub fn clear_alarm() {
    alarm(0);
}

pub fn clear_all_flags() {
    unsafe {
        (*THREADLOCAL_exception_flag).store(0);
        compiler_barrier();
        (*THREADLOCAL_interrupted_flag).store(0);
    }
    set_stepping_flag(false);
    set_alarmed_flag(false);
    set_interrupt_pending(false);
}

pub fn set_interrupt_flag() {
    // Order matters: interrupt flag, then exception flag
    unsafe {
        (*THREADLOCAL_interrupted_flag).store(1);
        compiler_barrier();
        (*THREADLOCAL_exception_flag).store(1);
    }
}

pub fn set_alarmed_flag_internal() {
    unsafe {
        (*THREADLOCAL_interrupted_flag).store(1);
    }
    set_alarmed_flag(true);
    unsafe {
        (*THREADLOCAL_exception_flag).store(1);
    }
}

pub fn set_stepping_flag_internal() {
    set_stepping_flag(true);
    unsafe {
        (*THREADLOCAL_exception_flag).store(1);
    }
}

pub fn clear_interrupt_flag() {
    unsafe {
        (*THREADLOCAL_interrupted_flag).store(0);
        compiler_barrier();
    }
    determine_exception_flag();
}

pub fn clear_alarmed_flag_internal() {
    unsafe {
        (*THREADLOCAL_interrupted_flag).store(0);
    }
    set_alarmed_flag(false);
    determine_exception_flag();
}

pub fn clear_stepping_flag() {
    set_step_count(-1);
    set_micro_step_count(-1);
    set_stepping_flag(false);
    determine_exception_flag();
}
