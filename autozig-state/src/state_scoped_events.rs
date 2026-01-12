//! State-scoped event management

use crate::state::States;

/// Extension trait for [`App`] adding methods for registering state scoped events.
///
/// Note: This is a simplified trait definition. Full implementation would require
/// integration with the event/message system.
pub trait StateScopedMessagesAppExt {
    /// Clears an event/message when exiting the specified `state`.
    ///
    /// Note that message cleanup is ambiguously ordered relative to
    /// [`DespawnOnExit`](crate::scoped::DespawnOnExit) entity cleanup,
    /// and the [`OnExit`](crate::transition::OnExit) schedule for the target state.
    fn clear_messages_on_exit<M>(&mut self, state: impl States) -> &mut Self;

    /// Clears an event/message when entering the specified `state`.
    ///
    /// Note that message cleanup is ambiguously ordered relative to
    /// [`DespawnOnEnter`](crate::scoped::DespawnOnEnter) entity cleanup,
    /// and the [`OnEnter`](crate::transition::OnEnter) schedule for the target state.
    fn clear_messages_on_enter<M>(&mut self, state: impl States) -> &mut Self;
}