//! Grid square code

// Note:
//
// primary:   [YY][XX]
// secondary: [YY][XX][0-7][0-7]
// (x5):      [YY][XX][0-7][0-7][1-4]
// (x2):      [YY][XX][0-7][0-7][even][even]5
// standard:  [YY][XX][0-7][0-7][0-9][0-9]
// half:      [YY][XX][0-7][0-7][0-9][0-9][1-4]
// quarter:   [YY][XX][0-7][0-7][0-9][0-9][1-4][1-4]
// eights:    [YY][XX][0-7][0-7][0-9][0-9][1-4][1-4][1-4]
//
// 3 4
// 1 2

use super::lnglat::{LngLat, LngLatBox};
use crate::Error;
use std::{fmt::Display, str::FromStr};

// pub enum Level {
//     Primary,
//     Secondary,
//     // X5,
//     // X2,
//     Standard,
//     Half,
//     Quarter,
//     Eighth,
// }

#[derive(Debug)]
pub enum LevelAndCode {
    Primary(PrimaryCode),
    Secondary(SecondaryCode),
    // X5,
    // X2,
    Standard(StandardCode),
    Half(HalfCode),
    Quarter(QuarterCode),
    Eighth(EighthCode),
}

impl LevelAndCode {
    /// Automatically determine the level and code from a integer value
    pub fn from_int(code: u64) -> Result<LevelAndCode, Error> {
        let digits = code.ilog10() as usize + 1;
        Ok(match digits {
            4 => LevelAndCode::Primary(PrimaryCode::from_int(code as u16)?),
            6 => LevelAndCode::Secondary(SecondaryCode::from_int(code as u32)?),
            7 => todo!(), // X5
            8 => LevelAndCode::Standard(StandardCode::from_int(code as u32)?),
            9 => LevelAndCode::Half(HalfCode::from_int(code as u32)?), // TODO: or X2
            10 => LevelAndCode::Quarter(QuarterCode::from_int(code)?),
            11 => LevelAndCode::Eighth(EighthCode::from_int(code)?),
            _ => return Err(Error::InvalidCode),
        })
    }
}

impl FromStr for LevelAndCode {
    type Err = Error;

    /// Automatically determine the level and code from a string value
    fn from_str(code: &str) -> Result<LevelAndCode, Error> {
        let digits = code.len();
        Ok(match digits {
            4 => LevelAndCode::Primary(PrimaryCode::from_str(code)?),
            6 => LevelAndCode::Secondary(SecondaryCode::from_str(code)?),
            7 => todo!(), // X5
            8 => LevelAndCode::Standard(StandardCode::from_str(code)?),
            9 => LevelAndCode::Half(HalfCode::from_str(code)?), // TODO: or X2
            10 => LevelAndCode::Quarter(QuarterCode::from_str(code)?),
            11 => LevelAndCode::Eighth(EighthCode::from_str(code)?),
            _ => return Err(Error::InvalidCode),
        })
    }
}

pub trait GridSquareCode {
    /// Returns the bounding box of the code
    fn patch(&self) -> LngLatBox;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PrimaryCode {
    /// "YY__"
    y: u8,
    /// "--XX"
    x: u8,
}

impl PrimaryCode {
    #[inline]
    pub fn from_int(code: u16) -> Result<Self, Error> {
        if code > 9999 {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            y: (code / 100) as u8,
            x: (code % 100) as u8,
        })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let y = (lnglat.vlat / 20.) % 100.0;
        let x = lnglat.lng() % 100.0;
        if y >= 100.0 || x >= 100.0 || y < 0.0 || x < 0.0 {
            return Err(Error::OutOfBounds);
        }
        Ok(Self {
            y: y as u8,
            x: x as u8,
        })
    }

    #[inline]
    pub fn y1(&self) -> u8 {
        self.y
    }

    #[inline]
    pub fn x1(&self) -> u8 {
        self.x
    }

    pub fn iter_secondary(&self) -> impl Iterator<Item = SecondaryCode> {
        (0..=7).flat_map(move |x2| {
            (0..=7).map(move |y2| SecondaryCode {
                primary: *self,
                y2,
                x2,
            })
        })
    }
}

