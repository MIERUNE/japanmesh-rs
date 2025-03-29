use super::{constants::PRIMARIES_IN_LAND, *};

pub fn primary_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (PrimaryCode, LngLatBox)> {
    primary_iter.filter_map(move |prim| {
        let patch = prim.envelope();
        boundary
            .is_none_or(|b| b.intersects_box(&patch))
            .then_some((prim, patch))
    })
}

pub fn secondary_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (SecondaryCode, LngLatBox)> {
    primary_iter.flat_map(move |prim| {
        prim.iter_secondary().filter_map(move |sec| {
            let patch = sec.envelope();
            boundary
                .is_none_or(|b| b.intersects_box(&patch))
                .then_some((sec, patch))
        })
    })
}

pub fn standard_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (StandardCode, LngLatBox)> {
    secondary_codes(primary_iter, boundary).flat_map(move |sec| {
        sec.iter_standard().filter_map(move |std| {
            let patch = std.envelope();
            boundary
                .is_none_or(|b| b.intersects_box(&patch))
                .then_some((std, patch))
        })
    })
}

pub fn half_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (HalfCode, LngLatBox)> {
    standard_codes(primary_iter, boundary).flat_map(move |std| {
        std.iter_half().filter_map(move |half| {
            let patch = half.envelope();
            boundary
                .is_none_or(|b| b.intersects_box(&patch))
                .then_some((half, patch))
        })
    })
}

pub fn quarter_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (QuarterCode, LngLatBox)> {
    half_codes(primary_iter, boundary).flat_map(move |half| {
        half.iter_quad().filter_map(move |quarter| {
            let patch = quarter.envelope();
            boundary
                .is_none_or(|b| b.intersects_box(&patch))
                .then_some((quarter, patch))
        })
    })
}

pub fn eighth_patches(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = (EighthCode, LngLatBox)> {
    quarter_codes(primary_iter, boundary).flat_map(move |quarter| {
        quarter.iter_quad().filter_map(move |eighth| {
            let patch = eighth.envelope();
            boundary
                .is_none_or(|b| b.intersects_box(&patch))
                .then_some((eighth, patch))
        })
    })
}

pub fn primaries_in_land() -> impl Iterator<Item = PrimaryCode> {
    PRIMARIES_IN_LAND.iter().cloned()
}

pub fn primaries_from_bounds(bounds: LngLatBox) -> impl Iterator<Item = PrimaryCode> {
    let xa = (bounds.min().lng() as i32 - 100).clamp(0, 100);
    let xb = (bounds.max().lng().ceil() as i32 - 100).clamp(0, 100);
    let ya = ((bounds.min().vlat / 20.0) as i32).clamp(0, 99);
    let yb = ((bounds.max().vlat / 20.0).ceil() as i32).clamp(0, 100);
    (ya..yb).flat_map(move |y| {
        (xa..xb).map(move |x| PrimaryCode {
            y: y as u8,
            x: x as u8,
        })
    })
}

fn secondary_codes(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = SecondaryCode> {
    primary_iter.flat_map(move |prim| {
        prim.iter_secondary()
            .filter(move |sec| boundary.is_none_or(|b| b.intersects_box(&sec.envelope())))
    })
}

fn standard_codes(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = StandardCode> {
    secondary_codes(primary_iter, boundary).flat_map(move |sec| {
        sec.iter_standard()
            .filter(move |std| boundary.is_none_or(|b| b.intersects_box(&std.envelope())))
    })
}

fn half_codes(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = HalfCode> {
    standard_codes(primary_iter, boundary).flat_map(move |std| {
        std.iter_half()
            .filter(move |half| boundary.is_none_or(|b| b.intersects_box(&half.envelope())))
    })
}

fn quarter_codes(
    primary_iter: impl Iterator<Item = PrimaryCode>,
    boundary: Option<LngLatBox>,
) -> impl Iterator<Item = QuarterCode> {
    half_codes(primary_iter, boundary).flat_map(move |half| {
        half.iter_quad()
            .filter(move |quarter| boundary.is_none_or(|b| b.intersects_box(&quarter.envelope())))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_patches() {
        let boundary = LngLatBox::new(
            LngLat::new(141.305438074, 42.939466350),
            LngLat::new(141.563765511, 43.129434849),
        );

        let count = primary_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 1);

        let count = secondary_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 9);

        let count = standard_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 528);

        let count = half_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 2021);

        let count = quarter_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 7812);

        let count = eighth_patches(primaries_in_land(), Some(boundary))
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 30544);
    }

    #[test]
    fn test_primaries_from_bounds() {
        assert_eq!(
            primaries_from_bounds(LngLatBox::new(
                LngLat::new(141.305438074, 42.939466350),
                LngLat::new(141.563765511, 43.129434849),
            ))
            .count(),
            1
        );
        assert_eq!(
            primaries_from_bounds(LngLatBox::new(
                LngLat::new(141.305438074, 42.939466350),
                LngLat::new(142.0, 43.529434849), // lat exceeds the boundary
            ))
            .count(),
            2
        );
        assert_eq!(
            primaries_from_bounds(LngLatBox::new(
                LngLat::new(141.305438074, 42.939466350),
                LngLat::new(142.01, 43.129434849), // lng exceeds the boundary
            ))
            .count(),
            2
        );
        assert_eq!(
            primaries_from_bounds(LngLatBox::new(
                LngLat::new(141.305438074, 42.939466350),
                LngLat::new(142.01, 43.529434849), // both lng and lat exceeds the boundary
            ))
            .count(),
            4
        );
    }
}
