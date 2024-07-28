#[cfg(test)]
use crate::core::validation::validated_types::YouTubeUrl;

#[test]
fn test_valid_youtube_url() {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ";
    let yt_url = YouTubeUrl::new(url).unwrap();
    assert_eq!(yt_url.value(), "https://youtu.be/dQw4w9WgXcQ");

    let url = "http://youtu.be/dQw4w9WgXcQ";
    let yt_url = YouTubeUrl::new(url).unwrap();
    assert_eq!(yt_url.value(), "https://youtu.be/dQw4w9WgXcQ");
}

#[test]
fn test_invalid_youtube_url() {
    let url = "https://www.youtube.com/watch?v=dQw4w9WgXcQ&feature=related";
    let yt_url = YouTubeUrl::new(url);
    assert!(yt_url.is_err());

    let url = "https://www.notyoutube.com/watch?v=dQw4w9WgXcQ";
    let yt_url = YouTubeUrl::new(url);
    assert!(yt_url.is_err());

    let url = "https://www.youtube.com/";
    let yt_url = YouTubeUrl::new(url);
    assert!(yt_url.is_err());

    let url = "https://youtu.be/";
    let yt_url = YouTubeUrl::new(url);
    assert!(yt_url.is_err());
}

#[test]
fn test_valid_youtube_url_no_protocol() {
    let url = "www.youtube.com/watch?v=dQw4w9WgXcQ";
    let yt_url = YouTubeUrl::new(url).unwrap();
    assert_eq!(yt_url.value(), "https://youtu.be/dQw4w9WgXcQ");

    let url = "youtu.be/dQw4w9WgXcQ";
    let yt_url = YouTubeUrl::new(url).unwrap();
    assert_eq!(yt_url.value(), "https://youtu.be/dQw4w9WgXcQ");
}
