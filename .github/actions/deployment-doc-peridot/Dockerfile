FROM node:13

# Install tools
RUN sudo npm install -g firebase-tools
RUN apt-get update && apt-get install -y rsync

# Copy pre-initialized firebase configs
RUN mkdir -p /doc/public/ja
ADD .firebaserc /doc/
ADD firebase.json /doc/
ADD gen.sh /
ADD translate_ja.js /

ENTRYPOINT ["/gen.sh"]
