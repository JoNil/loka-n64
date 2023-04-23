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
    //addi t7, t7, 4
    // Loop between 2048 - 4195
    //andi t7, t7, 4095
    //ori t7, t7, 2048
}

macro DbgDisableSingleStep (tmp_reg, ss_reg) {
    // Read if in Single step
    if 0 {
        mfc0 {ss_reg}, c4
        andi {ss_reg}, {ss_reg}, RSP_STP
        li {tmp_reg}, CLR_STP
        mtc0 {tmp_reg}, c4
    }
}

macro DbgEnableSingleStep (tmp_reg, ss_reg) {    
    //li {tmp_reg}, 0x40
    // If was in single step before disable, ss_reg will contain 0x20
    if 0 {
        sll {tmp_reg}, {ss_reg}, 1
        mtc0 {tmp_reg}, c4
    }
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

    // Make sure rdp is not using command buffer which will be overwritten by dma
    DbgDisableSingleStep(t5, 30)
wait_rdp_busy_pre_cpu_to_dmem:
    mfc0 t5, c11
    andi t5, t5, RDP_CMB
    bne t5, 0, wait_rdp_busy_pre_cpu_to_dmem // Loop while rdp busy.
    nop
    DbgEnableSingleStep(t5, 30)

    //li t5, SET_SG2|CLR_SG0
    //mtc0 t5, c4
    
    // DMEM
    // Request semaphore
if 0 {    
    DbgDisableSingleStep(t5, 30)
request_semaphore:
    mfc0 t5, c7
    bnez t5, request_semaphore
    nop
    DbgEnableSingleStep(t5, 30)
}

    //li t5, CLR_SG1
    //mtc0 t5, c4

    DbgPrint(t5)
    
    li   t5, 1
    DbgPrint(t5)

    //li t5, SET_SG3
    //mtc0 t5, c4

    // Wait until spot available in DMA
    DbgDisableSingleStep(t5, 30)
wait_dma_available:
    if 1 {
        mfc0 t5, c4
        andi t5, RSP_BSY|RSP_FUL
        DbgPrint(t5)
    } else {
        mfc0 t5, c5
    }
    bne t5, 0, wait_dma_available
    nop
    DbgEnableSingleStep(t5, 30)

    //li t5, CLR_SG2
    //mtc0 t5, c4
    
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

    //li t5, SET_SG4
    //mtc0 t5, c4

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

    //li t5, CLR_SG3
    //mtc0 t5, c4

    li   t5, 42
    DbgPrint(t5)
// TMP debugprint rdp status
    mfc0 t5, c11
    DbgPrint(t5)

    // Release semaphore
if 0 {
    mtc0 0, c7 
}

    DbgPrint(t5)

    //li t5, CLR_SG4|SET_SG5
    //mtc0 t5, c4
    
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

    //li t5, SET_SG6
    //mtc0 t5, c4

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

    //li t5, SET_SG0|SET_SG1|SET_SG2
    //mtc0 t5, c4

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
    //li t8, CLR_STP
    //mtc0 t8, c4

    li t5, 9101
    DbgPrint(t5)

    // Store upper bits of clock in signal bits.
    //set_signals_from_upper_clock(t2, t3, t4)

    li t5, 1213
    DbgPrint(t5)

    //li t5, SET_SG7
    //mtc0 t5, c4

    mfc0 t5, c4
    DbgPrint(t5)

    //li t5, SET_HLT
    //mtc0 t5, c4
    
    break
    nop
