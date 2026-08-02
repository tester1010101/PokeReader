#  ☀️🌈 PokeReader, with a touch of rainbow 🌈🍀
    => Reworked with seed on start and a tint of RGB, fully customizable with any value on start possible.
[`3DS_Release_V3/sdmc_seed:`](https://github.com/tester1010101/PokeReader/releases/tag/0.8.0-rework-v3) `https://github.com/tester1010101/PokeReader/releases/tag/0.8.0-rework-v3`

[`Check out: 3DSRNG-RGB_Ver.`](https://github.com/tester1010101/3DSRNGTool/releases/tag/1.0.6-rework-v3) `https://github.com/tester1010101/3DSRNGTool/releases/tag/1.0.6-rework-v3`

New features: 
> ✓ Colored Main Menu with customizable colors/flashspeed.
> 
> ✓ Colored Submenus (can be made MORE colored, as needed).
> 
> ✓ Custom Headers (tester1010101 rework / CST.3GX).
>
> ✓ Seed on Start Page with information+support (src).
>
> ✓ Seed file on SDMC gets updated at each game restart (ms latency).
>
> ✓ Metrics added on start with RNG seed, RGB effects, live frames, calibrate easily your tapping...
>
> ✓ Any data can be on start menu, modifications/implementations may be required though.
>
> ✕ Only display RNG on start with Gen7 (US/UM), no other gen planned either...
>
> ✕ If you're adventurous you can adjust to your needs...



<img width="402" height="574" alt="pr1" src="https://github.com/user-attachments/assets/64be57b3-d073-44e8-a8a0-599062348f38" />

#

PokeReader is a 3gx plugin that allows viewing information about 3ds Pokemon games, such as:

- RNG states
- Party/Wild Pokemon
- Trainer info

This information can be used to RNG shiny and high IV Pokemon, similar to lua scripts on emulators.

## Commands

- Start + Up (D-Pad): Show/Hide the plugin
- X + Y: Lock/Unlock inputs to the plugin
- D-Pad keys: Navigate the plugin when unlocked (right & left to get in and out of the different tabs)
- Start + Select: Pause game and enable manual frame advancing
- Select: Advance one frame while paused
- Start or A: Unpause game
- SOS Menu Commands:
    - X + Right (D-Pad): Set SOS Caller slot to current Ally slot
    - X + Up/Down: Manually increment/decrement Caller slot

## Installing

1. Update to the latest [Luma](https://github.com/LumaTeam/Luma3DS/releases) or set up [Citra](https://github.com/citra-emu/citra).
1. Download and unzip the [latest PokeRGB release](https://github.com/tester1010101/PokeReader/releases/tag/0.8.0-rework-v3).
1. Copy `default.3gx` to `/luma/plugins/default.3gx` on your sd card (create the directory if it doesn't exist).

## Building

1. Install rust and the armv6k-nintendo-3ds target, devkitarm, and [3gxtool](https://gitlab.com/thepixellizeross/3gxtool)
1. Run `make`

## Credits

Thanks to these projects, teams, and individuals for being great resources:
- [Zaksabeast](https://github.com/zaksabeast/PokeReader/) for all the code snippets
- [PKHeX](https://github.com/kwsch/PKHeX/) for Pokemon related documentation, examples, and code
- [ShinySylveon04](https://github.com/ShinySylveon04/) for building most of the UI
- [Bambo-Rambo for the DexNav and Radar addresses](https://github.com/Bambo-Rambo/TinyFinder/blob/99917164b43bf79bd7432b271cced7a4d62b8431/Subforms/NTR/NtrClient.cs#L319-L326)
