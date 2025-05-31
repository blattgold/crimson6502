These instructions transfer the accumulator or status register (flags) to and from the stack. The processor stack is a last-in-first-out (LIFO) stack of 256 bytes length, implemented at addresses $0100 - $01FF. The stack grows down as new values are pushed onto it with the current insertion point maintained in the stack pointer register.  
(When a byte is pushed onto the stack, it will be stored in the address indicated by the value currently in the stack pointer, which will be then decremented by 1. Conversely, when a value is pulled from the stack, the stack pointer is incremented. The stack pointer is accessible by the TSX and TXS instructions.)

### PHA
Push accumulator onto stack  
- flags: none — no flags affected  
- [ ] implied — 3 cycles

### PHP
Push processor status register onto stack
- The **break flag (B)** is set in the pushed copy of the status register (even though it is not a real flag in the status register).  
- flags affected: none (no flags changed)
flags: none — no flags affected  
- [ ] implied — 3 cycles

### PLA
Pull accumulator from stack  
- N (negative) — set if bit 7 of result is 1
- Z (zero) — set if result is 0
- [ ] implied — 4 cycles

### PLP
Pull processor status register from stack (restores all flags except B and unused flags)
- The **break flag (B)** and **unused flag** bits are ignored when restoring status from the stack.  flags affected: all processor flags set according to restored status (except B and unused bits)
- [ ] implied — 4 cycles
