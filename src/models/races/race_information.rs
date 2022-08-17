use super::race_track_name::RaceTrackName;

#[derive(Default, Clone, Copy)]
pub struct RaceInformation {
    pub race_number: u32,
    pub race_track_name: RaceTrackName,
}
