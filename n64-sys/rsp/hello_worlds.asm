arch n64.rsp
endian msb
output "hello_world.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000

align(8)
start:
    xor t0, t0, t0
    addi t2, t0, 42 // t0 = 42
    xor t1, t1, t1 // t1 = 0
    addi t1, t1, 1 // ++t1
write_addr:
    sw  t0, 0(t1)
    addi t1, t1, 4 // t1 += 4
    bne t1, 4096, write_addr // t1 != 0, loop
    j return

// Zero t0
xor t0, t0, t0

handle_commands:
    // t0 contains ptr
    j poll

poll:
    la t0, 0
    beq t0, 0, poll
    nop
    j handle_commands

start_2:
    xor t1, t1, t1 // Zero offset
    j loop
handle:

loop:    
    la t0, 0   // Load operation
    bne t0, 0, handle
    j poll
    nop // Delay Slot

return:
    break