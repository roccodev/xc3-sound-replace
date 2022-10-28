use fnv::FnvHashSet;
use skyline::{hook, hooks::Region};

static mut REPLACEMENT_SET: Option<FnvHashSet<u32>> = None;

#[hook(offset = 0x000642bc)]
unsafe fn wwise_file_open(
    this: u64,
    file_name: u32,
    p3: u32,
    p4: *const u32,
    p5: *const i8,
    p6: u64,
) -> u64 {
    // The game will try to load the file from the File Package (.pck) archive, but it will fall back
    // to the rom:/sound directory if it's not found. By skipping the file lookup, we can force it to
    // load from the base directory instead of the archive.
    if REPLACEMENT_SET
        .as_ref()
        .and_then(|s| s.get(&file_name))
        .is_some()
    {
        let txt_ptr = skyline::hooks::getRegionAddress(Region::Text) as *mut u8;
        let super_ptr = txt_ptr.offset(0x0005db3c);
        let font_fn: extern "C" fn(u64, u32, u32, *const u32, *const i8, u64) -> u64 =
            std::mem::transmute(super_ptr);
        (font_fn)(this, file_name, p3, p4, p5, p6)
    } else {
        call_original!(this, file_name, p3, p4, p5, p6)
    }
}

#[skyline::main(name = "xc3_sound_replace")]
pub fn main() {
    println!("[XC3-SND] Loading...");

    let mut file_set = FnvHashSet::default();
    let sound_dir = std::fs::read_dir("rom:/sound/").expect("TODO");

    for sound_file in sound_dir {
        if let Ok(entry) = sound_file {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.ends_with(".wem") || name.ends_with(".bnk") {
                let id = name.split('.').next().unwrap();
                if let Ok(id) = id.parse() {
                    file_set.insert(id);
                }
            }
        }
    }

    if file_set.is_empty() {
        println!("[XC3-SND] No replacement files found, aborting.");
        return;
    }

    unsafe { REPLACEMENT_SET = Some(file_set) };

    println!("[XC3-SND] Installing hooks");
    skyline::install_hooks!(wwise_file_open);

    println!("[XC3-SND] Loaded!");
}
