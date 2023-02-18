#![allow(dead_code)]
#![allow(unused)]
extern crate anyhow;
extern crate nfc1 as nfc;
extern crate serde;
mod card_types;
mod checksums;
mod mifare;
use anyhow::{bail, Result};
pub use card_types::{Pass, TicketType};
pub use mifare::{detect_card, wait_for_card, NfcReader};
use mifare::{CardError, MifareCard, Sector};
use nfc::{target_info::TargetInfo, Target};
use serde::Serialize;
use time::{
    macros::datetime, macros::format_description, serde::timestamp, Date, Duration, Month,
    OffsetDateTime, PrimitiveDateTime, Time,
};
use time_tz::{timezones::db::america::NEW_YORK, OffsetDateTimeExt, PrimitiveDateTimeExt};

// the CharlieCard epoch begins on midnight on January 1st
static CHARLIE_EPOCH: PrimitiveDateTime = datetime!(2003-01-01 0:00);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct CharlieCard {
    pub uid: [u8; 4],
    pub serial_num: u32,
    pub card_type: TicketType,
    pub pass: Option<Pass>,
    pub money_value: u16,
    pub num_uses: u16,
    pub transaction_history: Vec<Transaction>,
    pub end_validity: OffsetDateTime,
    pub expiration: OffsetDateTime,
    card_data: MifareCard,
}

impl CharlieCard {
    pub fn new(mut device: &mut NfcReader, target_card: Target) -> Result<CharlieCard> {
        let target_info = match target_card.target_info {
            TargetInfo::Iso14443a(info) => info,
            _ => return Err(CardError::NotCharlieCard.into()),
        };

        let uid = [target_info.uid[0], target_info.uid[1], target_info.uid[2], target_info.uid[3]];
        // convert the four bytes into one number
        let serial_num = (uid[0] as u32 * 0x1000000)
            + (uid[1] as u32 * 0x10000)
            + (uid[2] as u32 * 0x100)
            + uid[3] as u32;

        let mut card_data = MifareCard::new(target_card)?;

        // read the sectors that contain important information
        for i in 1..4 {
            card_data.read_sector(device, i)?;
        }
        for i in 6..8 {
            card_data.read_sector(device, i)?;
        }

        let card_type = card_data.card_type();
        let expiration = card_data.expiration(card_type);
        let end_validity = card_data.end_validity();

        Ok(CharlieCard {
            uid,
            serial_num,
            card_type,
            pass: None,
            num_uses: card_data.num_uses(),
            money_value: card_data.money_value(),
            transaction_history: card_data.transaction_history(),
            end_validity,
            expiration,
            card_data,
        })
    }

