use clap::Parser;
use flatgeobuf::GeometryType;
use geozero::{ColumnValue, FeatureProcessor, GeozeroGeometry};
use std::{io::BufWriter, path::PathBuf};

use japanmesh::gridsquare::{LngLatBox, primaries_in_land, standard_patches};

#[derive(Parser)]
#[command(author, version, about = "Generate Japanese grid square mesh")]
struct Args {
    /// Output file path (.fgb for Flatgeobuf, .geojson.gz for GeoJSON)
    #[arg(short, long, default_value = "output.fgb")]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // let boundary = LngLatBox::new(LngLat::new(100.0, 0.0), LngLat::new(180.0, 90.0));
    let iter = standard_patches(primaries_in_land(), None);

    let output_path = args.output.to_string_lossy().to_string();

    if output_path.ends_with(".fgb") {
        // Flatgeobuf output
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
        add_geometries(&mut fgb, iter)?;
        eprintln!("Writing Flatgeobuf file to {}...", output_path);
        let file = std::fs::File::create(&output_path)?;
        fgb.write(BufWriter::new(file))?;
    } else if output_path.ends_with(".geojson.gz") {
        // GeoJSON output
        eprintln!("Writing GeoJSON file to {}...", output_path);
        let file = std::fs::File::create(&output_path)?;
        let gz_writer =
            flate2::write::GzEncoder::new(BufWriter::new(file), flate2::Compression::default());
        let mut processor = geozero::geojson::GeoJsonWriter::new(gz_writer);
        add_geometries(&mut processor, iter)?;
    } else {
        return Err(
            "Output file must have .fgb (Flatgeobuf) or .geojson.gz (GeoJSON) extension".into(),
        );
    }
    Ok(())
}

fn add_geometries(
    processor: &mut impl FeatureProcessor,
    iter: impl Iterator<Item = (impl ToString, LngLatBox)>,
) -> geozero::error::Result<()> {
    processor.dataset_begin(None)?;
    for (count, (code, envelope)) in iter.enumerate() {
        if count % 100_000 == 0 {
            eprintln!("{} features written", count);
        }
        let min = envelope.min();
        let max = envelope.max();
        let polygon =
            geo::geometry::Rect::new([min.lng(), min.lat()], [max.lng(), max.lat()]).to_polygon();
        let geom = geo::geometry::Geometry::Polygon(polygon);

        processor.feature_begin(count as u64)?;
        processor.properties_begin()?;
        processor.property(0, "code", &ColumnValue::String(&code.to_string()))?;
        processor.properties_end()?;
        processor.geometry_begin()?;
        geom.process_geom(processor)?;
        processor.geometry_end()?;
        processor.feature_end(count as u64)?;
    }
    processor.dataset_end()?;
    Ok(())
}
