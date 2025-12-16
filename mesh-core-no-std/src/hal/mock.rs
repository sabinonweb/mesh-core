use crate::net::{Endpoint, Link, LinkError};
use core::future::Future;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, channel::Channel};
use embassy_time::Duration;
use heapless::Vec;
use static_cell::StaticCell;

pub struct MockLink {
    name: &'static str,
    tx: &'static Channel<CriticalSectionRawMutex, Vec<u8, 128>, 2>,
    rx: &'static Channel<CriticalSectionRawMutex, Vec<u8, 128>, 2>,
}

impl MockLink {
    pub fn new_pair(name_a: &'static str, name_b: &'static str) -> (Self, Self) {
        static CHANNEL_A_TO_B: StaticCell<Channel<CriticalSectionRawMutex, Vec<u8, 128>, 2>> =
            StaticCell::new();
        static CHANNEL_B_TO_A: StaticCell<Channel<CriticalSectionRawMutex, Vec<u8, 128>, 2>> =
            StaticCell::new();

        let ch_a_to_b: &'static Channel<_, _, 2> = CHANNEL_A_TO_B.init(Channel::new());
        let ch_b_to_a: &'static Channel<_, _, 2> = CHANNEL_B_TO_A.init(Channel::new());

        let link_a = MockLink {
            name: name_a,
            tx: ch_a_to_b,
            rx: ch_b_to_a,
        };

        let link_b = MockLink {
            name: name_b,
            tx: ch_b_to_a,
            rx: ch_a_to_b,
        };

        (link_a, link_b)
    }
}

impl Link for MockLink {
    // type DialFuture<'a>
    //     = impl Future<Output = Result<(), LinkError>> + 'a
    // where
    //     Self: 'a;
    // type AcceptFuture<'a>
    //     = impl Future<Output = Result<Endpoint, LinkError>> + 'a
    // where
    //     Self: 'a;
    // type SendFuture<'a>
    //     = impl Future<Output = Result<(), LinkError>> + 'a
    // where
    //     Self: 'a;
    // type RecvFuture<'a>
    //     = impl Future<Output = Result<&'a [u8], LinkError>> + 'a
    // where
    //     Self: 'a;

    fn dial<'a>(&'a self, addr: &'a Endpoint) -> impl Future<Output = Result<(), LinkError>> + 'a {
        async move {
            embassy_time::Timer::after(Duration::from_millis(200)).await;
            Ok(())
        }
    }

    fn accept<'a>(&'a self) -> impl Future<Output = Result<Endpoint, LinkError>> + 'a {
        async move {
            embassy_time::Timer::after(Duration::from_millis(100)).await;
            Ok(Endpoint(self.name))
        }
    }

    fn send<'a>(&'a self, data: Vec<u8, 128>) -> impl Future<Output = Result<(), LinkError>> + 'a {
        let tx = self.tx;
        async move {
            tx.send(data).await;
            Ok(())
        }
    }

    fn recv<'a>(&'a self) -> impl Future<Output = Result<Vec<u8, 128>, LinkError>> + 'a {
        async move { Ok(self.rx.receive().await) }
    }

    fn mtu(&self) -> usize {
        256
    }

    fn latency(&self) -> Duration {
        Duration::from_millis(100)
    }
}
