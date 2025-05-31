Generally, comparison instructions subtract the operand from the given register without affecting that register. Flags are set as with a normal subtraction, allowing the relation between register and operand to be tested using the Zero, Carry, and Negative flags.  
(See the branch instructions below for how to evaluate flags.)

| Relation R − Op       | Z | C | N                   |
|----------------------|---|---|---------------------|
| Register < Operand    | 0 | 0 | sign bit of result   |
| Register = Operand    | 1 | 1 | 0                   |
| Register > Operand    | 0 | 1 | sign bit of result   |

### CMP  
Compare accumulator with memory (A − M), does not change A.  
- Sets flags N, Z, C based on result of subtraction (A − M).  
- Carry set if A ≥ M, clear if A < M.  
- Zero set if A = M.  
- Negative reflects bit 7 of (A − M).  

flags affected: **N, Z, C**  
addressing modes and typical cycles:  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] zeropage,X — 4 cycles  
- [ ] absolute — 4 cycles  
- [ ] absolute,X — 4 cycles +1 if page crossed  
- [ ] absolute,Y — 4 cycles +1 if page crossed  
- [ ] (indirect,X) — 6 cycles  
- [ ] (indirect),Y — 5 cycles +1 if page crossed  

### CPX  
Compare X register with memory (X − M), does not change X.  
- Sets flags N, Z, C based on result of subtraction (X − M).  
- Carry set if X ≥ M, clear if X < M.  
- Zero set if X = M.  
- Negative reflects bit 7 of (X − M).  

flags affected: **N, Z, C**  
addressing modes and typical cycles:  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] absolute — 4 cycles  

### CPY  
Compare Y register with memory (Y − M), does not change Y.  
- Sets flags N, Z, C based on result of subtraction (Y − M).  
- Carry set if Y ≥ M, clear if Y < M.  
- Zero set if Y = M.  
- Negative reflects bit 7 of (Y − M).  

flags affected: **N, Z, C**  
addressing modes and typical cycles:  
- [ ] immediate — 2 cycles  
- [ ] zeropage — 3 cycles  
- [ ] absolute — 4 cycles  
