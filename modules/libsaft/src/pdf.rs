mod worlds;

use serde::Serialize;
use typst::eval::Tracer;
use typst::World;
use crate::pdf::worlds::{ResourceGeneratorTypstWorld, TemplateGeneratorTypstWorld};
use crate::err::SaftResult;

const FONTS: &[&[u8]] = &[
    include_bytes!("../compiled-assets/fonts/Roboto-Regular.ttf"),
    include_bytes!("../compiled-assets/fonts/Roboto-Bold.ttf"),
    include_bytes!("../compiled-assets/fonts/Roboto-Italic.ttf"),
    include_bytes!("../compiled-assets/fonts/Roboto-BoldItalic.ttf")
];

const LIBRARIES: &[(&str, &str)] = &[
    ("/tablex.typ", include_str!("../compiled-assets/typst-libraries/tablex.typ")),
    ("/letter-pro.typ", include_str!("../compiled-assets/typst-libraries/letter-pro.typ"))
];

const ASSETS: &[(&str, &[u8])] = &[
    ("/trafo-bw.svg", include_bytes!("../compiled-assets/img/trafo-bw.svg")),
];

pub fn gen_pdf_from_typst_source(source: String) -> SaftResult<Vec<u8>> {
    let world = ResourceGeneratorTypstWorld::new(source, FONTS, LIBRARIES, ASSETS);
    let doc = typst::compile(&world, &mut Tracer::new()).map_err(|e| (e, &world))?;
    Ok(typst_pdf::pdf(&doc, None, world.today(Some(0))))
}

pub fn gen_pdf_from_typst_template<T: Serialize>(template: String, data: &T) -> SaftResult<Vec<u8>> {
    let data = serde_json::to_string(data).expect("serialisation failed");
    let world = TemplateGeneratorTypstWorld::new(template, data.as_bytes(), FONTS, LIBRARIES, ASSETS);
    let doc = typst::compile(&world, &mut Tracer::new()).map_err(|e| (e, &world))?;
    Ok(typst_pdf::pdf(&doc, None, world.today(Some(0))))
}
