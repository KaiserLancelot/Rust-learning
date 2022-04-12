use allsorts::binary::read::ReadScope;
use allsorts::font_data::FontData;

pub fn main() {
    let data = include_bytes!("../data/SourceHanSansSC-Bold.otf");
    let _font_file = ReadScope::new(data)
        .read::<FontData>()
        .expect("Can not read font");
}
