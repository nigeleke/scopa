#!/bin/bash

SRC=../design/images/base_denari.svg

magick "$SRC" \
  -define icon:auto-resize=256,128,64,48,32,16 \
  "icon.ico"

TMP="icon.iconset"
mkdir -p "$TMP"

magick "$SRC" -resize 16x16   -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/16x16.png"
magick "$SRC" -resize 32x32   -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/16x16@2x.png"
magick "$SRC" -resize 32x32   -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/32x32.png"
magick "$SRC" -resize 64x64   -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/32x32@2x.png"
magick "$SRC" -resize 128x128 -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/128x128.png"
magick "$SRC" -resize 256x256 -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/128x128@2x.png"
magick "$SRC" -resize 256x256 -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/256x256.png"
magick "$SRC" -resize 512x512 -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/256x256@2x.png"
magick "$SRC" -resize 512x512 -strip -define png:color-type=6 -define png:bit-depth=8 "$TMP/512x512.png"

magick "$TMP/*x*.png" icon.icns
cp $TMP/*x*.png .
rm -rf "$TMP"
