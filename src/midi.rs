// Copyright © 2021 The Dimi Crate Developers
//
// Licensed under any of:
// - Apache License, Version 2.0 (https://www.apache.org/licenses/LICENSE-2.0)
// - MIT License (https://mit-license.org/)
// - Boost Software License, Version 1.0 (https://www.boost.org/LICENSE_1_0.txt)
// At your option (See accompanying files LICENSE_APACHE_2_0.txt,
// LICENSE_MIT.txt and LICENSE_BOOST_1_0.txt).  This file may not be copied,
// modified, or distributed except according to those terms.
//
//! Type-safe MIDI event types.

use core::num::NonZeroI8;
use core::convert::TryFrom;

/// A note and octave of the Western scale.
///
///  In this library, C4 is middle C and octaves range from -1 to 9
#[derive(Debug, Copy, Clone)]
pub enum Note {
    C(i8),
    Db(i8),
    D(i8),
    Eb(i8),
    E(i8),
    F(i8),
    Gb(i8),
    G(i8),
    Ab(i8),
    A(i8),
    Bb(i8),
    B(i8),
}

impl TryFrom<u8> for Note {
    type Error = std::num::TryFromIntError;

    fn try_from(note: u8) -> std::result::Result<Self, Self::Error> {
        let note = i8::try_from(note)?;
        let octave = note / 12;
        let note = match note % 12 {
            0 => Note::C,
            1 => Note::Db,
            2 => Note::D,
            3 => Note::Eb,
            4 => Note::E,
            5 => Note::F,
            6 => Note::Gb,
            7 => Note::G,
            8 => Note::Ab,
            9 => Note::A,
            10 => Note::Bb,
            11 => Note::B,
            _ => unreachable!(),
        };
        Ok(note(octave - 1))
    }
}

/// A control change message for continuous controllers
#[derive(Debug, Clone, Copy)]
pub enum Control {
    /// Bank select/change 0-127
    Bank(i8),
    /// Modulation by Wheel
    Wheel(i8),
    /// Modulation by Breath
    Breath(i8),
    /// Modulation by Pedal
    Pedal(i8),
    /// Control rate of portamento slide between two notes.
    PortamentoTime(i8),
    /// Control value for NRPN, RPN parameters.  FIXME: What is NRPN, RPN
    DataEntry(i8),
    /// Control volume
    Volume(i8),
    /// Control stereo patch left-right balance (64 is center)
    Balance(i8),
    /// Control mono patch left-right balance (64 is center)
    Pan(i8),
    /// Control partial volume adjustment
    Expression(i8),
    /// Synth/Workstation effect parameter controller A
    EffectA(i8),
    /// Synth/Workstation effect parameter controller B
    EffectB(i8),
    /// General-Purpose A
    UserA(i8),
    /// General-Purpose B
    UserB(i8),
    /// General-Purpose C
    UserC(i8),
    /// General-Purpose D
    UserD(i8),
    /// LSB FIXME: What is LSB
    Lsb {
        /// Which 0-32 FIXME of what
        which: i8,
        /// The value associated with the LSB
        value: i8,
    },
    /// Damper Pedal (Sustain all notes On/Off Switch)
    Damper(bool),
    /// Bend Pedal (Portamento On/Off Switch)
    Bend(bool),
    /// Sostenuto Pedal (Sustain only notes when first pressed)
    Sostenuto(bool),
    /// Soft Pedal
    Soft(bool),
    /// Legato Pedal,
    Legato(bool),
    /// Hold Pedal (Sustain notes but fade out based on release parameter,
    /// instead of when the pedal is released)
    Hold(bool),
    /// Change the way the sound is produced
    Variation(i8),
    /// Shape the Voltage-Controlled-Filter (VCF), change timbre, harmonics
    Resonance(i8),
    /// Shape the Voltage-Controlled-Amplifier (VCA), change release time
    ReleaseTime(i8),
    /// Shape the Voltage-Controlled-Amplifier (VCA), change attack time
    AttackTime(i8),
    /// Shape the Voltage-Controlled-Filter (VCF), change filter cutoff
    /// frequency
    CutoffFrequency(i8),
    /// Custom Sound Shaping A
    ShaperA(i8),
    /// Custom Sound Shaping B
    ShaperB(i8),
    /// Custom Sound Shaping C
    ShaperC(i8),
    /// Custom Sound Shaping D
    ShaperD(i8),
    /// Custom Sound Shaping E
    ShaperE(i8),
    /// Decay On/Off Switch
    Decay(bool),
    /// Hi-Pass Filter On/Off Switch
    HiPassFilter(bool),
    /// Generic On/Off Switch A
    SwitchA(bool),
    /// Generic On/Off Switch B
    SwitchB(bool),
    /// Control the amount of portamento
    Portamento(i8),
    /// High-Resolution Velocity Prefix
    Velocity(i8),
    /// Change Reverb Send Amount
    Reverb(i8),
    /// Change Tremelo Amount
    Tremelo(i8),
    /// Change Chorus Amount
    Chorus(i8),
    /// Change Detune Amount
    Detune(i8),
    /// Change Phaser Amount
    Phaser(i8),
    /// Increment data for RPN & NRPN messages
    DataIncrement,
    /// Decrement data for RPN & NRPN messages
    DataDecrement,
    /// Non-Registered Parameter Number LSB, FIXME what is LSB
    NrpnLsbSelect(i8),
    /// Non-Registered Parameter Number MSB, FIXME what is MSB
    NrpnMsbSelect(i8),
    /// Registered Parameter Number LSB, FIXME what is LSB
    RpnLsbSelect(i8),
    /// Registered Parameter Number MSB, FIXME what is MSB
    RpnMsbSelect(i8),
    /// Mute all audio immediately
    Mute,
    /// Reset all controllers
    Reset,
    /// Internal connection On/Off
    Local(bool),
    /// Stop all audio (play with release parameters, unlike `Mute`).
    Stop,
    /// Turn off omni mode
    OmniOff,
    /// Turn on omni mode
    OmniOn,
    /// Set device to monophonic mode.
    Monophonic(Option<NonZeroI8>),
    /// Set device to polyphonic mode
    Polyphonic,
    /// Undefined CC
    Undefined {
        /// Which undefined CC was used
        which: i8,
        /// The value associated with the undefined CC
        value: i8,
    }
}

