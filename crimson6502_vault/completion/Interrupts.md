A hardware interrupt (maskable IRQ and non-maskable NMI) causes the processor to:

1. Push the current program counter (PC) onto the stack, **high byte first**, then **low byte**.
2. Push the status register (SR) onto the stack.
   
The stack will then contain, from bottom (older) to top (newer):  
`SR`, `PC-Low`, `PC-High`, with the stack pointer pointing just below the pushed SR.  

The processor then loads the interrupt vector address from:  
- `$FFFA-$FFFB` for IRQ (maskable interrupt)  
- `$FFFE-$FFFF` for NMI (non-maskable interrupt)  

**Interrupt disable flag (I):**  
- When set, IRQ interrupts are ignored (masked).  
- NMIs always execute regardless of this flag.  

**BRK instruction:**  
- Acts like a software interrupt (similar to NMI).  
- Pushes `PC + 2` onto the stack (return address after BRK).  
- Pushes status register with the **break flag (B)** set.  
- Sets interrupt disable flag to mask further IRQs.  
- Transfers control to the NMI vector at `$FFFE-$FFFF`.

**RTI instruction:**  
- Pulls the status register from the stack and restores all flags.  
- Pulls the return address (PC) from the stack.  
- Resumes normal execution from the restored PC.  
- The break flag is ignored when restoring status (it is not a true flag stored in the processor status).  

See *Jump Vectors and Stack Operations* for more details on stack behavior.

---

### BRK  
Force break / software interrupt  
- Pushes `PC + 2` and status register (with break flag set) onto stack  
- Sets Interrupt Disable flag (I = 1)  
- Transfers control to IRQ vector at `$FFFE-$FFFF`  
flags affected: I set  
- [ ] implied — 7 cycles

### RTI  
Return from interrupt  
- Pulls status register from stack and restores flags (break flag ignored)  
- Pulls return address (PC) from stack  
- Resumes execution at restored PC  
flags affected: restores all flags from stack  
- [ ] implied — 6 cycles
