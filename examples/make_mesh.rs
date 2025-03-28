use flatgeobuf::geozero::PropertyProcessor;
use flatgeobuf::{ColumnType, GeometryType};
use geozero::ColumnValue;
use japanmesh::gridsquare::{LngLat, LngLatBox, iter_standard_patches_with_boundary};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut fgb = flatgeobuf::FgbWriter::create_with_options(
        "japanese-grid-square-code",
        GeometryType::Polygon,
        flatgeobuf::FgbWriterOptions {
            crs: flatgeobuf::FgbCrs {
                code: 6668, // JGD2011
                ..Default::default()
            },
            write_index: true,
            ..Default::default()
        },
    )?;
    fgb.add_column("code", ColumnType::String, |_fbb, _col| {});
    let boundary = LngLatBox::new(LngLat::new(100.0, 0.0), LngLat::new(180.0, 90.0));
    let iter = iter_standard_patches_with_boundary(boundary);

    add_geometries(&mut fgb, iter)?;

    // Write .fgb file
    eprintln!("Writing .fgb file...");
    let file = std::fs::File::create("output.fgb")?;
    fgb.write(file)?;

    Ok(())
}

fn add_geometries(
    fgb: &mut flatgeobuf::FgbWriter<'_>,
    iter: impl Iterator<Item = (impl ToString, LngLatBox)>,
) -> geozero::error::Result<()> {
    for (count, (code, envelope)) in iter.enumerate() {
        if count % 100_000 == 0 {
            eprintln!("{} features written", count);
        }
        let min = envelope.min();
        let max = envelope.max();
        let polygon =
            geo::geometry::Rect::new([min.lng(), min.lat()], [max.lng(), max.lat()]).to_polygon();
        let geom = geo::geometry::Geometry::Polygon(polygon);

        fgb.add_feature_geom(geom, |feat| {
            feat.property(0, "code", &ColumnValue::String(&code.to_string()))
                .unwrap();
        })?;
    }
    Ok(())
}
