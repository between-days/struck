use std::str::FromStr;

use crate::theory::{
    chord::ChordQuality,
    error::ScaleDegreeParseError,
    interval::Interval,
    note::Note,
    pitch_class::{self, PitchClass},
};

// just western for now
// essentially all scales are the major scale, but the mode changes what the tonic is
// so ionian is the name of the typical major scale, and it's tonic is at position 0 in the major scale of a given note
// dorian has it's tonic at position 1 and so on down the list
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
    Ionian = 0,
    Dorian = 1,
    Phyrgian = 2,
    Lydian = 3,
    Mixolydian = 4,
    Aeolian = 5,
    Locrian = 6,

    // TODO: fix later
    Unknown = 100,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
// steps/semitones
pub enum Step {
    W = 2,
    H = 1,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct Key {
    pub tonic: PitchClass,
    pub mode: Mode,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Scale {
    pub key: Key,
    pub pitch_classes: Vec<PitchClass>,
}

// each scale will have positions like iv, v, vi
// those are the chords build from the progression where the chord root is the scale degree in question
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum ScaleDegree {
    I = 0,
    II = 1,
    III = 2,
    IV = 3,
    V = 4,
    VI = 5,
    VII = 6,
}

impl ScaleDegree {
    // the scale degrees within a scale each have a 'required' chord quality to keep the chord in key
    // example being I in scale is a major chord rooted at the tonic, VII in scale should actually be viidim = diminished chord rooted at the 7th of the scale etc
    pub fn get_default_quality(scale_degree: ScaleDegree) -> ChordQuality {
        match scale_degree {
            ScaleDegree::I => ChordQuality::Major,
            ScaleDegree::II => ChordQuality::Minor,
            ScaleDegree::III => ChordQuality::Minor,
            ScaleDegree::IV => ChordQuality::Major,
            ScaleDegree::V => ChordQuality::Major,
            ScaleDegree::VI => ChordQuality::Minor,
            ScaleDegree::VII => ChordQuality::Diminished,
        }
    }
}

impl FromStr for ScaleDegree {
    type Err = ScaleDegreeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "i" | "I" => Ok(ScaleDegree::I),
            "ii" | "II" => Ok(ScaleDegree::II),
            "iii" | "III" => Ok(ScaleDegree::III),
            "iv" | "IV" => Ok(ScaleDegree::IV),
            "v" | "V" => Ok(ScaleDegree::V),
            "vi" | "VI" => Ok(ScaleDegree::VI),
            "vii" | "VII" => Ok(ScaleDegree::VII),

            _ => Err(ScaleDegreeParseError::InvalidStringValue(s.to_string())),
        }
    }
}

impl Mode {
    // the w w h w etc pattern for major scale
    // we can reorder this to get all the modes
    pub fn get_default_semitones() -> Vec<Step> {
        return vec![
            Step::W,
            Step::W,
            Step::H,
            Step::W,
            Step::W,
            Step::W,
            Step::H,
        ];
    }

    pub fn get_default_qualities() -> Vec<ChordQuality> {
        return vec![
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Minor,
            ChordQuality::Major,
            ChordQuality::Major,
            ChordQuality::Minor,
            ChordQuality::Diminished,
        ];
    }

    pub fn get_quality_at_scale_degree(&self, scale_degree: ScaleDegree) -> ChordQuality {
        let mut defaults = Mode::get_default_qualities();

        // TODO: this could be wasted cycles, might want to put mode chord qualities in mode struct for one time compute
        defaults.rotate_left(*self as usize);

        return *defaults.get(scale_degree as usize).expect("TODO:");
    }

    // get the step pattern for the mode
    // pattern shown here is good https://mixedinkey.com/captain-plugins/wiki/an-introduction-to-modes/
    // shift the pattern forward by 1,2,3, etc going from ionian, dorian etc, locrian.
    pub fn get_mode_steps(&self) -> Vec<Step> {
        let mut steps = Mode::get_default_semitones();

        steps.rotate_left(*self as usize);

        return steps;
    }
}

impl Scale {
    pub fn from_key(key: Key) -> Scale {
        let tonic = key.tonic;

        // cumulative add on steps in scan, map to get the corresponding pitch class
        let mut pitch_classes: Vec<PitchClass> = std::iter::once(tonic)
            .chain(
                key.mode
                    .get_mode_steps()
                    .iter()
                    .scan(0, |sum, &step| {
                        *sum += step as i8;
                        println!("sum: {}", sum.abs());
                        Some(sum.clone())
                    })
                    .map(|total| {
                        let t = total;
                        println!(
                            "total: {}\n, interval: {}\n class: {}",
                            t,
                            Interval::from(t as i8),
                            tonic.get_interval(Interval::from(t as i8))
                        );
                        return tonic.get_interval(Interval::from(t as i8));
                    }),
            )
            .collect();

        // TODO: clean this, do we want the octave in the steps/pitch classes for scale?
        // cheese to drop last
        pitch_classes.pop();

        return Scale {
            key: key,
            pitch_classes: pitch_classes,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //
    // get_mode_steps
    //

    // TODO: parameterize these tests with csv inputs
    #[test]
    fn test_get_ionian_steps() {
        let steps = vec![
            Step::W,
            Step::W,
            Step::H,
            Step::W,
            Step::W,
            Step::W,
            Step::H,
        ];

        let ret = Mode::Ionian.get_mode_steps();

        assert_eq!(ret, steps);
    }

    #[test]
    fn test_get_dorian_steps() {
        let steps = vec![
            Step::W,
            Step::H,
            Step::W,
            Step::W,
            Step::W,
            Step::H,
            Step::W,
        ];

        let ret = Mode::Dorian.get_mode_steps();

        assert_eq!(ret, steps);
    }

    // #[test]
    // fn test_get_phryrigian_steps() {
    //     let ionian = vec![
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //         Step::W,
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //     ];
    // }

    // #[test]
    // fn test_get_lydian_steps() {
    //     let ionian = vec![
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //         Step::W,
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //     ];
    // }

    // #[test]
    // fn test_get_mixolydian_steps() {
    //     let ionian = vec![
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //         Step::W,
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //     ];
    // }

    // #[test]
    // fn test_get_aeolian_steps() {
    //     let ionian = vec![
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //         Step::W,
    //         Step::W,
    //         Step::H,
    //         Step::W,
    //     ];
    // }

    #[test]
    fn test_get_locrian_steps() {
        let steps = vec![
            Step::H,
            Step::W,
            Step::W,
            Step::H,
            Step::W,
            Step::W,
            Step::W,
        ];

        let ret = Mode::Locrian.get_mode_steps();

        assert_eq!(ret, steps);
    }

    //
    // Scale.from_key
    //

    #[test]
    fn test_scale_from_key_a_m() {
        let key = Key {
            tonic: PitchClass::A,
            mode: Mode::Aeolian,
        };

        let expected = Scale {
            key: key,
            pitch_classes: vec![
                PitchClass::A,
                PitchClass::B,
                PitchClass::C,
                PitchClass::D,
                PitchClass::E,
                PitchClass::F,
                PitchClass::G,
            ],
        };

        let ret = Scale::from_key(key.clone());

        assert_eq!(ret, expected);

        println!("ret: {:?}", ret)
    }
}
