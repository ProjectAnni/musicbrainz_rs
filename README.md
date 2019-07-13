# Music brainz Rust

This is still WIP. 

Music brainz rust is a utility crate for the the [music brainz](https://musicbrainz.org/doc/Development/XML_Web_Service/Version_2) API. 

## Status 
[![Build Status](https://travis-ci.org/oknozor/music_brainz_rs.svg?branch=master)](https://travis-ci.org/oknozor/music_brainz_rs)

### Model implementation : 

- [ ] Artist  
    - [x] MBID
    - [x] Name
    - [x] Sort name
    - [x] Type
    - [x] Gender
    - [x] Area
    - [x] Begin and end dates
    - [ ] IPI code
    - [ ] ISNI code
    - [ ] Alias
    - [x] Disambiguation comment
    - [ ] Annotation
- Recording
    - [x] MBID
    - [x] Title
    - [ ] Artist
    - [x] Length
    - [ ] ISRC
    - [x] Disambiguation comment
    - [ ] Annotation
- Release group
    - [x] MBID
    - [x] Title
    - [ ] Artist
    - [x] Type
    - [x] Disambiguation comment
    - [ ] Annotation

### Available method : 

- Artist : 
    - [x] artist by id
    - [ ] artist search
- Recording :
    - [x] recording by id
    - [ ] recording search
- Release group
    - [x] release group by id
    - [ ] release group search

