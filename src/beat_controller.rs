pub struct BeatControllerConfig {
    pub bpm_min: u32,
    pub bpm_max: u32,
    pub ticks_per_beat: u32,
    pub miss_time_early: f32,
    pub miss_time_late: f32,
}

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
    tick: u32,
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

    /// Update the controller.
    /// Returns the number of ticks that happen in the `delta_time`.
    pub fn update(&mut self, delta_time: f32) -> u32 {
        self.last_beat += delta_time;
        self.last_player_beat += delta_time;

        if self.last_player_beat > 60.0 / self.current_bpm + self.config.miss_time_late {
            // Late player beat -> slow down
            let target_bpm = 60.0 / self.last_player_beat;
            let alpha = 0.5;
            self.current_bpm = ((1.0 - alpha) * self.last_player_bpm + alpha * target_bpm)
                .clamp(self.config.bpm_min as f32, self.config.bpm_max as f32);
        }

        // Music tick
        self.tick_t = 60.0 / (self.current_bpm * self.config.ticks_per_beat as f32);
        self.next_tick -= delta_time / self.tick_t;

        let mut ticks = 0;
        while self.next_tick < 0.0 {
            ticks += 1;
            self.next_tick += 1.0;
        }
        ticks
    }

    /// Player inputs the beat event.
    /// Updates the BPM to match the timings between inputs.
    pub fn player_beat(&mut self) {
        // Early action
        let next_tick = self.next_tick * self.tick_t;
        let next_beat = next_tick
            + (self.config.ticks_per_beat - 1 - self.tick % self.config.ticks_per_beat) as f32
                * self.tick_t;
        if next_beat < self.config.miss_time_early {
            self.last_player_beat += next_beat;
            self.update_bpm();
            self.last_player_beat = -next_beat;
            return;
        }

        // Late action
        if self.last_beat < self.config.miss_time_late {
            self.last_player_beat -= self.last_beat;
            self.update_bpm();
            self.last_player_beat = self.last_beat;
            return;
        }

        self.update_bpm();
        self.skip_to_next_beat();
    }

    /// Updates the BPM based on `last_player_beat` time.
    fn update_bpm(&mut self) {
        if self.last_player_beat < 0.01 {
            return;
        }

        let target_bpm = 60.0 / self.last_player_beat;
        let alpha = 0.5;
        self.current_bpm = ((1.0 - alpha) * self.last_player_bpm + alpha * target_bpm)
            .clamp(self.config.bpm_min as f32, self.config.bpm_max as f32);
        self.last_player_bpm = self.current_bpm;
        self.last_player_beat = 0.0;
    }

    /// Skips over ticks straight to the next beat.
    pub fn skip_to_next_beat(&mut self) {
        // TODO: manage skipped ticks
        self.next_tick = 0.0;
        let skip = self.config.ticks_per_beat - 1 - self.tick % self.config.ticks_per_beat;
        self.tick += skip;
    }
}

impl Default for BeatControllerConfig {
    fn default() -> Self {
        Self {
            bpm_min: 50,
            bpm_max: 240,
            ticks_per_beat: 4,
            miss_time_early: 0.1,
            miss_time_late: 0.2,
        }
    }
}
