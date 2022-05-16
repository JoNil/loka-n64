# loka-n64

- Make gpu paralell with cpu and sync correctly
- Reexport everything from n64 crate
- Remove all calls to memory_barrier, they emit unneeded cache 0x0 instructions
- Remove lazy_static use once_cell

Fixes
- Project with less fov

Ideas
- Z buffer
- Rsp triangle setup
- Spawn Wave
- 640x480

Optimizations
- Font to 1 bit per pixel texture

Crazy idea
- Spirv till rsp?