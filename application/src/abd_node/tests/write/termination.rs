
use super::*;

// If writes don't terminates, neither will the tests.
#[test]
fn test_that_write_terminates() {
    create_mediator_perform_write_and_ack_from_node_ids(node_ids_for_tests());
}

#[test]
fn test_that_write_terminates_even_if_not_all_nodes_ack() {
    let mut node_ids = HashSet::new();
    node_ids.insert(1);
    node_ids.insert(2);
    node_ids.insert(3);

    create_mediator_perform_write_and_ack_from_node_ids(node_ids);
}

#[test]
fn test_that_write_does_not_terminate_without_acks() {
    let mediator = create_mediator();
    perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);

    check_that_write_fails(&mediator);
}

#[test]
fn test_that_write_does_not_terminate_without_acks_from_majority() {
    let mediator = create_mediator();
    perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);

    let mut node_ids = HashSet::new();
    node_ids.insert(2);
    node_ids.insert(3);
    send_write_ack_message_from_node_ids(&mediator, node_ids);

    check_that_write_fails(&mediator);
}

fn check_that_write_fails(mediator: &Arc<MockMediator>) {
    // If this test terminates, it means that retransmissions had
    // to be done, because no write acks received.
    while mediator
        .sent_write_messages
        .lock()
        .unwrap()
        .len()
        <= node_ids_for_tests().len() * 3
    {}

    let accessing = mediator.abd_node().write_quorum.accessing().lock().unwrap();

    assert!(*accessing);
}