# hidden-watermark-rs
Text Blind Watermark in Rust

Put message(blind watermark) into a text. so that the message is invisible, and the changes of the text are not perceptible.

[![stars](https://img.shields.io/github/stars/guofei9987/hidden-watermark-rs.svg?style=social)](https://github.com/guofei9987/hidden-watermark-rs/)
[![fork](https://img.shields.io/github/forks/guofei9987/hidden-watermark-rs?style=social)](https://github.com/guofei9987/hidden-watermark-rs/fork)


- Video demo：[https://www.bilibili.com/video/BV1m3411s7kT](https://www.bilibili.com/video/BV1m3411s7kT)
- Online demo(from old version, for demo only): [https://www.guofei.site/pictures_for_blog/app/text_watermark/v1.html](https://www.guofei.site/pictures_for_blog/app/text_watermark/v1.html)
- Python version: [https://github.com/guofei9987/text_blind_watermark](https://github.com/guofei9987/text_blind_watermark)
- **Source code:** [https://github.com/guofei9987/hidden-watermark-rs](https://github.com/guofei9987/hidden-watermark-rs)
- **crates.io**: [https://crates.io/crates/hidden-watermark-rs](https://crates.io/crates/hidden-watermark-rs)



Can be used in
- [x] Wechat
- [x] dingding
- [x] zhihu.com
- [x] ...

## How to Use

Cargo.toml
```
[dependencies]
hidden-watermark-rs = "*"
```


### embed&extract:

```Rust
use text_blind_watermark::TextBlindWM;

fn test2() {
    let pwd = "这是一段密码. This is password";
    let wm = "不可见的暗水印. This is watermark";
    let text = "这是一段文本，之后这段文本将会被嵌入不可见盲水印";

    let text_blind_watermark = TextBlindWM::new(pwd);

    // embed
    let text_with_wm = text_blind_watermark.embed(text, wm);
    println!("text with watermark：{}", text_with_wm);

    // extract
    let wm_extract = text_blind_watermark.extract(text_with_wm.as_str());

    println!("watermark extracted：{}", String::from_utf8_lossy(wm_extract.as_slice()))
}
```


### embed&extract .txt file:

```Rust
fn test3() {
    let pwd = "这是一段密码. This is password";
    let wm = "不可见的暗水印. This is watermark";
    let ori_filename = "file.txt";
    let file_with_wm = "file_with_wm.txt";


    let text_blind_watermark = TextBlindWM::new(pwd);

    let text = std::fs::read_to_string(ori_filename).unwrap();

    // embed
    let text_with_wm = text_blind_watermark.embed(text.as_str(), wm);
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
