# Virtual machine

Zinc code is compiled into bytecode which can be run by Zinc VM.

Zinc VM is a virtual machine that serves three purposes: executing arbitrary
computations, generating zero-knowledge proof of performed computations, and
verification of the provided proof without knowing the input data.

Zinc VM is a stack-based virtual machine which is similar to many others like
the Python VM. Even though the VM is designed considering specifics and
limitations of zero-knowledge computations, bytecode instructions only
manipulate data on the stack while all zero-knowledge constraints are
automatically applied by the virtual machine.
