
use super::*;

#[test]
fn test_that_value_is_default() {
    let mediator = create_mediator();
    assert_eq!(*mediator.abd_node().value.lock().unwrap(), String::default());
}

#[test]
fn test_that_timestamp_is_0() {
    let mediator = create_mediator();
    assert_eq!(
        *mediator.abd_node().timestamp.lock().unwrap(),
        0
    );
}

#[test]
fn test_that_read1_sequence_number_is_0() {
    let mediator = create_mediator();
    assert_eq!(
        *mediator.abd_node().read1_sequence_number.lock().unwrap(),
        0
    );
}

#[test]
fn test_that_read2_sequence_number_is_0() {
    let mediator = create_mediator();
    assert_eq!(
        *mediator.abd_node().read2_sequence_number.lock().unwrap(),
        0
    );
}

#[test]
fn test_that_quorums_are_idle() {
    let mediator = create_mediator();

    assert!(mediator.abd_node().write_quorum.is_idle());
    assert!(mediator.abd_node().read1_quorum.is_idle());
    assert!(mediator.abd_node().read2_quorum.is_idle());
}