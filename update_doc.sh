#!/bin/sh

cargo doc --features=FeImplements,FePresentation && rsync --delete -auv target/doc/ docs
