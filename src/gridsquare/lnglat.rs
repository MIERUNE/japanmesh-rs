// Special LngLat utility to avoid floating point errors
//
// Internal (lng, lat) values are multiplied by 30.0 to avoid floating point errors

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
    pub const fn lng(&self) -> f64 {
        self.vlng / MULTIPLYER
    }

    #[inline]
    pub const fn lat(&self) -> f64 {
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
    pub fn min(&self) -> LngLat {
        self.min
    }

    #[inline]
    pub fn max(&self) -> LngLat {
        self.max
    }

    #[inline]
    pub fn contains_point(&self, lnglat: LngLat) -> bool {
        lnglat.vlng >= self.min.vlng
            && lnglat.vlat >= self.min.vlat
            && lnglat.vlng < self.max.vlng
            && lnglat.vlat < self.max.vlat
    }

    #[inline]
    pub fn contains_box(&self, target: &LngLatBox) -> bool {
        self.max.vlng >= target.max.vlng
            && self.min.vlng <= target.min.vlng
            && self.max.vlat >= target.max.vlat
            && self.min.vlat <= target.min.vlat
    }

    #[inline]
    pub fn intersects_box(&self, target: &LngLatBox) -> bool {
        self.min.vlng <= target.max.vlng
            && self.max.vlng >= target.min.vlng
            && self.min.vlat <= target.max.vlat
            && self.max.vlat >= target.min.vlat
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lnglat() {
        let lnglat = LngLat::new(139.25, 35.5);
        assert_eq!(lnglat.lng(), 139.25);
        assert_eq!(lnglat.lat(), 35.5);
        assert_eq!(lnglat.to_string(), "LngLat(139.25, 35.5)");
    }

    #[test]
    fn test_lnglat_box() {
        // normal
        let box1 = LngLatBox::new(LngLat::new(139.25, 35.5), LngLat::new(139.30, 35.6));
        assert_eq!(box1.min(), LngLat::new(139.25, 35.5));
        assert_eq!(box1.max(), LngLat::new(139.30, 35.6));
        // swap
        let box1 = LngLatBox::new(LngLat::new(139.30, 35.5), LngLat::new(139.25, 35.6));
        assert_eq!(box1.min, LngLat::new(139.25, 35.5));
        assert_eq!(box1.max, LngLat::new(139.30, 35.6));
        // swap
        let box2 = LngLatBox::new(LngLat::new(139.25, 35.6), LngLat::new(139.30, 35.5));
        assert_eq!(box2.min, LngLat::new(139.25, 35.5));
        assert_eq!(box2.max, LngLat::new(139.30, 35.6));
    }

    #[test]
    fn test_lnglat_box_contains() {
        let box1 = LngLatBox::new(LngLat::new(139.25, 35.5), LngLat::new(139.30, 35.6));
        assert!(box1.contains_point(LngLat::new(139.27, 35.55)));
        assert!(!box1.contains_point(LngLat::new(139.27, 35.65)));
        assert!(!box1.contains_point(LngLat::new(139.27, 35.45)));
        assert!(!box1.contains_point(LngLat::new(139.22, 35.55)));
        assert!(!box1.contains_point(LngLat::new(139.22, 35.55)));

        assert!(box1.contains_box(&box1));
        assert!(box1.contains_box(&LngLatBox::new(
            LngLat::new(139.25, 35.5),
            LngLat::new(139.30, 35.6)
        )));
        assert!(!box1.contains_box(&LngLatBox::new(
            LngLat::new(139.25, 35.5),
            LngLat::new(139.30, 35.7)
        )));
        assert!(!box1.contains_box(&LngLatBox::new(
            LngLat::new(139.25, 35.4),
            LngLat::new(139.30, 35.6)
        )));
        assert!(!box1.contains_box(&LngLatBox::new(
            LngLat::new(139.24, 35.5),
            LngLat::new(139.30, 35.6)
        )));
        assert!(!box1.contains_box(&LngLatBox::new(
            LngLat::new(139.25, 35.4),
            LngLat::new(139.30, 35.6)
        )));
    }
}
