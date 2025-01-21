//! Interrupt handling for targets that comply with the RISC-V interrupt handling standard.
//!
//! In direct mode (i.e., `v-trap` feature disabled), interrupt dispatching is performed by the
//! [`_dispatch_core_interrupt`] function. This function is called by the [crate::start_trap_rust]
//! whenever an interrupt is triggered. This approach relies on the [`__CORE_INTERRUPTS`] array,
//! which sorts all the interrupt handlers depending on their corresponding interrupt source code.
//!
//! In vectored mode (i.e., `v-trap` feature enabled), interrupt dispatching is handled by hardware.
//! To support this mode, we provide inline assembly code that defines the interrupt vector table.
//!
//! # Note
//!
//! If your target has custom core interrupt sources, the target PAC might provide equivalent
//! code to adapt for the target needs. In this case, you may need to opt out this module.
//! To do so, activate the `no-interrupts` feature of the `riscv-rt` crate.

#[cfg(not(feature = "v-trap"))]
extern "C" {
    fn SupervisorSoft();
    fn MachineSoft();
    fn SupervisorTimer();
    fn MachineTimer();
    fn SupervisorExternal();
    fn MachineExternal();
}

/// Array with all the core interrupt handlers sorted according to their interrupt source code.
///
/// # Note
///
/// This array is necessary only in direct mode (i.e., `v-trap` feature disabled).
#[cfg(not(feature = "v-trap"))]
#[no_mangle]
pub static __CORE_INTERRUPTS: [Option<unsafe extern "C" fn()>; 12] = [
    None,
    Some(SupervisorSoft),
    None,
    Some(MachineSoft),
    None,
    Some(SupervisorTimer),
    None,
    Some(MachineTimer),
    None,
    Some(SupervisorExternal),
    None,
    Some(MachineExternal),
];

/// It calls the corresponding interrupt handler depending on the interrupt source code.
///
/// # Note
///
/// This function is only required in direct mode (i.e., `v-trap` feature disabled).
/// In vectored mode, interrupt handler dispatching is performed directly by hardware.
///
/// # Safety
///
/// This function must be called only from the [`crate::start_trap_rust`] function.
/// Do **NOT** call this function directly.
#[cfg(not(feature = "v-trap"))]
#[inline]
#[no_mangle]
pub unsafe extern "C" fn _dispatch_core_interrupt(code: usize) {
    extern "C" {
        fn DefaultHandler();
    }
    match __CORE_INTERRUPTS.get(code) {
        Some(Some(handler)) => handler(),
        _ => DefaultHandler(),
    }
}

// In vectored mode, we also must provide a vector table. This is done with a proc-macro to control
// the alignment constraint from the `RISCV_RT_MTVEC_ALIGN` environment variable.
#[cfg(all(
    any(target_arch = "riscv32", target_arch = "riscv64"),
    feature = "v-trap"
))]
riscv_rt_macros::vector_table!();
