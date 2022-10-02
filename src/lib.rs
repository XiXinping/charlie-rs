#![allow(dead_code)]
#![allow(unused)]
extern crate anyhow;
extern crate nfc1 as nfc;
extern crate serde;
mod checksums;
mod mifare;
use anyhow::Result;
pub use mifare::{find_card, NfcReader};
use mifare::{MifareCard, Sector};
use nfc::{target_info::TargetInfo, Target};
use serde::Serialize;
use std::time::Duration;
use time::{self, macros::format_description, serde::timestamp, Date, PrimitiveDateTime, Time};
use time_tz::{timezones::db::america::NEW_YORK, PrimitiveDateTimeExt};

#[derive(Serialize)]
pub struct CharlieCard {
    pub uid: [u8; 4],
    pub serial_num: u32,
    pub money_value: u16,
    pub num_uses: u16,
    transaction_history: Vec<Transaction>,
    card_data: MifareCard,
}

impl CharlieCard {
    pub fn new(mut device: &mut NfcReader, target_card: Target) -> Result<CharlieCard> {
        let target_info = match target_card.target_info {
            TargetInfo::Iso14443a(info) => info,
            _ => panic!(),
        };

        let uid = [
            target_info.uid[0],
            target_info.uid[1],
            target_info.uid[2],
            target_info.uid[3],
        ];
        // convert the four bytes into one number
        let serial_num = (uid[0] as u32 * 16777216)
            + (uid[1] as u32 * 65536)
            + (uid[2] as u32 * 255)
            + uid[3] as u32;

        let mut card_data = MifareCard::new(target_card);

        // read the sectors that contain important information
        for i in 1..4 {
            card_data.read_sector(device, i)?;
        }
        for i in 6..8 {
            card_data.read_sector(device, i)?;
        }

        let num_uses = num_uses(&card_data);

        let money_value = money_value(&card_data);

        let transaction_history = transaction_history(&card_data);

        Ok(CharlieCard {
            uid,
            serial_num,
            num_uses,
            money_value,
            transaction_history,
            card_data,
        })
    }

    pub fn update_money(&mut self, new_money_value: u16) -> Result<()> {
        let money_xor_value = self.money_value ^ new_money_value;
        // finds the checksum xor value for the given money xor value
        let checksum_xor_value = checksums::MONEY_CHECKSUMS[money_xor_value as usize];

        let block5 = self.card_data.sectors[1].unwrap().data_blocks[1];
        let block6 = self.card_data.sectors[1].unwrap().data_blocks[2];

        // the first two bytes of each block is the count variable
        let block5_count = block5.data[0] as u16 * 256 + block5.data[1] as u16;
        let block6_count = block6.data[0] as u16 * 256 + block6.data[1] as u16;

        let current_money_sector: usize = if block5_count < block6_count { 2 } else { 3 };

        let mut new_money_block = self.card_data.sectors[current_money_sector]
            .unwrap()
            .data_blocks[1];

        let doubled_money_value = new_money_value << 1;
        // adjusting the money variable
        // representing a 16 bit integer as two 8-bit integers
        new_money_block.data[5] = (doubled_money_value >> 8) as u8;
        new_money_block.data[6] = (doubled_money_value % 256) as u8;

        // adjusting the checksum
        new_money_block.data[14] ^= (checksum_xor_value >> 8) as u8;
        new_money_block.data[15] ^= (checksum_xor_value % 256) as u8;

        self.card_data.sectors[current_money_sector] = Some(Sector {
            sector_num: current_money_sector as u8,
            data_blocks: [
                self.card_data.sectors[current_money_sector]
                    .unwrap()
                    .data_blocks[0],
                new_money_block,
                self.card_data.sectors[current_money_sector]
                    .unwrap()
                    .data_blocks[2],
            ],
            trailer_block: self.card_data.sectors[current_money_sector]
                .unwrap()
                .trailer_block,
        });

        self.money_value = new_money_value;

        Ok(())
    }

    pub fn write_card(&self, reader: &mut NfcReader) -> Result<()> {
        for sector in self.card_data.sectors.into_iter().flatten() {
            self.card_data.write_sector(reader, sector.sector_num)?;
        }

        Ok(())
    }

    pub fn last_transaction(&self) -> Option<Transaction> {
        self.transaction_history.get(0).cloned()
    }

