use std::process::Command;

use clap::Parser;
use image::imageops::FilterType;

const FILTER: FilterType = FilterType::Lanczos3;

#[derive(Parser)]
struct Cli {
    path: String,
}

fn main() {
    let args = Cli::parse();

    let img = image::open(args.path).unwrap();
    let mut size: u32 = 16;

    let folder = "icons/".to_owned() + "app-icon" + ".iconset";

    Command::new("mkdir")
        .arg("icons")
        .arg(folder.clone())
        .output()
        .expect("app-icon.iconsetの作成に失敗");

    for _ in 0..7 {
        let name = format!("icon_{size}x{size}");
        let path = format!("{folder}/{name}.png");

        img.resize(size, size, FILTER)
            .save(path)
            .expect("リサイズした画像の保存に失敗");

        if size == 1024 {
            img.save("icons/icon.ico").expect("icoファイルの作成に失敗");
        }

        size *= 2;
    }

    Command::new("iconutil")
        .args(["-c", "icns", &folder])
        .output()
        .expect("icnsファイルの作成に失敗");
}
