Flag instructions directly set or clear specific processor status flags.

All these instructions are implied addressing mode and take 2 CPU cycles.

### CLC  
Clear Carry flag (C = 0)  
flags affected: C cleared  
- [ ] implied — 2 cycles

### CLD  
Clear Decimal mode flag (D = 0, disables BCD arithmetic)  
flags affected: D cleared  
- [ ] implied — 2 cycles

### CLI  
Clear Interrupt Disable flag (I = 0, enables IRQ interrupts)  
flags affected: I cleared  
- [ ] implied — 2 cycles

### CLV  
Clear Overflow flag (V = 0)  
flags affected: V cleared  
- [ ] implied — 2 cycles

### SEC  
Set Carry flag (C = 1)  
flags affected: C set  
- [ ] implied — 2 cycles

### SED  
Set Decimal mode flag (D = 1, enables BCD arithmetic)  
flags affected: D set  
- [ ] implied — 2 cycles

### SEI  
Set Interrupt Disable flag (I = 1, disables IRQ interrupts)  
flags affected: I set  
- [ ] implied — 2 cycles
