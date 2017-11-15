# Decoding
This is a Rust implementation of a lab for D0013E Microcomputer engineering. The code from assignment 3b has been ported to run on a STM32F401 Nucleo-64 with semihosting.

A coded string is decoded and then sent to the host. Data is heap allocated using `static` declarations. This requires some `unsafe` code. If the coded data is not null terminated, the implementation tries to index outside the array and the program panics.

# Running
The program isn't interactive, so all you can do is and read the decoded string.

Compile the program with

```$ xargo build```

You can optionally compile in release mode using the `--release` flag. The target is already configured for the STM32F401. Next, connect the Nucleo-64 to a USB port on the host computer. Connect to it using OpenOCD.

```$ openocd -f interface/stlink-v2-1.cfg -f target/stm32f4x.cfg
Open On-Chip Debugger 0.10.0
Licensed under GNU GPL v2
For bug reports, read
	http://openocd.org/doc/doxygen/bugs.html
Info : auto-selecting first available session transport "hla_swd". To override use 'transport select <transport>'.
Info : The selected transport took over low-level target control. The results might differ compared to plain JTAG/SWD
adapter speed: 2000 kHz
adapter_nsrst_delay: 100
none separate
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : Unable to match requested speed 2000 kHz, using 1800 kHz
Info : clock speed 1800 kHz
Info : STLINK v2 JTAG v27 API v2 SWIM v15 VID 0x0483 PID 0x374B
Info : using stlink api v2
Info : Target voltage: 3.236079
Info : stm32f4x.cpu: hardware has 6 breakpoints, 4 watchpoints
```

Open another terminal and flash and debug the program using GDB.

```$ arm-none-eabi-gdb target/thumbv7em-none-eabihf/debug/coding_embedded```

This will give you a GDB prompt. Run the program.

```(gdb) continue```

OpenOCD will give some output, including the decoded string:

```
...
Info : halted: PC: 0x0800018c
Info : halted: PC: 0x0800018e
But unto the wicked God saith, What hast thou to do to declare  my statutes, or that thou shouldest take my covenant in thy mouth ?
```

# Memory safety
The compiler is unable to detect situations where the stack grows into and overlaps with the heap. Because the code uses a lot of `unsafe`, we also lose some of the memory safety that the compiler would normally provide. We are still protected against buffer overflows and over-reads, although the checks only happen at runtime.

# Performance
Code was added to measure the number of the recursive `decode` function takes for in debug and release mode. Note that it takes some cycles after resetting the clock counter before `decode` runs, as well as after `decode returns`, before the clock counter is read. The measurements in the table do not take this overhead into account. I tried to minimize this by using `DWT.get` when accessing the DWT instead of using the slower (but safer) `interrupt::free` block.

| Mode  |String                |Cycles|
|-------|:---------------------|-----:|
|debug  |abc                   |  4240|
|release|abc                   |   325|
|debug  |But unto the wicked...|172048|
|release|But unto the wicked...| 11589|
