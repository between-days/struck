# Struck

![a business man](./words_smallest_violin.png)

Rust chord kreater<-RUST Chord Kreater<-(RU<-STCK)<-STRUCK
Tool for getting chords from notes and vice versa

## Notes

- chord.rs is getting heaving with all the parsing specific stuff. Maybe should have chord_parser.rs
- need to look into error handling and layering

## Running

### CLI

after `cargo run` the cli will present two dialogue options

- Get info on a chord -> produces information on a chord constructed from a chord name entered. information includes notes in the chord.
- Get a chord from some notes -> finds the name of any chords present across the notes given.

## Background info

### Models and Concepts

#### Note

The notes used in this code aren't really notes, they're pitch classes. This should change to Note and PitchClass structs but note in the general sense of A, B, C is good enough for now.

#### Interval

An interval as defined here is an enum value that represents semitone distance between two given notes.

Intervals are like the term for 'the gap' between notes in music and sets of these make up scales and therefore chords. A chord is just 3 or more (a triad) notes from the scale.
A major chord will contain the notes at the following intervals from the root note:

- MajorThird (4 semitones from root)
- PerfectFifth (7)

For a minor chord:

- MinorThird (3 semitones from root)
- perfectFifth = (7)

There are similar definitions for diminished and augmented. However where the above plays with the 3rd, diminished and augmented play with the fifth _once the 3rd is in_:

minor->diminished,
major->augmented

we're going to pause on 7ths. 9ths and so on for now.

#### Chord

As mentioned above a chord is just a set of notes belonging on particular intervals from a root note.
Chords have certain features, two of which are chord quality and triad quality.
Triad quality is like the 'fundamental quality' of a chord. Chord quality is like the 'overall quality' of a chord.

The triad qualities are:

- Minor
- Major
- Diminished
- Augmented

Chord quality extends these with:

- Suspended (2 and 4)
- Dominant (Only does straight dominants for now, no diminished 7ths yet)

The idea here is to build simple models with good mappings between fields, like getting from a chord name to a chord quality or getting from a chord quality to a set of notes.

Still working on a good way to handle patterns from 7 upwards. The general idea is:

- A G7 (G dominant 7th) is a major triad with a minor 7th. - this has dominant chord quality
- A G9 (G dominant 9th) is a major triad with a minor 7th and a major 9th. - this has dominant chord quality
- A G11 (G dominant 11th) is a major triad with a minor 7th, a major 9th and a perfect 11th. - this has dominant chord quality

It can be thought of like G9 is a default triad, default 7th. default 9th => major triad + minor 7th + major 9th so it's just G9

G7dim9 is a major triad, a minor 7th (dominant 7th) and a diminished 9 - this has dominant chord quality

To be crystal clear _the chord quality is only affected by intervals up to the 7th_

TODO: look into below chunk
At this point it's worth mentioning all chord naming is based on the concept of 'stacked thirds'.
That is to say skipping every other note in a scale gives you 1,3,5,7,9...
7ths, 9ths... are considered extensions. It might then make sense to derive the triad quality/suspension first, then handling extensions as basically:
'continue looking for the next stacked 3rd, when we find the last one, that's the number at the end of the chord, and then from there we look for the major next, then we look for addxs.

We for now treat 7ths as special extensions responsible for the chord quality, with 9ths and so on being treated as extensions
TODO: 9th, 11th addx etc

## What's in a name

### How do you get one

The name of a chord (ignoring 7ths etc for now) is composed of (Root)(chord quality) e.g.:

- Amin
- Gaug

So to create a name for a chord, all that's needed is a root and a chord quality.

Chord quality comes from a set of intervals, intervals are drawn across notes and so a list of notes can be used to generate a chord with a name.

### How do you get out of one

To get chord quality and therefore notes etc from a name, a pattern is needed.
(Pretending 7ths, adds and beyond don't count for now ) a chord name looks like this:
(Root)(chord quality) e.g.

- Ddim
- A

once the root and chord quality are pulled out, and from those the intervals and those the notes can be found.
e.g.:
Ddmin => (root = D) (quality = diminished).
For handling 7ths and so on,

## Testing

`cargo test`