    pub fn print_info(&self) {
        println!("\nCharlieCard Info:");
        println!("Serial Number: {}", self.serial_num);
        println!("Money Value: ${:.2}", self.money_value as f32 / 100.0);
        println!("Total Number of Transactions: {}", self.num_uses);

        if let Some(transaction) = self.last_transaction() {
            // get the timestamp of the transaction
            let transaction_timestamp = transaction.timestamp;
            // format the timestamp into something that's not ugly as sin
            let format_item = format_description!(
                "[weekday], [month repr:long] [day], [year] [hour repr:12]:[minute] [period] "
            );
            println!(
                "Last Transaction: {}\n",
                transaction_timestamp.format(&format_item).unwrap()
            );
        } else {
            // if there aren't any transactions yet
            println!("Last Transaction: Never");
        };
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    #[serde(with = "timestamp")]
    pub timestamp: time::OffsetDateTime,
}

impl Transaction {
    // generates a new transaction based on the time it occurred
    fn new(raw_timestamp: u32) -> Transaction {
        // the CharlieCard epoch begins on midnight on January 1st
        let epoch_start = PrimitiveDateTime::new(
            Date::from_ordinal_date(2003, 1).unwrap(),
            Time::from_hms(0, 0, 0).unwrap(),
        )
        .assume_timezone(NEW_YORK)
        .unwrap();
        let raw_timestamp_seconds = Duration::from_secs(raw_timestamp as u64 * 60);

        Transaction {
            timestamp: epoch_start + raw_timestamp_seconds,
        }
    }
}

pub fn num_uses(card_data: &MifareCard) -> u16 {
    // blocks 5 and 6 contain variables that record the number of uses
    let block5 = card_data.sectors[1].unwrap().data_blocks[1];
    let block6 = card_data.sectors[1].unwrap().data_blocks[2];

    // the first two bytes of each block is the count variable
    let block5_count = block5.data[0] as u16 * 256 + block5.data[1] as u16;
    let block6_count = block6.data[0] as u16 * 256 + block6.data[1] as u16;

    // the variable with the lowest value minus one is the number of uses
    if block5_count < block6_count {
        block5_count - 1
    } else {
        block6_count - 1
    }
}

pub fn money_value(card_data: &MifareCard) -> u16 {
    // the variables that count the number of uses are stored in blocks 5 and 6
    let block5 = card_data.sectors[1].unwrap().data_blocks[1];
    let block6 = card_data.sectors[1].unwrap().data_blocks[2];

    // the actual money values are stored on blocks 9 and 13
    // one of the variables
    let block9 = card_data.sectors[2].unwrap().data_blocks[1];
    let block13 = card_data.sectors[3].unwrap().data_blocks[1];

    // the first count variable on block 5 corresponds to the first money
    // variable on block 9
    let first_count = block5.data[0] as u16 * 256 + block5.data[1] as u16;
    // similarly, the second count variable on block 5 corresponds to the
    // second money varialbe on block 13
    let second_count = block6.data[0] as u16 * 256 + block6.data[1] as u16;

    let first_money_variable = block9.data[5] as u16 * 256 + block9.data[6] as u16;
    let second_money_variable = block13.data[5] as u16 * 256 + block13.data[6] as u16;

    // the lowest count variable corresponds to the current money value
    // for example, if the first count variable is lower than the second count
    // variable, then the current money value is stored in the first money
    // variable, while the previous money value is stored in the second money
    // variable
    if first_count < second_count {
        first_money_variable >> 1
    } else {
        // likewise, if the second count variable is lower than the fist count
        // variable, then the current money value must be stored in the second
        // money variable, while the previous money value is stored in the
        // first money variable
        second_money_variable >> 1
    }
}

fn transaction_history(card_data: &MifareCard) -> Vec<Transaction> {
    // CharlieCards keep a log of the last 10 transactions and their timestamps
    // the timestamps record the number of minutes between midnight on January
    // 1, 2003 and the time of the transaction
    let mut raw_timestamps = Vec::new();
    for sector_num in 6..8 {
        for (_, block) in card_data.sectors[sector_num]
            .unwrap()
            .data_blocks
            .iter()
            .enumerate()
        {
            // each block has two timestamps, each of which are made up of three bytes
            let first_timestamp = (block.data[0] as u32 * 65536)
                + (block.data[1] as u32 * 256)
                + (block.data[2] as u32);
            let second_timestamp = (block.data[7] as u32 * 65536)
                + (block.data[8] as u32 * 256)
                + (block.data[9] as u32);
            raw_timestamps.push(first_timestamp);
            raw_timestamps.push(second_timestamp);
        }
    }
    // remove any blank timestamps
    raw_timestamps = raw_timestamps.into_iter().filter(|t| *t != 0).collect();
    // sort the timestamps in descending order
    raw_timestamps.sort_by(|a, b| b.cmp(a));

    // generate a transaction for each timestamp
    raw_timestamps.into_iter().map(Transaction::new).collect()
}

pub fn print_charlie() {
    let ascii_charlie = " _______________________________ \n\
                         |                  Chahlie Cahd |\n\
                         |                 --------------|\n\
                         |###############################|\n\
                         |      #   _||_      ##   ##   #|\n\
                         |      #  /(ツ) []   ##   ##   #|\n\
                         |      #  \\-||--/    ##   ##   #|\n\
                         |      #    ||       ##   ##   #|\n\
                         |###############################|\n\
                         | (T) Free the T                |\n\
                         |_______________________________|\n";
    println!("{}", ascii_charlie);
}
