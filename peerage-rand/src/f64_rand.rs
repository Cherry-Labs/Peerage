use std::time::{SystemTime, UNIX_EPOCH};



fn sixteen_u8s_from_u128(u: u128) -> [u8; 16] {
    let u128_bits = format!("{u:0128b}");

    let mut u8s = [0u8; 16];

    for i in (0usize..128usize).step_by(16) {
        let sub_str = &u128_bits[i..i + 16];
        let u = u8::from_str_radix(sub_str, 2).unwrap();

        let ind = i / 16;

        u8s[ind] = u;
    }

    u8s
}


pub fn random_f64() -> f64 {
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    

    let mut duration = since_the_epoch.as_nanos();

    let u8s = sixteen_u8s_from_u128(duration);

    let num1 = u8s[0]
                    % u8s[1]
                    * u8s[2]
                    ^ u8s[3]
                    / u8s[4]
                    & u8s[5]
                    + u8s[6]
                    - u8s[7];

    let num2 = u8s[8]
                    * u8s[9]
                    - u8s[10]
                    & u8s[11]
                    - u8s[12]
                    | u8s[13]
                    * u8s[14]
                    / u8s[15];

    match num1 > num2 {
        true => num2 as f64 / num1 as f64,
        false => num1 as f64 / num2 as f64,
    }

}