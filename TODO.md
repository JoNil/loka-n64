# loka-n64

- Make gpu paralell with cpu and sync correctly

Fixes
- Project with less fov
- Remove all calls to memory_barrier, they emit unneeded cache 0x0 instructions

Ideas
- Z buffer
- Rsp triangle setup
- Spawn Wave
- 640x480

Optimizations
- Font to 1 bit per pixel texture

Crazy idea
- Spirv till rsp?