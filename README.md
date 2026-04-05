# git-large-blobs

<https://mastodon.social/@mcc/116352961470167933>

Find all blobs larger than a cutoff size in bytes, then figure out which commit/files those refer to.
Naive implementation, could be made faster with some caching, or resolve branches etc.

## Install

    cargo install --git https://github.com/passcod/git-large-blobs.git git-large-blobs

## Usage

```
$ git large-blobs

Found 1 blobs above 1024 bytes, resolving to objects...
size	blob                                    	commit                                  	filename
16171	31b1e4d081eec081aae2d343ba004cd5481f757a	4b4891e15d2e75a8ffb2193d2712b9f34962f624	Cargo.lock
```
