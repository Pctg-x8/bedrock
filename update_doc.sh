#!/bin/sh

cargo doc --features=FeImplements,FePresentation && cp -rv target/doc/* docs/
