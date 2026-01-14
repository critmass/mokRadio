[33mcommit a7de487de3d4dc2e655a8adfa82fc18db9bd487e[m[33m ([m[1;36mHEAD[m[33m -> [m[1;32mmain[m[33m, [m[1;31morigin/main[m[33m)[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Tue Jan 13 17:00:44 2026 -0800

    finished initial pass on station.rs

[33mcommit 6d8ed86d139f886eafb3f94189622e424e677b42[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Sat Jan 3 23:41:23 2026 -0800

    Reoganized everything and built messages, and work
    on the radio object and the station object.
    
    modified:   src/main.rs
    modified:   src/messages.rs
    renamed:    src/station/manager.rs -> src/radio.rs
    new file:   src/radio/station.rs
    renamed:    src/station/config.rs -> src/radio/station/config.rs
    renamed:    src/station/content.rs -> src/radio/station/content.rs
    renamed:    src/station/content/live.rs -> src/radio/station/content/live.rs
    renamed:    src/station/content/track.rs -> src/radio/station/content/track.rs
    new file:   src/radio/station/utilities.rs
    new file:   src/radio/station/utilities/whats_next.rs
    deleted:    src/station.rs
    deleted:    src/station/structure.rs

[33mcommit 8b7a64fb4717f9a35327abbd852392c4c2065515[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Fri Oct 17 20:57:00 2025 -0700

    Reorganizing

[33mcommit ebc0c13112b01a6ecaa518fab43d5edd8fc042bc[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Wed Oct 1 00:41:27 2025 -0700

    feat: implement Station and PlayType constructors with playlist loading
    
    - Created Station::new() constructor that loads config and initializes audio sink
    - Implemented PlayType::new() with support for Random, Chronologic, Reverse, and Shuffle modes
    - Added StationConfig for loading station.info JSON files
    - Created Track struct with MP3 duration parsing and modification time ordering
    - Implemented load_tracks_from_path() iterator for efficient track loading
    - Added shuffle functionality using rand crate
    - Refactored code to eliminate duplication in playlist loading
    - Added LiveStream struct with scheduling support (not yet fully implemented)
    - Moved station module from state/radio to state for better organization

[33mcommit 843a181033a0a03da836ebdef74c75232dde9559[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Tue Sep 30 00:57:16 2025 -0700

    deleting test files

[33mcommit 9be8dd0382892ae9920a6bd14c094ee54bb54976[m
Author: Jason Cox <jgcox.softwareDeveloper@gmail.com>
Date:   Tue Sep 30 00:02:37 2025 -0700

    Initial commit
