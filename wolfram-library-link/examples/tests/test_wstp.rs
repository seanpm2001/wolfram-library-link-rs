use wolfram_library_link::{
    self as wll,
    wstp::{self, Link},
};

wll::export_wstp![
    test_wstp_fn_empty;
    test_wstp_fn_panic_immediately;
    test_wstp_fn_panic_immediately_with_formatting;
    test_wstp_panic_with_empty_link;
    test_wstp_fn_poison_link_and_panic;
];

fn test_wstp_fn_empty(_link: &mut Link) {
    // Do nothing.
}

fn test_wstp_fn_panic_immediately(_link: &mut Link) {
    panic!("successful panic")
}

fn test_wstp_fn_panic_immediately_with_formatting(_link: &mut Link) {
    panic!("successful {} panic", "formatted")
}

/// Test that the wrapper function generated by `export_wstp!` will correctly handle
/// panicking when `link` has been left in a `!link.is_ready()` state.
fn test_wstp_panic_with_empty_link(link: &mut Link) {
    link.raw_get_next().unwrap();
    link.new_packet().unwrap();

    assert!(!link.is_ready());
    assert!(link.error().is_none());

    // Panic while there is no content on `link` to be cleared by the panic handler.
    // This essentially tests that the panic handler checks `if link.is_ready() { ... }`
    // before trying to clear content off of the link.
    panic!("panic while !link.is_ready()");
}

/// Test that the wrapper function generated by `export_wstp!` will check for and clear
/// any link errors that might have occurred within the user code.
fn test_wstp_fn_poison_link_and_panic(link: &mut Link) {
    // Cause a link failure by trying to Get the wrong head.
    assert!(link.test_head("NotTheRightHead").is_err());

    // Assert that the link now has an uncleared error.
    assert_eq!(link.error().unwrap().code(), Some(wstp::sys::WSEGSEQ));

    // Verify that trying to do an operation on the link returns the same error as
    // `link.error()`.
    assert_eq!(
        link.put_str("some result").unwrap_err().code(),
        Some(wstp::sys::WSEGSEQ)
    );

    // Panic while leaving the link in the error state.
    panic!("successful panic")
}
