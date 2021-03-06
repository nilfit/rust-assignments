#![no_std]

extern crate cortex_m_semihosting;
extern crate stm32f40x;

use core::fmt::Write;
use core::str::from_utf8;

use stm32f40x::DWT;

use cortex_m_semihosting::hio;

static mut SEED: u32 = 0x0e0657c1;
// coded string "abc"
/*
static CODED: [u32; 4] = [0x9fdd9158, 0x85715808, 0xac73323a, 0];
static mut DECODED: [u8; 4] = [0; 4];
*/
// the real coded data
static CODED: [u32; 132] = [0x015e7a47, 0x2ef84ebb, 0x177a8db4, 0x1b722ff9, 0x5dc7cff0, 0x5dc9dea6, 0x1da0c15a, 0xe4c236a2, 0x3d16b0d0, 0x1f397842, 0xaae0d2ba, 0x11246674, 0x0845317f, 0xd5512dad, 0xb6184977, 0xd293a53e, 0x7d9c2716, 0xd917eae6, 0xd8852384, 0x286e46f9, 0xce566029, 0xcefe7daf, 0x62d726d4, 0x0dbaeb2d, 0x95f57c60, 0xed515141, 0x29b77d0f, 0x9f7b8d0c, 0x45a8395a, 0xfead2b72, 0x883d434c, 0xed8ddf60, 0xe51e65e4, 0x19bf6bb1, 0xfeb505ec, 0x662aa23c, 0xf6827cf8, 0xd1dc7a5c, 0x4fa5b066, 0x7ddd25a4, 0xa8ba8e8a, 0x72846227, 0xf8f636fb, 0x2b389a9c, 0xe4038bf6, 0x6e169877, 0xad028132, 0x84dbfe8c, 0x243762ff, 0x59c8f80c, 0xb6e0db4b, 0xedb8cab7, 0xcd4b39f6, 0xaf263741, 0x18d9965f, 0x1ab1f037, 0x5b458792, 0xc94d960d, 0xd45cedea, 0x2160aca3, 0x93c77766, 0x2d66e105, 0x9ff74d4f, 0x6dc22f21, 0x6b03d689, 0x5fc48de0, 0x1138f000, 0xccb58e57, 0xf9c8e200, 0x7ab26e3c, 0xc61dcb3e, 0x6aefccb0, 0x7a452f05, 0xa5cf0731, 0xa249383f, 0x628fe534, 0xcad81710, 0x7f616276, 0x3ce18308, 0xed4857ff, 0xd1e5b1d1, 0xc2e84dc2, 0xaa003742, 0xaf637488, 0x831afc48, 0x287a69a0, 0x6e04546e, 0x13dffa07, 0x3232fb10, 0xd69e2e09, 0x355d8dc7, 0xef902301, 0x9a89ac15, 0x967dc900, 0x08dc2b1c, 0x6b5be690, 0x894b0e02, 0xe26af9af, 0xa6fd3b23, 0xfcf213e5, 0x85217608, 0x7fd3be8b, 0xa2e757fb, 0x3717a341, 0x85ee426d, 0x394bb856, 0x12ac98c3, 0xec7d4ab5, 0x721b6989, 0x30e36360, 0xaa018403, 0x9ee61196, 0xa8697adc, 0x51e9d65a, 0x11023594, 0xc4c4b36b, 0xda80bf7a, 0xbd5a645e, 0x18cea918, 0xa723dda8, 0x0126c05e, 0x2962d48a, 0xd5f7d312, 0xb8947041, 0x7c1e2e9a, 0x945eeac3, 0x7110fb1c, 0xa7bc72cc, 0xdf47dfbb, 0x09a1c6c8, 0xc2e41061, 0];
static mut DECODED: [u8; 132] = [0; 132];

fn main() {
    let mut stdout = hio::hstdout().unwrap();
    unsafe {
        (*DWT.get()).enable_cycle_counter();
        (*DWT.get()).cyccnt.write(0);
        decode(&CODED, &mut DECODED);
        let cnt = (*DWT.get()).cyccnt.read();
        let _ = writeln!(stdout, "decoded in {} cycles", cnt);
    }
    unsafe {
        let _ = writeln!(stdout, "{}", from_utf8(&DECODED[..]).unwrap());
    }
}


fn codgen() -> u32 {
    unsafe {
        let n: u32 = SEED.count_zeros();
        let x: u32 = SEED.rotate_left(30);
        let y: u32 = (SEED as i32 >> 6) as u32;
        SEED = x ^ y ^ n;
        SEED ^ 0x464b713e
    }
}

fn decode(wordarr: &[u32], bytearr: &mut [u8]) -> u32 {
    // one's complement
    let x = 0xffffffff ^ codgen();
    if wordarr[0] == 0 {
        bytearr[0] = 0;
        x
    } else {
        let y = decode(&wordarr[1..], &mut bytearr[1..]);
        let m = x.wrapping_sub(y).wrapping_sub(wordarr[0]);
        bytearr[0] = (m >> 13) as u8;
        // two's complement
        let r: u32 = - (codgen() as i32) as u32;
        x.wrapping_add(y).wrapping_add(m).wrapping_add(r).wrapping_add(5)
    }
}
