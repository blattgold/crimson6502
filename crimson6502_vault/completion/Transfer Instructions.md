Load, store, and inter-register transfer instructions move data between memory and registers or between registers. All loads (LDA/LDX/LDY) and register transfers update the Negative (N) and Zero (Z) flags based on the resulting value. Stores (STA/STX/STY) do not affect any flags.

### LDA  
Load accumulator with memory (A = M)  
- Sets:  
  - N (negative) if bit 7 of A is 1  
  - Z (zero) if A = 0  
- Does not affect: C, V, D, I, B, U (unused)  
- Implementation notes:  
  - Fetch operand, load into A, update N/Z.  
  - Handle zero‐page indexing wrap for zeropage,X.  
- Addressing modes & typical cycles:  
  - [x] immediate — 2 cycles  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,X — 4 cycles  
  - [x] absolute — 4 cycles  
  - [x] absolute,X — 4 cycles + 1 if page boundary crossed  
  - [x] absolute,Y — 4 cycles + 1 if page boundary crossed  
  - [x] (indirect,X) — 6 cycles  
  - [x] (indirect),Y — 5 cycles + 1 if page boundary crossed  

### LDX  
Load X register with memory (X = M)  
- Sets:  
  - N if bit 7 of X is 1  
  - Z if X = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Fetch operand, load into X, update N/Z.  
  - Handle zero‐page indexing wrap for zeropage,Y.  
- Addressing modes & typical cycles:  
  - [x] immediate — 2 cycles  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,Y — 4 cycles  
  - [x] absolute — 4 cycles  
  - [x] absolute,Y — 4 cycles + 1 if page boundary crossed  

### LDY  
Load Y register with memory (Y = M)  
- Sets:  
  - N if bit 7 of Y is 1  
  - Z if Y = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Fetch operand, load into Y, update N/Z.  
  - Handle zero‐page indexing wrap for zeropage,X.  
- Addressing modes & typical cycles:  
  - [x] immediate — 2 cycles  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,X — 4 cycles  
  - [x] absolute — 4 cycles  
  - [x] absolute,X — 4 cycles + 1 if page boundary crossed  

### STA  
Store accumulator to memory (M = A)  
- Does not affect any flags.  
- Implementation notes:  
  - Compute effective address, write A to memory.  
  - No read‐modify or flag updates.  
- Addressing modes & typical cycles:  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,X — 4 cycles  
  - [x] absolute — 4 cycles  
  - [x] absolute,X — 5 cycles  
  - [x] absolute,Y — 5 cycles  
  - [x] (indirect,X) — 6 cycles  
  - [x] (indirect),Y — 6 cycles  

### STX  
Store X register to memory (M = X)  
- Does not affect any flags.  
- Implementation notes:  
  - Compute effective address, write X to memory.  
- Addressing modes & typical cycles:  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,Y — 4 cycles  
  - [x] absolute — 4 cycles  

### STY  
Store Y register to memory (M = Y)  
- Does not affect any flags.  
- Implementation notes:  
  - Compute effective address, write Y to memory.  
- Addressing modes & typical cycles:  
  - [x] zeropage — 3 cycles  
  - [x] zeropage,X — 4 cycles  
  - [x] absolute — 4 cycles  

### TAX  
Transfer accumulator to X register (X = A)  
- Sets:  
  - N if bit 7 of X is 1 (i.e., bit 7 of A)  
  - Z if X = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Copy A → X, update N/Z.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  

### TAY  
Transfer accumulator to Y register (Y = A)  
- Sets:  
  - N if bit 7 of Y is 1  
  - Z if Y = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Copy A → Y, update N/Z.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  

### TSX  
Transfer stack pointer to X register (X = SP)  
- Sets:  
  - N if bit 7 of X is 1 (bit 7 of SP)  
  - Z if X = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Copy SP → X, update N/Z.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  

### TXA  
Transfer X register to accumulator (A = X)  
- Sets:  
  - N if bit 7 of A is 1 (bit 7 of X)  
  - Z if A = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Copy X → A, update N/Z.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  

### TXS  
Transfer X register to stack pointer (SP = X)  
- Does not affect any flags.  
- Implementation notes:  
  - Copy X → SP.  
  - No flag updates.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  

### TYA  
Transfer Y register to accumulator (A = Y)  
- Sets:  
  - N if bit 7 of A is 1 (bit 7 of Y)  
  - Z if A = 0  
- Does not affect: C, V, D, I, B, U  
- Implementation notes:  
  - Copy Y → A, update N/Z.  
- Addressing mode & cycles:  
  - [x] implied — 2 cycles  
