use std::io::Read;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use ggez::graphics::Image as GgezImage;
use ggez::filesystem;
use ggez::Context;

use {Image, Tile};
use error::TiledError;

/// A tileset, usually the tilesheet image.
#[derive(Debug, PartialEq, Clone)]
pub struct Tileset<I = GgezImage> {
    /// The GID of the first tile stored
    pub first_gid: u32,
    pub name: String,
    pub tile_width: u32,
    pub tile_height: u32,
    pub spacing: u32,
    pub margin: u32,
    /// The Tiled spec says that a tileset can have mutliple images so a `Vec`
    /// is used. Usually you will only use one.
    pub images: Vec<Image<I>>,
    pub tiles: Vec<Tile<I>>,
}

impl Tileset<()> {
    pub(crate) fn new<R: Read, P: AsRef<Path>>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
        fs: &mut filesystem::Filesystem,
        map_path: P,
    ) -> Result<Tileset<()>, TiledError> {
        Tileset::<()>::new_internal(parser, &attrs).or_else(|_| Tileset::<()>::new_reference(&attrs, fs, map_path))
    }

    pub(crate) fn new_internal<R: Read>(
        parser: &mut EventReader<R>,
        attrs: &Vec<OwnedAttribute>,
    ) -> Result<Tileset<()>, TiledError> {
        let ((spacing, margin), (first_gid, name, width, height)) = get_attrs!(
           attrs,
           optionals: [("spacing", spacing, |v:String| v.parse().ok()),
                       ("margin", margin, |v:String| v.parse().ok())],
           required: [("firstgid", first_gid, |v:String| v.parse().ok()),
                      ("name", name, |v| Some(v)),
                      ("tilewidth", width, |v:String| v.parse().ok()),
                      ("tileheight", height, |v:String| v.parse().ok())],
           TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

        let mut images = Vec::new();
        let mut tiles = Vec::new();
        parse_tag!(parser, "tileset",
                   "image" => |attrs| {
                        images.push(try!(Image::<()>::new(parser, attrs)));
                        Ok(())
                   },
                   "tile" => |attrs| {
                        tiles.push(try!(Tile::<()>::new(parser, attrs)));
                        Ok(())
                   });

        Ok(Tileset::<()> {
            first_gid: first_gid,
            name: name,
            tile_width: width,
            tile_height: height,
            spacing: spacing.unwrap_or(0),
            margin: margin.unwrap_or(0),
            images: images,
            tiles: tiles,
        })
    }

//     pub(crate) fn new_reference<P: AsRef<Path>>(
//         attrs: &Vec<OwnedAttribute>,
//         fs: &mut filesystem::Filesystem,
//         map_path: P,
//     ) -> Result<Tileset<()>, TiledError> {
//         let ((), (first_gid, source)) = get_attrs!(
//            attrs,
//            optionals: [],
//            required: [("firstgid", first_gid, |v:String| v.parse().ok()),
//                       ("source", name, |v| Some(v))],
//            TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

//         let tileset_path = map_path.as_ref().with_file_name(source);
//         let file = filesystem::open(ctx, &tileset_path).map_err(|err| {
//             TiledError::GgezError(err)
//         })?;
//         Tileset::<()>::new_external(file, first_gid)
//     }

//     pub(crate) fn new_external<R: Read>(file: R, first_gid: u32) -> Result<Tileset<()>, TiledError> {
//         let mut tileset_parser = EventReader::new(file);
//         loop {
//             match try!(tileset_parser.next().map_err(TiledError::XmlDecodingError)) {
//                 XmlEvent::StartElement {
//                     name, attributes, ..
//                 } => {
//                     if name.local_name == "tileset" {
//                         return Tileset::<()>::parse_external_tileset(
//                             first_gid,
//                             &mut tileset_parser,
//                             &attributes,
//                         );
//                     }
//                 }
//                 XmlEvent::EndDocument => {
//                     return Err(TiledError::PrematureEnd(
//                         "Tileset Document ended before map was parsed".to_string(),
//                     ))
//                 }
//                 _ => {}
//             }
//         }
//     }

    fn parse_external_tileset<R: Read>(
        first_gid: u32,
        parser: &mut EventReader<R>,
        attrs: &Vec<OwnedAttribute>,
    ) -> Result<Tileset<()>, TiledError> {
        let ((spacing, margin), (name, width, height)) = get_attrs!(
            attrs,
            optionals: [("spacing", spacing, |v:String| v.parse().ok()),
                        ("margin", margin, |v:String| v.parse().ok())],
            required: [("name", name, |v| Some(v)),
                       ("tilewidth", width, |v:String| v.parse().ok()),
                       ("tileheight", height, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

        let mut images = Vec::new();
        let mut tiles = Vec::new();
        parse_tag!(parser, "tileset",
                   "image" => |attrs| {
                       images.push(try!(Image::<()>::new(parser, attrs)));
                       Ok(())
                   },
                   "tile" => |attrs| {
                       tiles.push(try!(Tile::<()>::new(parser, attrs)));
                       Ok(())
                   });

        Ok(Tileset::<()> {
            first_gid: first_gid,
            name: name,
            tile_width: width,
            tile_height: height,
            spacing: spacing.unwrap_or(0),
            margin: margin.unwrap_or(0),
            images: images,
            tiles: tiles,
        })
    }
}

impl Tileset<GgezImage> {
    pub(crate) fn new<R: Read, P: AsRef<Path>>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
        ctx: &mut Context,
        map_path: P,
    ) -> Result<Tileset, TiledError> {
        Tileset::<GgezImage>::new_internal(parser, &attrs, ctx, map_path.as_ref()).or_else(|_| Tileset::<GgezImage>::new_reference(&attrs, ctx, map_path.as_ref()))
    }

    pub(crate) fn new_internal<R: Read, P: AsRef<Path>>(
        parser: &mut EventReader<R>,
        attrs: &Vec<OwnedAttribute>,
        ctx: &mut Context,
        map_path: P,
    ) -> Result<Tileset, TiledError> {
        let ((spacing, margin), (first_gid, name, width, height)) = get_attrs!(
           attrs,
           optionals: [("spacing", spacing, |v:String| v.parse().ok()),
                       ("margin", margin, |v:String| v.parse().ok())],
           required: [("firstgid", first_gid, |v:String| v.parse().ok()),
                      ("name", name, |v| Some(v)),
                      ("tilewidth", width, |v:String| v.parse().ok()),
                      ("tileheight", height, |v:String| v.parse().ok())],
           TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

        let mut images = Vec::new();
        let mut tiles = Vec::new();
        parse_tag!(parser, "tileset",
                   "image" => |attrs| {
                        images.push(try!(Image::<GgezImage>::new(parser, attrs, ctx, map_path.as_ref())));
                        Ok(())
                   },
                   "tile" => |attrs| {
                        tiles.push(try!(Tile::<GgezImage>::new(parser, attrs, ctx, map_path.as_ref())));
                        Ok(())
                   });

        Ok(Tileset::<GgezImage> {
            first_gid: first_gid,
            name: name,
            tile_width: width,
            tile_height: height,
            spacing: spacing.unwrap_or(0),
            margin: margin.unwrap_or(0),
            images: images,
            tiles: tiles,
        })
    }

    pub(crate) fn new_reference<P: AsRef<Path>>(
        attrs: &Vec<OwnedAttribute>,
        ctx: &mut Context,
        map_path: P,
    ) -> Result<Tileset, TiledError> {
        let ((), (first_gid, source)) = get_attrs!(
           attrs,
           optionals: [],
           required: [("firstgid", first_gid, |v:String| v.parse().ok()),
                      ("source", name, |v| Some(v))],
           TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

        let tileset_path = map_path.as_ref().with_file_name(source);
        let file = filesystem::open(ctx, &tileset_path).map_err(|err| {
            TiledError::GgezError(err)
        })?;
        Tileset::<GgezImage>::new_external(file, first_gid, ctx, tileset_path)
    }

    pub(crate) fn new_external<R: Read, P: AsRef<Path>>(file: R, first_gid: u32, ctx: &mut Context, path: P) -> Result<Tileset, TiledError> {
        let mut tileset_parser = EventReader::new(file);
        loop {
            match try!(tileset_parser.next().map_err(TiledError::XmlDecodingError)) {
                XmlEvent::StartElement {
                    name, attributes, ..
                } => {
                    if name.local_name == "tileset" {
                        return Tileset::<GgezImage>::parse_external_tileset(
                            first_gid,
                            &mut tileset_parser,
                            &attributes,
                            ctx,
                            path,
                        );
                    }
                }
                XmlEvent::EndDocument => {
                    return Err(TiledError::PrematureEnd(
                        "Tileset Document ended before map was parsed".to_string(),
                    ))
                }
                _ => {}
            }
        }
    }

    fn parse_external_tileset<R: Read, P: AsRef<Path>>(
        first_gid: u32,
        parser: &mut EventReader<R>,
        attrs: &Vec<OwnedAttribute>,
        ctx: &mut Context,
        path: P,
    ) -> Result<Tileset, TiledError> {
        let ((spacing, margin), (name, width, height)) = get_attrs!(
            attrs,
            optionals: [("spacing", spacing, |v:String| v.parse().ok()),
                        ("margin", margin, |v:String| v.parse().ok())],
            required: [("name", name, |v| Some(v)),
                       ("tilewidth", width, |v:String| v.parse().ok()),
                       ("tileheight", height, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("tileset must have a firstgid, name tile width and height with correct types".to_string()));

        let mut images = Vec::new();
        let mut tiles = Vec::new();
        parse_tag!(parser, "tileset",
                   "image" => |attrs| {
                       images.push(try!(Image::<GgezImage>::new(parser, attrs, ctx, path.as_ref())));
                       Ok(())
                   },
                   "tile" => |attrs| {
                       tiles.push(try!(Tile::<GgezImage>::new(parser, attrs, ctx, path.as_ref())));
                       Ok(())
                   });

        Ok(Tileset::<GgezImage> {
            first_gid: first_gid,
            name: name,
            tile_width: width,
            tile_height: height,
            spacing: spacing.unwrap_or(0),
            margin: margin.unwrap_or(0),
            images: images,
            tiles: tiles,
        })
    }
}
