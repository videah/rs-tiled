use std::io::Read;
use std::collections::HashMap;
use std::path::Path;
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use ggez::graphics::Image as GgezImage;
use ggez::Context;

use TiledError;
use Frame;
use Image;
use Properties;
use ObjectGroup;
use {parse_animation, parse_properties};

#[derive(Debug, PartialEq, Clone)]
pub struct Tile<I = GgezImage> {
    pub id: u32,
    pub images: Vec<Image<I>>,
    pub properties: Properties,
    pub objectgroup: Option<ObjectGroup>,
    pub animation: Option<Vec<Frame>>,
}

impl Tile<()> {
    pub(crate) fn new<R: Read>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
    ) -> Result<Tile<()>, TiledError> {
        let (_, i) = get_attrs!(
            attrs,
            optionals: [],
            required: [("id", id, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("tile must have an id with the correct type".to_string()));

        let mut images = Vec::new();
        let mut properties = HashMap::new();
        let mut objectgroup = None;
        let mut animation = None;
        parse_tag!(parser, "tile",
                   "image" => |attrs| {
                       images.push(Image::<()>::new(parser, attrs)?);
                       Ok(())
                   },
                   "properties" => |_| {
                       properties = parse_properties(parser)?;
                       Ok(())
                   },
                   "objectgroup" => |attrs| {
                       objectgroup = Some(ObjectGroup::new(parser, attrs)?);
                       Ok(())
                   },
                   "animation" => |_| {
                       animation = Some(parse_animation(parser)?);
                       Ok(())
                   });
        Ok(Tile::<()> {
            id: i,
            images: images,
            properties: properties,
            objectgroup: objectgroup,
            animation: animation,
        })
    }
}

impl Tile<GgezImage> {
    pub(crate) fn new<R: Read, P: AsRef<Path>>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
        ctx: &mut Context,
        base_path: P,
    ) -> Result<Tile, TiledError> {
        let (_, i) = get_attrs!(
            attrs,
            optionals: [],
            required: [("id", id, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("tile must have an id with the correct type".to_string()));

        let mut images = Vec::new();
        let mut properties = HashMap::new();
        let mut objectgroup = None;
        let mut animation = None;
        parse_tag!(parser, "tile",
                   "image" => |attrs| {
                       images.push(Image::<GgezImage>::new(parser, attrs, ctx, base_path.as_ref())?);
                       Ok(())
                   },
                   "properties" => |_| {
                       properties = parse_properties(parser)?;
                       Ok(())
                   },
                   "objectgroup" => |attrs| {
                       objectgroup = Some(ObjectGroup::new(parser, attrs)?);
                       Ok(())
                   },
                   "animation" => |_| {
                       animation = Some(parse_animation(parser)?);
                       Ok(())
                   });
        Ok(Tile {
            id: i,
            images: images,
            properties: properties,
            objectgroup: objectgroup,
            animation: animation,
        })
    }
}
