arch n64.rsp
endian msb
output "sandbox.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000


align(8)

start:
    xor t0, t0, t0                  // t0 = 0
    li t1, 65536        // t1 = 1.0 in fix_16_16

    li t2, 3            // t2 = 3
    sll t2, t2, 15      // t2 *= 2^15 (1.5 in 16.16)
    sw t2, 0(t0)

    li t2, 5            // t2 = 3
    sll t2, t2, 15      // t2 *= 2^15 (1.5 in 16.16)
    sw t2, 4(t0)
    
    li t2, 7            // t2 = 3
    sll t2, t2, 15      // t2 *= 2^15 (1.5 in 16.16)
    sw t2, 8(t0)
    
    li t2, 9            // t2 = 3
    sll t2, t2, 15      // t2 *= 2^15 (1.5 in 16.16)
    sw t2, 12(t0)

    lsv v2[e0],  0(t0)
    lsv v1[e0],  2(t0)
    lsv v2[e2],  4(t0)
    lsv v1[e2],  6(t0)
    lsv v2[e4],  8(t0)
    lsv v1[e4], 10(t0)
    lsv v2[e6], 12(t0)
    lsv v1[e6], 14(t0)

    // V2 dst
    li t2, 3            // t2 = 3
    sll t2, t2, 16      // t2 *= 2^16
    sw t2, 16(t0)
    
    li t2, 4            // t2 = 3
    sll t2, t2, 16      // t2 *= 2^16
    sw t2, 20(t0)
    
    li t2, 5            // t2 = 3
    sll t2, t2, 16      // t2 *= 2^16
    sw t2, 24(t0)
    
    li t2, 6            // t2 = 3
    sll t2, t2, 16      // t2 *= 2^16
    sw t2, 28(t0)


    lsv v3[e0], 18(t0)
    lsv v3[e2], 22(t0)
    lsv v3[e4], 26(t0)
    lsv v3[e6], 30(t0)
    lsv v4[e0], 16(t0)
    lsv v4[e2], 20(t0)
    lsv v4[e4], 24(t0)
    lsv v4[e6], 28(t0)

    vmudl v5,v1,v3
    vmadm v6,v2,v3
    vmadn v7,v1,v4
    vmadh v8,v2,v4
    
    //sqv v0[e0],  48(t0)   // Store 128 bits from V0 into MEM[t0 + 0]
    sqv v1[e0],  64(t0)  // Store 128 bits from V1 into MEM[t0 + 16]
    sqv v2[e0],  80(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v3[e0],  96(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v4[e0], 112(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v5[e0], 128(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v6[e0], 144(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v7[e0], 160(t0)  // Store 128 bits from V2 into MEM[t0 + 32]
    sqv v8[e0], 176(t0)  // Store 128 bits from V2 into MEM[t0 + 32]

    lh t2,176(t0)
    sh t2,192(t0)
    lh t2,178(t0)
    sh t2,196(t0)
    lh t2,180(t0)
    sh t2,200(t0)
    lh t2,182(t0)
    sh t2,204(t0)
    
    lh t2,160(t0)
    sh t2,194(t0)
    lh t2,162(t0)
    sh t2,198(t0)
    lh t2,164(t0)
    sh t2,202(t0)
    lh t2,166(t0)
    sh t2,206(t0)

    vnop
    nop
    vnop
    nop
    vnop
    nop
    vnop
    nop

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