impl GridSquareCode for PrimaryCode {
    fn patch(&self) -> LngLatBox {
        LngLatBox::new(
            LngLat::new_raw(
                ((self.x1() as u32 + 100) * 30) as f64,
                (self.y1() as u32 * 20) as f64,
            ),
            LngLat::new_raw(
                ((self.x1() as u32 + 101) * 30) as f64,
                ((self.y1() as u32 + 1) * 20) as f64,
            ),
        )
    }
}

impl FromStr for PrimaryCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 4 {
            return Err(Error::InvalidCode);
        }
        if let Some((y1_str, x1_str)) = s.split_at_checked(2) {
            let y1 = y1_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
            let x1 = x1_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
            Ok(Self { y: y1, x: x1 })
        } else {
            Err(Error::InvalidCode)
        }
    }
}

impl Display for PrimaryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}{:02}", self.y, self.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SecondaryCode {
    primary: PrimaryCode,
    /// "----Y-"
    y2: u8,
    /// "-----X"
    x2: u8,
}

impl SecondaryCode {
    #[inline]
    pub fn from_int(code: u32) -> Result<Self, Error> {
        if code > 999999 {
            return Err(Error::InvalidCode);
        }
        let y2 = ((code % 100) / 10) as u8;
        let x2 = (code % 10) as u8;
        if y2 >= 7 || x2 >= 7 {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            primary: PrimaryCode::from_int((code / 100) as u16)?,
            y2,
            x2,
        })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let yd = (lnglat.vlat / 20. * 8.) as u32 % 8;
        let xd = (lnglat.vlng / 30. * 8.) as u32 % 8;
        Ok(Self {
            primary: PrimaryCode::from_lnglat(lnglat)?,
            y2: yd as u8,
            x2: xd as u8,
        })
    }

    #[inline]
    pub fn y1(&self) -> u8 {
        self.primary.y
    }

    #[inline]
    pub fn x1(&self) -> u8 {
        self.primary.x
    }

    #[inline]
    pub fn y2(&self) -> u8 {
        self.y2
    }

    #[inline]
    pub fn x2(&self) -> u8 {
        self.x2
    }

    #[inline]
    pub fn primary(&self) -> PrimaryCode {
        self.primary
    }

    pub fn iter_standard(&self) -> impl Iterator<Item = StandardCode> {
        (0..=9).flat_map(move |x3| {
            (0..=9).map(move |y3| StandardCode {
                secondary: *self,
                y3,
                x3,
            })
        })
    }
}

impl GridSquareCode for SecondaryCode {
    fn patch(&self) -> LngLatBox {
        LngLatBox::new(
            LngLat::new_raw(
                ((self.x1() as u32 + 100) * 30) as f64 + self.x2() as f64 * 3.75,
                (self.y1() as u32 * 20) as f64 + self.y2() as f64 * 2.5,
            ),
            LngLat::new_raw(
                ((self.x1() as u32 + 100) * 30) as f64 + (self.x2() + 1) as f64 * 3.75,
                (self.y1() as u32 * 20) as f64 + (self.y2() + 1) as f64 * 2.5,
            ),
        )
    }
}

impl FromStr for SecondaryCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 6 {
            return Err(Error::InvalidCode);
        }
        let Some((primary, rest)) = s.split_at_checked(4) else {
            return Err(Error::InvalidCode);
        };
        let Some((y2_str, x2_str)) = rest.split_at_checked(1) else {
            return Err(Error::InvalidCode);
        };
        let y2 = y2_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        let x2 = x2_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        if y2 >= 7 || x2 >= 7 {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            primary: PrimaryCode::from_str(primary)?,
            y2,
            x2,
        })
    }
}

impl Display for SecondaryCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}{:02}{}{}",
            self.primary.y, self.primary.x, self.y2, self.x2
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StandardCode {
    secondary: SecondaryCode,
    /// "------Y-"
    y3: u8,
    /// "-------X"
    x3: u8,
}

