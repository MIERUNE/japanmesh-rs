use super::code::*;
use super::constants::PRIMARY_MESH_CODES;
use super::lnglat::LngLatBox;

pub fn iter_primary_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    PRIMARY_MESH_CODES
        .iter()
        .map(|&code| PrimaryCode::from_int(code).unwrap())
        .filter(move |prim| boundary.intersects_box(&prim.envelope()))
        .map(|prim| prim.envelope())
}

pub fn iter_secondary_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    iter_filtered_primary(boundary).flat_map(move |prim| {
        prim.iter_secondary()
            .filter(move |sec| boundary.intersects_box(&sec.envelope()))
            .map(|sec| sec.envelope())
    })
}

pub fn iter_standard_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    iter_filtered_secondary(boundary).flat_map(move |sec| {
        sec.iter_standard()
            .filter(move |std| boundary.intersects_box(&std.envelope()))
            .map(|std| std.envelope())
    })
}

pub fn iter_half_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    iter_filtered_standard(boundary).flat_map(move |std| {
        std.iter_half()
            .filter(move |half| boundary.intersects_box(&half.envelope()))
            .map(|half| half.envelope())
    })
}

pub fn iter_quarter_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    iter_filtered_half(boundary).flat_map(move |half| {
        half.iter_quad()
            .filter(move |quarter| boundary.intersects_box(&quarter.envelope()))
            .map(|quarter| quarter.envelope())
    })
}

pub fn iter_eighth_envelopes(boundary: LngLatBox) -> impl Iterator<Item = LngLatBox> {
    iter_filtered_quarter(boundary).flat_map(move |quarter| {
        quarter
            .iter_quad()
            .filter(move |eighth| boundary.intersects_box(&eighth.envelope()))
            .map(|eighth| eighth.envelope())
    })
}

fn iter_filtered_primary(boundary: LngLatBox) -> impl Iterator<Item = PrimaryCode> {
    PRIMARY_MESH_CODES
        .iter()
        .map(|&code| PrimaryCode::from_int(code).unwrap())
        .filter(move |prim| boundary.intersects_box(&prim.envelope()))
}

fn iter_filtered_secondary(boundary: LngLatBox) -> impl Iterator<Item = SecondaryCode> {
    iter_filtered_primary(boundary).flat_map(move |prim| {
        prim.iter_secondary()
            .filter(move |sec| boundary.intersects_box(&sec.envelope()))
    })
}

fn iter_filtered_standard(boundary: LngLatBox) -> impl Iterator<Item = StandardCode> {
    iter_filtered_secondary(boundary).flat_map(move |sec| {
        sec.iter_standard()
            .filter(move |std| boundary.intersects_box(&std.envelope()))
    })
}

fn iter_filtered_half(boundary: LngLatBox) -> impl Iterator<Item = HalfCode> {
    iter_filtered_standard(boundary).flat_map(move |std| {
        std.iter_half()
            .filter(move |half| boundary.intersects_box(&half.envelope()))
    })
}

fn iter_filtered_quarter(boundary: LngLatBox) -> impl Iterator<Item = QuarterCode> {
    iter_filtered_half(boundary).flat_map(move |half| {
        half.iter_quad()
            .filter(move |quarter| boundary.intersects_box(&quarter.envelope()))
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::gridsquare::lnglat::LngLat;

    #[test]
    fn test_envelopes() {
        let boundary = LngLatBox::new(
            LngLat::new(141.305438074, 42.939466350),
            LngLat::new(141.563765511, 43.129434849),
        );

        let count = iter_primary_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 1);

        let count = iter_secondary_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 9);

        let count = iter_standard_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 528);

        let count = iter_half_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 2021);

        let count = iter_quarter_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 7812);

        let count = iter_eighth_envelopes(boundary)
            .inspect(|e| {
                assert!(boundary.intersects_box(e));
            })
            .count();
        assert_eq!(count, 30544);
    }
}
