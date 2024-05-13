use std::collections::HashMap;
use chrono::{Datelike, DateTime, Local};
use comemo::Prehashed;
use typst::foundations::{Bytes, Datetime};
use typst::{Library, World};
use typst::diag::{FileError, FileResult};
use typst::syntax::{FileId, Source, VirtualPath};
use typst::text::{Font, FontBook};

pub struct SimpleGeneratorTypstWorld {
    library: Prehashed<Library>,
    font_book: Prehashed<FontBook>,
    fonts: Vec<Font>,
    source: Source,
    date: DateTime<Local>,
}

impl SimpleGeneratorTypstWorld {
    pub fn new(source: String, fonts: &[&[u8]]) -> Self {
        let fonts: Vec<Font> = fonts.iter().map(|f| {
            Font::new(Bytes::from(*f), 0).unwrap()
        }).collect();

        Self {
            library: Prehashed::new(Library::build()),
            font_book: Prehashed::new(FontBook::from_fonts(fonts.iter())),
            fonts,
            source: Source::new(FileId::new(None, VirtualPath::new("/main.typ")), source),
            date: Local::now(),
        }
    }
}

impl World for SimpleGeneratorTypstWorld {
    fn library(&self) -> &Prehashed<Library> {
        &self.library
    }

    fn book(&self) -> &Prehashed<FontBook> {
        &self.font_book
    }

    fn main(&self) -> Source {
        self.source.clone()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        if id == self.source.id() {
            Ok(self.source.clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rooted_path().to_path_buf()))
        }
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        Err(FileError::NotFound(id.vpath().as_rooted_path().to_path_buf()))
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.fonts.get(index).cloned()
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        let date = if let Some(offset) = offset {
            self.date.naive_utc() + chrono::Duration::try_hours(offset)?
        } else {
            self.date.naive_local()
        };

        Datetime::from_ymd(date.year(), date.month().try_into().ok()?, date.day().try_into().ok()?)
    }
}

pub struct ResourceGeneratorTypstWorld {
    simple_world: SimpleGeneratorTypstWorld,
    ext_libraries: HashMap<FileId, Source>,
    assets: HashMap<FileId, Bytes>
}

impl ResourceGeneratorTypstWorld {
    pub fn new(source: String, fonts: &[&[u8]], ext_libraries: &[(&str, &str)], assets: &[(&str, &[u8])]) -> Self {
        let ext_libraries: HashMap<FileId, Source> = ext_libraries.iter().map(|(path, data)| {
            let src = Source::new(FileId::new(None, VirtualPath::new(path)), String::from(*data));
            (src.id(), src)
        }).collect();

        let assets: HashMap<FileId, Bytes> = assets.iter().map(|(path, data)| {
            (FileId::new(None, VirtualPath::new(path)), Bytes::from(*data))
        }).collect();

        Self {
            simple_world: SimpleGeneratorTypstWorld::new(source, fonts),
            ext_libraries,
            assets
        }
    }
}

impl World for ResourceGeneratorTypstWorld {
    fn library(&self) -> &Prehashed<Library> {
        self.simple_world.library()
    }

    fn book(&self) -> &Prehashed<FontBook> {
        self.simple_world.book()
    }

    fn main(&self) -> Source {
        self.simple_world.main()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.simple_world.source(id).or(
            if self.ext_libraries.contains_key(&id) {
                Ok(self.ext_libraries.get(&id).unwrap().clone())
            } else {
                Err(FileError::NotFound(id.vpath().as_rooted_path().to_path_buf()))
            }
        )
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if self.assets.contains_key(&id) {
            Ok(self.assets.get(&id).unwrap().clone())
        } else {
            Err(FileError::NotFound(id.vpath().as_rooted_path().to_path_buf()))
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.simple_world.font(index)
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        self.simple_world.today(offset)
    }
}

pub struct TemplateGeneratorTypstWorld {
    world: ResourceGeneratorTypstWorld,
    data: Bytes
}

impl TemplateGeneratorTypstWorld {
    pub fn new(source: String, data: &[u8], fonts: &[&[u8]], ext_libraries: &[(&str, &str)], assets: &[(&str, &[u8])]) -> Self {
        Self {
            world: ResourceGeneratorTypstWorld::new(source, fonts, ext_libraries, assets),
            data: Bytes::from(data)
        }
    }
}

impl World for TemplateGeneratorTypstWorld {
    fn library(&self) -> &Prehashed<Library> {
        self.world.library()
    }

    fn book(&self) -> &Prehashed<FontBook> {
        self.world.book()
    }

    fn main(&self) -> Source {
        self.world.main()
    }

    fn source(&self, id: FileId) -> FileResult<Source> {
        self.world.source(id)
    }

    fn file(&self, id: FileId) -> FileResult<Bytes> {
        if id == FileId::new(None, VirtualPath::new("/data.json")) {
            Ok(self.data.clone())
        } else {
            self.world.file(id)
        }
    }

    fn font(&self, index: usize) -> Option<Font> {
        self.world.font(index)
    }

    fn today(&self, offset: Option<i64>) -> Option<Datetime> {
        self.world.today(offset)
    }
}