impl StandardCode {
    #[inline]
    pub fn from_int(code: u32) -> Result<Self, Error> {
        if code > 99999999 {
            return Err(Error::InvalidCode);
        }
        let secondary = SecondaryCode::from_int(code / 100)?;
        let y3 = ((code % 100) / 10) as u8;
        let x3 = (code % 10) as u8;
        Ok(Self { secondary, y3, x3 })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let yd = (lnglat.vlat / 20. * 8. * 10.) as u32 % 10;
        let xd = (lnglat.vlng / 30. * 8. * 10.) as u32 % 10;
        Ok(Self {
            secondary: SecondaryCode::from_lnglat(lnglat)?,
            y3: yd as u8,
            x3: xd as u8,
        })
    }

    #[inline]
    pub fn y1(&self) -> u8 {
        self.secondary.y1()
    }

    #[inline]
    pub fn x1(&self) -> u8 {
        self.secondary.x1()
    }

    #[inline]
    pub fn y2(&self) -> u8 {
        self.secondary.y2()
    }

    #[inline]
    pub fn x2(&self) -> u8 {
        self.secondary.x2()
    }

    #[inline]
    pub fn y3(&self) -> u8 {
        self.y3
    }

    #[inline]
    pub fn x3(&self) -> u8 {
        self.x3
    }

    #[inline]
    pub fn primary(&self) -> PrimaryCode {
        self.secondary.primary
    }

    #[inline]
    pub fn secondary(&self) -> SecondaryCode {
        self.secondary
    }

    pub fn iter_half(&self) -> impl Iterator<Item = Quad<Self>> {
        (1..=4).map(move |quad| HalfCode {
            parent: *self,
            quad,
        })
    }
}

impl GridSquareCode for StandardCode {
    fn patch(&self) -> LngLatBox {
        LngLatBox::new(
            LngLat::new_raw(
                ((self.x1() as u32 + 100) * 30) as f64
                    + self.x2() as f64 * 3.75
                    + self.x3() as f64 * 0.375,
                (self.y1() as u32 * 20) as f64 + self.y2() as f64 * 2.5 + self.y3() as f64 * 0.25,
            ),
            LngLat::new_raw(
                ((self.x1() as u32 + 100) * 30) as f64
                    + self.x2() as f64 * 3.75
                    + (self.x3() + 1) as f64 * 0.375,
                (self.y1() as u32 * 20) as f64
                    + self.y2() as f64 * 2.5
                    + (self.y3() + 1) as f64 * 0.25,
            ),
        )
    }
}

impl FromStr for StandardCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 8 {
            return Err(Error::InvalidCode);
        }
        let Some((secondary, rest)) = s.split_at_checked(6) else {
            return Err(Error::InvalidCode);
        };
        let Some((y3_str, x3_str)) = rest.split_at_checked(1) else {
            return Err(Error::InvalidCode);
        };
        let y3 = y3_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        let x3 = x3_str.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        Ok(Self {
            secondary: SecondaryCode::from_str(secondary)?,
            y3,
            x3,
        })
    }
}

impl Display for StandardCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:02}{:02}{}{}{}{}",
            self.secondary.y1(),
            self.secondary.x1(),
            self.secondary.y2(),
            self.secondary.x2(),
            self.y3,
            self.x3
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quad<P: GridSquareCode> {
    parent: P,
    /// 1-4
    quad: u8,
}

impl<P: GridSquareCode + Copy> Quad<P> {
    pub fn new(parent: P, quad: u8) -> Self {
        assert!(quad < 4);
        Self { parent, quad }
    }

    pub fn iter_quad(&self) -> impl Iterator<Item = Quad<Self>> {
        (1..=4).map(move |quad| Quad::<Self> {
            parent: *self,
            quad,
        })
    }
}

impl<P: GridSquareCode> GridSquareCode for Quad<P> {
    fn patch(&self) -> LngLatBox {
        let patch = self.parent.patch();
        let d = self.quad - 1;
        patch.split::<2>(d & 1, d >> 1)
    }
}

impl<P: GridSquareCode + Display> Display for Quad<P> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.parent.fmt(f)?;
        write!(f, "{}", self.quad)
    }
}

pub type HalfCode = Quad<StandardCode>;
pub type QuarterCode = Quad<HalfCode>;
pub type EighthCode = Quad<QuarterCode>;

