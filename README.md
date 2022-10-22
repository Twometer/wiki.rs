# wiki.rs

wiki.rs is a high-performance reader for Wikipedia dumps

## About

Wikipedia regularly provides full dumps of their database as described [here](https://en.wikipedia.org/wiki/Wikipedia:Database_download). These are multi-gigabyte
files that contain every Wikipedia page in a compressed XML file.

A special reader is required to read these kinds of files, but I have found most of these to be very slow and complex, often taking hours to load the whole dump
into the program.

wiki.rs uses the _multistream_ format, which consists of a compressed archive of articles, and a separate index file. It can index the whole database in a few seconds,
and accesses and searches articles in a few tens to hundreds of milliseconds. 

## Usage

You need to first download a multistream dump and extract it. Then, set the follwing environment variables:

| Key               | Description                            |
| ----------------- | -------------------------------------- |
| `WIKI_INDEX_FILE` | Full path to the index file            |
| `WIKI_ARTICLE_DB` | Full path to the article database file |
