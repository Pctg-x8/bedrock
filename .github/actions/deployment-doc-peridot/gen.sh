#!/bin/sh -xe

# Copy generated docs for deployment
cd /doc
rsync --delete -auv $GITHUB_WORKSPACE/target/doc/ public
rsync --delete -auv $GITHUB_WORKSPACE/target/doc/ public/ja
node /translate_ja.js
echo $GOOGLE_APPLICATION_CREDENTIALS
firebase deploy --project docs-541f3 --only hosting:peridot-branch
