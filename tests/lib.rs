extern crate tiled;
extern crate ggez;

use std::path::{Path, PathBuf};
use tiled::{parse, parse_tileset, Map, PropertyValue, TiledError};
use ggez::{GameResult, filesystem::Filesystem};

fn init_filesystem() -> GameResult<Filesystem> {
    Filesystem::new("rs-tiled", "Difarem")
}

fn read_from_file<P: AsRef<Path>>(fs: &mut Filesystem, p: P) -> Result<Map<()>, TiledError> {
    parse(fs, p)
}

#[test]
fn test_gzip_and_zlib_encoded_and_raw_are_the_same() {
    let mut fs = init_filesystem().unwrap();

    let z = read_from_file(&mut fs, "/tiled_base64_zlib.tmx").unwrap();
    let g = read_from_file(&mut fs, "/tiled_base64_gzip.tmx").unwrap();
    let r = read_from_file(&mut fs, "/tiled_base64.tmx").unwrap();
    let c = read_from_file(&mut fs, "/tiled_csv.tmx").unwrap();
    assert_eq!(z, g);
    assert_eq!(z, r);
    assert_eq!(z, c);
}

#[test]
fn test_external_tileset() {
    let mut fs = init_filesystem().unwrap();

    let r = read_from_file(&mut fs, "/tiled_base64.tmx").unwrap();
    let e = read_from_file(&mut fs, "/tiled_base64_external.tmx").unwrap();
    assert_eq!(r, e);
}

#[test]
fn test_just_tileset() {
    let mut fs = init_filesystem().unwrap();

    let r = read_from_file(&mut fs, "/tiled_base64.tmx").unwrap();
    let t = parse_tileset(fs.open("/tilesheet.tsx").unwrap(), 1).unwrap();
    assert_eq!(r.tilesets[0], t);
}

#[test]
fn test_image_layers() {
    let mut fs = init_filesystem().unwrap();

    let r = read_from_file(&mut fs, "/tiled_image_layers.tmx").unwrap();
    assert_eq!(r.image_layers.len(), 2);
    {
        let first = &r.image_layers[0];
        assert_eq!(first.name, "Image Layer 1");
        assert!(
            first.image.is_none(),
            "{}'s image should be None",
            first.name
        );
    }
    {
        let second = &r.image_layers[1];
        assert_eq!(second.name, "Image Layer 2");
        let image = second
            .image
            .as_ref()
            .expect(&format!("{}'s image shouldn't be None", second.name));
        assert_eq!(image.source, PathBuf::from("tilesheet.png"));
        assert_eq!(image.width, 448);
        assert_eq!(image.height, 192);
    }
}

#[test]
fn test_tile_property() {
    let mut fs = init_filesystem().unwrap();

    let r = read_from_file(&mut fs, "/tiled_base64.tmx").unwrap();
    let prop_value: String = if let Some(&PropertyValue::StringValue(ref v)) =
        r.tilesets[0].tiles[0].properties.get("a tile property")
    {
        v.clone()
    } else {
        String::new()
    };
    assert_eq!("123", prop_value);
}
