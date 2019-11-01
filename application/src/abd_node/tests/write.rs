
use super::*;

mod message_sending;
mod termination;
mod variable_changes;

fn create_mediator_perform_write_and_ack_from_node_ids(node_ids: HashSet<NodeId>) -> Arc<MockMediator> {
    let mediator = create_mediator();
    let write_thread_handle =
        perform_single_write_on_background_thread(&mediator);
    wait_until_local_timestamp_is_updated(&mediator);
    send_write_ack_message_from_node_ids(&mediator, node_ids);
    write_thread_handle.join().unwrap();
    mediator
}

fn perform_single_write_on_background_thread(
    mediator: &Arc<MockMediator>,
) -> JoinHandle<()> {
    let mediator_for_write_thread = Arc::clone(&mediator);
    thread::spawn(move || {
        mediator_for_write_thread.write("Haskell".to_string());
    })
}

fn wait_until_local_timestamp_is_updated(
    mediator: &Arc<MockMediator>,
) {
    while *mediator
        .abd_node()
        .timestamp
        .lock()
        .unwrap()
        == 0
    {}
}

fn send_write_ack_message_from_node_ids(
    mediator: &Arc<MockMediator>,
    node_ids: HashSet<NodeId>,
) {
    let timestamp = mediator.abd_node().timestamp.lock().unwrap().clone();

    for &node_id in node_ids.iter() {
        let write_ack_message = WriteAckMessage {
            sender: node_id,
            timestamp: timestamp,
        };
        let json = mediator.abd_node().jsonify_message(&write_ack_message);
        mediator.json_received(&json);
    }
}
