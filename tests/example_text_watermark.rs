#[cfg(test)]
mod example_text_hidden_watermark {
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

        println!(
            "watermark extracted：{}",
            String::from_utf8_lossy(wm_extract.as_slice())
        )
    }
}
