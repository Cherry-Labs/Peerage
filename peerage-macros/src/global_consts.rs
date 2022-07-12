use std::collections::HashMap;

lazy_static! {
    pub static ref SIZE_TABLE: HashMap<usize, usize> = {
        let mut hm = HashMap::<usize, usize>::new();

        hm.insert(2, 1);
        hm.insert(4, 2);
        hm.insert(16, 8);
        hm.insert(256, 64);
        hm.insert(512, 128);
        hm.insert(1024, 256);
        hm.insert(2046, 512);
        hm.insert(4096, 1024);

        hm
    };

    pub static ref NAME_TABLE: HashMap<usize, &'static str> = {
        let mut hm = HashMap::<usize, &'static str>::new();

        hm.insert(2, "return_*_two");
        hm.insert(4, "return_*_four");
        hm.insert(16, "return_*_sixteen");
        hm.insert(256, "return_*_two_fifty_six");
        hm.insert(512, "return_*_five_twelve");
        hm.insert(1024, "return_*_ten_twenty_four");
        hm.insert(2048, "return_*_two_fourty_eight");
        hm.insert(4096, "return_*_forty_nighty_six");

        hm
    };
}