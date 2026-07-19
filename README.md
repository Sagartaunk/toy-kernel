# toy-kernel

A freestanding `x86_64` kernel written in Rust, built by following the
[*Writing an OS in Rust*](https://os.phil-opp.com/) series. This is a learning
project, currently through paging / heap-allocation territory (posts 1–11 of
the series), and is not intended for production use.

## NOTE: 
I have followed `Phillip Oppermann's` blog for building this 
kernel. You can check it out here: `https://os.phil-opp.com`.

## What's implemented so far

- **Freestanding binary** — `#![no_std]` / `#![no_main]` with a custom entry
  point via the `bootloader` crate's `entry_point!` macro.
- **VGA text-mode output** — a `Writer` over the `0xb8000` VGA buffer with
  `print!` / `println!` macros.
- **Serial output** — a 16550 UART driver used to pipe kernel output to the
  host terminal when running under QEMU (`serial_print!` / `serial_println!`).
- **Custom test framework** — `#![feature(custom_test_frameworks)]` with a
  `Testable` trait and a `test_runner` that exits QEMU with a status code via
  the `isa-debug-exit` device.
- **CPU exception handling** — an IDT with breakpoint and double-fault
  handlers, plus a page-fault handler that reads `CR2` and halts.
- **A dedicated double-fault stack** — a GDT/TSS setup using the Interrupt
  Stack Table so double faults don't triple-fault on a corrupted stack.
- **Hardware interrupts** — a chained 8259 PIC setup driving a timer tick and
  a PS/2 keyboard handler (scancode → `DecodedKey` via `pc_keyboard`).
- **Paging groundwork** — reading the active level-4 page table from `CR3`,
  and a `BootInfoFrameAllocator` that walks the bootloader-provided memory map
  for usable physical frames.

## Project layout

```
src/
├── main.rs        # Binary entry point, panic handlers, test runner glue
├── lib.rs          # Library entry point re-exporting all kernel modules
├── gdt.rs          # Global Descriptor Table + Task State Segment (IST)
├── interrupts.rs   # IDT, exception handlers, PIC-driven hardware interrupts
├── memory.rs       # Level-4 page table access, physical frame allocator
├── serial.rs       # UART 16550 driver + serial_print!/serial_println!
├── vga_buffer.rs   # VGA text buffer driver + print!/println!
└── tests.rs        # Custom test harness, QemuExitCode, Testable trait
```

## Module reference

### `main.rs`
The binary crate. `#![no_std]` and `#![no_main]` disable the standard library
and the normal `main` entry point, since — as the module doc comment notes —
under `no_main` the compiler instead expects `_start`, provided here by
`bootloader::entry_point!(kernel_main)`.

Two panic handlers exist side by side, gated by `cfg(test)`:
- Normal builds print the panic info to the VGA buffer and loop forever.
- Test builds print to serial, exit QEMU with `QemuExitCode::Failed`, then
  `hlt_loop()`.

`kernel_main` currently walks the level-4 page table and prints every
non-unused entry — a diagnostic left over from following along with the
paging chapter, per the comment `L4 Entry {}: {:?}` in the loop.

### `lib.rs`
The library crate that `main.rs` (and the test binaries) depend on. It wires
up the shared `#![no_std]` / custom-test-framework attributes and exposes
`kernel::init()`, which — in order — loads the GDT, loads the IDT, remaps and
initializes the PICs, and finally enables interrupts with `sti`.

`hlt_loop()` is documented as letting "the cpu halt until the next
instruction," which parks the core in a low-power state between interrupts
instead of busy-spinning.

### `gdt.rs`
Sets up a minimal GDT with a kernel code segment and a TSS segment. The TSS
reserves a dedicated 20 KiB stack (`4096 * 5`) at
`interrupt_stack_table[DOUBLE_FAULT_IST_INDEX]`, which `interrupts.rs` then
points the double-fault handler at via `set_stack_index`. This is what stops
a double fault from double-faulting again on a blown/corrupted stack.

`init()` loads the GDT, reloads the `CS` segment register, and loads the TSS
selector — all three are required for the new GDT to actually take effect.

### `interrupts.rs`
Builds the IDT and installs handlers:
- `breakpoint_handler` — pretty-prints the stack frame (used by the
  `int3`-triggering `test_breakpoint_exception` test).
- `double_fault_handler` — panics on the dedicated IST stack from `gdt.rs`.
- `page_fault_handler` — reads the faulting address from `CR2` and the
  `PageFaultErrorCode`, then calls `hlt_loop()`.
- `timer_interrupt_handler` / `keyboard_interrupt_handler` — hardware
  interrupts routed through a `ChainedPics` (primary offset `32`, secondary
  immediately after). Each handler sends an End-Of-Interrupt signal; per the
  inline `SAFETY` comment, the caller has to use the *correct* interrupt
  vector or risk "an unset interrupt could be deleted or a system hang."
  The keyboard handler decodes scancodes with a `Us104Key` layout and prints
  the resulting Unicode character or raw key.

`InterruptIndex` maps the PIC's hardware IRQ lines onto usable IDT vector
numbers (`Timer = 32`, `Keyboard = 33`).

### `memory.rs`
`active_level_4_table` converts the physical address in `CR3` into a virtual
address using the bootloader's physical-memory offset mapping, and returns a
`&'static mut PageTable`. The doc comment is explicit about the two safety
invariants this relies on: the *entire* physical memory must be mapped at
that offset, and the function **must only be called once**, since two live
`&mut` references to the same table would be undefined behavior.

`BootInfoFrameAllocator` implements `FrameAllocator<Size4KiB>` by filtering
the bootloader's `MemoryMap` down to `MemoryRegionType::Usable` regions,
splitting each region into 4 KiB-aligned frame addresses, and handing them
out one at a time via `.nth(self.next)`. This is a simple, non-reclaiming
bump allocator — frames are never freed.

### `serial.rs`
Wraps a `Uart16550Tty` on the standard COM1 I/O port (`0x3F8`) behind a
`spin::Mutex`, matching the comment that "the rest of the ports are
calculated by `Uart` itself." `serial_print!` / `serial_println!` write to
this port so kernel output (including test results) is visible in the host
terminal when QEMU is run headless (`-serial stdio`).

### `vga_buffer.rs`
A `Writer` over the memory-mapped VGA text buffer at `0xb8000`
(`BUFFER_WIDTH = 80`, `BUFFER_HEIGHT = 25`). Each cell is a `ScreenChar`
(ASCII byte + `ColorCode`), accessed through `Volatile` to stop the compiler
from optimizing away writes the CPU thinks are dead. `ColorCode` packs
foreground and background into a single byte, background in the high
nibble — safe, per the doc comment, only because `Color` never exceeds the
4 bits it's given. `write_string` filters to printable ASCII plus `\n`,
substituting `0xfe` for anything outside that range. Includes three
`#[test_case]` tests exercising simple output, buffer scrolling
(`test_println_many`), and that written characters actually land in the
expected buffer cells (`test_println_output`).

### `tests.rs`
The custom test harness. `Testable` is blanket-implemented for any `Fn()`,
wrapping each test with a name print, the test body, and an `[OK]` line.
`exit_qemu` writes a `QemuExitCode` (`Success = 0x10`, `Failed = 0x11`) to
I/O port `0xf4`, which QEMU's `isa-debug-exit` device turns into a process
exit code — this is how `cargo test` can determine pass/fail from inside a
kernel with no OS underneath it to report back to.

## Building & running

This kernel targets bare-metal `x86_64` and needs nightly Rust:

```bash
rustup override set nightly
rustup component add rust-src llvm-tools-preview
```

**Important:** pin `bootloader = "0.9"` in `Cargo.toml`. The 0.11.x line
changed its API significantly (no more `entry_point!`/`BootInfo` in the same
shape) and will not build against this code as written.

Build and run in QEMU with `bootimage`:

```bash
cargo install bootimage
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86_64-<target>/debug/bootimage-<crate>.bin
```

Run the test suite (exits QEMU automatically via the `isa-debug-exit`
device and reports pass/fail through the process exit code):

```bash
cargo test
```

A custom target JSON and a `-Zbuild-std` config are expected in
`.cargo/config.toml` per the standard phil-opp setup, since `core` and
`alloc` need to be rebuilt for the bare-metal target.

## Known limitations 

- `BootInfoFrameAllocator` never reclaims frames there's no `deallocate`.
- No heap allocator is wired up yet, so `alloc`-based collections aren't
  available.
- The diagnostic level-4 table dump in `kernel_main` is exploratory.
- `hlt_loop()` after a page fault means the kernel currently halts rather
  than recovering from a fault.

## Learning Notes:
This field contains stuff I think is important and will be writing
here for later reference.
-> `#[unsafe(no_mangle)]` This macro is used to tell rust not to 
change the name of the function. Otherwise, `Rust` will convert it
to some arbitrary string to differentiate between the functions.
-> `#[repr(X)]` is used to tell rust to follow a specific semantic 
for a struct. For example: 
  `#[repr(u8)]` formats the fields as `u8` by default.
  `#[repr(transparent)]` formats the fields exactly as written in 
  binary.
