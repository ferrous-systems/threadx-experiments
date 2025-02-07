//! Build script for the Rust/ThreadX demo

// SPDX-FileCopyrightText: Copyright (c) 2023 Ferrous Systems
// SPDX-License-Identifier: MIT OR Apache-2.0

use std::{env, error::Error, io::Write, path::PathBuf};

static TX_PORT_FILES: &[&str] = &[
    "tx_thread_context_restore.S",
    "tx_thread_fiq_nesting_end.S",
    "tx_thread_interrupt_restore.S",
    "tx_thread_stack_build.S",
    "tx_thread_context_save.S",
    "tx_thread_fiq_nesting_start.S",
    "tx_thread_irq_nesting_end.S",
    "tx_thread_system_return.S",
    "tx_thread_fiq_context_restore.S",
    "tx_thread_interrupt_control.S",
    "tx_thread_irq_nesting_start.S",
    "tx_thread_vectored_context_save.S",
    "tx_thread_fiq_context_save.S",
    "tx_thread_interrupt_disable.S",
    "tx_thread_schedule.S",
    "tx_timer_interrupt.S",
];

static TX_COMMON_FILES: &[&str] = &[
    "tx_block_allocate.c",
    "tx_block_pool_cleanup.c",
    "tx_block_pool_create.c",
    "tx_block_pool_delete.c",
    "tx_block_pool_info_get.c",
    "tx_block_pool_initialize.c",
    "tx_block_pool_performance_info_get.c",
    "tx_block_pool_performance_system_info_get.c",
    "tx_block_pool_prioritize.c",
    "tx_block_release.c",
    "tx_byte_allocate.c",
    "tx_byte_pool_cleanup.c",
    "tx_byte_pool_create.c",
    "tx_byte_pool_delete.c",
    "tx_byte_pool_info_get.c",
    "tx_byte_pool_initialize.c",
    "tx_byte_pool_performance_info_get.c",
    "tx_byte_pool_performance_system_info_get.c",
    "tx_byte_pool_prioritize.c",
    "tx_byte_pool_search.c",
    "tx_byte_release.c",
    "tx_event_flags_cleanup.c",
    "tx_event_flags_create.c",
    "tx_event_flags_delete.c",
    "tx_event_flags_get.c",
    "tx_event_flags_info_get.c",
    "tx_event_flags_initialize.c",
    "tx_event_flags_performance_info_get.c",
    "tx_event_flags_performance_system_info_get.c",
    "tx_event_flags_set.c",
    "tx_event_flags_set_notify.c",
    "tx_initialize_high_level.c",
    "tx_initialize_kernel_enter.c",
    "tx_initialize_kernel_setup.c",
    "tx_mutex_cleanup.c",
    "tx_mutex_create.c",
    "tx_mutex_delete.c",
    "tx_mutex_get.c",
    "tx_mutex_info_get.c",
    "tx_mutex_initialize.c",
    "tx_mutex_performance_info_get.c",
    "tx_mutex_performance_system_info_get.c",
    "tx_mutex_prioritize.c",
    "tx_mutex_priority_change.c",
    "tx_mutex_put.c",
    "tx_queue_cleanup.c",
    "tx_queue_create.c",
    "tx_queue_delete.c",
    "tx_queue_flush.c",
    "tx_queue_front_send.c",
    "tx_queue_info_get.c",
    "tx_queue_initialize.c",
    "tx_queue_performance_info_get.c",
    "tx_queue_performance_system_info_get.c",
    "tx_queue_prioritize.c",
    "tx_queue_receive.c",
    "tx_queue_send.c",
    "tx_queue_send_notify.c",
    "tx_semaphore_ceiling_put.c",
    "tx_semaphore_cleanup.c",
    "tx_semaphore_create.c",
    "tx_semaphore_delete.c",
    "tx_semaphore_get.c",
    "tx_semaphore_info_get.c",
    "tx_semaphore_initialize.c",
    "tx_semaphore_performance_info_get.c",
    "tx_semaphore_performance_system_info_get.c",
    "tx_semaphore_prioritize.c",
    "tx_semaphore_put.c",
    "tx_semaphore_put_notify.c",
    "tx_thread_create.c",
    "tx_thread_delete.c",
    "tx_thread_entry_exit_notify.c",
    "tx_thread_identify.c",
    "tx_thread_info_get.c",
    "tx_thread_initialize.c",
    "tx_thread_performance_info_get.c",
    "tx_thread_performance_system_info_get.c",
    "tx_thread_preemption_change.c",
    "tx_thread_priority_change.c",
    "tx_thread_relinquish.c",
    "tx_thread_reset.c",
    "tx_thread_resume.c",
    "tx_thread_shell_entry.c",
    "tx_thread_sleep.c",
    "tx_thread_stack_analyze.c",
    "tx_thread_stack_error_handler.c",
    "tx_thread_stack_error_notify.c",
    "tx_thread_suspend.c",
    "tx_thread_system_preempt_check.c",
    "tx_thread_system_resume.c",
    "tx_thread_system_suspend.c",
    "tx_thread_terminate.c",
    "tx_thread_time_slice.c",
    "tx_thread_time_slice_change.c",
    "tx_thread_timeout.c",
    "tx_thread_wait_abort.c",
    "tx_time_get.c",
    "tx_time_set.c",
    "tx_timer_activate.c",
    "tx_timer_change.c",
    "tx_timer_create.c",
    "tx_timer_deactivate.c",
    "tx_timer_delete.c",
    "tx_timer_expiration_process.c",
    "tx_timer_info_get.c",
    "tx_timer_initialize.c",
    "tx_timer_performance_info_get.c",
    "tx_timer_performance_system_info_get.c",
    "tx_timer_system_activate.c",
    "tx_timer_system_deactivate.c",
    "tx_timer_thread_entry.c",
    "tx_trace_buffer_full_notify.c",
    "tx_trace_enable.c",
    "tx_trace_event_filter.c",
    "tx_trace_event_unfilter.c",
    "tx_trace_disable.c",
    "tx_trace_initialize.c",
    "tx_trace_interrupt_control.c",
    "tx_trace_isr_enter_insert.c",
    "tx_trace_isr_exit_insert.c",
    "tx_trace_object_register.c",
    "tx_trace_object_unregister.c",
    "tx_trace_user_event_insert.c",
    "txe_block_allocate.c",
    "txe_block_pool_create.c",
    "txe_block_pool_delete.c",
    "txe_block_pool_info_get.c",
    "txe_block_pool_prioritize.c",
    "txe_block_release.c",
    "txe_byte_allocate.c",
    "txe_byte_pool_create.c",
    "txe_byte_pool_delete.c",
    "txe_byte_pool_info_get.c",
    "txe_byte_pool_prioritize.c",
    "txe_byte_release.c",
    "txe_event_flags_create.c",
    "txe_event_flags_delete.c",
    "txe_event_flags_get.c",
    "txe_event_flags_info_get.c",
    "txe_event_flags_set.c",
    "txe_event_flags_set_notify.c",
    "txe_mutex_create.c",
    "txe_mutex_delete.c",
    "txe_mutex_get.c",
    "txe_mutex_info_get.c",
    "txe_mutex_prioritize.c",
    "txe_mutex_put.c",
    "txe_queue_create.c",
    "txe_queue_delete.c",
    "txe_queue_flush.c",
    "txe_queue_front_send.c",
    "txe_queue_info_get.c",
    "txe_queue_prioritize.c",
    "txe_queue_receive.c",
    "txe_queue_send.c",
    "txe_queue_send_notify.c",
    "txe_semaphore_ceiling_put.c",
    "txe_semaphore_create.c",
    "txe_semaphore_delete.c",
    "txe_semaphore_get.c",
    "txe_semaphore_info_get.c",
    "txe_semaphore_prioritize.c",
    "txe_semaphore_put.c",
    "txe_semaphore_put_notify.c",
    "txe_thread_create.c",
    "txe_thread_delete.c",
    "txe_thread_entry_exit_notify.c",
    "txe_thread_info_get.c",
    "txe_thread_preemption_change.c",
    "txe_thread_priority_change.c",
    "txe_thread_relinquish.c",
    "txe_thread_reset.c",
    "txe_thread_resume.c",
    "txe_thread_suspend.c",
    "txe_thread_terminate.c",
    "txe_thread_time_slice_change.c",
    "txe_thread_wait_abort.c",
    "txe_timer_activate.c",
    "txe_timer_change.c",
    "txe_timer_create.c",
    "txe_timer_deactivate.c",
    "txe_timer_delete.c",
    "txe_timer_info_get.c",
];

