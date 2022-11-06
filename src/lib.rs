#![allow(dead_code)]
#![allow(unused)]
extern crate anyhow;
extern crate nfc1 as nfc;
extern crate serde;
mod checksums;
mod mifare;
use anyhow::Result;
pub use mifare::{detect_card, wait_for_card, NfcReader};
use mifare::{MifareCard, Sector};
use nfc::{target_info::TargetInfo, Target};
use serde::Serialize;
use time::{
    macros::datetime, macros::format_description, serde::timestamp, Date, Duration, Month,
    OffsetDateTime, PrimitiveDateTime, Time,
};
use time_tz::{timezones::db::america::NEW_YORK, OffsetDateTimeExt, PrimitiveDateTimeExt};

// the CharlieCard epoch begins on midnight on January 1st
static CHARLIE_EPOCH: PrimitiveDateTime = datetime!(2003-01-01 0:00);

#[derive(Serialize)]
pub struct CharlieCard {
    pub uid: [u8; 4],
    pub serial_num: u32,
    pub card_type: CharlieCardType,
    pub money_value: u16,
    pub num_uses: u16,
    transaction_history: Vec<Transaction>,
    pub expiration_date: Date,
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
        let serial_num = (uid[0] as u32 * 0x1000000)
            + (uid[1] as u32 * 0x10000)
            + (uid[2] as u32 * 0x100)
            + uid[3] as u32;

        let mut card_data = MifareCard::new(target_card);

        // read the sectors that contain important information
        for i in 1..4 {
            card_data.read_sector(device, i)?;
        }
        for i in 6..8 {
            card_data.read_sector(device, i)?;
        }

        let card_type = card_data.card_type();
        let expiration_date = card_data.expiration_date(card_type);

