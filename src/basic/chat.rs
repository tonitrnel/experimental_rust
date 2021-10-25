#[derive(Debug, Copy, Clone)]
#[allow(dead_code)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug, Clone)]
struct User {
    id: UserId,
    name: String,
    gender: Gender,
}

#[derive(Debug, Clone)]
struct Topic {
    id: TopicId,
    name: String,
    owner_id: UserId,
}

#[derive(Debug, Clone)]
enum Event {
    Join((TopicId, UserId)),
    #[allow(dead_code)]
    Leave((TopicId, UserId)),
    Message((UserId, TopicId, String)),
}

fn process_event(event: &Event) {
    match event {
        Event::Join((tid, uid)) => println!("user {:?} joined {:?}", uid, tid),
        Event::Leave((tid, uid)) => println!("user {:?} left {:?}", uid, tid),
        Event::Message((uid, tid, message)) => {
            println!("user {:?} send message {:?} to {:?}", uid, message, tid)
        }
    }
}

fn process_message(event: &Event) {
    if let Event::Message((_, _, msg)) = event {
        println!("broadcast message {:?}", msg)
    }
}

#[allow(dead_code)]
pub fn run() {
    let alice = User {
        id: UserId(1),
        name: "Alice".into(),
        gender: Gender::Female,
    };
    let bob = User {
        id: UserId(2),
        name: "Bob".into(),
        gender: Gender::Male,
    };

    let topic = Topic {
        id: TopicId(1),
        name: "rust".into(),
        owner_id: UserId(1),
    };

    let event1 = Event::Join((topic.id, alice.id));
    let event2 = Event::Join((topic.id, bob.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world".into()));

    println!(
        "event1: {:?}, event2: {:?}, event3: {:?}",
        event1, event2, event3
    );

    println!("\nProcess event:");

    process_event(&event1);
    process_event(&event2);
    process_event(&event3);

    println!("\nProcess message:");

    process_message(&event1);
    process_message(&event2);
    process_message(&event3)
}
