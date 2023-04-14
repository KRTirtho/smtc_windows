use windows::core::HSTRING;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MusicMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub album_artist: Option<String>,
    pub thumbnail: Option<String>,
}

impl MusicMetadata {
    pub fn h_title(&self) -> Option<windows::core::HSTRING> {
        self.title.clone().map(|s| HSTRING::from(s))
    }

    pub fn h_artist(&self) -> Option<windows::core::HSTRING> {
        self.artist.clone().map(|s| HSTRING::from(s))
    }

    pub fn h_album(&self) -> Option<windows::core::HSTRING> {
        self.album.clone().map(|s| HSTRING::from(s))
    }

    pub fn h_album_artist(&self) -> Option<windows::core::HSTRING> {
        self.album_artist.clone().map(|s| HSTRING::from(s))
    }

    pub fn h_thumbnail(&self) -> Option<windows::core::HSTRING> {
        self.thumbnail.clone().map(|s| HSTRING::from(s))
    }
}
