Decrement and increment instructions modify memory or registers by subtracting or adding one.

All instructions set the Negative (N) and Zero (Z) flags based on the result:
- N is set if the result's most significant bit (bit 7) is 1 (negative)
- Z is set if the result is zero

### DEC  
Decrement memory by one  
flags: N, Z — set if result is negative or zero  
- [ ] zeropage       — 5 cycles  
- [ ] zeropage,X     — 6 cycles  
- [ ] absolute       — 6 cycles  
- [ ] absolute,X     — 7 cycles  

### DEX  
Decrement X register by one  
flags: N, Z — set if result is negative or zero  
- [ ] implied       — 2 cycles  

### DEY  
Decrement Y register by one  
flags: N, Z — set if result is negative or zero  
- [ ] implied       — 2 cycles  

### INC  
Increment memory by one  
flags: N, Z — set if result is negative or zero  
- [ ] zeropage       — 5 cycles  
- [ ] zeropage,X     — 6 cycles  
- [ ] absolute       — 6 cycles  
- [ ] absolute,X     — 7 cycles  

### INX  
Increment X register by one  
flags: N, Z — set if result is negative or zero  
- [ ] implied       — 2 cycles  

### INY  
Increment Y register by one  
flags: N, Z — set if result is negative or zero  
- [ ] implied       — 2 cycles  
