# Building Rust for Production

This repository serves to highlight three important topics in rust development, when producting artifacts. 

1. Local development
2. Continuous integration
3. Production

Each stage of development has its own requirements, Rust already has decent defaults for each stage, but also exposes handles such that we can tune the experience and get more bang for our buck.

In this repo I will highlight each topic in a separate sub directory, all basis itself off, of a control: `sample-app`, each folder / sub app will contain stats on what we gain by following some of the tips and tricks implemented against the control.

## Bonus different targets

I will also show what we can gain from using other targets than regular gnu for linux, such as musl, or wasm.
