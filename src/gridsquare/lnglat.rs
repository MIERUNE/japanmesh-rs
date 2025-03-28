// Special LngLat utility to avoid floating point errors

pub(crate) const MULTIPLYER: f64 = 30.0;

#[derive(Clone, Copy, PartialEq)]
pub struct LngLat {
    pub(crate) vlng: f64,
    pub(crate) vlat: f64,
}

impl LngLat {
    #[inline]
    pub fn new(lng: f64, lat: f64) -> Self {
        LngLat {
            vlng: lng * MULTIPLYER,
            vlat: lat * MULTIPLYER,
        }
    }

    /// Create a new LngLat from pre-multiplied values
    pub fn new_raw(vlng: f64, vlat: f64) -> Self {
        LngLat { vlng, vlat }
    }

    #[inline]
    pub fn lng(&self) -> f64 {
        self.vlng / MULTIPLYER
    }

    #[inline]
    pub fn lat(&self) -> f64 {
        self.vlat / MULTIPLYER
    }
}

impl std::fmt::Debug for LngLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LngLat({}, {})", self.lng(), self.lat())
    }
}

impl std::fmt::Display for LngLat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LngLat({}, {})", self.lng(), self.lat())
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LngLatBox {
    min: LngLat,
    max: LngLat,
}

impl LngLatBox {
    #[inline]
    pub fn new(mut min: LngLat, mut max: LngLat) -> Self {
        if min.vlng > max.vlng {
            core::mem::swap(&mut min.vlng, &mut max.vlng);
        }
        if min.vlat > max.vlat {
            core::mem::swap(&mut min.vlat, &mut max.vlat);
        }
        LngLatBox { min, max }
    }

    #[inline]
    pub fn contains_point(&self, lnglat: LngLat) -> bool {
        lnglat.vlng >= self.min.vlng
            && lnglat.vlat >= self.min.vlat
            && lnglat.vlng < self.max.vlng
            && lnglat.vlat < self.max.vlat
    }

    #[inline]
    pub fn contains_box(&self, box2: &LngLatBox) -> bool {
        self.min.vlng <= box2.max.vlng
            && self.max.vlng >= box2.min.vlng
            && self.min.vlat <= box2.max.vlat
            && self.max.vlat >= box2.min.vlat
    }

    /// Divides this box into an N×N grid and returns the sub-box at position (x, y)
    pub fn split<const N: u8>(&self, x: u8, y: u8) -> Self {
        let dlng = (self.max.vlng - self.min.vlng) / N as f64;
        let dlat = (self.max.vlat - self.min.vlat) / N as f64;
        Self {
            min: LngLat::new_raw(
                self.min.vlng + dlng * x as f64,
                self.min.vlat + dlat * y as f64,
            ),
            max: LngLat::new_raw(
                self.min.vlng + dlng * (x + 1) as f64,
                self.min.vlat + dlat * (y + 1) as f64,
            ),
        }
    }
}
