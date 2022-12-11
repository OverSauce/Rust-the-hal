use core::ptr::{
  write_volatile,
  read_volatile,
};

/*
  Address Table:
    TCCR1A = 0x80
    TCCR1B = 0x81
    TCCR1C = 0x82
    TCNT1 = 0x84

    TIMSK1 = 0x6f

    TOIE1 = 0

  TCCR1B:
    ... - CS12 - CS11 - CS10

  Prescale Combinations:
    1    = 1<<CS10
    8    = 1<<CS11
    64   = 1<<CS11 | 1<<CS10
    256  = 1<<CS12
    1024 = 1<<CS12 | 1<<CS10
*/

pub const F_CPU: u32 = 16_000_000; // 16 MHz

#[repr(C)]
pub struct Timer { pub prescale: u16 }

impl Timer {
  pub fn init(prescale: u16) -> Self {
    /*
      Write the prescale value to the TCCR1B register
      while also intializing the prescale variable p.
    */
    let p = match prescale {
      1    => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0001); 1    },
      8    => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0010); 8    },
      64   => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0011); 64   },
      256  => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0100); 256  },
      1024 => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0101); 1024 },
      _    => unsafe { write_volatile(0x81 as *mut u8, 0b0000_0101); 1024 },
    };

    // TCCR1A = 0x00 Normal port operation
    unsafe { write_volatile(0x80 as *mut u8, 0b0000_0000); };

    Self { prescale: p }
  }

  #[inline(always)] // Inline makes great difference in program size in this function
  pub fn tick_tock(&self, ms: f32, clsr: impl FnOnce() -> () + core::marker::Copy) -> ! {
    /* 
      This is will cause a never ending loop.
      Until I think of a better and safer way, 
      I will only call it for simple timing stuff.
    */

    let count: u16 = {
      
      let secs = ms / 1000.0;
      let ticks = (F_CPU / self.prescale as u32) as u16;
      let ct = (secs * ticks as f32) as u16;

      match 0xFFFF - ct {
        0      => 1,
        0xFFFF => 0xFFFF - 1,
        _      => 0xFFFF - ct,
      }
    };

    // Never ending loop where closure is called every time the timer overflows
    loop {
      clsr();
      unsafe { write_volatile(0x84 as *mut u16, count); }
      while unsafe { read_volatile(0x84 as *mut u16) } != 0 { /* Wait until 0 */ }
    }
  }
}
