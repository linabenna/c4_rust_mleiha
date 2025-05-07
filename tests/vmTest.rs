// tests/vm_test.rs
const IMM: i64 = 1;
const PSH: i64 = 13;
const ADD_OP: i64 = 25; 
const EXIT: i64 = 38;


#[test]
fn test_vm_imm_and_psh() {
    let bytecode = vec![
        IMM, 42, // ax = 42
        PSH,    // push ax (42) onto stack
        IMM, 10, // ax = 10
        PSH,    // push ax (10) onto stack
        EXIT, 0 // exit with code 0 (ax will be 10 from last IMM)
    ];
    let data_size = 0;
    let stack_size = 100;
    let mut vm = VM::new(bytecode, data_size, stack_size);

    vm.run(); // Assumes vm.run() is pub

    assert_eq!(vm.ax, 10, "AX should be 10 after last IMM");
    // Stack grows downwards from stack_size.
    // After two pushes, sp should be stack_size - 2.
    assert_eq!(vm.sp, stack_size - 2, "Stack pointer incorrect");
    assert_eq!(vm.stack[stack_size - 1], 42, "First pushed value (42) incorrect");
    assert_eq!(vm.stack[stack_size - 2], 10, "Second pushed value (10) incorrect");
    assert_eq!(vm.running, false, "VM should not be running after EXIT");
}

#[test]
fn test_vm_add_operation() {
    let bytecode = vec![
        IMM, 5,     // ax = 5
        PSH,        // push 5
        IMM, 7,     // ax = 7 (this value will be the right-hand side of ADD)
        ADD_OP,     // ax = stack.pop() + ax = 5 + 7 = 12
        EXIT, 0     // exit
    ];
    let mut vm = VM::new(bytecode, 0, 100);
    vm.run();

    assert_eq!(vm.ax, 12, "AX after ADD should be 12");
    // ADD pops one value, sp should be back to where it was before the first PSH if stack was empty
    // After PSH, sp = stack_size - 1. After ADD, sp = stack_size.
    assert_eq!(vm.sp, 100, "Stack pointer after ADD incorrect");
    assert_eq!(vm.running, false, "VM should not be running after EXIT");
}

#[test]
fn test_vm_jmp() {
    let bytecode = vec![
        IMM, 1, // ax = 1
        JMP, 4, // Jump to instruction at index 4
        IMM, 99,// This should be skipped, ax = 99
        PSH,   // This should be skipped
        IMM, 42,// ax = 42 (target of JMP)
        EXIT, 0
    ];
    let mut vm = VM::new(bytecode, 0, 100);
    vm.run();
    assert_eq!(vm.ax, 42, "AX should be 42 after JMP");
}

#[test]
fn test_vm_bz_branch_taken() { // Branch if Zero
    let bytecode = vec![
        IMM, 0,  // ax = 0 (condition for BZ)
        BZ, 4,   // Branch to index 4 because ax is 0
        IMM, 99, // Skipped
        PSH,     // Skipped
        IMM, 10, // Target of branch, ax = 10
        EXIT, 0
    ];
    let mut vm = VM::new(bytecode, 0, 100);
    vm.run();
    assert_eq!(vm.ax, 10, "AX after BZ (taken) should be 10");
}

#[test]
fn test_vm_bz_branch_not_taken() {
    let bytecode = vec![
        IMM, 1,  // ax = 1 (condition for BZ)
        BZ, 5,   // Branch to index 5, NOT taken because ax is not 0
        IMM, 20, // ax = 20 (this is executed)
        PSH,     // push 20
        EXIT, 0  // Exit (target of branch is after this)
        // Index 5: IMM, 99 // Should not be reached
    ];
    let mut vm = VM::new(bytecode, 0, 100);
    vm.run();
    assert_eq!(vm.ax, 20, "AX after BZ (not taken) should be 20");
    assert_eq!(vm.stack[99], 20, "Value 20 should be on stack");
}

