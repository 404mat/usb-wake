#!/bin/bash
# Lists only real peripherals (skips root hubs and USB hubs) with power/wakeup state

for d in /sys/bus/usb/devices/*; do
  if [[ -f "$d/idVendor" ]]; then
    vendor=$(cat "$d/idVendor")
    [[ "$vendor" == "1d6b" ]] && continue  # skip root hubs
    bDeviceClass=$(cat "$d/bDeviceClass")
    [[ "$bDeviceClass" == "09" ]] && continue # skip hubs

    product=$(cat "$d/idProduct")
    driver="none"
    [[ -L "$d/driver" ]] && driver=$(basename $(readlink "$d/driver"))
    name=$(lsusb -d ${vendor}:${product} 2>/dev/null | cut -d':' -f2 | xargs)

    wake="N/A"
    [[ -f "$d/power/wakeup" ]] && wake=$(cat "$d/power/wakeup")

    echo "[Vendor $vendor Product $product] $name (driver: $driver, wakeup: $wake)"
  fi
done
