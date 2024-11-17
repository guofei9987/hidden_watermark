# hidden_watermark



- Documentation: [https://docs.rs/hidden_watermark](https://docs.rs/hidden_watermark)
- Source code: [https://github.com/guofei9987/hidden_watermark](https://github.com/guofei9987/hidden_watermark)

[![Crates.io](https://img.shields.io/crates/v/hidden_watermark)](https://crates.io/crates/hidden_watermark)
[![Build Status](https://github.com/guofei9987/hidden_watermark/actions/workflows/rust.yml/badge.svg)](https://github.com/guofei9987/hidden_watermark/actions)
[![Docs.rs](https://docs.rs/hidden_watermark/badge.svg)](https://docs.rs/hidden_watermark)
[![License](https://img.shields.io/crates/l/hidden_watermark)](https://github.com/guofei9987/hidden_watermark/blob/master/LICENSE)
[![GitHub stars](https://img.shields.io/github/stars/guofei9987/hidden_watermark.svg?style=social&label=Star)](https://github.com/guofei9987/hidden_watermark)
[![Forks](https://img.shields.io/github/forks/guofei9987/hidden_watermark.svg?style=social&label=Fork)](https://github.com/guofei9987/hidden_watermark/fork)
![Rust](https://img.shields.io/badge/Rust-1.60+-orange.svg)
[![Crates.io Downloads](https://img.shields.io/crates/d/hidden_watermark)](https://crates.io/crates/hidden_watermark)
[![GitHub Discussions](https://img.shields.io/github/discussions/guofei9987/hidden_watermark)](https://github.com/guofei9987/hidden_watermark/discussions)





Hidden Watermark in Rust
- [x] Hidden Watermark in **Text**
- [ ] Hidden Watermark in **Image** (png/jpg/...)

Put message(blind watermark) into a text. so that the message is invisible, and the changes of the text are not perceptible.

[![stars](https://img.shields.io/github/stars/guofei9987/hidden_watermark_rs.svg?style=social)](https://github.com/guofei9987/hidden_watermark_rs/)
[![fork](https://img.shields.io/github/forks/guofei9987/hidden_watermark_rs?style=social)](https://github.com/guofei9987/hidden_watermark_rs/fork)


- Video demo：[https://www.bilibili.com/video/BV1m3411s7kT](https://www.bilibili.com/video/BV1m3411s7kT)
- Online demo(from old version, for demo only): [https://www.guofei.site/pictures_for_blog/app/text_watermark/v1.html](https://www.guofei.site/pictures_for_blog/app/text_watermark/v1.html)
- Python version: [https://github.com/guofei9987/text_blind_watermark](https://github.com/guofei9987/text_blind_watermark)
- **Source code:** [https://github.com/guofei9987/hidden_watermark_rs](https://github.com/guofei9987/hidden_watermark_rs)
- **crates.io**: [https://crates.io/crates/hidden_watermark_rs](https://crates.io/crates/hidden_watermark_rs)


## How to Use

Cargo.toml
```
[dependencies]
hidden_watermark = "*"
```

### Text Hidden Watermark

Can be used in
- [x] Wechat
- [x] dingding
- [x] zhihu.com
- [x] ...


```rust
use hidden_watermark::TextBlindWM;
use std::fs;
#[test]
fn example_text_hidden_watermark() {
    let pwd = "This is password".as_bytes();
    let wm = "This is a hidden message".as_bytes();
    let ori_filename = "./files/file.txt";
    let file_with_wm = "./files/outputs/file_with_wm.txt";


    let text_blind_watermark = TextBlindWM::new(pwd);

    let text = fs::read_to_string(ori_filename).unwrap();

    // embed
    let text_with_wm = text_blind_watermark.add_wm_rnd(text.as_str(), wm);
    // write into file
    fs::write(file_with_wm, text_with_wm).unwrap();
    println!("text with watermark saved in file <{}>", file_with_wm);

    // read text and extract the watermark
    let text_with_wm = fs::read_to_string(file_with_wm).unwrap();

    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());

    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}
```


**It does not display well in IDE. Use other text editor or text viewer.**

## Related Project




HideInfo：[https://github.com/guofei9987/HideInfo](https://github.com/guofei9987/HideInfo)


| 算法   | 说明                |
|------|-------------------|
| [migrate tank](https://github.com/guofei9987/HideInfo/blob/main/example/example_mirage_tank.py) | 使图片在不同的背景下显示不同的图片 |
| [hide as image](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_as_img.py) | 把数据以图片形式存放        |
| [hide in image](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_in_img.py) | 把数据藏在一个图片中          |
| [image seed](https://github.com/guofei9987/HideInfo/blob/main/example/example_img_seed.py)   | 把图片和文件黏在一起，并存为图片  |
| [EXIF](https://github.com/guofei9987/HideInfo/blob/main/example/example_img_exif.py) | 把一段信息放到图片的EXIF中   |
| [hide as music](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_as_music.py) | 把数据以音频的形式存放       |
| [hide in music](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_in_music.py) | 把数据隐藏在一个音频中       |
| [hide as text](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_as_txt.py) | 把数据以文本文件的形式存放 |
| [hide in text](https://github.com/guofei9987/HideInfo/blob/main/example/example_hide_in_txt.py) | 把数据隐藏在一段文本中 |


Python version: [https://github.com/guofei9987/text_blind_watermark](https://github.com/guofei9987/text_blind_watermark)
