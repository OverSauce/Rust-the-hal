#[panic_handler]
pub fn panik(_info: &core::panic::PanicInfo<'_>) -> ! {
  // Just loop forever when panic happens
	loop {}
}
