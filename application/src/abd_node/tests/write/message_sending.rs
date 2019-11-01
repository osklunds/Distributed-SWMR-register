
use super::*;

#[test]
fn test_that_write_sends_correct_write_messages() {
    let mediator = create_mediator_perform_write_and_ack_from_node_ids(node_ids_for_tests());
    check_that_sent_write_messages_are_the_expected_form(&mediator);
}

fn check_that_sent_write_messages_are_the_expected_form(
    mediator: &Arc<MockMediator>,
) {
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
}

#[test]
fn test_that_write_sends_write_messages_to_the_correct_nodes() {
    let mediator = create_mediator_perform_write_and_ack_from_node_ids(node_ids_for_tests());
    check_that_write_messages_are_sent_to_the_correct_nodes(&mediator);
}

fn check_that_write_messages_are_sent_to_the_correct_nodes(
    mediator: &Arc<MockMediator>,
) {
    assert_eq!(
        *mediator
            .write_message_receivers
            .lock()
            .unwrap(),
        mediator.node_ids
    );
}
/*
#[test]
fn test_that_write_sends_correct_write_ack_messages() {
    let mediator = create_mediator_perform_write_and_ack();
    check_that_sent_write_ack_messages_are_the_expected_form(
        &mediator,
    );
}

fn check_that_sent_write_ack_messages_are_the_expected_form(
    mediator: &Arc<MockMediator>,
) {
    let reg_array = mediator.abd_node().reg.lock().unwrap();
    let expected_write_ack_message = WriteAckMessage {
        sender: mediator.node_id,
        register_array: Cow::Borrowed(&reg_array),
    };

    for write_ack_message in
        mediator.sent_write_ack_messages.lock().unwrap().iter()
    {
        assert_eq!(*write_ack_message, expected_write_ack_message);
    }
}
*/