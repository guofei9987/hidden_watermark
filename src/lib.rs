/*!

# hidden_watermark

## Hidden Watermark in Text

### Add watermark & Extract watermark

```rust
use hidden_watermark::TextBlindWM;
use std::fs;
let password = "p@ssw0rd".as_bytes();
let wm = "This is a hidden message".as_bytes();
let ori_file = "./files/file.txt";
let file_with_wm = "./files/outputs/file_with_wm.txt";

let text_blind_watermark = TextBlindWM::new(password);

let text = fs::read_to_string(ori_file).unwrap();

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
```


*/

pub mod text_wm;

pub use text_wm::TextBlindWM;
