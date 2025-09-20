use crate::MeshError;
use btleplug::{
    api::{Characteristic, Manager as _, Peripheral},
    platform::{Adapter, Manager, Peripheral as PlatformPeripheral},
};
use std::sync::Arc;
use tokio::{
    sync::{
        mpsc::{self, Receiver},
        Mutex,
    },
    task,
};
use tokio_stream::StreamExt;
use uuid::Uuid;

pub const HEADER_SIZE: usize = 3; // header ko lagi 2 byte and seq number ko lagi 1
pub const DEFAULT_MTU: usize = 20;

// Entry point for ble, like endpoint WifiLink ko jasto
#[derive(Clone, Debug)]
pub struct BleLink {
    // physical ble adaptor for MAC, h/w interface which transmits/receives actual BLE signals
    pub adapter: Adapter,

    // uuid of service that link wants to interact with
    pub service_uuid: Uuid,

    // uuid of the characteristic that link wants to interact with within tyo mathi ko service
    pub characteristic_uuid: Uuid,
}

impl BleLink {
    pub async fn new(service_uuid: Uuid, characteristic_uuid: Uuid) -> Result<Self, MeshError> {
        let manager = Manager::new().await.unwrap(); // devices list garcha, scans, manages
                                                     // connection
        let adapters = manager.adapters().await?;
        let adapter = adapters
            .into_iter()
            .next()
            .ok_or_else(|| "No BLE adapter found")?;

        Ok(Self {
            adapter,
            service_uuid,
            characteristic_uuid,
        })
    }
}

#[derive(Debug)]
pub struct BleLinkConnection {
    // connected peripheral we talk to
    pub peripheral: Arc<PlatformPeripheral>,

    // characteristic where we send/receive data
    pub characteristic: Characteristic,

    // channel to send message to receive
    pub rx: Arc<Mutex<Receiver<Vec<u8>>>>,
}

impl BleLinkConnection {
    pub async fn new(
        peripheral: PlatformPeripheral,
        characteristic: Characteristic,
    ) -> Result<Self, MeshError> {
        let (tx, rx) = mpsc::channel::<Vec<u8>>(32);
        let peripheral = Arc::new(peripheral);
        let c = characteristic.clone();

        peripheral.subscribe(&c).await?;
        task::spawn(Self::notification_task(peripheral.clone(), c.clone(), tx));
        Ok(Self {
            peripheral,
            characteristic: c,
            rx: Arc::new(Mutex::new(rx)),
        })
    }

    pub async fn notification_task(
        peripheral: Arc<PlatformPeripheral>,
        characteristic: Characteristic,
        tx: mpsc::Sender<Vec<u8>>,
    ) {
        let mut buffer = Vec::new();
        let mut expected_length: Option<usize> = None;
        let mut last_sequence: Option<u8> = None;

        let mut events = peripheral.notifications().await.unwrap();

        while let Some(event) = events.next().await {
            let chunk = event.value;

            if chunk.len() < HEADER_SIZE {
                continue;
            }

            let len = u16::from_le_bytes([chunk[0], chunk[1]]);
            let seq = chunk[2];

            let payload = &chunk[HEADER_SIZE..];

            if expected_length.is_none() {
                buffer.clear();
                buffer.extend_from_slice(&payload);
                expected_length = Some(len.into());
                last_sequence = Some(seq);
            } else {
                if let Some(last) = last_sequence {
                    if last != seq.wrapping_add(1) {
                        log::error!("Fragment sequence error: last {} current {}", last, seq);
                        buffer.clear();
                        expected_length = None;
                        last_sequence = None;
                        continue;
                    }
                }
                buffer.extend_from_slice(payload);
                last_sequence = Some(seq);
            }
        }

        if let Some(total) = expected_length {
            if buffer.len() >= total {
                let _ = tx.send(buffer.clone()).await;
                buffer.clear();
            }
        }
    }
}
