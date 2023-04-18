arch n64.rsp
endian msb
output "rsp.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000


constant debug_print_start = 2048
constant command_block_start = 1024
constant command_block_count = command_block_start + 4
constant rdp_start_cmd = command_block_start + 8
//constant rdp_end_cmd = 2048
constant rdp_start_flags = SET_XBS|CLR_FRZ|CLR_FLS|CLR_CLK
constant rdp_dma_flags = CLR_XBS
constant rdp_command_count_offset = 512
constant rdp_busy_mask = RDP_CMB//RDP_PLB|RDP_CMB
constant clear_signals = CLR_SG0|CLR_SG1|CLR_SG2|CLR_SG3|CLR_SG4|CLR_SG5|CLR_SG6|CLR_SG7

macro DbgPrint(reg) {
    sw 0, 4(t7) // Write 0 AFTER current value, to mark last print
    sw {reg}, 0(t7)
    addi t7, t7, 4
    // Loop between 2048 - 4195
    andi t7, t7, 4095
    ori t7, t7, 2048
}

macro DbgDisableSingleStep (tmp_reg, ss_reg) {
    // Read if in Single step
    mfc0 {ss_reg}, c4
    andi {ss_reg}, {ss_reg}, RSP_STP
    li {tmp_reg}, CLR_STP
    mtc0 {tmp_reg}, c4
}

macro DbgEnableSingleStep (tmp_reg, ss_reg) {    
    //li {tmp_reg}, 0x40
    // If was in single step before disable, ss_reg will contain 0x20
    sll {tmp_reg}, {ss_reg}, 1
    mtc0 {tmp_reg}, c4
}

macro DbgPrintStatusRegs (reg) {
    if 0 {
    li t5, 1122
    DbgPrint(t5)
    mfc0 t5, c0
    DbgPrint(t5)
    mfc0 t5, c1
    DbgPrint(t5)
    mfc0 t5, c2
    DbgPrint(t5)
    mfc0 t5, c3
    DbgPrint(t5)
    mfc0 t5, c4
    DbgPrint(t5)
    mfc0 t5, c5
    DbgPrint(t5)
    mfc0 t5, c6
    DbgPrint(t5)
    //mfc0 t5, c7
    //DbgPrint(t5)
    mfc0 t5, c8
    DbgPrint(t5)
    mfc0 t5, c9
    DbgPrint(t5)
    mfc0 t5, c10
    DbgPrint(t5)

    li t5, 4321
    DbgPrint(t5)

    mfc0 t5, c11
    DbgPrint(t5)
    mfc0 t5, c12
    DbgPrint(t5)
    mfc0 t5, c13
    DbgPrint(t5)
    li t5, 3344
    DbgPrint(t5)
    }
}

macro set_signals_from_upper_clock(reg, tmp0, tmp1) {
        li {reg}, 1111
        DbgPrint({reg})
        
        li {tmp1}, clear_signals
        DbgPrint({tmp1})
        mtc0 {tmp1}, c4

        li {reg}, 2222
        DbgPrint({reg})
    if 1 {
        mfc0 {reg}, c12      // Read RDP clock

        DbgPrint({reg})
        // CLock is 24 bits, will only fit 7 (since sig_7 is used to signal "clear") shift 17
        srl {reg}, {reg}, 17
        // Need to move bits to fit "set_signal_x"
        DbgPrint({reg})

        // Clear tmp1
        xor {tmp1}, {tmp1}, {tmp1}

// Read bit 0, and shift left to "set" signal bit 0 part of status
        andi {tmp0}, {reg}, 0x01 // reg = clock bits: 24_23_22_21_20_19_18
        sll {tmp1}, {tmp0}, SET_SG0 // tmp1 = (SET_SIG0 & bit_18)

        srl {reg}, {reg}, 1  // reg = clock bits: 24_23_22_21_20_19
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG1
        or {tmp1}, {tmp1}, {tmp0}  // tmp1 = tmp1 | (SET_SIG1 & bit_19)

        srl {reg}, {reg}, 1  // reg = clock bits: 24_23_22_21_20
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG2
        or {tmp1}, {tmp1}, {tmp0}  // tmp1 = tmp1 | (SET_SIG2 & bit_20)

        srl {reg}, {reg}, 1  // reg = clock bits: 24_23_22_21
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG3
        or {tmp1}, {tmp1}, {tmp0}

        srl {reg}, {reg}, 1  // reg = clock bits: 24_23_22
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG4
        
        srl {reg}, {reg}, 1  // reg = clock bits: 24_23
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG5
        or {tmp1}, {tmp1}, {tmp0}

        srl {reg}, {reg}, 1  // reg = clock bits: 24
        andi {tmp0}, {reg}, 0x01
        sll {tmp0}, {tmp0}, SET_SG6
        or {tmp1}, {tmp1}, {tmp0}

        DbgPrint({tmp1})
        mtc0 {tmp1}, c4 // Set the signals corresponding to the upper 8 bits of the clock.
        sw {reg}, 0(0)       // Store clock at adress 0 in dmem when done.
    } else {
        li {tmp1}, SET_SG0|SET_SG1|SET_SG2|SET_SG3
        mtc0 {tmp1}, c4
    }
}

