# bnd4

A Rust library for parsing BND4 data, typically used for FromSoftware game save files (Dark Souls, Elden Ring).

## Overview

The BND4 format is proprietary; this crate only ports the reverse engineering efforts of others from other languages
to Rust. BND4 acts as a container for game data. It consists of (typically 11) named entries. Each entry (typically)
stores information about a character player, e.g., a "save slot."

This crate does not try to make sense of the game data - parsing the `data` fields in the BND4Entry is up to you.


