#!/bin/bash
# Lists all USB devices including root hubs, with power/wakeup state

found=0

for d in /sys/bus/usb/devices/*; do
  if [[ -f "$d/idVendor" ]]; then
    found=1
    vendor=$(cat "$d/idVendor")
    product=$(cat "$d/idProduct")
    driver="none"
    [[ -L "$d/driver" ]] && driver=$(basename $(readlink "$d/driver"))
    name=$(lsusb -d ${vendor}:${product} 2>/dev/null | cut -d':' -f2 | xargs)
    [[ -z "$name" ]] && name="Unknown Device"

    wake="N/A"
    [[ -f "$d/power/wakeup" ]] && wake=$(cat "$d/power/wakeup")

    echo "[Vendor $vendor Product $product] $name (driver: $driver, wakeup: $wake)"
  fi
done

if [[ $found -eq 0 ]]; then
  echo "No USB devices found."
fi
