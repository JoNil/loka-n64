arch n64.rsp
endian msb
output "hello_world.bin", create

include "lib/n64.inc"
include "lib/n64_rsp.inc"

base $0000
origin $0000

align(8)
start:
    j start
    nop // Delay Slot