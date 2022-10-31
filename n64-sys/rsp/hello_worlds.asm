arch n64.rsp
endian msb
output "hello_world.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000

align(8)
start:
    xor t0, t0, t0    // t0 = 0
    xor t1, t1, t1    // t1 = 0
    xor t2, t2, t2    // t2 = 0
    addi t2, t2, 4096 // t2 = 4096
write_addr:
    sw  t1, 0(t1)          // MEM[t1] = t1
    addi t1, t1, 4         // t1 += 4
    bne t1, t2, write_addr // t1 != 0, loop
    nop
    xor t0, t0, t0    // t0 = 0
    lqv v0[e0],  0(t0)  // Load 128 bits from MEM[t0] into V0
    lqv v1[e0], 16(t0)  // Load 128 bits from MEM[t0 + 16] into V1
    
    //vnxor v2, v0, v1[e0]  // v2 = V0 NXOR V1 This hangs

    sqv v0[e0], 32(t0)  // Store 128 bits from V0 into MEM[t0 + 32]
    sqv v1[e0], 48(t0)  // Store 128 bits from V0 into MEM[t0 + 48]
    sqv v2[e0], 64(t0)  // Store 128 bits from V0 into MEM[t0 + 64]
    j return
    nop

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
    nop

start_3:
    xor t1, t1, t1 // Zero offset
    j loop
    nop
handle:

loop:    
    la t0, 0   // Load operation
    bne t0, 0, handle
    j poll
    nop // Delay Slot

return:
    break