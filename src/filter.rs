
pub struct BoxFilter {
    pub xradius: f32,
    pub yradius: f32,
}

impl BoxFilter {
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        if x.abs() > self.xradius || y.abs() > self.yradius {
            return 0.0;
        }
        return 1.0;
    }
}

pub struct TriangleFilter {
    pub xradius: f32,
    pub yradius: f32,
}

impl TriangleFilter {
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        (self.xradius - x.abs()).max(0.0) * (self.yradius - y.abs()).max(0.0)
    }
}

pub struct GaussianFilter {
    pub xradius: f32,
    pub yradius: f32,
    pub alpha: f32,
    exp_x: f32,
    exp_y: f32,
}

impl GaussianFilter {
    pub fn new(xradius: f32, yradius: f32, alpha: f32) -> Self {
        let exp_x = (-alpha * xradius * xradius).exp();
        let exp_y = (-alpha * yradius * yradius).exp();
        Self { xradius, yradius, alpha, exp_x, exp_y }
    }

    fn gaussian(&self, d: f32, expv: f32) -> f32 {
        return ((-self.alpha * d * d).exp() - expv).max(0.0);
    }

    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        self.gaussian(x, self.exp_x) * self.gaussian(y, self.exp_y)
    }
}

pub struct MitchellFilter {
    pub xradius: f32,
    pub yradius: f32,
    pub b: f32,
    pub c: f32,
    inv_xradius: f32,
    inv_yradius: f32,
}

impl MitchellFilter {
    pub fn new(xradius: f32, yradius: f32, b: f32, c: f32) -> Self {
        let inv_xradius = 1.0 / xradius;
        let inv_yradius = 1.0 / yradius;
        Self { xradius, yradius, b, c, inv_xradius, inv_yradius }
    }

    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        if x.abs() > self.xradius || y.abs() > self.yradius {
            return 0.0;
        }
        return 1.0;
    }
}

pub struct LanczosSincFilter {
    pub xradius: f32,
    pub yradius: f32,
    pub tau: f32,
}

impl LanczosSincFilter {
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        if x.abs() > self.xradius || y.abs() > self.yradius {
            return 0.0;
        }
        return 1.0;
    }
}

pub enum Filter {
    Box(BoxFilter),
    Triangle(TriangleFilter),
    Gaussian(GaussianFilter),
    Mitchell(MitchellFilter),
    LanczosSinc(LanczosSincFilter),
}

impl Filter {
    pub fn evaluate(&self, x: f32, y: f32) -> f32 {
        match self {
            Filter::Box(filter) => filter.evaluate(x, y),
            Filter::Triangle(filter) => filter.evaluate(x, y),
            Filter::Gaussian(filter) => filter.evaluate(x, y),
            Filter::Mitchell(filter) => filter.evaluate(x, y),
            Filter::LanczosSinc(filter) => filter.evaluate(x, y),
        }
    }

    pub fn max_radius(&self) -> f32 {
        match self {
            Filter::Box(filter) => filter.xradius.max(filter.yradius),
            Filter::Triangle(filter) => filter.xradius.max(filter.yradius),
            Filter::Gaussian(filter) => filter.xradius.max(filter.yradius),
            Filter::Mitchell(filter) => filter.xradius.max(filter.yradius),
            Filter::LanczosSinc(filter) => filter.xradius.max(filter.yradius),
        }
    }
}

pub enum FilterType {
    Box,
    Triangle,
    Gaussian,
    Mitchell,
    LanczosSinc,
}

pub struct FilterDescriptor {
    pub filter_type: FilterType,
    pub xradius: f32,
    pub yradius: f32,
    pub alpha: f32,
    pub b: f32,
    pub c: f32,
    pub tau: f32,
}

impl FilterDescriptor {
    pub fn create(&self) -> Filter {
        match self.filter_type {
            FilterType::Box => Filter::Box(BoxFilter { xradius: self.xradius, yradius: self.yradius }),
            FilterType::Triangle => Filter::Triangle(TriangleFilter { xradius: self.xradius, yradius: self.yradius }),
            FilterType::Gaussian => Filter::Gaussian(GaussianFilter::new(self.xradius, self.yradius, self.alpha)),
            FilterType::Mitchell => Filter::Mitchell(MitchellFilter::new(self.xradius, self.yradius, self.b, self.c)),
            FilterType::LanczosSinc => Filter::LanczosSinc(LanczosSincFilter { xradius: self.xradius, yradius: self.yradius, tau: self.tau }),
        }
    }

    pub fn set_default_radius(&mut self) {
        match self.filter_type {
            FilterType::Box => {
                self.xradius = 0.5;
                self.yradius = 0.5;
            }
            FilterType::Triangle=> {
                self.xradius = 2.0;
                self.yradius = 2.0;
            }
            FilterType::Gaussian => {
                self.xradius = 1.5;
                self.yradius = 1.5;
            }
            FilterType::Mitchell => {
                self.xradius = 2.0;
                self.yradius = 2.0;
            }
            FilterType::LanczosSinc => {
                self.xradius = 4.0;
                self.yradius = 4.0;
            }
        }
    }
}

impl Default for FilterDescriptor {
    fn default() -> Self {
        Self {
            filter_type: FilterType::Triangle,
            xradius: 2.0,
            yradius: 2.0,
            alpha: 0.5,
            b: 0.3333333333,
            c: 0.3333333333,
            tau: 3.0,
        }
    }
}

