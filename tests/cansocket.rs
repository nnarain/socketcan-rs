use socketcan_hal::{constants::*, CanFrame, CanSocket, ShouldRetry};
use std::time;

#[test]
fn test_nonexistant_device() {
    assert!(CanSocket::open("invalid").is_err());
}

#[test]
fn vcan0_timeout() {
    let cs = CanSocket::open("vcan0").unwrap();
    cs.set_read_timeout(time::Duration::from_millis(100))
        .unwrap();
    assert!(cs.read_frame().should_retry());
}

#[test]
fn vcan0_set_error_mask() {
    let cs = CanSocket::open("vcan0").unwrap();
    cs.set_error_mask(ERR_MASK_ALL).unwrap();
    cs.set_error_mask(ERR_MASK_NONE).unwrap();
}

#[test]
fn vcan0_enable_own_loopback() {
    let cs = CanSocket::open("vcan0").unwrap();
    cs.set_loopback(true).unwrap();
    cs.set_recv_own_msgs(true).unwrap();

    let frame = CanFrame::init(0x123, &[], true, false).unwrap();

    cs.write_frame(&frame).unwrap();

    cs.read_frame().unwrap();
}

// #[test]
// fn vcan0_set_down() {
//     let can_if = CanInterface::open("vcan0").unwrap();
//     can_if.bring_down().unwrap();
// }

#[test]
fn vcan0_test_nonblocking() {
    let cs = CanSocket::open("vcan0").unwrap();
    assert!(cs.set_nonblocking(true).is_ok());

    // no timeout set, but should return immediately
    assert!(cs.read_frame().should_retry());
}
