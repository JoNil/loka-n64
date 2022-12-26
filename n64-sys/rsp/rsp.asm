arch n64.rsp
endian msb
output "rsp.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000


align(8)

start:

    xor t0, t0, t0                  // t0 = 0
    li t1, SET_XBS|CLR_FRZ|CLR_FLS|CLR_CLK  // t1 = 00001001 (GCLK | XBUS_MEM)

    lhu t2, 4094(0)   // Load 16bits as number of commands
    sll t2, t2, 3     // Number of commands -> number of bytes (64bits / command)

    li t4, 4092       // t4 = 4092
    subi t4, t4, 8    // t4 -= 8
    sw t0, 0(t4)
    subi t4, t4, 8    // t4 -= 8
    sw t1, 0(t4)
    subi t4, t4, 8    // t4 -= 8
    sw t2, 0(t4)
    
    mtc0 t1, c11      // CMD_STATUS = run
    mtc0 t0, c8       // CMD_START = 0
    mtc0 t2, c9       // CMD_END = 8*nCommands

    mfc0 t8, c12      // Read RDP clock

    mfc0 t3, c11      // t3 = CMD_STATUS
    subi t4, t4, 8    // t4 -= 8
    sw t3, 0(t4)

    li t5, RDP_CMB    // t5 =  RDP_CMB (00100000)
    
    subi t4, t4, 8    // t4 -= 8
    sw t5, 0(t4)

    mfc0 t3, c8       // t3 = CMD_START
    subi t4, t4, 8    // t4 -= 8
    sw t3, 0(t4)

    mfc0 t3, c9       // t3 = CMD_END
    subi t4, t4, 8    // t4 -= 8
    sw t3, 0(t4)

wait_for_rdp:
    subi t4, t4, 8 // t4 -= 8
    ori t4, t4, 1024

    mfc0 t3, c11    // t3 = CMD_STATUS
    sw t3, 0(t4)
    and t3, t3, t5  // t3 &= t5

    beq t3, t5, wait_for_rdp // Loop while RDP busy
    nop

    mfc0 t9, c12   // Read RDP clock
    sub t9, t9, t8 // t9 = RDP clock end - clock start
    subi t4, t4, 4
    sw t9, 0(t4)

return:
 break

// Triangle data input as vertex index * 3
// Vertex data as fixpoint vertex coordinates
// V0 at dram 3*nTriangles
// Last in Dram is nTriangles, nVertices (unsigned bytes)
// Vertex size is 3*sizeof(fixpoint)(32bit) => 96bit = 12byte
if 0 {

// process triangle, vertex 0 byte start is assumed to be in register t6
//                   vertex 1 byte start is assumed to be in register t7
//                   vertex 2 byte start is assumed to be in register t8
process_triangle:
    // TODO: Collect 4 triangles before doing work, flush after last to handle any extras
    llv v0[0], t6(0)
    llv v0[4], t7(0)
    llv v0[8], t8(0)
    llv v1[0], t6(4)
    llv v1[4], t7(4)
    llv v1[8], t8(4)
    llv v1[0], t6(8)
    llv v1[4], t7(8)
    llv v1[8], t8(8)
    // Vector registers:
    // v0 : v0x, v1x, v2x
    // v1 : v0y, v1y, v2y
    // v2 : v0z, v1z, v2z
    // TOOD: BEtter to store t0v0x in element 0 of v0 t1v0x in element 1 of v0 etc
    // Need 9 vo registers, one per dimension of each vertex. v0 : v0x, V1 v0y etc.
    //   Vi[j] : tiVj
    // Vector registers: 
    // v0 : v0x | v1 : v0y | v2 : v0z
    // v1 : v1x | v1 : v1y | v2 : v1z
    // v2 : v2x | v1 : v2y | v2 : v2z

    

    jr t31 // Return to caller



    xor t1, t1, t1 // t1 = 0 (tmp)

    lbu t2, 4095(0) // t2:  Load number of triangles as 8bit unsigned
    lbu t3, 4094(0) // t3 : Load number of vertices as 8bit unsigned

    // 3*ntriangles is byte end for triangle vertex indices
    sll t4, t3, 2 // t4 byte start of current triangle
    addu t4, t4, t3

    addi t5, t4, 0 // t5 = t4 (save vertex byte start in t5)

loop_triangles:
    subi t4, t2, 3          // t4 -= 3 (byte start of current triangle's vertexs index)

    // vertex index at t4, t4 + 1, t4 + 2
    lbu t1, t4(0) // t6 = v0 idx
    sll t6, t1, 1 // t6 = index to offset (tmp*2 + tmp)
    add t6, t6, t1 // t6 = index to offset
    lbu t1, t4(1) // t6 = v0 idx
    sll t7, t1, 1 // t6 = index to offset (tmp*2 + tmp)
    add t7, t7, t1 // t6 = index to offset
    lbu t1, t4(2) // t6 = v0 idx
    sll t8, t1, 1 // t6 = index to offset (tmp*2 + tmp)
    add t8, t8, t1 // t6 = index to offset

    // Add vertex offset
    add t6, t6, t5
    add t7, t6, t5
    add t8, t6, t5 // now points to dram start for vertex 2 of current triangle

    // Process this triangle
    jal process_triangle
    nop

    bgtz t2, loop_triangles // Loop again if not at id 0
    nop
}

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