align(8)

start:
    xor 30, 30, 30        // reg 30 = 0
    xor t0, t0, t0        // t0 = 0
    li t6, rdp_busy_mask  // RDP Command busy
    lw t2, 0(0)           // Load 32 bits as pointer count
    li t3, 0              // t3 = 0

    // Reset rdp clock counter
    ori t5, 0, CLR_CLK
    mtc0 t5, c11

process_chunk_pointer:
    li t5, clear_signals
    mtc0 t5, c4

    li t5, SET_SG0
    mtc0 t5, c4

    // Debug print base address
    li t7, debug_print_start

    DbgPrint(t3)
    DbgPrint(t2)

    beq t3, t2, return // i == count => done
    nop

    sll t4, t3, 2 // index -> bytes (*4)
    lw  t4, 4(t4) // Command block start (Lower 4 bytes)

    DbgPrint(t4)

    li  t5, 42
    DbgPrint(t5)

    li t5, SET_SG1
    mtc0 t5, c4

    // Make sure rdp is not using command buffer which will be overwritten by dma
    DbgDisableSingleStep(t5, 30)
wait_rdp_busy_pre_cpu_to_dmem:
    mfc0 t5, c11
    andi t5, t5, RDP_CMB
    bne t5, 0, wait_rdp_busy_pre_cpu_to_dmem // Loop while rdp busy.
    nop
    DbgEnableSingleStep(t5, 30)

    li t5, SET_SG2|CLR_SG0
    mtc0 t5, c4
    
    // DMEM
    // Request semaphore
    DbgDisableSingleStep(t5, 30)
request_semaphore:
    mfc0 t5, c7
    bnez t5, request_semaphore
    nop
    DbgEnableSingleStep(t5, 30)

    li t5, CLR_SG1
    mtc0 t5, c4

    DbgPrint(t5)
    
    li   t5, 1
    DbgPrint(t5)

if 0 {
    li t5, rdp_dma_flags // Load rdp DMA (Not XBUS) flags
    mtc0 t5, c11         // Load rdp DMA (Not XBUS) flags
}

    li t5, SET_SG3
    mtc0 t5, c4

    // Wait until spot available in DMA
    DbgDisableSingleStep(t5, 30)
wait_dma_available:
    if 0 {
        mfc0 t5, c4
        DbgPrint(t5)
        andi t5, RSP_BSY|RSP_FUL
    } else {
        mfc0 t5, c5
    }
    bne t5, 0, wait_dma_available
    nop
    DbgEnableSingleStep(t5, 30)

    li t5, CLR_SG2
    mtc0 t5, c4
    
    li t5, 2
    DbgPrint(t5)
    
    // Setup DMA request
    li   t5, command_block_start // Rdp command block destination
    mtc0 t5, c0          // DMA destination
    DbgPrint(t5)
    li   t5, 3
    DbgPrint(t5)
    mtc0 t4, c1          // DMA source ptr
    DbgPrint(t4)
    li   t5, 4
    DbgPrint(t5)

    // Data size as "Number of bytes to read LESS ONE"
    li   t5, 1023          // 128 commands per request, 8 bytes per command (128*8 : << 7 + 3 : << 10) => 1<<10 - 1 : 1023
    mtc0 t5, c2

    li   t5, 5
    DbgPrint(t5)
    
    DbgPrint(t5)

    li   t5, 6
    DbgPrint(t5)

    li t5, SET_SG4
    mtc0 t5, c4

    mfc0 t5, c4
    DbgPrint(t5)
    mfc0 t5, c11
    DbgPrint(t5)
    DbgDisableSingleStep(t5, 30)
wait_dma_busy:
    if 1 {
        mfc0 t5, c11
        andi t5, t5, RDP_DMA
    } else {
        mfc0 t5, c6
    }
    bne t5, 0, wait_dma_busy
    nop
    DbgEnableSingleStep(t5, 30)

    li t5, CLR_SG3
    mtc0 t5, c4

    li   t5, 42
    DbgPrint(t5)
