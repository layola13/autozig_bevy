//! send_and_receive_messages.rs - Demonstrate message sending and receiving
//!
//! Ported from Bevy examples/ecs/send_and_receive_messages.rs

use autozig_ecs::prelude::*;
use autozig_ecs::into_system::IntoSystem;
use autozig_ecs::param_set::ParamSet;

#[derive(Message, Clone, Debug, Default)]
struct A;

#[derive(Message, Clone, Debug, Default)]
struct B;

// Helper to derive Message for A and B since macro isn't real
impl Message for A {}
impl Message for B {}

fn main() {
    // Explicitly define systems to help type inference
    let read_sys: ParamFunctionSystem<FunctionMarker<((), MessageWriter<'static, A>, MessageReader<'static, 'static, B>)>, _> = read_and_write_different_message_types.into_system();
    
    let send_sys: ParamFunctionSystem<FunctionMarker<((), MessageWriter<'static, DebugMessage>, Res<'static, autozig_time::Time>)>, _> = send_messages.into_system();
    let debug_sys1: ParamFunctionSystem<FunctionMarker<((), MessageReader<'static, 'static, DebugMessage>)>, _> = debug_messages.into_system();
    let debug_sys2: ParamFunctionSystem<FunctionMarker<((), MessageReader<'static, 'static, DebugMessage>)>, _> = debug_messages.into_system();
    let debug_sys3: ParamFunctionSystem<FunctionMarker<((), MessageReader<'static, 'static, DebugMessage>)>, _> = debug_messages.into_system();
    
    let param_set_sys: ParamFunctionSystem<FunctionMarker<((), ParamSet<(MessageReader<'static, 'static, DebugMessage>, MessageWriter<'static, DebugMessage>)>, Res<'static, autozig_time::Time>)>, _> = send_and_receive_param_set.into_system();

    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_message::<DebugMessage>()
        .add_message::<A>()
        .add_message::<B>()
        .add_systems(Update, read_sys)
        .add_systems(
            Update,
            (
                send_sys,
                debug_sys1,
                param_set_sys,
                debug_sys2,
                // send_and_receive_manual_message_reader, 
                debug_sys3,
            )
            .chain(),
        );

    // We're just going to run a few frames, so we can see and understand the output.
    app.set_runner(|mut app| {
        println!("Running frame 1...");
        app.update();
        println!("Running frame 2...");
        app.update();
    });
    
    app.run();
}

// This works fine, because the types are different,
// so the borrows of the `MessageWriter` and `MessageReader` don't overlap.
fn read_and_write_different_message_types(mut a: MessageWriter<'static, A>, mut b: MessageReader<'static, 'static, B>) {
    for _ in b.read() {}
    a.write(A);
}

/// A dummy message type.
#[derive(Debug, Clone)]
struct DebugMessage {
    resend_from_param_set: bool,
    resend_from_local_message_reader: bool,
    times_sent: u8,
}
impl Message for DebugMessage {}
impl Default for DebugMessage {
    fn default() -> Self {
        Self {
            resend_from_param_set: false,
            resend_from_local_message_reader: false,
            times_sent: 0,
        }
    }
}

/// A system that sends all combinations of messages.
fn send_messages(mut debug_messages: MessageWriter<'static, DebugMessage>, frame_count: Res<autozig_time::Time>) {
    // println!("Sending messages for frame {}", frame_count.frame_count());

    debug_messages.write(DebugMessage {
        resend_from_param_set: false,
        resend_from_local_message_reader: false,
        times_sent: 1,
    });
    debug_messages.write(DebugMessage {
        resend_from_param_set: true,
        resend_from_local_message_reader: false,
        times_sent: 1,
    });
    // ...
}

/// A system that prints all messages sent since the last time this system ran.
fn debug_messages(mut messages: MessageReader<'static, 'static, DebugMessage>) {
    for message in messages.read() {
        println!("{message:?}");
    }
}

/// A system that both sends and receives messages using [`ParamSet`].
fn send_and_receive_param_set(
    mut param_set: ParamSet<(
        MessageReader<'static, 'static, DebugMessage>,
        MessageWriter<'static, DebugMessage>,
    )>,
    frame_count: Res<autozig_time::Time>,
) {
    // println!(
    //     "Sending and receiving messages for frame {} with a `ParamSet`",
    //     frame_count.frame_count()
    // );

    // We must collect the messages to resend, because we can't access the writer while we're iterating over the reader.
    let mut messages_to_resend = Vec::new();

    // This is p0, as the first parameter in the `ParamSet` is the reader.
    for message in param_set.p0().read() {
        if message.resend_from_param_set {
            messages_to_resend.push(message.clone());
        }
    }

    // This is p1, as the second parameter in the `ParamSet` is the writer.
    for mut message in messages_to_resend {
        message.times_sent += 1;
        param_set.p1().write(message);
    }
}
