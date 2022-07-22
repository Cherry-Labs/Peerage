use peerage_utils::bin_utils::QuadrupleWord;


#[derive(Clone, Copy)]
pub struct PassPhraseJumblator {
    passphrase: [u128; 8],
    qw_passphrase: [QuadrupleWord; 8],

}