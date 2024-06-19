# Sound replacement mod for Xenoblade 3
This mod allows you to load custom sound files and sound banks, replacing the game's defaults.

> **Important**: While this mod does not deal with persistent data, you are still using it at your own risk. I am
not responsible for anything that could happen to your saves, game, console, account, etc.

## Usage

#### Switch
1. Download the latest version of the mod from the [Releases](https://github.com/RoccoDev/xc3-sound-replace/releases/latest) page.
2. Extract the archive to root of your SD card.

#### Ryujinx
1. Download the latest version of the mod from the [Releases](https://github.com/RoccoDev/xc3-sound-replace/releases/latest) page.
2. Open Ryujinx, then right-click on the game and select "Open Atmosphere Mods Directory".
3. From the archive, extract the `exefs` and `romfs` directory into the folder you opened.

#### Replacing sound files

After installing the mod, you can place `.wem`  files in the `/atmosphere/contents/010074f013262000/romfs/sound` folder.
When the game tries to load those files from the game's original archive, the mod will load the custom ones instead.

#### Replacing sound banks

For sound banks (`.bnk` files), the procedure is slightly different. You need both the hashed and unhashed name of the bank.

For example, for `bgm_es.bnk`, create an empty (doesn't matter) file in `romfs/sound` called `3214266068.bnk`
(`3214266068` is the FNV1-32 hash for `bgm_es`), then put your modified bank in `romfs/sound/bgm_es.bnk` (using the real name).

## Build instructions
To build the project, install [Rust](https://rustup.rs/) and run
```sh
./build.sh
```

## License
This mod is distributed under the terms of the [GPLv3](https://www.gnu.org/licenses/gpl-3.0.html). See [COPYING](COPYING) for details.
