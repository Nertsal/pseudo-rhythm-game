use crate::config::Ticks;

#[derive(Debug, Clone)]
pub struct BeatControllerConfig {
    pub bpm_min: Ticks,
    pub bpm_max: Ticks,
    pub ticks_per_beat: Ticks,
    pub miss_time_early: f32,
    pub miss_time_late: f32,
}

#[derive(Debug, Clone)]
pub struct BeatController {
    /// Controller configuration.
    config: BeatControllerConfig,
    /// The time since last beat.
    last_beat: f32,
    /// The time since last player's beat event.
    last_player_beat: f32,
    /// BPM that was current at the moment of last player's beat event.
    last_player_bpm: f32,
    /// Current tick number.
    tick: Ticks,
    /// Current BPM.
    current_bpm: f32,
    /// Current tick time based on the current BPM.
    tick_t: f32,
    /// Normalized (in range 0..1) time until the next tick.
    next_tick: f32,
}

impl BeatController {
    pub fn new(config: BeatControllerConfig) -> Self {
        Self {
            last_beat: 0.0,
            last_player_beat: 0.0,
            last_player_bpm: 0.0,
            tick: 0,
            current_bpm: config.bpm_min as f32,
            tick_t: 1.0,
            next_tick: 1.0,
            config,
        }
    }

    /// Returns the current bpm.
    pub fn get_bpm(&self) -> f32 {
        self.current_bpm
    }

    /// Update the controller.
    /// Returns the number of ticks that happen in the `delta_time`.
    pub fn update(&mut self, delta_time: f32) -> Ticks {
        self.last_beat += delta_time;
        self.last_player_beat += delta_time;

        if self.last_player_beat > 60.0 / self.current_bpm + self.config.miss_time_late {
            // Late player beat -> slow down
            let target_bpm = 60.0 / self.last_player_beat;
            self.current_bpm = tween_bpm(self.last_player_bpm, target_bpm, &self.config);
        }

        // Music tick
        self.tick_t = 60.0 / (self.current_bpm * self.config.ticks_per_beat as f32);
        self.next_tick -= delta_time / self.tick_t;

        let mut ticks = 0;
        while self.next_tick < 0.0 {
            self.tick();
            ticks += 1;
            self.next_tick += 1.0;
        }
        ticks
    }

    /// Player inputs the beat event.
    /// Updates the BPM to match the timings between inputs.
    /// Returns the number of ticks that should be skipped.
    pub fn player_beat(&mut self) -> Ticks {
        // Early action
        let next_tick = self.next_tick * self.tick_t;
        let next_beat = next_tick
            + (self.config.ticks_per_beat - 1 - self.tick % self.config.ticks_per_beat) as f32
                * self.tick_t;

        if next_beat < self.config.miss_time_early {
            // Early action
            self.last_player_beat += next_beat;
            self.update_bpm();
            self.last_player_beat = -next_beat;
            return 0;
        }

        if self.last_beat < self.config.miss_time_late {
            // Late action
            self.last_player_beat -= self.last_beat;
            self.update_bpm();
            self.last_player_beat = self.last_beat;
            return 0;
        }

        self.update_bpm();
        self.skip_to_next_beat()
    }

    fn tick(&mut self) {
        self.tick += 1;

        if self.tick % self.config.ticks_per_beat == 0 {
            self.last_beat = 0.0;
        }
    }

    /// Updates the BPM based on `last_player_beat` time.
    fn update_bpm(&mut self) {
        if self.last_player_beat < 0.01 {
            return;
        }

        let target_bpm = 60.0 / self.last_player_beat;
        self.current_bpm = tween_bpm(self.last_player_bpm, target_bpm, &self.config);
        self.last_player_bpm = self.current_bpm;
        self.last_player_beat = 0.0;
    }

    /// Skips over ticks straight to the next beat.
    pub fn skip_to_next_beat(&mut self) -> Ticks {
        // TODO: manage skipped ticks
        self.next_tick = 0.0;
        let skip = self.config.ticks_per_beat - 1 - self.tick % self.config.ticks_per_beat;
        self.tick += skip;
        skip
    }
}

impl Default for BeatControllerConfig {
    fn default() -> Self {
        Self {
            bpm_min: 30,
            bpm_max: 240,
            ticks_per_beat: 4,
            miss_time_early: 0.1,
            miss_time_late: 0.2,
        }
    }
}

fn tween_bpm(current: f32, target: f32, config: &BeatControllerConfig) -> f32 {
    let alpha = 0.5;
    ((1.0 - alpha) * current + alpha * target).clamp(config.bpm_min as f32, config.bpm_max as f32)
}
