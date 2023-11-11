fn hexToRgba(hex: &str) -> smart_leds::RGBW<u8> {
    let chars = hex.trim_start_matches("#").trim_start_matches("0x");

    let (r, gb) = chars.split_at(2);
    let (g, b_rest) = gb.split_at(2);
    let (b) = b_rest.split_at(2);

    let r_hex = u8::from_str_radix(r, 16);
    let g_hex = u8::from_str_radix(g, 16);
    let b_hex = u8::from_str_radix(b, 16);

    RGBW::new_alpha(
        r
        u8::from_str_radix(r, 16).expect("invalid r"),
        u8::from_str_radix(g, 16).expect("invalid g"),
        u8::from_str_radix(b.trim_end(), 16).expect("invalid b"),
        smart_leds::White(0),
    )
}

