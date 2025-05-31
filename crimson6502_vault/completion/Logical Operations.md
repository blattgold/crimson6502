Logical operations perform bitwise manipulation between the accumulator and memory.

### AND  
Bitwise AND with accumulator  
flags affected: N (negative), Z (zero) — set if result is negative or zero  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles (+1 cycle if page crossed)  
- [ ] absolute,Y — 4 cycles (+1 cycle if page crossed)  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles (+1 cycle if page crossed)

### EOR  
Bitwise Exclusive OR (XOR) with accumulator  
flags affected: N (negative), Z (zero) — set if result is negative or zero  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles (+1 cycle if page crossed)  
- [ ] absolute,Y — 4 cycles (+1 cycle if page crossed)  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles (+1 cycle if page crossed)

### ORA  
Bitwise Inclusive OR with accumulator  
flags affected: N (negative), Z (zero) — set if result is negative or zero  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles (+1 cycle if page crossed)  
- [ ] absolute,Y — 4 cycles (+1 cycle if page crossed)  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles (+1 cycle if page crossed)
