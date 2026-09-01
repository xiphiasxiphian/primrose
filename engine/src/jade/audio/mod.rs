use kira::{
    manager::{AudioManager, AudioManagerSettings, DefaultBackend, backend::Backend, error::PlaySoundError},
    sound::static_sound::{StaticSoundData, StaticSoundHandle, StaticSoundSettings},
};
use proc_macros::Resource;

pub type Sound = StaticSoundData;

#[derive(Resource)]
pub struct SoundHandler
{
    manager: AudioManager,
}

impl SoundHandler
{
    pub fn new() -> Result<Self, <DefaultBackend as Backend>::Error>
    {
        Ok(Self {
            manager: AudioManager::<DefaultBackend>::new(AudioManagerSettings::default())?,
        })
    }

    pub fn play(&mut self, sound: &Sound) -> Result<StaticSoundHandle, PlaySoundError<()>>
    {
        self.manager.play(sound.clone())
    }

    pub fn play_with(
        &mut self,
        sound: &Sound,
        settings: StaticSoundSettings,
    ) -> Result<StaticSoundHandle, PlaySoundError<()>>
    {
        self.manager.play(sound.clone().with_settings(settings))
    }
}
