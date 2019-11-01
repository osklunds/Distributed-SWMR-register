
use super::*;

#[test]
fn test_that_local_timestamp_and_value_are_updated_correctly_on_write() {
    let mediator = create_mediator();
    perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);

    let timestamp = mediator.abd_node().timestamp.lock().unwrap();
    assert_eq!(*timestamp, 1);

    let value = mediator.abd_node().value.lock().unwrap();
    assert_eq!(&*value, "Haskell");
}

#[test]
fn test_that_a_write_message_updates_local_timestamp_and_value_if_greater() {
    let mediator = create_mediator();
    *mediator.abd_node().timestamp.lock().unwrap() = 20;

    let write_message = WriteMessage {
        sender: 2,
        timestamp: 30,
        value: "Rust".to_string(),
    };
    let json = mediator.abd_node().jsonify_message(&write_message);
    mediator.json_received(&json);

    assert_eq!(*mediator.abd_node().timestamp.lock().unwrap(), 30);
    assert_eq!(*mediator.abd_node().value.lock().unwrap(), "Rust".to_string());
}

#[test]
fn test_that_a_write_message_does_not_updates_local_timestamp_and_value_if_equal() {
    let mediator = create_mediator();
    *mediator.abd_node().timestamp.lock().unwrap() = 20;

    let write_message = WriteMessage {
        sender: 2,
        timestamp: 20,
        value: "Rust".to_string(),
    };
    let json = mediator.abd_node().jsonify_message(&write_message);
    mediator.json_received(&json);

    assert_eq!(*mediator.abd_node().timestamp.lock().unwrap(), 20);
    assert_eq!(*mediator.abd_node().value.lock().unwrap(), "".to_string());
}

#[test]
fn test_that_a_write_message_does_not_updates_local_timestamp_and_value_if_smaller() {
    let mediator = create_mediator();
    *mediator.abd_node().timestamp.lock().unwrap() = 20;

    let write_message = WriteMessage {
        sender: 2,
        timestamp: 3,
        value: "Rust".to_string(),
    };
    let json = mediator.abd_node().jsonify_message(&write_message);
    mediator.json_received(&json);

    assert_eq!(*mediator.abd_node().timestamp.lock().unwrap(), 20);
    assert_eq!(*mediator.abd_node().value.lock().unwrap(), "".to_string());
}




/*
#[test]
fn test_that_a_write_message_does_not_change_register_being_written() {
    let mediator = create_mediator();
    send_a_register_array_in_a_write_message(&mediator);
    assert_eq!(
        *mediator
            .abd_node()
            .register_array_being_written
            .lock()
            .expect("Could not lock register array being written"),
        None
    );
}

#[test]
fn test_that_a_write_ack_message_updates_own_register_array() {
    let mediator = create_mediator();
    let reg_array =
        send_a_register_array_in_a_write_ack_message(&mediator);
    let reg_array_abd_node = mediator
        .abd_node()
        .reg
        .lock()
        .expect("Could not lock register array.");

    assert_eq!(*reg_array_abd_node, reg_array);
}

fn send_a_register_array_in_a_write_ack_message(
    mediator: &Arc<MockMediator>,
) -> RegisterArray<String> {
    let mut reg_array = mediator
        .abd_node()
        .reg
        .lock()
        .expect("Could not lock register array.")
        .clone();
    reg_array.set(2, Register::new(7, "Haskell".to_string()));
    reg_array.set(3, Register::new(10, "Idris".to_string()));

    let write_ack_message = WriteAckMessage {
        sender: 2,
        register_array: Cow::Owned(reg_array.clone()),
    };
    let json = serde_json::to_string(&write_ack_message)
        .expect("Could not serialize a write ack message");

    mediator.json_received(&json);
    reg_array
}

#[test]
fn test_that_a_write_ack_message_does_not_change_register_array_being_written_when_there_is_no_write(
) {
    let mediator = create_mediator();
    send_a_register_array_in_a_write_message(&mediator);
    let register_array_being_written = mediator
        .abd_node()
        .register_array_being_written
        .lock()
        .expect("Could not lock register array being written.");

    assert_eq!(*register_array_being_written, None);
}

#[test]
fn test_that_a_write_ack_message_does_not_change_acking_processors_for_write_when_there_is_no_write(
) {
    let mediator = create_mediator();
    send_a_register_array_in_a_write_message(&mediator);
    let acking_processors_for_write = mediator
        .abd_node()
        .acking_processors_for_write
        .lock()
        .expect("Could not lock acking_processors_for_write.");

    assert!(acking_processors_for_write.is_empty());
}

*/