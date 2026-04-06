# git-large-blobs

Find large files you may have accidentally committed to the repo.

As per: <https://mastodon.social/@mcc/116352961470167933>

Find all blobs larger than a cutoff size in bytes, then figure out which commit/files those refer to.
Naive implementation, could be made faster with some caching, or resolve branches etc.

## Install

    cargo install --git https://github.com/passcod/git-large-blobs.git git-large-blobs

## Usage

```console
$ git large-blobs
Blobs found: 119071
Over cutoff: 49

Found 47 blobs above 1048576 bytes, resolving to objects...
size           	blob                                    	commit                                  	filename
38563731       	9f8cce12431425c4ecc851b43d4ae7c290024a0c	d358f3ebb102ff91bd029bcdb201e5b2b9c23eff	msi-packager
7281664        	ff7dcba19509fa77f715429a40ef1b7511e24e02	820bcd71f65e03b6270586a16cbbf78009df8231	realm.node
3386892        	ae3dd40ce47c4d4396778187469c5ac955534b26	ecc18567e925b5ad6616fcaaa4469e7baa655beb	fhir.schema.json
3271036        	3e736938fb328f82089242508c1373e68341286b	b4dd60b0952873de5ffc623487fc6b73ab5292ee	binding.node
2267648        	c534dc8fbe51b84d4055e1cf5aa06236be081982	d358f3ebb102ff91bd029bcdb201e5b2b9c23eff	node_sqlite3.node
2126999        	50e3d3b69611bd4a357da36d85bbb6b776be39db	0ee3b9df335e915b2fc1c498bee6c38112d76759	Avatar_12.tsx
2126979        	5edfe5acd6153ef0cb1af1463f16e711cdedfff3	3df9b9e48b603eb4927aa37af5ddd7d711d4fec7	Avatar_12.tsx
2126919        	b1b7c4dff7840a8009cd46e79408ae13966fdc89	1f1bc23b6e9cc5715e07e8e22f35ccc3dceb3cd5	Avatar12.tsx
2126627        	2ecf3572b88e3d05ce040100765736e83bce993d	9626f862e4c9e5f08d14d1f0651e0baae604c50d	Avatar_12.svg
1803347        	e308b4ae74f332e07f2a42dc9b37215baf085487	f79b70b9a33d3f19478cce09e00f0f199637a32a	bundle.js.map
1268930        	62a59eea65772d23997a6515057a9941b6279e29	d358f3ebb102ff91bd029bcdb201e5b2b9c23eff	screen_1.jpg
1241286        	369904d02fa9ca1b490956de845b8fda1b41c931	207eaa4a6563d2e98db2ac7540c227c815afe158	screen_2.jpg
1237547        	26bbfd6b032095fb48499f8d23c59d9ce38ff15b	038517447dab8a99f604949412b7740b2b4f7be8	yarn.lock
```