impl HalfCode {
    pub fn from_int(code: u32) -> Result<Self, Error> {
        let parent = StandardCode::from_int(code / 10)?;
        let quad = (code % 10) as u8;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self { parent, quad })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let yd = ((lnglat.vlat / 20. * 8. * 10. * 2.) as u64 % 2) as u8;
        let xd = ((lnglat.vlng / 30. * 8. * 10. * 2.) as u64 % 2) as u8;
        let quad = (yd << 1) + xd + 1;
        Ok(Self {
            parent: StandardCode::from_lnglat(lnglat)?,
            quad,
        })
    }

    pub fn y1(&self) -> u8 {
        self.parent.y1()
    }

    pub fn x1(&self) -> u8 {
        self.parent.x1()
    }

    pub fn y2(&self) -> u8 {
        self.parent.y2()
    }

    pub fn x2(&self) -> u8 {
        self.parent.x2()
    }

    pub fn y3(&self) -> u8 {
        self.parent.y3()
    }

    pub fn x3(&self) -> u8 {
        self.parent.x3()
    }

    pub fn quad1(&self) -> u8 {
        self.quad
    }
}

impl FromStr for HalfCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 9 {
            return Err(Error::InvalidCode);
        }
        let Some((standard, rest)) = s.split_at_checked(8) else {
            return Err(Error::InvalidCode);
        };
        let quad = rest.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            parent: StandardCode::from_str(standard)?,
            quad,
        })
    }
}

impl QuarterCode {
    pub fn from_int(code: u64) -> Result<Self, Error> {
        let parent = HalfCode::from_int((code / 10) as u32)?;
        let quad = (code % 10) as u8;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self { parent, quad })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let yd = ((lnglat.vlat / 20. * 8. * 10. * 4.) as u64 % 2) as u8;
        let xd = ((lnglat.vlng / 30. * 8. * 10. * 4.) as u64 % 2) as u8;
        let quad = (yd << 1) + xd + 1;
        Ok(Self {
            parent: HalfCode::from_lnglat(lnglat)?,
            quad,
        })
    }

    pub fn y1(&self) -> u8 {
        self.parent.y1()
    }

    pub fn x1(&self) -> u8 {
        self.parent.x1()
    }

    pub fn y2(&self) -> u8 {
        self.parent.y2()
    }

    pub fn x2(&self) -> u8 {
        self.parent.x2()
    }

    pub fn y3(&self) -> u8 {
        self.parent.y3()
    }

    pub fn x3(&self) -> u8 {
        self.parent.x3()
    }

    pub fn quad1(&self) -> u8 {
        self.parent.quad1()
    }

    pub fn quad2(&self) -> u8 {
        self.quad
    }
}

impl FromStr for QuarterCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 10 {
            return Err(Error::InvalidCode);
        }
        let Some((half, rest)) = s.split_at_checked(9) else {
            return Err(Error::InvalidCode);
        };
        let quad = rest.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            parent: HalfCode::from_str(half)?,
            quad,
        })
    }
}

impl EighthCode {
    pub fn from_int(code: u64) -> Result<Self, Error> {
        let parent = QuarterCode::from_int(code / 10)?;
        let quad = (code % 10) as u8;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self { parent, quad })
    }

    #[inline]
    pub fn from_lnglat(lnglat: LngLat) -> Result<Self, Error> {
        let yd = ((lnglat.vlat / 20. * 8. * 10. * 8.) as u64 % 2) as u8;
        let xd = ((lnglat.vlng / 30. * 8. * 10. * 8.) as u64 % 2) as u8;
        let quad = (yd << 1) + xd + 1;
        Ok(Self {
            parent: QuarterCode::from_lnglat(lnglat)?,
            quad,
        })
    }

    pub fn y1(&self) -> u8 {
        self.parent.y1()
    }

    pub fn x1(&self) -> u8 {
        self.parent.x1()
    }

    pub fn y2(&self) -> u8 {
        self.parent.y2()
    }

    pub fn x2(&self) -> u8 {
        self.parent.x2()
    }

    pub fn y3(&self) -> u8 {
        self.parent.y3()
    }

    pub fn x3(&self) -> u8 {
        self.parent.x3()
    }

    pub fn quad1(&self) -> u8 {
        self.parent.quad1()
    }

    pub fn quad2(&self) -> u8 {
        self.parent.quad2()
    }

    pub fn quad3(&self) -> u8 {
        self.quad
    }
}

