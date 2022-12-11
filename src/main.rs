#![no_std]      // No standard library 
#![no_main]     // No standard Rust entry point

mod avr;
use avr::{
    pin::Port,
    timer::Timer,
};

/*
	This function will be exported as "main" 
	gcc-avr will look for this name as the entry point
	This is why we prevent it's name from being mangled
*/
#[no_mangle]    
pub extern "C" fn main() -> ! {

	let timer = Timer::init(1024);
	let portb = Port::set(0x25, 5, 1);
	
	timer.tick_tock(100.0, || {
		// Curly braces are required here, 
		// because this closure shouldn't return anything 
		let _ = portb.toggle(5); // Consume the return value
	});
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo<'_>) -> ! {
  // Just loop forever when panic happens
	loop {}
}
