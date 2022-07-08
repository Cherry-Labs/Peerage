use peerage_utils::bin_utils::*;


fn convert_to_byte(v: Vec<u8>) -> Vec<Byte> {
    v.into_iter()
        .map(|x| Byte::from_decimal(x, Endian::Little))
        .collect::<Vec<Byte>>()
}


pub fn convert_256_byte_chunk_to_words(chunks: Vec<u8>) -> Vec<ByteWord> {
    let mut ret: Vec<ByteWord> = vec![];

    for i in (0..256).step_by(32) {
        let step = &chunks[i..i + 32].to_vec();

        let step_clone = step.clone();

        let bytes_vec = convert_to_byte(step_clone);

        let bw = ByteWord::from_byte_vec(bytes_vec);

        ret.push(bw);
    }

    ret
}


pub fn xor_all_bws_in_a_vec(v: Vec<ByteWord>) -> ByteWord {
    let mut ret = v[0];

    for bw in &v[1..] {
        ret = ret ^ *bw;
    }

    ret
}


pub fn add_all_bws_together(v: Vec<ByteWord>) -> ByteWord {
    let mut ret = v[0];

    for bw in &v[1..] {
        ret = ret + *bw;
    }

    ret
}

pub fn set_at_index_odd_even(added_together: &ByteWord, fin_bw: &mut ByteWord, index: usize) {
    let set = match added_together.assert_is_odd() {
        true => Bit::One,
        false => Bit::Zero,
    };

    fin_bw.set_at_index(index, set)
}