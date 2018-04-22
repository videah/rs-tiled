extern crate tiled;
extern crate ggez;

use tiled::parse;
use ggez::filesystem::Filesystem;

fn main() {
    let mut fs = Filesystem::new("rs-tiled", "Difarem").unwrap();

    let map = parse(&mut fs, "/tiled_base64_zlib.tmx").unwrap();
    println!("{:?}", map);
    println!("{:?}", map.get_tileset_by_gid(22));
}
