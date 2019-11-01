
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
fn test_that_a_write_message_does_not_update_local_timestamp_and_value_if_equal() {
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
fn test_that_a_write_message_does_not_update_local_timestamp_and_value_if_smaller() {
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

#[test]
fn test_that_a_write_message_does_not_make_the_write_quorum_non_idle() {
    let mediator = create_mediator();
    let write_message = WriteMessage {
        sender: 2,
        timestamp: 3,
        value: "Rust".to_string(),
    };
    let json = mediator.abd_node().jsonify_message(&write_message);
    mediator.json_received(&json);

    assert!(mediator.abd_node().write_quorum.is_idle());
}

#[test]
fn test_that_a_write_ack_message_does_not_change_local_timestamp() {
    let mediator = create_mediator();
    let write_ack_message = WriteAckMessage {
        sender: 2,
        timestamp: 3,
    };
    let json = mediator.abd_node().jsonify_message(&write_ack_message);
    mediator.json_received(&json);

    assert_eq!(*mediator.abd_node().timestamp.lock().unwrap(), 0);
}

#[test]
fn test_that_a_write_ack_message_does_not_cause_idle_quorum_to_become_non_idle() {
    let mediator = create_mediator();
    let write_ack_message = WriteAckMessage {
        sender: 2,
        timestamp: 3,
    };
    let json = mediator.abd_node().jsonify_message(&write_ack_message);
    mediator.json_received(&json);

    assert!(mediator.abd_node().write_quorum.is_idle());
}

#[test]
fn test_that_a_write_ack_with_wrong_timestamp_does_not_count_as_an_ack() {
    let mediator = create_mediator();
    perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);

    let write_ack_message = WriteAckMessage {
        sender: 2,
        timestamp: 3003,
    };
    let json = mediator.abd_node().jsonify_message(&write_ack_message);
    mediator.json_received(&json);

    assert!(mediator.abd_node().write_quorum.acking_nodes().lock().unwrap().is_empty());
}

#[test]
fn test_that_write_causes_quorum_to_become_non_idle() {
    let mediator = create_mediator();
    perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);

    assert!(!mediator.abd_node().write_quorum.is_idle());
}
