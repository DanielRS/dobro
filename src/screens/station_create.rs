use super::super::Dobro;
use super::StationScreen;

use ui::*;
use state::*;

use pandora::music::ToMusicToken;

use ncurses as nc;

const RESULTS_LENGTH: usize = 3;

pub struct StationCreateScreen {
    search_string: String,
}

impl StationCreateScreen {
    pub fn new() -> Self {
        StationCreateScreen {
            search_string: "".to_owned(),
        }
    }
}

impl State for StationCreateScreen {
    fn start(&mut self, _ctx: &mut Dobro) {
        nc::attron(nc::A_BOLD());
        nc::printw("Create station from artist or song: ");
        nc::attroff(nc::A_BOLD());
        self.search_string = getstring();
        nc::printw("\n");
    }

    fn update(&mut self, ctx: &mut Dobro) -> Trans {
        use std::cmp::min;

        let mut no_results = true;
        if let Ok(results) = ctx.pandora().music().search(&self.search_string) {
            no_results = false;

            let artists_len = min(RESULTS_LENGTH, results.artists().len()) as i32;
            let songs_len = min(RESULTS_LENGTH, results.songs().len()) as i32;

            nc::printw("Artists:\n");
            for (i, artist) in results.artists().iter().enumerate().take(RESULTS_LENGTH) {
                nc::printw(&format!("{} - {}\n", i, artist.artist_name));
            }
            nc::printw("Songs:\n");
            for (i, song) in results.songs().iter().enumerate().take(RESULTS_LENGTH) {
                nc::printw(
                    &format!("{} - {} by {}\n", i as i32 + artists_len, song.song_name, song.artist_name)
                );
            }

            let mut music_token = None;
            loop {
                nc::attron(nc::A_BOLD());
                nc::printw("Music choice (negative to cancel): ");
                nc::attroff(nc::A_BOLD());
                let choice = getchoice();
                nc::printw("\n");

                if choice < 0 {
                    break;
                }
                else if choice < artists_len {
                    music_token = Some(results.artists()[choice as usize].to_music_token());
                    break;
                }
                else if choice < artists_len + songs_len {
                    music_token = Some(results.songs()[(choice - artists_len) as usize].to_music_token());
                    break;
                }
            }

            if let Ok(station) = ctx.pandora().stations().create(&music_token.unwrap_or("".to_owned())) {
                ctx.player_mut().play(station);
            }

            return Trans::Replace(Box::new(StationScreen::new()));
        }

        if no_results {
            nc::printw("No results!\n");
            nc::getch();
        }

        Trans::Pop
    }
}

