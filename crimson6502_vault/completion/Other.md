### BIT  
Bit test (tests bits in memory against accumulator without changing accumulator)  
flags affected:  
- Z (zero) — set if (accumulator & memory) == 0  
- N (negative) — set to bit 7 of memory  
- V (overflow) — set to bit 6 of memory  
- [ ] zeropage — 3 cycles  
- [ ] absolute — 4 cycles  

### NOP  
No operation (does nothing, useful for timing)  
flags affected: none  
- [x] implied — 2 cycles  

