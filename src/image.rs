use std::io::Read;
use std::path::{Path, PathBuf};
use xml::reader::{EventReader, XmlEvent};
use xml::attribute::OwnedAttribute;
use ggez::graphics::Image as GgezImage;
use ggez::graphics::Rect;
use ggez::Context;

use TiledError;
use Colour;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Image<I = GgezImage> {
    /// The filepath of the image
    pub source: PathBuf,
    pub width: i32,
    pub height: i32,
    pub transparent_colour: Option<Colour>,
    pub image: I,
}

impl Image<()> {
    pub(crate) fn new<R: Read>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
    ) -> Result<Image<()>, TiledError> {
        let (c, (s, w, h)) = get_attrs!(
            attrs,
            optionals: [("trans", trans, |v:String| v.parse().ok())],
            required: [("source", source, |v| Some(v)),
                       ("width", width, |v:String| v.parse().ok()),
                       ("height", height, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("image must have a source, width and height with correct types".to_string()));

        parse_tag!(parser, "image", "" => |_| Ok(()));
        Ok(Image::<()> {
            source: PathBuf::from(s),
            width: w,
            height: h,
            transparent_colour: c,
            image: (),
        })
    }
}

impl Image<GgezImage> {
    pub(crate) fn new<R: Read, P: AsRef<Path>>(
        parser: &mut EventReader<R>,
        attrs: Vec<OwnedAttribute>,
        ctx: &mut Context,
        base_path: P,
    ) -> Result<Image, TiledError> {
        let (c, (s, w, h)) = get_attrs!(
            attrs,
            optionals: [("trans", trans, |v:String| v.parse().ok())],
            required: [("source", source, |v| Some(v)),
                       ("width", width, |v:String| v.parse().ok()),
                       ("height", height, |v:String| v.parse().ok())],
            TiledError::MalformedAttributes("image must have a source, width and height with correct types".to_string()));

        let image_path = base_path.as_ref().with_file_name(&s);
        let image = GgezImage::new(ctx, image_path).map_err(|e| TiledError::GgezError(e))?;

        parse_tag!(parser, "image", "" => |_| Ok(()));
        Ok(Image::<GgezImage> {
            source: PathBuf::from(s),
            width: w,
            height: h,
            transparent_colour: c,
            image,
        })
    }
}

use ggez::graphics::{Drawable, DrawParam, BlendMode};
use ggez::GameResult;

impl Drawable for Image<GgezImage> {
    fn draw(&self, ctx: &mut Context, param: DrawParam) -> GameResult<()> {
        self.image.draw(ctx, param)
    }

    fn set_blend_mode(&mut self, mode: Option<BlendMode>) {
        self.image.set_blend_mode(mode)
    }

    fn blend_mode(&self) -> Option<BlendMode> {
        self.image.blend_mode()
    }
    
    fn dimensions(&self, ctx: &mut Context) -> Option<Rect> {
        self.image.dimensions()
    }
}
