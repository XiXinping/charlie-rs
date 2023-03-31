#![allow(unused)]
extern crate anyhow;
extern crate nfc1 as nfc;
extern crate thiserror;
use anyhow::{anyhow, Context, Result};
use nfc::{
    target_info::TargetInfo, BaudRate, Device, Modulation, ModulationType, Property, Target,
    Timeout,
};
use ouroboros::self_referencing;
use serde::Serialize;
use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::thread;
use std::time::{Duration, Instant};

#[self_referencing]
pub struct NfcReader<'a> {
    pub context: nfc::Context<'a>,
    #[borrows(mut context)]
    #[covariant]
    pub device: Device<'this>,
}

impl<'a> NfcReader<'a> {
    pub fn build() -> Result<Self, nfc::Error> {
        let context = nfc::Context::new()?;
        let reader = NfcReaderTryBuilder {
            context,
            device_builder: |ctx: &mut nfc::Context| {
                let mut device = ctx.open()?;

                // initialize the device with the proper settings
                // have the reader only try once to find a tag
                device.set_property_bool(Property::InfiniteSelect, false)?;
                device.set_property_bool(Property::AutoIso144434, false)?;
                Ok::<Device, nfc::Error>(device)
            },
        }
        .try_build()?;
        Ok(reader)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct ManufacturerBlock {
    uid: [u8; 4],
    bcc: u8,
    sak: u8,
    atqa: [u8; 2],
    data: [u8; 8],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct DataBlock {
    pub data: [u8; 16],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct TrailerBlock {
    key_a: [u8; 6],
    access_bits: [u8; 3],
    key_b: [u8; 6],
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct Sector {
    pub sector_num: u8,
    pub data_blocks: [DataBlock; 3],
    pub trailer_block: TrailerBlock,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
pub struct MifareCard {
    uid: [u8; 4],
    pub sectors: [Option<Sector>; 16],
}

#[derive(thiserror::Error, Debug)]
pub enum CardError {
    #[error("Error while reading card!")]
    ReaderError,
    #[error("Could not find CharlieCard! Make sure card is placed on reader!")]
    CardNotFound,
    #[error("Card is not a CharlieCard!")]
    NotCharlieCard,
    #[error("Error while writing to CharlieCard!")]
    WriteError,
}

/// Tries to find a card for a specific amount of time
pub fn wait_for_card(reader: &mut NfcReader, timeout: Duration) -> Result<Target> {
    let timer = Instant::now();

    while timer.elapsed() < timeout {
        match detect_card(reader)? {
            Some(card) => return Ok(card),
            None => (),
        };
    }

    Err(CardError::CardNotFound.into())
}

/// Try to find a card. Returns None if no card is found, and an error if something went wrong
pub fn detect_card(reader: &mut NfcReader) -> Result<Option<Target>> {
    let target = reader.with_device_mut(|device| -> Result<Target, nfc::Error> {
        Ok(device.initiator_select_passive_target(&MIFARE_MODULATION)?)
    })?;
    let target_info = match target.target_info {
        TargetInfo::Iso14443a(info) => info,
        _ => return Err(CardError::ReaderError.into()),
    };

    if target_info.atqa == [0, 0] && target_info.sak == 0 && target_info.uid_len == 0 {
        return Ok(None);
    }

    // make sure the card is a MIFARE card
    if target_info.sak != 0x08
        // check to make sure the card doesn't have 4KB of memory (CharlieCards
        // only have 1KB)
        || target_info.atqa[1] == 0x02
        || target_info.sak == 0x18
        // cards with 320B of memory aren't CharlieCards either
        || target_info.sak == 0x09
    {
        return Err(CardError::NotCharlieCard.into());
    }
    return Ok(Some(target));
}

impl MifareCard {
    pub fn new(target: Target) -> Result<MifareCard> {
        let uid = match target.target_info {
            TargetInfo::Iso14443a(info) => [info.uid[0], info.uid[1], info.uid[2], info.uid[3]],
            _ => return Err(CardError::NotCharlieCard.into()),
        };

        Ok(MifareCard {
            uid,
            sectors: [None; 16],
        })
    }
    pub fn authenticate(
        &self,
        reader: &mut NfcReader,
        sector_num: u8,
        key_type: KeyType,
    ) -> Result<()> {
        let mifare_command = match key_type {
            // 0x60 is the command for authenticating with key A
            KeyType::KeyA => 0x60,
            // 0x61 is for authenticating with key B
            KeyType::KeyB => 0x61,
        };
        let key = match key_type {
            KeyType::KeyA => CHARLIE_KEYS_A[sector_num as usize],
            KeyType::KeyB => CHARLIE_KEYS_B[sector_num as usize],
        };
        let block_address = (sector_num + 1) * 4 - 1;

        // command structure: (Mifare command, block address, key, uid)
        let command = [
            mifare_command,
            block_address,
            key[0],
            key[1],
            key[2],
            key[3],
            key[4],
            key[5],
            self.uid[0],
            self.uid[1],
            self.uid[2],
            self.uid[3],
        ];
        // make sure the card is still there
        wait_for_card(reader, Duration::from_secs(5))?;

        reader.with_device_mut(|device| -> Result<Vec<u8>> {
            device
                .initiator_transceive_bytes(&command, 256, Timeout::None)
                .with_context(|| {
                    format!(
                        "Error while authenticating sector {}! Card might not be a CharlieCard!",
                        sector_num
                    )
                })
        })?;
        Ok(())
    }

    pub fn read_sector<'a>(&mut self, reader: &'a mut NfcReader, sector_num: u8) -> Result<()> {
        // read command
        let mifare_command = 0x30;
        let block_address = (sector_num + 1) * 4 - 4;

        // authenticate using key A to gain read access to the data on the card
        self.authenticate(reader, sector_num, KeyType::KeyA)?;

        let mut data_blocks: [DataBlock; 3] = [DataBlock { data: [0; 16] }; 3];

        // read out each block
        for (i, block) in data_blocks.iter_mut().enumerate() {
            let data = reader.with_device_mut(|device| -> Result<[u8; 16]> {
                let block_vec = device
                    .initiator_transceive_bytes(
                        &[mifare_command, block_address + i as u8],
                        16,
                        Timeout::None,
                    )
                    .with_context(|| {
                        format!(
                            "Unable to read sector {}! Error while reading block {}",
                            sector_num,
                            block_address + i as u8
                        )
                    })?;
                //
                // initiator_transceive_bytes returns a Vec, but the data in a
                // data block is stored in an array
                let block_array: [u8; 16] = block_vec.try_into().unwrap();
                Ok(block_array)
            })?;

            *block = DataBlock { data };
        }

        self.sectors[sector_num as usize] = Some(Sector {
            sector_num,
            data_blocks,
            trailer_block: TrailerBlock {
                key_a: CHARLIE_KEYS_A[sector_num as usize],
                access_bits: CHARLIE_ACCESS_BITS,
                key_b: CHARLIE_KEYS_B[sector_num as usize],
            },
        });

        Ok(())
    }

    pub fn write_sector(&self, reader: &mut NfcReader, sector_num: u8) -> Result<()> {
        let mifare_command = 0xA0;
        let mut block_address = (sector_num + 1) * 4 - 4;
        let mut command = [0u8; 18];
        let sector = self.sectors[sector_num as usize].unwrap();

        // authenticate using key B to gain write access
        self.authenticate(reader, sector_num, KeyType::KeyB)?;

        for block in sector.data_blocks {
            command[0] = mifare_command;
            command[1] = block_address;
            command[2..18].copy_from_slice(&block.data[..]);
            block_address += 1;

            reader.with_device_mut(|device| -> Result<Vec<u8>> {
                device
                    .initiator_transceive_bytes(&command, 256, Timeout::None)
                    .with_context(|| {
                        format!(
                            "Unable to write to sector {}! Error while writing block {}",
                            sector_num, block_address
                        )
                    })
            })?;
        }

        Ok(())
    }
}

pub enum KeyType {
    KeyA,
    KeyB,
}

static CHARLIE_KEYS_A: [[u8; 6]; 16] = [
    [0x30, 0x60, 0x20, 0x6F, 0x5B, 0x0A],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x5E, 0xC3, 0x9B, 0x02, 0x2F, 0x2B],
    [0x3A, 0x09, 0x59, 0x4C, 0x85, 0x87],
    [0xF2, 0x38, 0xD7, 0x8F, 0xF4, 0x8F],
    [0xAF, 0xD0, 0xBA, 0x94, 0xD6, 0x24],
    [0xB3, 0x5A, 0x0E, 0x4A, 0xCC, 0x09],
    [0x44, 0x7A, 0xB7, 0xFD, 0x5A, 0x6B],
    [0x1F, 0x1A, 0x0A, 0x11, 0x1B, 0x5B],
    [0xD5, 0x80, 0x23, 0xBA, 0x2B, 0xDC],
    [0x25, 0x48, 0xA4, 0x43, 0xDF, 0x28],
];

static CHARLIE_KEYS_B: [[u8; 6]; 16] = [
    [0xF1, 0xB9, 0xF5, 0x66, 0x9C, 0xC8],
    [0xF6, 0x62, 0x24, 0x8E, 0x7E, 0x89],
    [0xF6, 0x62, 0x24, 0x8E, 0x7E, 0x89],
    [0xF6, 0x62, 0x24, 0x8E, 0x7E, 0x89],
    [0xF6, 0x62, 0x24, 0x8E, 0x7E, 0x89],
    [0xF6, 0x62, 0x24, 0xEE, 0x1E, 0x89],
    [0xF6, 0x62, 0x24, 0xEE, 0x1E, 0x89],
    [0xF6, 0x62, 0x24, 0xEE, 0x1E, 0x89],
    [0x62, 0x38, 0x7B, 0x8D, 0x25, 0x0D],
    [0x9D, 0xC2, 0x82, 0xD4, 0x62, 0x17],
    [0x92, 0xEE, 0x4D, 0xC8, 0x71, 0x91],
    [0x75, 0x6E, 0xF5, 0x5E, 0x25, 0x07],
    [0x93, 0x2B, 0x9C, 0xB7, 0x30, 0xEF],
    [0xAD, 0x9E, 0x0A, 0x1C, 0xA2, 0xF7],
    [0x62, 0xCE, 0xD4, 0x2A, 0x6D, 0x87],
    [0x2E, 0xD3, 0xB1, 0x5E, 0x7C, 0x0F],
];

static CHARLIE_ACCESS_BITS: [u8; 3] = [0x78, 0x77, 0x88];

static MIFARE_MODULATION: Modulation = Modulation {
    modulation_type: ModulationType::Iso14443a,
    baud_rate: BaudRate::Baud106,
};
