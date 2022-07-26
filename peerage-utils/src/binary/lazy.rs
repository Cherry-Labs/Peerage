use crate::binary::bit::Bit;
use std::{collections::HashMap, ops::Rem, vec};

lazy_static! {
    pub static ref HEX_MAP: HashMap<Vec<Bit>, String> = {
        let mut hm = HashMap::<Vec<Bit>, String>::new();

        hm.insert(
            vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::Zero],
            "0".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::Zero, Bit::Zero, Bit::One],
            "1".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::Zero, Bit::One, Bit::Zero],
            "2".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::Zero, Bit::One, Bit::One],
            "3".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::One, Bit::Zero, Bit::Zero],
            "4".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::One, Bit::Zero, Bit::One],
            "5".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::One, Bit::One, Bit::Zero],
            "6".to_string(),
        );
        hm.insert(
            vec![Bit::Zero, Bit::One, Bit::One, Bit::One],
            "7".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::Zero, Bit::Zero, Bit::Zero],
            "8".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::Zero, Bit::Zero, Bit::One],
            "9".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::Zero, Bit::One, Bit::Zero],
            "A".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::Zero, Bit::One, Bit::One],
            "B".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::One, Bit::Zero, Bit::Zero],
            "C".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::One, Bit::Zero, Bit::One],
            "D".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::One, Bit::One, Bit::Zero],
            "E".to_string(),
        );
        hm.insert(
            vec![Bit::One, Bit::One, Bit::One, Bit::One],
            "F".to_string(),
        );

        hm
    };
}
