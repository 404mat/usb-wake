# Plan: USB/Bluetooth Wakeup Manager (Ratatui)

- [x] **1. Init project**

  - [x] `cargo new usb-wakeup --bin`
  - [x] Add deps: `ratatui`, `crossterm`, `anyhow`

- [ ] **2. Device discovery**

  - [ ] Read `/sys/bus/usb/devices/*`
  - [ ] Extract: `idVendor`, `idProduct`, `product`, `power/wakeup`
  - [ ] Represent as `Device` struct

- [ ] **3. Basic TUI setup**

  - [ ] Init Ratatui terminal
  - [ ] Render list of devices with `[ ]` or `[x]`
  - [ ] Show help bar at bottom

- [ ] **4. Navigation**

  - [ ] Arrow keys: move selection
  - [ ] Highlight selected row

- [ ] **5. Toggle wakeup**

  - [ ] Space key toggles device wakeup
  - [ ] Write to `/sys/.../power/wakeup`
  - [ ] Update state

- [ ] **6. Quit handling**

  - [ ] Press `q` to quit or toggle `exit` option at the bottom
  - [ ] Restore terminal cleanly

- [ ] **7. Polish**

  - [ ] Handle errors (`N/A` if no wakeup file)
  - [ ] Require root warning
  - [ ] Display vendor/product IDs alongside names

- [ ] **8. Build & distribute**
  - [ ] `cargo build --release`
  - [ ] Package binary (~2 MB)
  - [ ] Optional: `.deb` / `.rpm` / `.AppImage`
