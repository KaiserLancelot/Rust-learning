use allsorts::binary::read::ReadScope;
use allsorts::font_data::FontData;

pub fn main() {
    let buffer = std::fs::read("SourceHanSansSC-Bold.otf").expect("Can not read file");
    let _font_file = ReadScope::new(&buffer)
        .read::<FontData>()
        .expect("Can not read font");
}