fn main() -> Result<(), Box<dyn Error>> {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);

    arm_targets::process();

    match std::env::var("TARGET").expect("TARGET not set").as_str() {
        "armv8r-none-eabihf" => {
            write("memory.x", include_bytes!("mps3-an536.ld"));
        }
        _ => {
            write("memory.x", include_bytes!("versatileab.ld"));
        }
    }
    // Use the cortex-m-rt linker script
    println!("cargo:rustc-link-arg=-Tlink.x");

    // Build our ThreadX static library
    let tx_common_dir = crate_dir.join("../threadx/common/src");
    let tx_common_inc = crate_dir.join("../threadx/common/inc");
    let tx_port_dir = crate_dir.join("../threadx/ports/cortex_r5/gnu/src");
    let tx_port_inc = crate_dir.join("../threadx/ports/cortex_r5/gnu/inc");
    cc::Build::new()
        .include(&tx_common_inc)
        .include(&tx_port_inc)
        .flag("-g")
        .define("TX_ENABLE_VFP_SUPPORT", "1")
        .files(TX_PORT_FILES.iter().map(|&s| tx_port_dir.join(s)))
        .files(TX_COMMON_FILES.iter().map(|&s| tx_common_dir.join(s)))
        .compile("threadx");

    Ok(())
}

fn write(file: &str, contents: &[u8]) {
    // Put linker file in our output directory and ensure it's on the
    // linker search path.
    let out = &std::path::PathBuf::from(std::env::var_os("OUT_DIR").unwrap());
    std::fs::File::create(out.join(file))
        .unwrap()
        .write_all(contents)
        .unwrap();
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed={}", file);
}