impl FromStr for EighthCode {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() != 11 {
            return Err(Error::InvalidCode);
        }
        let Some((half, rest)) = s.split_at_checked(10) else {
            return Err(Error::InvalidCode);
        };
        let quad = rest.parse::<u8>().map_err(|_| Error::InvalidCode)?;
        if !(1..=4).contains(&quad) {
            return Err(Error::InvalidCode);
        }
        Ok(Self {
            parent: QuarterCode::from_str(half)?,
            quad,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_primary_code() {
        assert!(matches!(
            LevelAndCode::from_int(9999),
            Ok(LevelAndCode::Primary(_))
        ));
        assert!(matches!(
            LevelAndCode::from_str("9999"),
            Ok(LevelAndCode::Primary(_))
        ));

        let code = PrimaryCode::from_int(1234).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);

        let code = PrimaryCode::from_str("1234").unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);

        assert_eq!(code.to_string(), "1234");
        PrimaryCode::from_str("12345").expect_err("must be 4 digits");

        let code = PrimaryCode::from_int(6441).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(30. * 141.0, 20. * 64.),
                LngLat::new_raw(30. * 142.0, 20. * 65.)
            )
        );

        let code = PrimaryCode::from_lnglat(LngLat::new(141.99, 43.33)).unwrap();
        assert_eq!(code.to_string(), "6441");

        let code = PrimaryCode::from_lnglat(LngLat::new(142.01, 43.34)).unwrap();
        assert_eq!(code.to_string(), "6542");
    }

    #[test]
    fn test_secondary_code() {
        assert!(matches!(
            LevelAndCode::from_int(123456),
            Ok(LevelAndCode::Secondary(_))
        ));

        let code = SecondaryCode::from_int(123456).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.to_string(), "123456");
        assert_eq!(code.primary(), PrimaryCode::from_int(1234).unwrap());

        let code = SecondaryCode::from_str("123456").unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);

        SecondaryCode::from_str("1234567").expect_err("must be 6 digits");
        SecondaryCode::from_str("123488").expect_err("y2 and x2 must be less than 8");

        let code = SecondaryCode::from_int(644142).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(30. * (141.0 + 2. / 8.), 20. * (64. + 4. / 8.)),
                LngLat::new_raw(30. * (141.0 + 3. / 8.), 20. * (64. + 5. / 8.))
            )
        );

        let code = SecondaryCode::from_lnglat(LngLat::new(141.87132, 43.24550)).unwrap();
        assert_eq!(code.to_string(), "644166");

        let code = SecondaryCode::from_lnglat(LngLat::new(141.88596, 43.25935)).unwrap();
        assert_eq!(code.to_string(), "644177");
    }

    #[test]
    fn test_standard_code() {
        assert!(matches!(
            LevelAndCode::from_int(12345678),
            Ok(LevelAndCode::Standard(_))
        ));
        assert!(matches!(
            LevelAndCode::from_str("12345678"),
            Ok(LevelAndCode::Standard(_))
        ));

        let code = StandardCode::from_int(12345678).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.y3(), 7);
        assert_eq!(code.x3(), 8);
        assert_eq!(code.to_string(), "12345678");
        assert_eq!(code.secondary(), SecondaryCode::from_int(123456).unwrap());
        assert_eq!(code.primary(), PrimaryCode::from_int(1234).unwrap());

        let code = StandardCode::from_str("12345678").unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.y3(), 7);
        assert_eq!(code.x3(), 8);

        SecondaryCode::from_str("123456789").expect_err("must be 8 digits");
        SecondaryCode::from_str("12348899").expect_err("y2 and x2 must be less than 8");

        let code = StandardCode::from_int(64414278).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + 8. / 10.) / 8.,
                    20. * 64. + 20. * (4. + 7. / 10.) / 8.,
                ),
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + 9. / 10.) / 8.,
                    20. * 64. + 20. * (4. + 8. / 10.) / 8.,
                ),
            )
        );

        let code = StandardCode::from_lnglat(LngLat::new(141.861882, 43.249259)).unwrap();
        assert_eq!(code.to_string(), "64416698");
    }

    #[test]
    fn test_half_code() {
        assert!(matches!(
            LevelAndCode::from_int(123456781),
            Ok(LevelAndCode::Half(_))
        ));
        assert!(matches!(
            LevelAndCode::from_str("123456781"),
            Ok(LevelAndCode::Half(_))
        ));

        let code = HalfCode::from_int(123456781).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.y3(), 7);
        assert_eq!(code.x3(), 8);
        assert_eq!(code.quad1(), 1);
        assert_eq!(code.to_string(), "123456781");

        let code2 = HalfCode::from_str("123456781").unwrap();
        assert_eq!(code, code2);

        let code = HalfCode::from_int(644142782).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + (8. + 0.5) / 10.) / 8.,
                    20. * 64. + 20. * (4. + 7. / 10.) / 8.,
                ),
                LngLat::new_raw(
                    30. * (141.0 + (2. + 9. / 10.) / 8.),
                    20. * 64. + 20. * (4. + (7. + 0.5) / 10.) / 8.,
                ),
            )
        );

        let code = HalfCode::from_lnglat(LngLat::new(141.8686782, 43.2405564)).unwrap();
        assert_eq!(code.to_string(), "644166893");
    }

    #[test]
    fn test_quarter_code() {
        assert!(matches!(
            LevelAndCode::from_int(1234567812),
            Ok(LevelAndCode::Quarter(_))
        ));
        assert!(matches!(
            LevelAndCode::from_str("1234567812"),
            Ok(LevelAndCode::Quarter(_))
        ));

        let code = QuarterCode::from_int(1234567812).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.y3(), 7);
        assert_eq!(code.x3(), 8);
        assert_eq!(code.quad1(), 1);
        assert_eq!(code.quad2(), 2);
        assert_eq!(code.to_string(), "1234567812");

        let code2 = QuarterCode::from_str("1234567812").unwrap();
        assert_eq!(code, code2);

        let code = QuarterCode::from_int(6441427823).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + (8. + 0.5) / 10.) / 8.,
                    20. * 64. + 20. * (4. + (7. + 0.25) / 10.) / 8.,
                ),
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + (8. + 0.5 + 0.25) / 10.) / 8.,
                    20. * 64. + 20. * (4. + (7. + 0.5) / 10.) / 8.,
                ),
            )
        );

        let code = QuarterCode::from_lnglat(LngLat::new(141.8686782, 43.2405564)).unwrap();
        assert_eq!(code.to_string(), "6441668934");
    }

    #[test]
    fn test_eighth_code() {
        assert!(matches!(
            LevelAndCode::from_int(12345678123),
            Ok(LevelAndCode::Eighth(_))
        ));
        assert!(matches!(
            LevelAndCode::from_str("12345678123"),
            Ok(LevelAndCode::Eighth(_))
        ));

        let code = EighthCode::from_int(12345678123).unwrap();
        assert_eq!(code.y1(), 12);
        assert_eq!(code.x1(), 34);
        assert_eq!(code.y2(), 5);
        assert_eq!(code.x2(), 6);
        assert_eq!(code.y3(), 7);
        assert_eq!(code.x3(), 8);
        assert_eq!(code.quad1(), 1);
        assert_eq!(code.quad2(), 2);
        assert_eq!(code.quad3(), 3);
        assert_eq!(code.to_string(), "12345678123");

        let code2 = EighthCode::from_str("12345678123").unwrap();
        assert_eq!(code, code2);

        let code = EighthCode::from_int(64414278234).unwrap();
        assert_eq!(
            code.patch(),
            LngLatBox::new(
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + (8. + 0.5 + 0.125) / 10.) / 8.,
                    20. * 64. + 20. * (4. + (7. + 0.25 + 0.125) / 10.) / 8.,
                ),
                LngLat::new_raw(
                    30. * 141.0 + 30. * (2. + (8. + 0.5 + 0.25) / 10.) / 8.,
                    20. * 64. + 20. * (4. + (7. + 0.5) / 10.) / 8.,
                ),
            )
        );

        let code = EighthCode::from_lnglat(LngLat::new(141.8686372, 43.2404931)).unwrap();
        assert_eq!(code.to_string(), "64416689342");
    }
}