impl Control {
    fn new(which: i8, value: i8) -> Self {
        match which {
            0 => Control::Bank(value),
            1 => Control::Wheel(value),
            2 => Control::Breath(value),
            // 3 undefined
            4 => Control::Pedal(value),
            5 => Control::PortamentoTime(value),
            6 => Control::DataEntry(value),
            7 => Control::Volume(value),
            8 => Control::Balance(value),
            // 9 undefined
            10 => Control::Pan(value),
            11 => Control::Expression(value),
            12 => Control::EffectA(value),
            13 => Control::EffectB(value),
            // 14..=15 undefined
            16 => Control::UserA(value),
            17 => Control::UserB(value),
            18 => Control::UserC(value),
            19 => Control::UserD(value),
            // 20..=31 undefined
            32..=63 => Control::Lsb { which: which & 0x1F, value },
            64 => Control::Damper(value >= 64),
            65 => Control::Bend(value >= 64),
            66 => Control::Sostenuto(value >= 64),
            67 => Control::Soft(value >= 64),
            68 => Control::Legato(value >= 64),
            69 => Control::Hold(value >= 64),
            70 => Control::Variation(value),
            71 => Control::Resonance(value),
            72 => Control::ReleaseTime(value),
            73 => Control::AttackTime(value),
            74 => Control::CutoffFrequency(value),
            75 => Control::ShaperA(value),
            76 => Control::ShaperB(value),
            77 => Control::ShaperC(value),
            78 => Control::ShaperD(value),
            79 => Control::ShaperE(value),
            80 => Control::Decay(value >= 64),
            81 => Control::HiPassFilter(value >= 64),
            82 => Control::SwitchA(value >= 64),
            83 => Control::SwitchB(value >= 64),
            84 => Control::Portamento(value),
            // 85..=87 Undefined
            88 => Control::Velocity(value),
            // 89..=90 Undefined
            91 => Control::Reverb(value),
            92 => Control::Tremelo(value),
            93 => Control::Chorus(value),
            94 => Control::Detune(value),
            95 => Control::Phaser(value),
            96 => Control::DataIncrement,
            97 => Control::DataDecrement,
            98 => Control::NrpnLsbSelect(value),
            99 => Control::NrpnMsbSelect(value),
            100 => Control::RpnLsbSelect(value),
            101 => Control::RpnMsbSelect(value),
            // 102..=119 Undefined
            120 => Control::Mute,
            121 => Control::Reset,
            122 => Control::Local(value >= 64),
            123 => Control::Stop,
            124 => Control::OmniOff,
            125 => Control::OmniOn,
            126 => Control::Monophonic(NonZeroI8::new(value)),
            127 => Control::Polyphonic,
            which => Control::Undefined { which, value },
        }
    }
}

/// A decoded MIDI Event
#[derive(Debug, Clone, Copy)]
pub enum Event {
    /// Note stopped
    NoteOff {
        /// Channel 0-15
        chan: u8,
        /// Which note was stopped
        note: Note,
        /// Velocity of the note 0-127
        value: i8,
    },
    /// Note playing
    NoteOn {
        /// Channel 0-15
        chan: u8,
        /// Which note was played
        note: Note,
        /// Velocity of the note 0-127
        value: i8,
    },
    /// Note aftertouch parameter change
    NoteTouch {
        /// Channel 0-15
        chan: u8,
        /// Which note was played
        note: Note,
        /// Touch parameter value 0-127.
        value: i8,
    },
    /// Control Change (Continous Controller)
    Control {
        /// Channel 0-15
        chan: u8,
        /// Which control change message.
        message: Control,
    },
    /// Patch Change
    Instrument {
        /// Channel 0-15
        chan: u8,
        /// Instrument Patch ID (`[0x00-0x7F, 0x00-0x7F]`)
        patch: [i8; 2],
    },
    /// Channel Pressure
    Pressure {
        /// Channel 0-15
        chan: u8,
        /// Pressure parameter value 0-127.
        value: i8,
    },
    /// Pitch-Bend
    Bend {
        /// Channel 0-15
        chan: u8,
        /// FIXME: what is LSB
        lsb: i8,
        /// FIXME: what is MSB
        msb: i8,
    },
    /// System Message
    System {
        message: Message,
    },
}

/// MIDI System Message
#[derive(Debug, Clone, Copy)]
pub enum Message {
    /// Start System Exclusive Message
    ExStart,
    /// MIDI Time Code quarter frame
    TimeCode,
    /// Song position pointer
    SongPosition,
    /// Song selection
    SongSelect,
    /// Tune Request
    TuneRequest,
    /// End System Exclusive Message
    ExEnd,
    /// Timing Clock
    TimingClock,
    /// Start
    Start,
    /// Continue
    Continue,
    /// Stop
    Stop,
    /// Active Sensing
    ActiveSensing,
    /// Reset System
    SystemReset,
    /// Unknown System Message
    Unknown(u8),
}
