use windows::core::HSTRING;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MusicMetadata {
    pub title: String,
    pub artist: String,
    pub album: String,
    pub album_artist: String,
    pub track_number: u32,
    pub thumbnail: Option<String>,
}

impl MusicMetadata {
    pub fn h_title(&self) -> windows::core::HSTRING {
        HSTRING::from(self.title.clone())
    }

    pub fn h_artist(&self) -> windows::core::HSTRING {
        HSTRING::from(self.artist.clone())
    }

    pub fn h_album(&self) -> windows::core::HSTRING {
        HSTRING::from(self.album.clone())
    }

    pub fn h_album_artist(&self) -> windows::core::HSTRING {
        HSTRING::from(self.album_artist.clone())
    }

    pub fn h_thumbnail(&self) -> Option<windows::core::HSTRING> {
        self.thumbnail.clone().map(|s| HSTRING::from(s))
    }
}
