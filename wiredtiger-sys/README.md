# Upgrading versions

Download a new version from https://github.com/wiredtiger/wiredtiger/releases

Check whether they've changed the build process. This definitely jumps from `./configure && make` to using `cmake` at some point.

For v3.x releases, there's an autogen integration. I am running that myself for now.