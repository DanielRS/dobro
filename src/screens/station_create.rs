use super::super::Dobro;

use ui::*;
use state::*;

use pandora::music::ToMusicToken;

use ncurses as nc;

const RESULTS_LENGTH: usize = 3;

pub struct StationCreateScreen {}

impl StationCreateScreen {
    pub fn new() -> Self {
        StationCreateScreen {}
    }
}

impl StationMusicScreen for StationCreateScreen {
    fn message(&self) -> &'static str {
        "Create station from artist or song: "
    }

    fn on_choice<T>(&mut self, ctx: &mut Dobro, music_token: &T)
        where T: ToMusicToken
    {
        nc::printw("Creating station... ");
        nc::refresh();
        if let Ok(station) = ctx.pandora().stations().create(music_token) {
            nc::printw("Done\n");
            ctx.player_mut().play(station);
        } else {
            nc::printw("Unable to create station\n");
        }
    }
}

impl State for StationCreateScreen {
    fn start(&mut self, ctx: &mut Dobro) {
        StationMusicScreen::start(self, ctx);
    }

    fn update(&mut self, _ctx: &mut Dobro) -> Trans {
        Trans::Pop
    }
}

pub trait StationMusicScreen {
    fn message(&self) -> &'static str;

    fn on_choice<T>(&mut self, ctx: &mut Dobro, music_token: &T) where T: ToMusicToken;

    fn start(&mut self, ctx: &mut Dobro) {
        use std::cmp::min;

        nc::attron(nc::A_BOLD());
        nc::printw(self.message());
        nc::attroff(nc::A_BOLD());
        let search_string = getstring();
        nc::printw("\n");

        nc::printw("Searching... ");
        nc::refresh();
        if let Ok(results) = ctx.pandora().music().search(&search_string) {

            let artists_len = min(RESULTS_LENGTH, results.artists().len()) as i32;
            let songs_len = min(RESULTS_LENGTH, results.songs().len()) as i32;

            if artists_len > 0 || songs_len > 0 {
                nc::printw("Done\n");

                nc::printw("Artists:\n");
                for (i, artist) in results.artists().iter().enumerate().take(RESULTS_LENGTH) {
                    nc::printw(&format!("{} - {}\n", i, artist.artist_name));
                }
                nc::printw("Songs:\n");
                for (i, song) in results.songs().iter().enumerate().take(RESULTS_LENGTH) {
                    nc::printw(&format!("{} - {} by {}\n",
                                       i as i32 + artists_len,
                                       song.song_name,
                                       song.artist_name));
                }

                let mut music_token = None;
                loop {
                    nc::attron(nc::A_BOLD());
                    nc::printw("Music choice (blank to cancel): ");
                    nc::attroff(nc::A_BOLD());
                    let choice = getchoice();
                    nc::printw("\n");

                    if choice < 0 {
                        break;
                    } else if choice < artists_len {
                        music_token = Some(results.artists()[choice as usize].to_music_token());
                        break;
                    } else if choice < artists_len + songs_len {
                        music_token = Some(results.songs()[(choice - artists_len) as usize]
                                               .to_music_token());
                        break;
                    }
                }

                if let Some(ref music_token) = music_token {
                    self.on_choice(ctx, music_token);
                }
            } else {
                nc::printw("No results\n");
            }
        } else {
            nc::printw("Error\n");
        }
    }
}
