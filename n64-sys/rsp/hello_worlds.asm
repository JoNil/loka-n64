arch n64.rsp
endian msb
output "hello_world.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000


align(8)

start:
    lhu t0, 4094(0) // Load 16bits as number of commands
    xor t1, t1, t1 // t1 = 0
    xor t2, t2, t2 // t2 = 0
    xor c8, c8, c9 // CMD_START = 0
    xor c9, c9, c9 // CMD_END = 0
    xor t3, t3, t3 // t3 = 0
    addi t3, t3, SET_XBS|CLR_FRZ|CLR_FLS // t3 = 00001001 (GCLK | XBUS_MEM)
    sll t0, t0, 3     // Number of commands -> number of bytes (64bits / command)
    mtc0 t1, c8       // CMD_START = 0
    mtc0 t1, c9       // CMD_END = 0
    mtc0 t3, c11      // CMD_STATUS = run
    mtc0 t0, c9       // CMD_END = 8*nCommands
    subi t4, t0, 8    // t4 = CMD_END-8
    xor t5, t5, t5    // t5 = 0
    addi t1, t1, 4092 // t1 = 4092
    li   t6,    RDP_CMB // t6 = bit for RDP_BUSY
    xor t7, t7, t7    // t7 = 0
wait_for_rdp:
    addi t5, t5, 1 // ++t5
    subi t1, t1, 4 // t1 -= 4
    mfc0 c10, t2   // t2 = CMD_CURRENT
    mfc0 c11, t3   // t3 = CMD_STATUS
    ori t1, t1, 1024
    sw t3, 0(t1)
    //beq t5, t0, return // Timeout
    //nop
    and t3, t3, t6
    bne t3, t7, wait_for_rdp // Loop while RDP busy
    //bne t2, t4, wait_for_rdp // Compare CMD_CURRENT and CMD_END-8
    nop

if 0 {
loop_commands:
    sw t1, 0(t2)    // Store word t1 at mem[t2]
    // Process command[t1]


    addi t1, t1, 1 // ++t1,
    addi t2, t2, 4 // t2 += 4
    bne t0, t1, loop_commands // loop until t1 is t0
    nop
}
return:
 break


if 0 {
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
    
    vnxor v2,v0,v1[e0]  // v2 = V0 NXOR V1
    //vadd v2,v0,v1[e0]  // v2 = V0 NXOR V1

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
}