Arithmetic operations perform addition or subtraction on the accumulator with the carry flag affecting the result.

### ADC  
Add memory to accumulator with carry (A = A + M + C).  
- The carry flag (C) is added as 1 if set, else 0.  
- Carry (C) set if result > 0xFF; cleared otherwise.  
- Overflow (V) set if signed overflow occurs.  
- Negative (N) reflects bit 7 of result.  
- Zero (Z) set if result is zero.  
- Decimal mode (if enabled) performs BCD addition (less common in emulation).  

flags affected: **N, Z, C, V**  
addressing modes and typical cycles:  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles +1 if page crossed  
- [ ] absolute,Y — 4 cycles +1 if page crossed  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles +1 if page crossed  

### SBC  
Subtract memory from accumulator with borrow (A = A - M - (1 - C)).  
- Carry acts as borrow flag: set = no borrow, clear = borrow.  
- Carry set if no borrow needed; cleared otherwise.  
- Overflow (V) set if signed overflow occurs.  
- Negative (N) reflects bit 7 of result.  
- Zero (Z) set if result is zero.  
- Decimal mode performs BCD subtraction (rarely used).  

flags affected: **N, Z, C, V**  
addressing modes and typical cycles:  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles +1 if page crossed  
- [ ] absolute,Y — 4 cycles +1 if page crossed  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles +1 if page crossed  