        Ok(CharlieCard {
            uid,
            serial_num,
            card_type,
            num_uses: card_data.num_uses(),
            money_value: card_data.money_value(),
            transaction_history: card_data.transaction_history(),
            expiration_date,
            card_data,
        })
    }

    pub fn update_money(&mut self, new_money_value: u16) -> Result<()> {
        let money_xor_value = self.money_value ^ new_money_value;
        // finds the checksum xor value for the given money xor value
        let checksum_xor_value = checksums::MONEY_CHECKSUMS[money_xor_value as usize];

        let active_sector = self.card_data.active_sector();

        let mut new_money_block = self.card_data.sectors[active_sector].unwrap().data_blocks[1];

        let doubled_money_value = new_money_value << 1;
        // adjusting the money variable
        // representing a 16 bit integer as two 8-bit integers
        new_money_block.data[5] = (doubled_money_value >> 8) as u8;
        new_money_block.data[6] = (doubled_money_value % 256) as u8;

        // adjusting the checksum
        new_money_block.data[14] ^= (checksum_xor_value >> 8) as u8;
        new_money_block.data[15] ^= (checksum_xor_value % 256) as u8;

        self.card_data.sectors[active_sector] = Some(Sector {
            sector_num: active_sector as u8,
            data_blocks: [
                self.card_data.sectors[active_sector].unwrap().data_blocks[0],
                new_money_block,
                self.card_data.sectors[active_sector].unwrap().data_blocks[2],
            ],
            trailer_block: self.card_data.sectors[active_sector].unwrap().trailer_block,
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
        println!("Card Type: {:?}", self.card_type);
        println!("Money Value: ${:.2}", self.money_value as f32 / 100.0);
        println!("Total Number of Transactions: {}", self.num_uses);

        if let Some(transaction) = self.last_transaction() {
            // get the timestamp of the transaction
            let transaction_timestamp = transaction.timestamp;
            // format the timestamp into something that's not ugly as sin
            let datetime_format = format_description!(
                "[weekday], [month repr:long] [day], [year] [hour repr:12]:[minute] [period] "
            );
            println!(
                "Last Transaction: {}",
                transaction_timestamp.format(&datetime_format).unwrap()
            );
        } else {
            // if there aren't any transactions yet
            println!("Last Transaction: Never");
        };
        println!(
            "Expiration Date: {}\n",
            self.expiration_date
                .format(&format_description!(
                    "[weekday], [month repr:long] [day], [year]"
                ))
                .unwrap()
        );
    }
}

#[derive(Clone, Copy, Debug, Serialize)]
pub enum CharlieCardType {
    Standard,
    Student,
}

#[derive(Debug, Clone, Serialize)]
pub struct Transaction {
    #[serde(with = "timestamp")]
    pub timestamp: OffsetDateTime,
}

impl Transaction {
    // generates a new transaction based on the time it occurred
    fn new(raw_timestamp: u32) -> Transaction {
        let raw_timestamp_seconds = Duration::seconds(raw_timestamp as i64 * 60);

        Transaction {
            timestamp: (CHARLIE_EPOCH + raw_timestamp_seconds)
                .assume_timezone(NEW_YORK)
                .unwrap(),
        }
    }
}

impl MifareCard {
    // there are two transaction sectors: sector 2 and 3
    // the active sector is the sector the fare machine will look at and
    // stores current transaction info, such as the balance, expiration date,
    // and various other information
    // meanwhile, the inactive sector stores info about the last transaction
    fn active_sector(&self) -> usize {
        // the variables that count the number of uses are stored in blocks 5
        // and 6
        let block5 = self.sectors[1].unwrap().data_blocks[1];
        let block6 = self.sectors[1].unwrap().data_blocks[2];

        // the first count variable on block 5 corresponds to sector 2
        let first_count = block5.data[0] as u16 * 256 + block5.data[1] as u16;
        // similarly, the second count variable on block 5 corresponds to
        // sector 3
        let second_count = block6.data[0] as u16 * 256 + block6.data[1] as u16;

        // the lower count variable corresponds to the active sector
        // for example, if the first count variable is lower than the second
        // count variable, then the second sector is the active sector, while
        // the third sector is inactive and stores the previous transaction
        // info
        if first_count < second_count {
            2
        } else {
            3
        }
    }
    pub fn num_uses(&self) -> u16 {
        if self.active_sector() == 2 {
            let block5 = self.sectors[1].unwrap().data_blocks[1];
            // the first two bytes of each block is the count variable
            // the variable with the lowest value minus one is the number of uses
            return block5.data[0] as u16 * 256 + block5.data[1] as u16 - 1;
        } else {
            let block6 = self.sectors[1].unwrap().data_blocks[2];
            return block6.data[0] as u16 * 256 + block6.data[1] as u16 - 1;
        }
    }

    pub fn card_type(&self) -> CharlieCardType {
        let active_sector = self.active_sector();
        let block = self.sectors[active_sector].unwrap().data_blocks[1];
        // the first byte in the second block of the active sector seems to
        // determine what type of card a CharlieCard is
        let card_type_byte = block.data[0];
        // 104 seems to be the code for student cards
        if card_type_byte == 104 {
            return CharlieCardType::Student;
        } else {
            // 91 seems to correspond to standard cards, but just in case it
            // reads something else, we'll just call it a standard card
            return CharlieCardType::Standard;
        }
    }

    pub fn money_value(&self) -> u16 {
        let active_sector = self.active_sector();
        // the actual money values are stored on blocks 9 and 13
        let money_block = self.sectors[active_sector].unwrap().data_blocks[1];
        // reads the two byte money variable, needs to be bitshifted once to
        // the right because the money value is stored in half cents
        money_block.data[5] as u16 * 256 + money_block.data[6] as u16 >> 1
    }

    pub fn expiration_date(&self, card_type: CharlieCardType) -> Date {
        let active_sector = self.active_sector();
        let expiration_block = self.sectors[active_sector].unwrap().data_blocks[0];
        let raw_timestamp = (expiration_block.data[6] as u32 * 65536)
            + (expiration_block.data[7] as u32 * 256)
            + (expiration_block.data[8] as u32);

        let issue_date = CHARLIE_EPOCH + Duration::seconds(raw_timestamp as i64 * 60);

        // student cards last one school year and expire at the end of August
        // the next year
        if let CharlieCardType::Student = card_type {
            return Date::from_calendar_date(issue_date.year() + 1, Month::August, 31).unwrap();
        }
        // CharlieCards issued before 2011 expire 7 years after they're issued
        if issue_date < datetime!(2011-01-01 0:00) {
            return issue_date
                .replace_year(issue_date.year() + 7)
                .unwrap()
                .date()
                .previous_day()
                .unwrap();
        }

        // CharlieCards expire one day before 10 years after their date of issue
        issue_date
            .replace_year(issue_date.year() + 10)
            .unwrap()
            .date()
            .previous_day()
            .unwrap()
    }
    fn transaction_history(&self) -> Vec<Transaction> {
        // CharlieCards keep a log of the last 10 transactions and their
        // timestamps the timestamps record the number of minutes between
        // midnight on January 1, 2003 and the time of the transaction
        let mut raw_timestamps = Vec::new();
        for sector_num in 6..8 {
            for (_, block) in self.sectors[sector_num]
                .unwrap()
                .data_blocks
                .iter()
                .enumerate()
            {
                // each block has two three-byte timestamps
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
