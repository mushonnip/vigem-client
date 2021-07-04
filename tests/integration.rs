use vigem_client as vigem;

#[test]
fn simple_success() {
	let mut target = vigem::Xbox360Wired::new(
		vigem::Client::connect().unwrap(),
		vigem::TargetId::XBOX360_WIRED);

	target.plugin().unwrap();
	target.wait_ready().unwrap();
	let result = target.update(&vigem::XGamepad::default());

	assert_eq!(result, Ok(()));
}

#[test]
fn target_not_ready() {
	let mut target = vigem::Xbox360Wired::new(
		vigem::Client::connect().unwrap(),
		vigem::TargetId::XBOX360_WIRED);

	target.plugin().unwrap();
	let result = target.update(&vigem::XGamepad::default());

	assert_eq!(result, Err(vigem::Error::TargetNotReady));
}
