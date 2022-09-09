# Module

This crate provides easy access to module data.

- It supports reading images from disk, using Windows' default search paths.
- It also supports simply loading the image into memory and then reading it. This will result in code execution.

Please note: these 2 methods DO provide differing results, as addresses and such inside a dynamically loaded image are filled out by the loader.