    pub fn update_money(&mut self, new_money_value: u16) -> Result<()> {
        // find the value that XORs with the current money value to yield the new one
        let money_xor = (self.money_value ^ new_money_value) << 1;
        // the money value spans two bytes/columns, so the value for each byte/column must be
        // isolated
        let column_5_xor = (money_xor >> 8) as u8;
        let column_6_xor = (money_xor & 0xFF) as u8;

        // find the checksum modifier for each byte of the money value and xor them together
        let checksum_modifier = checksums::get_checksum_modifier(5, column_5_xor)
            ^ checksums::get_checksum_modifier(6, column_6_xor);

        let active_sector = self.card_data.active_sector();

        let mut new_money_block = self.card_data.sectors[active_sector].unwrap().data_blocks[1];

        // adjusting the money variable
        // representing a 16 bit integer as two 8-bit integers
        new_money_block.data[5] = (new_money_value * 2 >> 8) as u8;
        new_money_block.data[6] = (new_money_value * 2 & 0xFF) as u8;

        // adjusting the checksum
        new_money_block.data[14] ^= (checksum_modifier >> 8) as u8;
        new_money_block.data[15] ^= (checksum_modifier & 0xFF) as u8;

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

    pub fn update_expiration(&mut self, new_expiration: OffsetDateTime) -> Result<()> {
        let active_sector = self.card_data.active_sector();
        // block that contains the expiration date
        let mut expiration_block = self.card_data.sectors[active_sector].unwrap().data_blocks[0];

        // the expiration date is actually not recorded on the card, rather the card only stores
        // the date of issue and adds 10 years minus 1 day if it's a non-student card, and 1 year
        // minues 1 day if it's a student card
        let new_issue_date = if self.card_type == TicketType::Student {
            new_expiration
                .replace_year(new_expiration.year() - 1)?
                .saturating_add(Duration::DAY)
        } else {
            new_expiration
                .replace_year(new_expiration.year() - 10)?
                .saturating_add(Duration::DAY)
        };
        // represent the new date in the form of number of minutes since 1/1/2003
        let new_timestamp =
            (new_issue_date - CHARLIE_EPOCH.assume_timezone(NEW_YORK).unwrap()).whole_minutes();

        if new_timestamp > 0xFF_FF_FF || new_timestamp < 0 {
            bail!("Expiration date out of range!");
        }

        // represent the new date as three bytes
        let new_timestamp_byte1 = (new_timestamp >> 16) as u8;
        let new_timestamp_byte2 = (new_timestamp >> 8 & 0xFF) as u8;
        let new_timestamp_byte3 = (new_timestamp & 0xFF) as u8;

        // XOR each byte of the old date with the new date to find to allow for calculating the
        // checksum modifier
        let byte1_xor_product = new_timestamp_byte1 ^ expiration_block.data[6];
        let byte2_xor_product = new_timestamp_byte2 ^ expiration_block.data[7];
        let byte3_xor_product = new_timestamp_byte3 ^ expiration_block.data[8];

        // compute the checksum modifier by looking up the checksum modifier for each byte and
        // XORing them together
        let checksum_modifier = checksums::get_checksum_modifier(6, byte1_xor_product)
            ^ checksums::get_checksum_modifier(7, byte2_xor_product)
            ^ checksums::get_checksum_modifier(8, byte3_xor_product);

        // update the expiration date
        expiration_block.data[6] = new_timestamp_byte1;
        expiration_block.data[7] = new_timestamp_byte2;
        expiration_block.data[8] = new_timestamp_byte3;

        // update the checksum
        expiration_block.data[14] ^= (checksum_modifier >> 8) as u8;
        expiration_block.data[15] ^= (checksum_modifier & 0xFF) as u8;

        // update the card data
        if let Some(ref mut sector) = self.card_data.sectors[active_sector] {
            sector.data_blocks[0] = expiration_block;
        }

        self.expiration = new_expiration;

        Ok(())
    }

    fn update_end_validity(&mut self, new_end_validity: OffsetDateTime) -> Result<()> {
        let active_sector = self.card_data.active_sector();
        let mut end_validity_block = self.card_data.sectors[active_sector].unwrap().data_blocks[1];

        // unlike the expiration date, the end validity timestamp stores the actual end validity
        // rather than the date of issue, and stores the number of 8 minute increments since
        // January 1, 2003, presumably to save space
        let new_timestamp = (new_end_validity - CHARLIE_EPOCH.assume_timezone(NEW_YORK).unwrap())
            .whole_minutes()
            / 8;

        if new_timestamp > 0xFF_FF_FF || new_timestamp < 0 {
            bail!("End validity out of range!");
        }

        let new_timestamp_byte1 = (new_timestamp >> 16 & 0x1F) as u8;
        let new_timestamp_byte2 = (new_timestamp >> 8 & 0xFF) as u8;
        let new_timestamp_byte3 = (new_timestamp & 0xFF) as u8;

        // the first three bits need to be preserved, as they're shared with the card type byte
        let byte1_xor_product = new_timestamp_byte1 ^ end_validity_block.data[1] & 0x1F;
        let byte2_xor_product = new_timestamp_byte2 ^ end_validity_block.data[2];
        let byte3_xor_product = new_timestamp_byte3 ^ end_validity_block.data[3];

        let checksum_modifier = checksums::get_checksum_modifier(1, byte1_xor_product)
            ^ checksums::get_checksum_modifier(2, byte2_xor_product)
            ^ checksums::get_checksum_modifier(3, byte3_xor_product);

        // preserve the first three bits
        end_validity_block.data[1] = end_validity_block.data[1] & 0xE0 | new_timestamp_byte1;
        end_validity_block.data[2] = new_timestamp_byte2;
        end_validity_block.data[3] = new_timestamp_byte3;

        end_validity_block.data[14] ^= (checksum_modifier >> 8) as u8;
        end_validity_block.data[15] ^= (checksum_modifier & 0xFF) as u8;

        if let Some(ref mut sector) = self.card_data.sectors[active_sector] {
            sector.data_blocks[1] = end_validity_block;
        }
        self.end_validity = new_end_validity;

        Ok(())
    }

    pub fn update_card_type(&mut self, card_type: TicketType) -> Result<()> {
        let active_sector = self.card_data.active_sector();
        let mut card_type_block = self.card_data.sectors[active_sector].unwrap().data_blocks[1];

        let new_card_type_byte1 = (card_type as usize >> 2) as u8;
        let card_type_byte1 = card_type_block.data[0];
        let byte1_xor_product = new_card_type_byte1 ^ card_type_byte1;

        let new_card_type_bits = (card_type as usize & 0b11) as u8;
        let byte2 = card_type_block.data[1];
        let new_byte2 = (byte2 & 0b00_111111) | (new_card_type_bits << 6);
        let byte2_xor_product = byte2 ^ new_byte2;

        let checksum_modifier = checksums::get_checksum_modifier(0, byte1_xor_product)
            ^ checksums::get_checksum_modifier(1, byte2_xor_product);

        // update the card type
        card_type_block.data[0] = new_card_type_byte1;
        card_type_block.data[1] = new_byte2;

        // update the checksum
        card_type_block.data[14] ^= (checksum_modifier >> 8) as u8;
        card_type_block.data[15] ^= (checksum_modifier & 0xFF) as u8;

        // update the card data
        if let Some(ref mut sector) = self.card_data.sectors[active_sector] {
            sector.data_blocks[1] = card_type_block;
        }
        self.card_type = card_type;
        self.update_expiration(self.expiration);

        Ok(())
    }

    pub fn write_card(&self, reader: &mut NfcReader) -> Result<()> {
        // write each sector to the card
        for sector in self.card_data.sectors.into_iter().flatten() {
            self.card_data.write_sector(reader, sector.sector_num)?;
        }

        Ok(())
    }

    // read the card again and check to make sure it matches the current card in memory
    pub fn verify_write(&self, reader: &mut NfcReader) -> Result<()> {
        let target = detect_card(reader)?.ok_or(CardError::CardNotFound)?;
        let mut card = CharlieCard::new(reader, target)?;

        if self == &card {
            return Ok(());
        } else {
            return Err(CardError::WriteError.into());
        }
    }

    pub fn last_transaction(&self) -> Option<Transaction> {
        self.transaction_history.get(0).cloned()
    }

    pub fn print_info(&self) {
        println!("\nCharlieCard Info:");
        println!("Serial Number: {}", self.serial_num);
        println!("Card Type: {}", self.card_type);
        println!("Money Value: ${:.2}", self.money_value as f32 / 100.0);
        println!(
            "Expiration Date: {}\n",
            self.expiration
                .format(&format_description!(
                    "[weekday], [month repr:long] [day], [year]"
                ))
                .unwrap()
        );
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
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize)]
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

