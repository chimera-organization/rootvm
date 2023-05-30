# root - a rust-based vm for any language

root is an optimized rust-based virtual machine for any language. It is designed to be a target for transpilers, and to be easily embeddable in other projects.

## sub-libraries

### rootir
rootir is a rust-based representation of the root intermediate representation (IR).

### vmalloc
vmalloc is a rust-based virtual memory allocator that makes working with pointers relatively safe in this context.


## planned sub-libraries / projects

### rally
rally is an assembly-like language that compiles to rootir, and can be used as a target for other languages. _Compare to Java's Smali_

### root-llvm
root-llvm is a bidirectional transpiler between rootir and llvm ir.

### eden
eden is a bidirectional transpiler from java bytecode to rootir.

### vstarr
vstarr is a virtual stack <-> register bidirectional ir assembler, allowing transpilers to easily convert between stack-based and register-based 

