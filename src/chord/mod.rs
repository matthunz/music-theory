use crate::{midi::MidiNote, set::IntervalSet, Interval, Pitch};
use core::{
    fmt::{self, Write},
    iter,
};

mod builder;
pub use builder::Builder;

pub struct Chord {
    root: Pitch,
    builder: Builder,
}

impl Chord {
    pub fn major() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    pub fn minor() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MAJOR_THIRD)
            .interval(Interval::PERFECT_FIFTH)
    }

    /// ```
    /// use music_note::{Chord, Pitch};
    ///
    /// // D7
    /// let chord = Chord::seventh().build(Pitch::D);
    ///
    /// let notes = [Pitch::D, Pitch::FSharp, Pitch::A, Pitch::C];
    /// assert!(chord.into_iter().eq(notes));
    /// ```
    pub fn seventh() -> Builder {
        Self::major().interval(Interval::MINOR_SEVENTH)
    }

    pub fn major_seventh() -> Builder {
        Self::major().interval(Interval::MAJOR_SEVENTH)
    }

    pub fn minor_seventh() -> Builder {
        Self::minor().interval(Interval::MINOR_SEVENTH)
    }

    pub fn minor_major_seventh() -> Builder {
        Self::minor().interval(Interval::MAJOR_SEVENTH)
    }

    pub fn half_diminished() -> Builder {
        Self::builder()
            .root()
            .interval(Interval::MINOR_THIRD)
            .interval(Interval::TRITONE)
            .interval(Interval::MINOR_SEVENTH)
    }

    pub fn builder() -> Builder {
        Builder {
            bass: None,
            is_inversion: false,
            intervals: IntervalSet::default(),
        }
    }

    /// ```
    /// use music_note::{midi, Chord, Pitch};
    ///
    /// let chord = Chord::from_midi(
    ///     midi!(C, 4),
    ///     [midi!(E, 3), midi!(G, 3), midi!(C, 4)]
    /// );
    ///
    /// assert_eq!(chord.to_string(), "C/E");
    ///
    /// let pitches = [Pitch::E, Pitch::G, Pitch::C];
    /// assert!(chord.into_iter().eq(pitches));
    /// ```
    pub fn from_midi<I>(root: MidiNote, iter: I) -> Self
    where
        I: IntoIterator<Item = MidiNote>,
    {
        let mut iter = iter.into_iter();
        let mut intervals = IntervalSet::default();

        let bass_note = iter.next().unwrap();
        let root_pitch = root.pitch();
        let bass = if bass_note != root {
            let bass_pitch = bass_note.pitch();
            intervals.push(bass_pitch - root_pitch);
            Some(bass_note.pitch())
        } else {
            intervals.push(Interval::UNISON);
            None
        };

        let is_inversion = if let Some(note) = iter.next() {
            let ret = if note == root { false } else { true };

            intervals.push(note.pitch() - root_pitch);
            intervals.extend(iter.map(|midi| midi - root));
            ret
        } else {
            false
        };

        Self {
            root: root.pitch(),
            builder: Builder {
                bass,
                is_inversion,
                intervals,
            },
        }
    }

    pub fn root(self) -> Pitch {
        self.root
    }

    pub fn intervals(self) -> Intervals {
        // TODO maybe use rotate_right?
        let (high, low) = if let Some(bass) = self.builder.bass {
            let bass_interval =
                Interval::new((self.root.into_byte() as i8 - bass.into_byte() as i8).abs() as u8);
            if self.builder.is_inversion {
                self.builder.intervals.split(bass_interval)
            } else {
                (
                    self.builder.intervals,
                    [bass_interval].into_iter().collect(),
                )
            }
        } else {
            (IntervalSet::default(), self.builder.intervals)
        };

        Intervals { low, high }
    }
}

impl FromIterator<MidiNote> for Chord {
    fn from_iter<T: IntoIterator<Item = MidiNote>>(iter: T) -> Self {
        let mut notes = iter.into_iter();
        let root = notes.next().unwrap_or(MidiNote::from_byte(0));

        Self::from_midi(root, iter::once(root).chain(notes))
    }
}

impl IntoIterator for Chord {
    type Item = Pitch;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            root: self.root,
            intervals: self.intervals(),
        }
    }
}

pub struct Intervals {
    low: IntervalSet,
    high: IntervalSet,
}

impl Iterator for Intervals {
    type Item = Interval;

    fn next(&mut self) -> Option<Self::Item> {
        self.low.next().or_else(|| self.high.next())
    }
}

pub struct Iter {
    root: Pitch,
    intervals: Intervals,
}

impl Iterator for Iter {
    type Item = Pitch;

    fn next(&mut self) -> Option<Self::Item> {
        self.intervals.next().map(|interval| self.root + interval)
    }
}

impl fmt::Display for Chord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.root.fmt(f)?;

        if self.builder.intervals.contains(Interval::MINOR_THIRD) {
            f.write_char('m')?
        } else if self.builder.intervals.contains(Interval::MAJOR_SECOND) {
            f.write_str("sus2")?
        } else if self.builder.intervals.contains(Interval::PERFECT_FOURTH) {
            f.write_str("sus4")?
        }

        let mut has_fifth = true;
        if self.builder.intervals.contains(Interval::TRITONE) {
            f.write_str("b5")?
        } else if !self.builder.intervals.contains(Interval::PERFECT_FIFTH) {
            has_fifth = false;
        }

        if self.builder.intervals.contains(Interval::MINOR_SEVENTH) {
            f.write_char('7')?
        } else if self.builder.intervals.contains(Interval::MAJOR_SEVENTH) {
            f.write_str("maj7")?
        }

        if let Some(bass) = self.builder.bass {
            write!(f, "/{}", bass)?;
        }

        if !self.builder.intervals.contains(Interval::UNISON) {
            f.write_str("(no root)")?
        }

        if !has_fifth {
            f.write_str("(no5)")?
        }

        Ok(())
    }
}