    pub fn card_type(&self) -> TicketType {
        let active_sector = self.active_sector();
        let block = self.sectors[active_sector].unwrap().data_blocks[1];
        // the first 10 bits in the second block of the active sector seems to
        // determine what type of card a CharlieCard is
        let card_type_number = ((block.data[0] as usize) << 2) + (block.data[1] as usize >> 6);
        match TicketType::from_repr(card_type_number) {
            Some(card_type) => card_type,
            None => TicketType::Nothing,
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

    pub fn expiration(&self, card_type: TicketType) -> OffsetDateTime {
        let active_sector = self.active_sector();
        let expiration_block = self.sectors[active_sector].unwrap().data_blocks[0];
        let raw_timestamp = (expiration_block.data[6] as u32 * 65536)
            + (expiration_block.data[7] as u32 * 256)
            + (expiration_block.data[8] as u32);

        let issue_date = CHARLIE_EPOCH + Duration::seconds(raw_timestamp as i64 * 60);

        // student cards last one school year and expire at the end of August
        // the next year
        if let TicketType::Student = card_type {
            return issue_date
                .replace_year(issue_date.year() + 1)
                .unwrap()
                .saturating_sub(Duration::DAY)
                .assume_timezone(NEW_YORK)
                .unwrap();
        }
        // CharlieCards issued before 2011 expire 7 years after they're issued
        if issue_date < datetime!(2011-01-01 0:00) {
            return issue_date
                .replace_year(issue_date.year() + 7)
                .unwrap()
                .saturating_sub(Duration::DAY)
                .assume_timezone(NEW_YORK)
                .unwrap();
        }

        // CharlieCards expire one day before 10 years after their date of issue
        issue_date
            .replace_year(issue_date.year() + 10)
            .unwrap()
            .saturating_sub(Duration::DAY)
            .assume_timezone(NEW_YORK)
            .unwrap()
    }

    pub fn end_validity(&self) -> OffsetDateTime {
        let active_sector = self.active_sector();
        let end_validity_block = self.sectors[active_sector].unwrap().data_blocks[1];
        // the end validity timestamp shares byte 1 with another variable, occupying only 3 bits
        // of byte 1 plus two other bytes, requiring a bitmask to calculate correctly
        let raw_timestamp = (end_validity_block.data[1] as i64 * 65536)
            + (end_validity_block.data[2] as i64 * 256)
            + (end_validity_block.data[3] as i64)
            & 0x1F_FF_FF;
        // the end validity date is the number of 8 minute increments (cause why not?) since
        // January 1st, 2003
        (CHARLIE_EPOCH + Duration::minutes(raw_timestamp * 8))
            .assume_timezone(NEW_YORK)
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
