// SPDX-License-Identifier: Apache-2.0 OR MIT

/// A viewport-aware wrapper around [`tokio::sync::watch`].
///
/// Works like the standard Tokio watch channel, but sending through the
/// `im_send*` methods additionally wakes up every viewport that holds a
/// registered [`Receiver`](watch::Receiver).
pub mod watch;

// TODO: oneshot
// TODO: mpsc
// TODO: broadcast
