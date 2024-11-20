#[cfg(test)]
mod test_text_hidden_watermark {
    use hidden_watermark::TextBlindWM;

    #[test]
    fn test_add_watermark() {
        let password = "p@ssw0rd".as_bytes();
        let watermark = "watermark: https://www.guofei.site".as_bytes();
        let text = "This is a text. It will be embedded with hidden watermark. 这是一段文本，之后这段文本将会被嵌入不可见盲水印";

        let text_blind_watermark = TextBlindWM::new(password);

        // embed
        let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);

        // extract
        let wm_extract = text_blind_watermark.extract(&text_with_wm);

        assert_eq!(watermark, wm_extract);

        //     add twice:
        let wm2 = "another watermark".as_bytes();
        let text_with_wm2 = text_blind_watermark.add_wm_rnd(&text_with_wm, wm2);

        assert_eq!(wm2, text_blind_watermark.extract(&text_with_wm2));
    }

    #[test]
    fn test_add_watermark2() {
        let pwd = "p@ssw0rd".as_bytes();
        let wm = "watermark: https://www.guofei.site".as_bytes();
        let text = "This is a text. It will be embedded with hidden watermark. 这是一段文本，之后这段文本将会被嵌入不可见盲水印";

        let text_blind_watermark = TextBlindWM::new(pwd);

        let text_with_wm = text_blind_watermark.add_wm_at_idx(text, wm, 5);
        assert_eq!(wm, text_blind_watermark.extract(&text_with_wm));

        let text_with_wm = text_blind_watermark.add_wm_at_last(text, wm);
        assert_eq!(wm, text_blind_watermark.extract(&text_with_wm));

        let text_with_wm = text_blind_watermark.add_wm_rnd(text, wm);
        assert_eq!(wm, text_blind_watermark.extract(&text_with_wm));
    }

    #[test]
    fn test_add_watermark_small_text() {
        // If there are two or more characters, do not embed at the beginning or the end.

        let password = "p@ssw0rd".as_bytes();
        let watermark = "watermark: https://www.guofei.site".as_bytes();
        let text = "你1";

        let text_blind_watermark = TextBlindWM::new(password);

        for _ in 0..10 {
            let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);

            assert_eq!(watermark, text_blind_watermark.extract(&text_with_wm));
            assert!(text_with_wm.starts_with("你") && text_with_wm.ends_with("1"));
        }

        let text = "ab";
        for _ in 0..10 {
            let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);
            assert_eq!(watermark, text_blind_watermark.extract(&text_with_wm));
            assert!(text_with_wm.starts_with("a") && text_with_wm.ends_with("b"));
        }

        let text = "2号";
        for _ in 0..10 {
            let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);
            assert_eq!(watermark, text_blind_watermark.extract(&text_with_wm));
            assert!(text_with_wm.starts_with("2") && text_with_wm.ends_with("号"));
        }

        let text = "1";
        let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);
        assert_eq!(watermark, text_blind_watermark.extract(&text_with_wm));

        let text = "😊";
        let text_with_wm = text_blind_watermark.add_wm_rnd(text, watermark);
        assert_eq!(watermark, text_blind_watermark.extract(&text_with_wm));
    }
}
