# Synacor Challenge
In this challenge, your job is to use this architecture spec to create a
virtual machine capable of running the included binary.  Along the way,
you will find codes; submit these to the challenge website to track
your progress.  Good luck!

## architecture
- three storage regions
  - memory with 15-bit address space storing 16-bit values
  - eight registers
  - an unbounded stack which holds individual 16-bit values
- all numbers are unsigned integers 0..32767 (15-bit)
- all math is modulo 32768; 32758 + 15 => 5

## binary format
- each number is stored as a 16-bit little-endian pair (low byte, high byte)
- numbers 0..32767 mean a literal value
- numbers 32768..32775 instead mean registers 0..7
- numbers 32776..65535 are invalid
- programs are loaded into memory starting at address 0
- address 0 is the first 16-bit value, address 1 is the second 16-bit value, etc

## execution
- After an operation is executed, the next instruction to read is immediately after the last argument of the current operation.  If a jump was performed, the next operation is instead the exact destination of the jump.
- Encountering a register as an operation argument should be taken as reading from the register or setting into the register as appropriate.

## hints
- Start with operations 0, 19, and 21.
- Here's a code for the challenge website: QJFdmOKXfzUI
- The program "9,32768,32769,4,19,32768" occupies six memory addresses and should:
  - Store into register 0 the sum of 4 and the value contained in register 1.
  - Output to the terminal the character with the ascii code contained in register 0.

## opcode listing

<dl>
<dt><code>halt: 0</code></dt>
<dd>stop execution and terminate the program</dd>
<dt><code>set: 1 a b</code></dt>
<dd>set register <code>&lt;a&gt;</code> to the value of <code>&lt;b&gt;</code></dd>
<dt><code>push: 2 a</code></dt>
<dd>push <code>&lt;a&gt;</code> onto the stack</dd>
<dt><code>pop: 3 a</code></dt>
<dd>remove the top element from the stack and write it into <code>&lt;a&gt;</code>; empty stack = error</dd>
<dt><code>eq: 4 a b c</code></dt>
<dd>set <code>&lt;a&gt;</code> to 1 if <code>&lt;b&gt;</code> is equal to <code>&lt;c&gt;</code>; set it to 0 otherwise</dd>
<dt><code>gt: 5 a b c</code></dt>
<dd>set <code>&lt;a&gt;</code> to 1 if <code>&lt;b&gt;</code> is greater than <code>&lt;c&gt;</code>; set it to 0 otherwise</dd>
<dt><code>jmp: 6 a</code></dt>
<dd>jump to <code>&lt;a&gt;</code></dd>
<dt><code>jt: 7 a b</code></dt>
<dd>if <code>&lt;a&gt;</code> is nonzero, jump to <code>&lt;b&gt;</code></dd>
<dt><code>jf: 8 a b</code></dt>
<dd>if <code>&lt;a&gt;</code> is zero, jump to <code>&lt;b&gt;</code></dd>
<dt><code>add: 9 a b c</code></dt>
<dd>assign into <code>&lt;a&gt;</code> the sum of <code>&lt;b&gt;</code> and <code>&lt;c&gt;</code> (modulo 32768)</dd>
<dt><code>mult: 10 a b c</code></dt>
<dd>store into <code>&lt;a&gt;</code> the product of <code>&lt;b&gt;</code> and <code>&lt;c&gt;</code> (modulo 32768)</dd>
<dt><code>mod: 11 a b c</code></dt>
<dd>store into <code>&lt;a&gt;</code> the remainder of <code>&lt;b&gt;</code> divided by <code>&lt;c&gt;</code></dd>
<dt><code>and: 12 a b c</code></dt>
<dd>stores into <code>&lt;a&gt;</code> the bitwise and of <code>&lt;b&gt;</code> and <code>&lt;c&gt;</code></dd>
<dt><code>or: 13 a b c</code></dt>
<dd>stores into <code>&lt;a&gt;</code> the bitwise or of <code>&lt;b&gt;</code> and <code>&lt;c&gt;</code></dd>
<dt><code>not: 14 a b</code></dt>
<dd>stores 15-bit bitwise inverse of <code>&lt;b&gt;</code> in <code>&lt;a&gt;</code></dd>
<dt><code>rmem: 15 a b</code></dt>
<dd>read memory at address <code>&lt;b&gt;</code> and write it to <code>&lt;a&gt;</code></dd>
<dt><code>wmem: 16 a b</code></dt>
<dd>write the value from <code>&lt;b&gt;</code> into memory at address <code>&lt;a&gt;</code></dd>
<dt><code>call: 17 a</code></dt>
<dd>write the address of the next instruction to the stack and jump to <code>&lt;a&gt;</code></dd>
<dt><code>ret: 18</code></dt>
<dd>remove the top element from the stack and jump to it; empty stack = halt</dd>
<dt><code>out: 19 a</code></dt>
<dd>write the character represented by ascii code <code>&lt;a&gt;</code> to the terminal</dd>
<dt><code>in: 20 a</code></dt>
<dd>read a character from the terminal and write its ascii code to <code>&lt;a&gt;</code>; it can be assumed that once input starts, it will continue until a newline is encountered; this means that you can safely read whole lines from the keyboard and trust that they will be fully read</dd>
<dt><code>noop: 21</code></dt>
<dd>no operation</dd>
</dl>
