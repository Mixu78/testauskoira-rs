use std::io::Cursor;

pub async fn build_award_image(user_img_url: &str) -> Result<String, ()> {
    let img_url_base = &user_img_url[..user_img_url.rfind('.').unwrap()];
    let profile_picture = reqwest::get(format!("{}.png?size=128", img_url_base))
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let pfp = image::io::Reader::new(Cursor::new(profile_picture)).
        with_guessed_format().unwrap()
        .decode().unwrap();
    let mask = image::io::Reader::open("img/blackcomposite.png").unwrap()
        .decode().unwrap();

    let mut pfp = pfp.to_rgba16();
    let mask = mask.to_rgba16();

    for (x, y, pixel) in pfp.enumerate_pixels_mut() {
        if mask.get_pixel(x, y)[3] == 0 {
            *pixel = *mask.get_pixel(x, y);
        }
    }

    image::imageops::overlay(&mut pfp, &mask, 0, 0);
    pfp.save("pfp_new.png").unwrap();

    Ok("pfp_new.png".to_string())
}
