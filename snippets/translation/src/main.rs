use mimalloc::MiMalloc;
use simplecc::dicts::S2TW;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

fn main() {
    println!(
        "{}",
        S2TW.replace_all("查看下一步即将推出的内容！立即下载任何Microsoft Edge预览频道。")
    );
}
