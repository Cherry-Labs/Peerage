use shuffle::shuffler::Shuffler;
use shuffle::irs::Irs;
use rand::rngs::mock::StepRng;



pub fn shufle_between_0_and_3() -> Vec<usize> {
    let mut rng = StepRng::new(4, 130);
    let mut irs = Irs::default();

    let mut input = vec![0usize, 1usize, 2usize, 3usize];

    irs.shuffle(&mut input, &mut rng);

    input
}