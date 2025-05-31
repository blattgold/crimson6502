Branch targets are relative, signed 8-bit address offsets.  
(An offset of zero corresponds to the immediately following address. These offsets are usually computed by assemblers from absolute addresses or labels. Branch instructions appear similar to absolute addressing but actually use a single-byte relative offset.)

Branch instructions test the specified flag condition and branch if true. If the branch is taken, the program counter is updated by adding the signed offset to the address of the next instruction. Branching takes extra cycles depending on whether the branch is taken and if a page boundary is crossed.

- All branch instructions take **2 cycles** if the branch is not taken.
- If the branch is taken, it takes **3 cycles**.
- If the branch is taken and crosses a page boundary, it takes **4 cycles**.

### BCC  
Branch if Carry Clear (C = 0)  
flags tested: **C** (carry flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BCS  
Branch if Carry Set (C = 1)  
flags tested: **C** (carry flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BEQ  
Branch if Equal (Zero flag set, Z = 1)  
flags tested: **Z** (zero flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BMI  
Branch if Minus (Negative flag set, N = 1)  
flags tested: **N** (negative flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BNE  
Branch if Not Equal (Zero flag clear, Z = 0)  
flags tested: **Z** (zero flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BPL  
Branch if Plus (Negative flag clear, N = 0)  
flags tested: **N** (negative flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BVC  
Branch if Overflow Clear (V = 0)  
flags tested: **V** (overflow flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)

### BVS  
Branch if Overflow Set (V = 1)  
flags tested: **V** (overflow flag)  
- [ ] relative  
cycles: 2 (not taken), 3 (taken), 4 (taken + page crossed)
