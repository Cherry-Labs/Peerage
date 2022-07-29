use peerage_utils::bin_utils::Bit;
use peerage_rand::f64_rand::random_f64;

pub struct TrigIdentities(f64);

impl TrigIdentities {
    pub fn new_rand() -> Self {
        let rand = random_f64();

        Self(rand)
    }

    pub fn unwrap(&self) -> f64 {
        let TrigIdentities(item) = self;
    
        *item
    }

    pub fn enc_bits(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let rand = self.unwrap();
        
        let bits_f64 = Bit::vec_self_to_vec_f64(bits);

        let mut m = 0usize;

        let v = bits_f64
            .into_iter()
            .map(|xx| {
                let x = match m % 2 == 0 {
                    true => xx - rand,
                    false => xx * rand,
                };

                m += 1;

                let sine_x = SineTheta::from(x);
                let sine_x_unwrap = sine_x.unwrap();

                let cosine_x_conv = sine_x.to_cosine();
                let cosine_x_conv_unwrap = cosine_x_conv.unwrap();

                let tan_x_conv = sine_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();

                let cotan_x_conv = sine_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand - sine_x_unwrap
                        / cosine_x_conv_unwrap
                        / tan_x_conv_unwrap
                        / (cotan_x_conv_unwrap + 0.00000001)    
            })
            .map(|x| {
                let cosine_x = CosineTheta::from(x);
                let cosine_x_unwrap = cosine_x.unwrap();

                let sine_x_conv = cosine_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let tan_x_conv = cosine_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();


                let cotan_x_conv = cosine_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand - cosine_x_unwrap
                        / sine_x_conv_unwrap
                        / tan_x_conv_unwrap
                        / (cotan_x_conv_unwrap + 0.00000001)          
            })
            .map(|x| {
                let tan_x = TangentTheta::from(x);
                let tan_x_unwrap = tan_x.unwrap();

                let sine_x_conv = tan_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let cosine_x_conv = tan_x.to_cosine();
                let cosine_x_conv_unwrap = cosine_x_conv.unwrap();

                let cotan_x_conv = tan_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand - tan_x_unwrap
                        / sine_x_conv_unwrap
                        / cosine_x_conv_unwrap
                        / (cotan_x_conv_unwrap + 0.00000001)  
            })
            .map(|x| {
                let cotan_x = CotangentTheta::from(x);
                let cotan_x_unwrap = cotan_x.unwrap();

                let sine_x_conv = cotan_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let cosine_x_conv = cotan_x.to_cosine();
                let cosine_x_conv_unwrap =cosine_x_conv.unwrap();

                let tan_x_conv = cotan_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();

                rand - tan_x_conv_unwrap
                        / sine_x_conv_unwrap
                        / cosine_x_conv_unwrap
                        / (cotan_x_unwrap + 0.00000001)  
            })
            .map(|x| x.abs())
            .collect::<Vec<f64>>();

        Bit::vec_f64_to_vec_self(v)
    }


    pub fn dec_bits(&self, bits: Vec<Bit>) -> Vec<Bit> {
        let rand = self.unwrap();
        
        let bits_f64 = Bit::vec_self_to_vec_f64(bits);

        let mut m = 0usize;

        let v = bits_f64
            .into_iter()
            .map(|xx| {
                let x = match m % 2 == 0 {
                    true => xx + rand,
                    false => xx / rand,
                };

                m += 1;

                let sine_x = SineTheta::from(x);
                let sine_x_unwrap = sine_x.unwrap();

                let cosine_x_conv = sine_x.to_cosine();
                let cosine_x_conv_unwrap = cosine_x_conv.unwrap();

                let tan_x_conv = sine_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();

                let cotan_x_conv = sine_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand + sine_x_unwrap
                        * cosine_x_conv_unwrap
                        * tan_x_conv_unwrap
                        * cotan_x_conv_unwrap    
            })
            .map(|x| {
                let cosine_x = CosineTheta::from(x);
                let cosine_x_unwrap = cosine_x.unwrap();

                let sine_x_conv = cosine_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let tan_x_conv = cosine_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();


                let cotan_x_conv = cosine_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand + cosine_x_unwrap
                        * sine_x_conv_unwrap
                        * tan_x_conv_unwrap
                        * (cotan_x_conv_unwrap + 0.00000001)          
            })
            .map(|x| {
                let tan_x = TangentTheta::from(x);
                let tan_x_unwrap = tan_x.unwrap();

                let sine_x_conv = tan_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let cosine_x_conv = tan_x.to_cosine();
                let cosine_x_conv_unwrap = cosine_x_conv.unwrap();

                let cotan_x_conv = tan_x.to_cotangent();
                let cotan_x_conv_unwrap = cotan_x_conv.unwrap();

                rand + tan_x_unwrap
                        * sine_x_conv_unwrap
                        * cosine_x_conv_unwrap
                        * (cotan_x_conv_unwrap + 0.00000001)  
            })
            .map(|x| {
                let cotan_x = CotangentTheta::from(x);
                let cotan_x_unwrap = cotan_x.unwrap();

                let sine_x_conv = cotan_x.to_sine();
                let sine_x_conv_unwrap = sine_x_conv.unwrap();

                let cosine_x_conv = cotan_x.to_cosine();
                let cosine_x_conv_unwrap =cosine_x_conv.unwrap();

                let tan_x_conv = cotan_x.to_tangent();
                let tan_x_conv_unwrap = tan_x_conv.unwrap();

                rand + tan_x_conv_unwrap
                        * sine_x_conv_unwrap
                        * cosine_x_conv_unwrap
                        * (cotan_x_unwrap + 0.00000001)  
            })
            .map(|x| x.abs())
            .collect::<Vec<f64>>();

        Bit::vec_f64_to_vec_self(v)
    }


}



