> [!IMPORTANT]
> **This project is strictly for educational purposes.** It demonstrates how game memory manipulation works and is not intended for actual gameplay advantage.  
> 
> By using this software, you:
> 1. Acknowledge that using this software violates Fortnite's Terms of Service
> 2. Agree that the developers assume no liability for damages
> 3. Confirm you will not use this in live gameplay environments
> 
> The developers do not condone cheating and provide this code solely for academic study of game security systems.

# Fortnite Cheat
A Windows-based undetectable cheat tool for Fortnite. Built in Rust with a graphical interface (GUI). This project is intended for educational use only to study how game interactions and memory manipulation work. 

## Download
1. Go to the [Releases](https://github.com/clairemavice/Fortnite-Cheat/releases) tab.
2. Download the latest `.zip` file.
3. Extract the files to a folder on your computer.
4. Disable antivirus software (may flag the tool as a false positive).
5. Run `FortniteCheat.exe` (admin privileges may be required).

Always verify the SHA256 hash of the download to ensure file integrity.

## Usage
1. Launch **Fortnite** and enter a match.
2. Run `FortniteCheat.exe` as Administrator.
3. Use the GUI to adjust settings:
   - Toggle features on/off with checkboxes.
   - Configure hotkeys in the "Settings" menu.
4. Close the tool completely before exiting Fortnite to avoid detection.

## Features
- **Visual Modifications**
  - Player ESP (highlight enemies through walls)
  - Item ESP (show weapons/ammo/loot locations)
  - Distance indicators for enemies/items
- **Combat Assistance**
  - Silent aimbot (adjustable hitbox targeting)
  - Triggerbot (auto-shoot when crosshair is near target)
  - Recoil reduction
- **Environment Manipulation**
  - No fog/brightness adjustment
  - LOD tweaks (render distant objects clearly)
- **Miscellaneous**
  - FPS counter overlay
  - Match statistics tracker
  - Configurable hotkey system
  - Memory cleaner (reduce game lag)

## Build from Source
### Requirements
- Rust 1.65+ ([install guide](https://www.rust-lang.org/tools/install))
- Windows SDK
- Microsoft Visual C++ Build Tools

### Instructions
1. Clone the repository:  
   `git clone https://github.com/clairemavice/Fortnite-Cheat.git`
2. Navigate to the project folder:  
   `cd Fortnite-Cheat`
3. Build the release:  
   `cargo build --release`
4. Find the executable in `target/release/FortniteCheat.exe`

## Contributing
Contributions are currently **closed**. The development team lacks the resources to review external code or manage collaborations. This may change in future versions.

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for full details.
