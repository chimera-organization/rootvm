# root - a rust-based vm for any language

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
eden is a unidirectional transpiler from java bytecode to rootir.

