#!/bin/bash
INITRAMFS=$1
FS_TYPE=$2
OUT=ramfs.img
SIZE=64M

# Sets to exit on any error
set -e

# Check if the user has provided the initramfs directory
if [[ ! -d "$INITRAMFS" ]]; then
    echo "Error: '$INITRAMFS' is not a valid directory."
    exit 1
fi

# Removes the old image
rm -f "$OUT"

# Check the filesystem type

# FAT32 filesystem
if [[ "$FS_TYPE" == "fat32" ]]; then
    # Create a FAT32 disk image
    dd if=/dev/zero of="$OUT" bs=1M count=${SIZE%M}
    mkfs.vfat "$OUT"

    TMPDIR=$(mktemp -d)
    sudo mount -o loop "$OUT" "$TMPDIR"
    sudo cp -r "$INITRAMFS"/* "$TMPDIR"
    sudo umount "$TMPDIR"
    rmdir "$TMPDIR"

    echo "FAT32 image created: $OUT"

# UStar filesystem
elif [[ "$FS_TYPE" == "ustar" ]]; then
    tar --format=ustar -cvf "$OUT" -C "$INITRAMFS" .
    
    echo "UStar archive created: $OUT"

# Invalid filesystem type
else
    echo "Error: Invalid filesystem type. Use 'fat32' or 'ustar'."
    exit 1
fi