pub struct SineTheta(f64);
pub struct CosineTheta(f64);
pub struct TangentTheta(f64);
pub struct CotangentTheta(f64);

impl SineTheta {
    pub fn new(sine: f64) -> Self {
        Self(sine)
    }

    pub fn from(angle: f64) -> Self {
        let angle_sine  = angle.sin();

        Self(angle_sine)
    }

    pub fn unwrap(&self) -> f64 {
        let SineTheta(sine) = self;

        sine.clone()
    }

    pub fn unwrap_mult_10(&self) -> i32 {
        let SineTheta(sine) = self;

        (sine.clone() * 10.0) as i32
    }

    pub fn mutate(&mut self, self_value: f64) {
        *self = SineTheta(self_value)
    }

    pub fn to_cosine(&self) -> CosineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_min_sq = 1.0 - self_squared;
        let all_sqrt = one_min_sq.sqrt();

        CosineTheta::new(all_sqrt)
    }

    pub fn to_tangent(&self) -> TangentTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_min_sq = 1.0 - self_squared;
        let all_sqrt = one_min_sq.sqrt();
        let sin_div_all = self_value / all_sqrt;

        TangentTheta::new(sin_div_all)
    }

    pub fn to_cotangent(&self) -> CotangentTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_min_sq = 1.0 - self_squared;
        let all_sqrt = one_min_sq.sqrt();
        let all_div_sin = all_sqrt / self_value;

        CotangentTheta::new(all_div_sin)
    }
}

impl CosineTheta {
    pub fn new(cosine: f64) -> Self {
        Self(cosine)
    }

    pub fn from(angle: f64) -> Self {
        let angle_cosine  = angle.cos();

        Self(angle_cosine)
    }

    pub fn unwrap(&self) -> f64 {
        let CosineTheta(cosine) = self;

        cosine.clone()
    }

    pub fn unwrap_mult_10(&self) -> i32 {
        let CosineTheta(cosine) = self;

        (cosine.clone() * 10.0) as i32
    }

    pub fn mutate(&mut self, cosine: f64) {
        *self = CosineTheta(cosine)
    }

    pub fn to_sine(&self) -> SineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_min_sq = 1.0 - self_squared;
        let all_sqrt = one_min_sq.sqrt();

        SineTheta::new(all_sqrt)
    }

    pub fn to_tangent(&self) -> TangentTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let self_min_one = 1.0 - self_squared;
        let self_min_sqrt = self_min_one.sqrt();
        let all_div_cos = self_min_sqrt / self_value;

        TangentTheta(all_div_cos)
    }

    pub fn to_cotangent(&self) -> CotangentTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let self_min_one = 1.0 - self_squared;
        let self_min_sqrt = self_min_one.sqrt();
        let cos_div_all = self_value / self_min_sqrt;

        CotangentTheta(cos_div_all)
    }
}

impl TangentTheta {
    pub fn new(tangent: f64) -> Self {
        Self(tangent)
    }

    pub fn from(angle: f64) -> Self {
        let angle_tangent  = angle.tan();

        Self(angle_tangent)
    }

    pub fn unwrap(&self) -> f64 {
        let TangentTheta(tangent) = self;

        tangent.clone()
    }

    pub fn unwrap_mult_10(&self) -> i32 {
        let TangentTheta(tangent) = self;

        (tangent.clone() * 10.0) as i32
    }

    pub fn mutate(&mut self, tangent: f64) {
        *self = TangentTheta(tangent)
    }

    pub fn to_sine(&self) -> SineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_pls_sq = 1.0 + self_squared;
        let all_sqrt = one_pls_sq.sqrt();
        let div_tan = self_value / all_sqrt;

        SineTheta::new(div_tan)
    }

    pub fn to_cosine(&self) -> CosineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let one_pls_sq = 1.0 + self_squared;
        let all_sqrt = one_pls_sq.sqrt();
        let tan_div = all_sqrt / self_value;

        CosineTheta::new(tan_div)
    }

    pub fn to_cotangent(&self) -> CotangentTheta {
        let self_value = self.unwrap();

        let cotangent = 1.0 / self_value;

        CotangentTheta::new(cotangent)
    }
}

impl CotangentTheta {
    pub fn new(cotangent: f64) -> Self {
        Self(cotangent)
    }

    pub fn from(angle: f64) -> Self {
        let angle_cotangent  = 1.0 / angle.tan();

        Self(angle_cotangent)
    }
    
    pub fn unwrap(&self) -> f64 {
        let CotangentTheta(cotangent) = self;

        cotangent.clone()
    }

    pub fn unwrap_mult_10(&self) -> i32 {
        let CotangentTheta(cotangent) = self;

        (cotangent.clone() * 10.0) as i32
    }

    pub fn mutate(&mut self, cotangent: f64) {
        *self = CotangentTheta(cotangent)
    }

    pub fn to_sine(&self) -> SineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let sqr_pls_one = 1.0 + self_squared;
        let sqr_pls_sqrt = sqr_pls_one.sqrt();
        let one_div_sqrt = 1.0 / sqr_pls_sqrt;

        SineTheta::new(one_div_sqrt)
    }

    pub fn to_cosine(&self) -> CosineTheta {
        let self_value = self.unwrap();

        let self_squared = self_value.powf(2.0);
        let sqr_pls_one = 1.0 + self_squared;
        let sqr_pls_sqrt = sqr_pls_one.sqrt();
        let self_div_sqrt = self_value / sqr_pls_sqrt;

        CosineTheta::new(self_div_sqrt)
    }

    pub fn to_tangent(&self) -> TangentTheta {
        let self_value = self.unwrap();

        let inverse = 1.0 / self_value;

        TangentTheta::new(inverse)
    }
}
