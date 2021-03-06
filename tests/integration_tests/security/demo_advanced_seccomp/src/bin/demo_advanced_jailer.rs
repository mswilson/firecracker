// Copyright 2018 Amazon.com, Inc. or its affiliates. All Rights Reserved.
// SPDX-License-Identifier: Apache-2.0

extern crate libc;
extern crate seccomp;

use std::env::args;
use std::os::unix::process::CommandExt;
use std::process::{Command, Stdio};

/// Returns a list of rules that allow syscalls required for running a rust program.
fn rust_required_rules() -> Vec<(i64, Vec<seccomp::SeccompRule>)> {
    vec![
        (
            libc::SYS_sigaltstack,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_munmap,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_exit_group,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
    ]
}

/// Returns a list of rules that allow syscalls required for executing another program.
fn jailer_required_rules() -> Vec<(i64, Vec<seccomp::SeccompRule>)> {
    vec![
        (
            libc::SYS_rt_sigprocmask,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_rt_sigaction,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_execve,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_mmap,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_arch_prctl,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_set_tid_address,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_readlink,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_open,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_read,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_close,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_brk,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
        (
            libc::SYS_sched_getaffinity,
            vec![seccomp::SeccompRule::new(
                vec![],
                seccomp::SeccompAction::Allow,
            )],
        ),
    ]
}

fn main() {
    let args: Vec<String> = args().collect();
    let exec_file = &args[1];
    let mut context = seccomp::SeccompFilterContext::new(
        vec![].into_iter().collect(),
        seccomp::SeccompAction::Trap,
    ).unwrap();

    // Adds required rules.
    rust_required_rules()
        .into_iter()
        .try_for_each(|(syscall_number, rules)| context.add_rules(syscall_number, None, rules))
        .unwrap();

    jailer_required_rules()
        .into_iter()
        .try_for_each(|(syscall_number, rules)| context.add_rules(syscall_number, None, rules))
        .unwrap();

    // Adds rule to allow the harmless demo Firecracker.
    context.add_rules(
        libc::SYS_write,
        None,
        vec![seccomp::SeccompRule::new(
            vec![
                seccomp::SeccompCondition::new(
                    0,
                    seccomp::SeccompCmpOp::Eq,
                    libc::STDOUT_FILENO as u64,
                ).unwrap(),
                seccomp::SeccompCondition::new(2, seccomp::SeccompCmpOp::Eq, 14).unwrap(),
            ],
            seccomp::SeccompAction::Allow,
        )],
    ).unwrap();

    // Loads filters generated by the context.
    seccomp::setup_seccomp(seccomp::SeccompLevel::Advanced(context)).unwrap();

    Command::new(exec_file)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .exec();
}