// TMP debugprint rdp status
    mfc0 t5, c11
    DbgPrint(t5)

    // Release semaphore
    mtc0 0, c7 

    DbgPrint(t5)

    li t5, CLR_SG4|SET_SG5
    mtc0 t5, c4
    
    li t5, 43
    DbgPrint(t5)
    DbgPrint(t5)
    DbgPrint(t5)

    // Send to RDP
    li t5, rdp_start_flags // Load rdp start flags
    mtc0 t5, c11           // Prepare rdp for commands.
    li t5, rdp_start_cmd   // Rdp commands start
    mtc0 t5, c8            // Rdp commands start

    lw t8, command_block_count(0) // Lower 32 bits of the first 64.
    DbgPrint(t8)
    sll t8, t8, 3               // *8
    DbgPrint(t8)
    addi t8, t8, rdp_start_cmd  // End at start + commands*8
    DbgPrint(t8)
    mtc0 t8, c9                 // Rdp commands end

    DbgPrint(t8)
    li t5, 44
    DbgPrint(t5)

    // Wait for rdp done TODO: Move to before emit for non-sync

    li t5, SET_SG6
    mtc0 t5, c4

    DbgDisableSingleStep(t5, 30)
wait_rdp_busy:
    if 0 {
        ori t5, 0, 45
        DbgPrint(t5)
        mfc0 t5, c9
        DbgPrint(t5)
        mfc0 t5, c10
        DbgPrint(t5)
        mfc0 t5, c11
        DbgPrint(t5)
        mfc0 t5, c12
        DbgPrint(t5)

        mfc0 t5, c13                
        DbgPrint(t5)

    } else {
        mfc0 t5, c11
        andi t5, t5, RDP_CMB
    }
    bne t5, 0, wait_rdp_busy // Loop while rdp busy.
    nop
    DbgEnableSingleStep(t5, 30)

    li t5, CLR_SG5
    mtc0 t5, c4

    addiu t3, t3, 1   // Next chunk pointer
    j process_chunk_pointer
    nop
    
return:

    li t5, SET_SG0|SET_SG1|SET_SG2
    mtc0 t5, c4

    li t5, 1234
    DbgPrint(t5)

    // Wait for rdp done TODO: Move to before emit for non-sync
if 1 {
    DbgDisableSingleStep(t5, 30)

wait_rdp_busy_end:
    if 1 {
        mfc0 t5, c11
        andi t5, t5, RDP_CMB
    } else {
        mfc0 t5, c4
        andi t5, t5, RSP_BSY // dma_busy_bit
    }
    bnez t5, wait_rdp_busy_end
    nop

    DbgEnableSingleStep(t5, 30)
}

    mfc0 t5, c4
    DbgPrint(t5)

    li t5, 5678
    DbgPrint(t5)

    mfc0 t5, c4
    DbgPrint(t5)
    mfc0 t5, c11
    DbgPrint(t5)

    // Disable single step for break
    li t8, CLR_STP
    mtc0 t8, c4

    li t5, 9101
    DbgPrint(t5)

    // Store upper bits of clock in signal bits.
    set_signals_from_upper_clock(t2, t3, t4)

    li t5, 1213
    DbgPrint(t5)

    li t5, SET_SG7
    mtc0 t5, c4

    mfc0 t5, c4
    DbgPrint(t5)

    li t5, SET_HLT
    mtc0 t5, c4
    
    break
    nop

if 0 {
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
}

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

// Command sizes:
// 00 1
// 25 2
// 24 2
// 28 1
// 27 1
// 31 1
// 29 1
// 2a 1
// 2b 1
// 2c 1
// 2d 1
// 2e 1
// 37 1
// 38 1
// 39 1
// 3a 1
// 3b 1
// 2f 1
// 3c 1
// 36 1
// 30 1
// 32 1
// 33 1
// 34 1
// 35 1
// 3e 1
// 3d 1
// 3f 1

// Edge coefficients
// (E  4 : tc  8 : shade 8 : z 2 : )
// 08 4
// 09 4         + 2
// 0a 4     + 8
// 0b 4     + 8 + 2
// 0c 4 + 8
// 0d 4 + 8     + 2
// 0e 4 + 8 + 8
// 0f 4 + 8 + 8 + 2
//
//if  (0 == cmd & 0xf0) //(cmd < x10)
//{
//	count = (cmd&8)>>1
//	+       (cmd&4)<<1
//	+       (cmd&2)<<2
//	+       (cmd&1)<<1
//}
//else if ((cmd >> 1)) == 18 // x24 or x25 (decimal 36 or 37)
//{
// count = 2
//}
//else
//1