# Crimson6502 Emulator (WIP)

Crimson6502 is a work-in-progress 6502 emulator written in Rust.

### Features
- Very early CPU and memory emulation
- Simple REPL interface for debugging and interaction
- Partial opcode support (see below)

### REPL Syntax

- quit - Exit the REPL
- init - Initialize all components (CPU and memory)
- init \<component> - Initialize a specific component: mem or cpu
- write \<addr> \<value> - Write a byte to memory at the specified address
- step - Execute 1 instruction cycle
- step \<n> - Execute n instruction cycles

### Opcode Coverage

To view current opcode implementation progress, refer to the Obsidian vault:  
**`crimson6502_obsidian`**

---

This project is in active development. Expect rapid changes and incomplete features.
