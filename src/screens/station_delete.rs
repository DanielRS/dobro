use super::super::Dobro;

use state::*;

use ncurses as nc;

pub struct StationDeleteScreen {}

impl StationDeleteScreen {
    pub fn new() -> Self {
        StationDeleteScreen {}
    }
}

impl State for StationDeleteScreen {
    fn start(&mut self, ctx: &mut Dobro) {
        let station = ctx.player().state().station().clone();
        if let Some(station) = station {
            nc::printw(&format!("Deleting \"{}\"... ", station.station_name));
            nc::refresh();

            if let Ok(_) = ctx.pandora().stations().delete(&station) {
                nc::printw("Done\n");
                ctx.player_mut().stop();
            } else {
                nc::printw("Unable to delete\n");
            }
        }
    }

    fn update(&mut self, _ctx: &mut Dobro) -> Trans {
        Trans::Pop
    }
}
