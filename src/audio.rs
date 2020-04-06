use amethyst::{
    assets::Loader,
    audio::{OggFormat, SourceHandle},
    ecs::{World, WorldExt},
};

const BOUNCE_SOUND: &str = "audio/bounce.ogg";
const SCORE_SOUND: &str = "audio/score.ogg";

pub struct Sounds {
    pub bounce_sfx: SourceHandle,
    pub score_sfx: SourceHandle,
}

pub fn initialize_audio(world: &mut World) {
    let sound_effects = {
        let loader = world.read_resource::<Loader>();
        Sounds {
            bounce_sfx: load_audio_track(&loader, world, BOUNCE_SOUND),
            score_sfx: load_audio_track(&loader, world, SCORE_SOUND),
        }
    };

    world.insert(sound_effects);
}

fn load_audio_track(loader: &Loader, world: &World, file: &str) -> SourceHandle {
    loader.load(
        file,
        OggFormat,
        (),
        &world.read_resource(),
    )
}
