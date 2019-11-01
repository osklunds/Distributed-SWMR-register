
use super::*;

#[test]
fn test_that_write_sends_correct_write_messages() {
    let mediator = create_mediator_perform_write_and_ack_from_node_ids(node_ids_for_tests());
    
    let timestamp = mediator.abd_node().timestamp.lock().unwrap();
    let value = mediator.abd_node().value.lock().unwrap();
    let expected_write_message = WriteMessage {
        sender: mediator.node_id,
        timestamp: *timestamp,
        value: value.clone(),
    };

    for write_message in
        mediator.sent_write_messages.lock().unwrap().iter()
    {
        assert_eq!(*write_message, expected_write_message);
    }

    assert_eq!(mediator.sent_write_messages.lock().unwrap().len(), 4);

    assert_eq!(
        *mediator
            .write_message_receivers
            .lock()
            .unwrap(),
        mediator.node_ids
    );
}

#[test]
fn test_that_a_write_message_reception_sends_correct_write_ack_message() {
    let mediator = create_mediator();

    let write_message = WriteMessage {
        sender: 3,
        timestamp: 7,
        value: "Haskell".to_string(),
    };
    let json = mediator.abd_node().jsonify_message(&write_message);
    mediator.json_received(&json);

    let expected_write_ack_message = WriteAckMessage {
        sender: mediator.node_id(),
        timestamp: 7,
    };

    let sent_write_ack_messages = mediator.sent_write_ack_messages.lock().unwrap().clone();
    assert_eq!(sent_write_ack_messages, vec![expected_write_ack_message]);
}