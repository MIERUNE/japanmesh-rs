use super::lnglat::LngLatBox;
use super::{PRIMARIES_IN_LAND, code::*};

pub fn iter_primary_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (PrimaryCode, LngLatBox)> {
    PRIMARIES_IN_LAND.iter().filter_map(move |prim| {
        let patch = prim.envelope();
        boundary.intersects_box(&patch).then_some((*prim, patch))
    })
}

pub fn iter_secondary_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (SecondaryCode, LngLatBox)> {
    iter_primary_with_boundary(boundary).flat_map(move |prim| {
        prim.iter_secondary().filter_map(move |sec| {
            let patch = sec.envelope();
            boundary.intersects_box(&patch).then_some((sec, patch))
        })
    })
}

pub fn iter_standard_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (StandardCode, LngLatBox)> {
    iter_secondary_with_boundary(boundary).flat_map(move |sec| {
        sec.iter_standard().filter_map(move |std| {
            let patch = std.envelope();
            boundary.intersects_box(&patch).then_some((std, patch))
        })
    })
}

pub fn iter_half_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (HalfCode, LngLatBox)> {
    iter_standard_with_boundary(boundary).flat_map(move |std| {
        std.iter_half().filter_map(move |half| {
            let patch = half.envelope();
            boundary.intersects_box(&patch).then_some((half, patch))
        })
    })
}

pub fn iter_quarter_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (QuarterCode, LngLatBox)> {
    iter_half_with_boundary(boundary).flat_map(move |half| {
        half.iter_quad().filter_map(move |quarter| {
            let patch = quarter.envelope();
            boundary.intersects_box(&patch).then_some((quarter, patch))
        })
    })
}

pub fn iter_eighth_patches_with_boundary(
    boundary: LngLatBox,
) -> impl Iterator<Item = (EighthCode, LngLatBox)> {
    iter_quarter_with_boundary(boundary).flat_map(move |quarter| {
        quarter.iter_quad().filter_map(move |eighth| {
            let patch = eighth.envelope();
            boundary.intersects_box(&patch).then_some((eighth, patch))
        })
    })
}

fn iter_primary_with_boundary(boundary: LngLatBox) -> impl Iterator<Item = PrimaryCode> {
    PRIMARIES_IN_LAND
        .iter()
        .filter(move |prim| boundary.intersects_box(&prim.envelope()))
        .copied()
}

fn iter_secondary_with_boundary(boundary: LngLatBox) -> impl Iterator<Item = SecondaryCode> {
    iter_primary_with_boundary(boundary).flat_map(move |prim| {
        prim.iter_secondary()
            .filter(move |sec| boundary.intersects_box(&sec.envelope()))
    })
}

fn iter_standard_with_boundary(boundary: LngLatBox) -> impl Iterator<Item = StandardCode> {
    iter_secondary_with_boundary(boundary).flat_map(move |sec| {
        sec.iter_standard()
            .filter(move |std| boundary.intersects_box(&std.envelope()))
    })
}

fn iter_half_with_boundary(boundary: LngLatBox) -> impl Iterator<Item = HalfCode> {
    iter_standard_with_boundary(boundary).flat_map(move |std| {
        std.iter_half()
            .filter(move |half| boundary.intersects_box(&half.envelope()))
    })
}

fn iter_quarter_with_boundary(boundary: LngLatBox) -> impl Iterator<Item = QuarterCode> {
    iter_half_with_boundary(boundary).flat_map(move |half| {
        half.iter_quad()
            .filter(move |quarter| boundary.intersects_box(&quarter.envelope()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gridsquare::LngLat;

    #[test]
    fn test_iter_primary() {
        let boundary = LngLatBox::new(
            LngLat::new(141.305438074, 42.939466350),
            LngLat::new(141.563765511, 43.129434849),
        );

        let count = iter_primary_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 1);

        let count = iter_secondary_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 9);

        let count = iter_standard_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 528);

        let count = iter_half_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 2021);

        let count = iter_quarter_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 7812);

        let count = iter_eighth_patches_with_boundary(boundary)
            .inspect(|(_, e)| assert!(boundary.intersects_box(e)))
            .count();
        assert_eq!(count, 30544);
    }
}
