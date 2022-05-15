# loka-n64

Fixes
- Project with less fov
- Remove all calls to memory_barrier, they emit unneeded cache 0x0 instructions

Ideas
- Z buffer
- Rsp triangle setup
- Spawn Wave
- 640x480

Optimizations
- Console gpu work overlap with cpu work, already does??? What is our gpu time?
- Font to 1 bit per pixel texture

Crazy idea
- Spirv till rsp?