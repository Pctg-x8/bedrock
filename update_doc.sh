#!/bin/sh

mkdir -p docs
cargo doc --features=FeImplements,FePresentation,VK_EXT_debug_report && rsync --delete -auv target/doc/ docs
mkdir docs/ja && rsync --delete -auv target/doc/ docs/ja && node translate_ja.js
