# Franklin VM

## Bytecode

[WIP Specification](https://github.com/matter-labs/franklin/wiki/VM-Specification)

## Usage

    let bytes: &[u8] = ...;
    let mut cs: TestConstraintSystem<E> = ...;

    let bytecode = Bytecode::new(bytes);
    let mut vm = VirtualMachine<E, TestConstraintSystem<E>>::new();
    vm.set_breakpoint_handler(...);
    vm.set_breakpoint(...);
    vm.unset_breakpoint(...);
    vm.run(&mut cs, bytecode);
