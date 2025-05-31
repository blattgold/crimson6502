JSR and RTS affect the stack by pushing and pulling the return address, respectively.

- **JSR** pushes the return address (PC + 2) onto the stack:
  - First, the **high byte** of (PC + 2) is pushed,
  - Then, the **low byte** is pushed.
- The stack then holds these bytes in order (from bottom to top):  
  `[PC+2]-Low`, `[PC+2]-High`.

- **RTS** pulls the return address from the stack:
  - First pulls the **low byte**,
  - Then pulls the **high byte**,
  - Increments the pulled address by 1, and sets the PC to this value.

---

### JMP  
Jump to address  
- Transfers program control to the specified address.  
flags affected: none  
- [ ] absolute — 3 cycles  
- [ ] indirect — 5 cycles  
  *(Note: Indirect JMP has a hardware bug on page boundary crossing)*

### JSR  
Jump to subroutine (pushes return address onto stack)  
- Pushes (PC + 2) address onto stack (high byte then low byte), then sets PC to target address.  
flags affected: none  
- [ ] absolute — 6 cycles

### RTS  
Return from subroutine (pulls return address from stack)  
- Pulls return address low byte, then high byte, increments by 1, sets PC to that value.  
flags affected: none  
- [ ] implied — 6 cycles
