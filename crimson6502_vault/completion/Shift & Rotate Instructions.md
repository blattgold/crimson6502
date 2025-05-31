All shift and rotate instructions preserve the bit shifted out in the carry flag.

### ASL  
Arithmetic Shift Left (shifts bits left by one, inserts 0 on right)  
flags affected:  
- N (negative) — set if bit 7 of result is 1  
- Z (zero) — set if result is 0  
- C (carry) — set if bit 7 of original value is 1 (bit shifted out)  
- [ ] accumulator — 2 cycles  
- [ ] zeropage — 5 cycles  
- [ ] zeropage,X — 6 cycles  
- [ ] absolute — 6 cycles  
- [ ] absolute,X — 7 cycles  

### LSR  
Logical Shift Right (shifts bits right by one, inserts 0 on left)  
flags affected:  
- N (negative) — always cleared (result bit 7 always 0 after shift)  
- Z (zero) — set if result is 0  
- C (carry) — set if bit 0 of original value is 1 (bit shifted out)  
- [ ] accumulator — 2 cycles  
- [ ] zeropage — 5 cycles  
- [ ] zeropage,X — 6 cycles  
- [ ] absolute — 6 cycles  
- [ ] absolute,X — 7 cycles  

### ROL  
Rotate Left (shifts bits left by one, inserts carry flag on right)  
flags affected:  
- N (negative) — set if bit 7 of result is 1  
- Z (zero) — set if result is 0  
- C (carry) — set if bit 7 of original value is 1 (bit shifted out)  
- [ ] accumulator — 2 cycles  
- [ ] zeropage — 5 cycles  
- [ ] zeropage,X — 6 cycles  
- [ ] absolute — 6 cycles  
- [ ] absolute,X — 7 cycles  

### ROR  
Rotate Right (shifts bits right by one, inserts carry flag on left)  
flags affected:  
- N (negative) — set if bit 7 of result is 1  
- Z (zero) — set if result is 0  
- C (carry) — set if bit 0 of original value is 1 (bit shifted out)  
- [ ] accumulator — 2 cycles  
- [ ] zeropage — 5 cycles  
- [ ] zeropage,X — 6 cycles  
- [ ] absolute — 6 cycles  
- [ ] absolute,X — 7 cycles  
