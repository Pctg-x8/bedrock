#!/bin/sh -xe

# Copy generated docs for deployment
cd /doc
rsync --delete -auv $GITHUB_WORKSPACE/target/doc/ public
rsync --delete -auv $GITHUB_WORKSPACE/target/doc/ public/ja
node /translate_ja.js
firebase deploy --token $FIREBASE_TOKEN --project docs-541f3 --only hosting:peridot-branch
