pub struct SineTheta(f64);
pub struct CosineTheta(f64);
pub struct TangentTheta(f64);
pub struct CotangentTheta(f64);

impl SineTheta {
    pub fn new(self_value: f64) -> Self {
        Self(self_value)
    }

    pub fn unwrap(&self) -> f64 {
        let SineTheta(sine) = self;

        sine.clone()
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
    pub fn new(self_value: f64) -> Self {
        Self(self_value)
    }

    pub fn unwrap(&self) -> f64 {
        let CosineTheta(cosine) = self;

        cosine.clone()
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
    pub fn new(self_value: f64) -> Self {
        Self(self_value)
    }

    pub fn unwrap(&self) -> f64 {
        let TangentTheta(tangent) = self;

        tangent.clone()
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
    pub fn new(self_value: f64) -> Self {
        Self(self_value)
    }

    pub fn unwrap(&self) -> f64 {
        let CotangentTheta(cotangent) = self;

        cotangent.clone()
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
