mod mapper0;
mod mapper2;
mod mapper4;

use crate::ines_loader::INesHeaderInfo;
use std::io::{BufReader, Read};
use crate::mappers::mapper0::create_mapper0_cartdrige;
use crate::cartdrige::Cartdrige;
use crate::mappers::mapper2::create_mapper2_cartdrige;
use crate::mappers::mapper4::create_mapper4_cartdrige;

pub fn create_cartdrige<T: Read>(header: INesHeaderInfo, mut reader: BufReader<T>) -> Box<dyn Cartdrige> {
    println!("Reading cartdrige (header: {:?})", &header);
    match header.mapper_type {
        0 => { return create_mapper0_cartdrige(header, &mut reader); }
        2 => { return create_mapper2_cartdrige(header, &mut reader); }
        4 => { return create_mapper4_cartdrige(header, &mut reader); }
        _ => {
            panic!("Unsupported mapper type: {}", header.mapper_type)
        }
    }
}