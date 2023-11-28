use std::{thread,time};
use kira::{
	clock::{ClockTime, ClockSpeed},
	manager::{
		AudioManager, AudioManagerSettings,
		backend::DefaultBackend,
	},
	sound::static_sound::{StaticSoundData, StaticSoundSettings},
	StartTime,
    track::{
		TrackBuilder,
        TrackHandle,
		effect::filter::FilterBuilder,
	},
};

fn main() {
    // let mut sound_manager =
    //     AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();

    // let sound_data =
    //     StaticSoundData::from_file("src/player_hit.ogg", 
    //     StaticSoundSettings::default()).unwrap();

    // while true {
    //     let _ = sound_manager.play(sound_data.clone());
    //     thread::sleep(time::Duration::from_millis(1000));
    // }

    /*
    ----------------------------------------------------------
                    SECTION DIVIDE - KIRA CLOCK
    ----------------------------------------------------------
     */

    // let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    // let clock = manager.add_clock(ClockSpeed::SecondsPerTick(1.0)).unwrap();

    // for i in 0..5 {
    //     let data = StaticSoundData::from_file(
    //         "src/player_hit.ogg",
    //         StaticSoundSettings::new().start_time(StartTime::ClockTime(ClockTime {
    //             clock: clock.id(),
    //             ticks: i,
    //         })),
    //     ).unwrap();
    //     manager.play(data.clone()).unwrap();
    // }

    // clock.start().unwrap();
    // while true {};

    /*
    ----------------------------------------------------------
                    SECTION DIVIDE - FILTER
    ----------------------------------------------------------
     */

    let mut manager = AudioManager::<DefaultBackend>::new(AudioManagerSettings::default()).unwrap();
    let clock = manager.add_clock(ClockSpeed::SecondsPerTick(0.2)).unwrap();

    let mut tracks: Vec<TrackHandle> = Vec::new();
    for i in 0..50 {
        let track = manager.add_sub_track({
            let mut builder = TrackBuilder::new();
            builder.add_effect(FilterBuilder::new().cutoff(i as f64 * 100.0));
            builder
        }).unwrap();
        tracks.push(track);
    }

    for i in 0..50 {

        let data = StaticSoundData::from_file(
            "src/player_hit.ogg",
            StaticSoundSettings::new()
                .start_time(StartTime::ClockTime(ClockTime {
                clock: clock.id(),
                ticks: i,
            })).output_destination(&tracks[i as usize]),
        ).unwrap();
        manager.play(data.clone()).unwrap();
    }

    clock.start().unwrap();
    while true {};
}
