use core::ptr::{
  write_volatile,
  read_volatile,
};

/*
  Port Addressess:
    DDRB  = 0x24
    PORTB = 0x25
    DDRC  = 0x27
    PORTC = 0x28
    DDRD  = 0x2A
    PORTD = 0x2B
  Then:
    DDRx = PORTx - 0x01
*/

#[repr(C)]
pub struct Port { p_addr: u8 }

impl Port {
  pub fn set(port_addr: u8, pin_num: u8, io: u8) -> Self {
    unsafe { write_volatile((port_addr - 0x01) as *mut u8, io<<pin_num); }
    Self { p_addr: port_addr }
  }
  pub fn toggle(&self, pin_num: u8) -> u8 {
    let state = unsafe { read_volatile(self.p_addr as *mut u8) };
    unsafe { write_volatile(self.p_addr as *mut u8, state ^ (1<<pin_num)); };
    
    // Return the new state of the pin
    state ^ (1<<pin_num) & (1<<pin_num)
  }
}
