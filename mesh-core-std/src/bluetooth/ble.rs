use crate::{
    link::link_trait::{Link, LinkConnection},
    types::ble_types::{BleLink, BleLinkConnection, DEFAULT_MTU, HEADER_SIZE},
};
use async_trait::async_trait;
use btleplug::api::{Central, Peripheral};
use std::{sync::Arc, time::Duration};
use tokio::sync::Mutex;

#[async_trait]
impl Link for BleLink {
    // finds the selected peripheral, acts as central which scans for peripherals
    async fn dial(
        &self,
        address: &str,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>
    {
        let (_tx, rx) = tokio::sync::mpsc::channel(32);

        self.adapter.start_scan(Default::default()).await?;
        tokio::time::sleep(Duration::from_secs(3)).await;

        let peripherals = self.adapter.peripherals().await?;
        let peripheral = peripherals
            .into_iter()
            .find(|p| p.address().to_string() == address.to_string())
            // address is a mac address
            .ok_or("no peripheral with this address")?;

        peripheral.connect().await?; // mac ra peripheral ko LinkConnection
        peripheral.discover_services().await?;

        let characteristic = peripheral
            .characteristics()
            .into_iter()
            .find(|c| c.uuid == self.characteristic_uuid)
            .ok_or("Characteristic not found")?;

        Ok(Box::new(BleLinkConnection {
            peripheral: Arc::new(peripheral),
            characteristic,
            rx: Arc::new(Mutex::new(rx)),
        }))
    }

    // acts for peripherals
    async fn accept(
        &self,
    ) -> Result<Box<dyn LinkConnection + Send + Sync>, Box<dyn std::error::Error + Send + Sync>>
    {
        unimplemented!()
    }

    fn mtu(&self) -> usize {
        DEFAULT_MTU
    }

    fn latency(&self) -> std::time::Duration {
        Duration::from_millis(50)
    }
}

#[async_trait]
impl LinkConnection for BleLinkConnection {
    async fn send(&self, data: &[u8]) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let mtu = DEFAULT_MTU;
        let total_data_len = data.len();
        let data_length = total_data_len.to_le_bytes();

        let mut seq = 0;
        let mut offset = 0;

        while offset < data.len() {
            let space = mtu - HEADER_SIZE;
            let end = (offset + space).min(data.len());

            let mut packet = Vec::with_capacity(mtu);
            packet.extend_from_slice(&data_length);
            packet.push(seq);
            packet.extend_from_slice(&data[offset..end]);

            self.peripheral
                .write(
                    &self.characteristic,
                    &packet,
                    btleplug::api::WriteType::WithResponse,
                )
                .await?;

            offset = end;
            seq = seq.wrapping_add(1);
        }

        Ok(())
    }

    async fn receive(&self) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
        let mut rx = self.rx.lock().await;
        match rx.recv().await.ok_or("Channel closed") {
            Ok(message) => {
                log::info!("Data read successfully!");
                Ok(message)
            }
            Err(e) => {
                log::error!("Error occurred while reading data!");
                Err(e.into())
            }
        }
    }
}
