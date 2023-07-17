use std::{marker::PhantomData, sync::Arc};

use atomic::{Atomic, Ordering};
use futures::Stream;
// use tarsier::Data;
use tokio::sync::watch::{self, Receiver, Sender};

pub trait Data {}

/// A Lens is a functional method to access one part of a larger data
/// structure, as long as the owner and its data implement [`Data`]. Lenses
/// can interact independently, store infromation separately, and using them
/// is always safe.
///
/// Cloning a Lens will return a reference to the same data. Modifying a
/// cloned Lens will modify all Lenses cloned from it.
#[derive(Clone)]
pub struct Lens<O, E: Data> {
    element: Arc<Atomic<E>>,
    tx: Arc<Sender<()>>,
    rx: Receiver<()>,
    /// We store `O` for ease of use in identifying which Lens comes from
    /// which data. The owner type is not relevant to the implementation.
    _ph: PhantomData<O>,
}

impl<O, E: Data> Lens<O, E> {
    /// Construct a new Lens containing some value of type E.
    pub fn new(element: E) -> Self {
        let (tx, rx) = watch::channel(());

        Self {
            element: Arc::new(Atomic::new(element)),
            tx: Arc::new(tx),
            rx,
            _ph: PhantomData,
        }
    }

    /// Get an immutable reference to the data.
    pub fn get(&self) -> E {
        self.element.load(Ordering::Relaxed)
    }

    pub fn set(&self, element: E) {
        self.element.store(element, Ordering::Relaxed);
        self.tx.send(()).unwrap();
    }

    pub fn stream(&self) -> impl Stream<Item = E> + '_ {
        // Clone an instance of `rx` to use in this stream.
        let mut rx = self.rx.clone();

        async_stream::stream! {
            loop {
                rx.changed().await.unwrap();
                yield self.get();
            }
        }
    }
}
