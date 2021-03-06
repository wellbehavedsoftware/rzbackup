# RZBackup

http://rzbackup.com

James Pharaoh <james@pharaoh.uk>

This is a partial Rust clone of [ZBackup](http://zbackup.org/), along with some
unique features of its own.

This project is free software and available under the [Apache 2.0 licence]
(https://www.apache.org/licenses/LICENSE-2.0).

Binaries for ubuntu are available can be downloaded [here]
(https://dist.wellbehavedsoftware.com/rzbackup/).

Online documentation is automatically generated by [docs.rs]
(https://docs.rs/rzbackup/).

## List of features

* Rust library for access to ZBackup repositories
* Supports encrypted and non encrypted formats
* Multi-threaded restore and configurable, multi-tier chunk cache
* Client/server utilities to efficiently restore multiple backups with a shared
  chunk cache
* RandomAccess implements Read and Seek to provide efficient random access for
  rust applications
* Balance tools to redistribute data in index and bundle files, typically useful
  to obtain a smaller number of consistently-sized files
* Thorough garbage collection tools to clean up indexes and chunks which are no
  longer in use
* Check tools, to verify the integrity of the repository in various ways, and
  repair simple problems
* Command line decrypt utility, mostly useful for debugging

Notable missing features

* No facility to create backups, these must be performed with the original
  ZBackup tool

## Library usage

In cargo.toml:

```toml
[dependencies]
output = "0.5"
rzbackup = "3.3"
```

A basic example (from `examples/restore.rs`):

```rust
extern crate output;
extern crate rzbackup;

use std::env;
use std::ffi::OsString;
use std::io;

use rzbackup::Repository;

fn main () {

	let output =
		output::open ();

	let arguments: Vec <OsString> =
		env::args_os ().collect ();

	let repository =
		Repository::open (
			& output,
			Repository::default_config (),
			& arguments [1],
			if arguments [2] != "" {
				Some (& arguments [2])
			} else { None },
		).unwrap ();

	let stdout =
		io::stdout ();

	let mut stdout_lock =
		stdout.lock ();

	repository.restore (
		& output,
		arguments [3],
		& mut stdout_lock,
	).unwrap ();

}
```

## Command usage

### Restore

The restore command is able to perform a one-off restore. It is basically
equivalent to ZBackup's own `restore` command. The restored content is piped to
stdout.

```sh
rzbackup restore \
	--repository REPOSITORY \
	[--password-file PASSWORD-FILE] \
	--backup-name BACKUP-NAME
```

### Server

The server process listens for client connections and streams backups over a
socket. It has a large cache and so will be more efficient than running separate
restore processes for backed up data with lots of shared deduplicated content.

```sh
rzbackup server \
	--listen-address HOST:PORT \
	--repository REPOSITORY \
	[--password-file PASSWORD-FILE] \
	... (lots more options, try --help)
```

### Client

The client connects to the server and streams a backup to standard output. It
can also tell the server to reload its indexes, which will be necessary if new
backups have been made.

```sh
rzbackup client reindex \
	--server-address HOST:PORT
```

```sh
rzbackup client restore \
	--server-address HOST:PORT \
	--backup-name BACKUP-NAME
```

### Convert

The convert tool makes low-level changes to the repository. It is able to
balance both index and bundle files, changing the number of entries they
contain. It can perform garbage collection, removing index entries and chunk
data which is no longer referenced by any backups. It is able to perform various
consistency checks.

#### Balance bundles

```sh
rzbackup convert balance-bundles \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE \
    [--chunks-per-bundle CHUNKS-PER-BUNDLE] \
    [--fill-factor FILL-FACTOR]
```

The `balance-bundles` tool balances the number of chunks contained in bundle
files. This consists of reading all index files in order to determine which
bundles contain more or less than the desired number of chunks, then reading
those bundles and writing them out to new bundles with the required number of
chunks in each one.

This writes out a new index file for each bundle, currently, so you may want to
follow this by running `balance-indexes`.

The number of chunks per bundle is configurable on the command line, this
currently defaults to 256, which is considerably higher than zbackup's own 32.
The fill factor defaults to 25, this is a percentage of the chunks per bundle
which will determine the minimum percentage of the total chunks which a bundle
should have in order to be left as-is.

#### Balance indexes

```sh
rzbackup convert balance-indexes \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE \
    [--bundles-per-index BUNDLES-PER-INDEX]
```

The `balance-indexes` tool balances the number of bundles indexed in individual
index files. This consists of reading all indexes to determine which have more
or less than the desired number of bundles, then writing out the information
about those bundles to new index files with the desired number of bundles in
each one.

The number of bundles per index is configurable on the command line, this
currently defaults to 16384. ZBackup defaults to writing out a single index file
every time a new backup is created (unless there are no bundles written), so
index files vary in size and there are typically a large number of small indexes
and a small number of large ones in a well-used repository.

Since, in general, every index file is loaded before doing any work on a
repository, it makes sense to have fewer files, in order to reduce the overhead
of opening a large number of files. This also makes synchronisation, eg to a
remote location, more efficient.

#### Check backups

```sh
rzbackup convert check-backups \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE \
    [--move-broken] \
    [--backup-name-hash-prefix] PREFIX
```

The `check-backups` command verifies all backups can be restored, by verifying
that all the required chunks are present in the indexes. It can optionally move
any backups which cannot be restored from `backups` to `backups-broken`, and it
can operate on a subset of backups by specifying a prefix which is matched
against the SHA1 sum of the backup name, eg `/dir/filename` for a backup in
`/backups/dir/filename`.

It is a good idea to run `rebuild-indexes` before running this command, so that
the state of the indexes is an accurate representation of the available chunks.

#### Check bundles

```sh
rzbackup convert check-bundles \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE \
    [--bundle-name-prefix] PREFIX
```

The `check-bundles` tool simply reads every bundle in its entirety, applying
consistency checks to the data they contain. This will verify the adler32
checksums after the bundle info at the start of the file and at the end of the
file, and also checks that each chunk's data matches its truncated SHA1 sum
which makes up the first 16 bytesof its chunk id. It also, implicitly, verifies
that the compression and encryption are intact, since these are required to read
the bundle.

Since this can take a very long time, a bundle name prefix can be specified, and
only bundles whose names start with this string will be checked. This can be
used, for example, to check a subset of bundles on a nightly basis.

This is useful to detect corruption in bundle files, which can then be restored
from backups, if those are available. If backups are not available, then bundles
can be removed, and these can be removed from the indexes using
`rebuild-indexes`, preventing any future backups from being creating referencing
these missing chunks.

Backups which are already broken can be identified by running `check-backups`
after running `rebuild-indexes`.
backups can be identified.

#### Check indexes

```sh
rzbackup convert check-indexes \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE \
    [--repair]
```

The `check-indexes` tool reads each backup file, building an index of all chunks
which they reference, and then reads index files, reporting any missing or
duplicated chunks. If the `--repair` option is specified, it will rewrite the
index files to remove these chunks.

#### Garbage collect indexes

```sh
rzbackup convert gc-indexes \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE
```

The `gc-indexes` tool performs garbage collection on index files. This consists
of identifying all the chunks which are referenced by backups, then rewriting
the index files to remove any chunks which are no longer needed.

Note that this will not make any changes to bundles, so you probably want to
then run `gc-bundles`, to remove the chunks which are no longer needed.

#### Garbage collect bundles

```sh
rzbackup convert gc-bundles \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE
```

The `gc-bundles` tool performs garbage collection on bundles. This consists of
identifying all chunks which are referenced in index files, then rewriting the
bundles to remove any chunks which are no longer needed.

This will also remove any chunks which are duplicated. These will not be removed
from the index, which can cause problems with restore operations which reference
the duplicated chunks, but this can be remedied with the `rebuild-indexes` tool.

#### Rebuild indexes

```sh
rzbackup convert rebuild-indexes \
    --repository REPOSITORY \
    --password-file PASSWORD-FILE
```

The `rebuild-indexes` tool scans all bundle files, reading only the chunk info
in their header, then builds a completely new set of index files based on the
information contained there. This is a useful way to ensure that the index files
are correct, and can fix a lot of problems caused by various types of
corruption.

This is important to ensure that restore operations can take place correctly
when corruption has occurred, and also may be required for some of the other
tools to work correctly, since they mostly rely on the index files only.

### Decrypt

This is mostly useful for debugging. It allows you to show the decrypted
contents of any backup, index or bundle file in a ZBackup repository.

```sh
rzbackup decrypt \
	--repository REPOSITORY \
	--password-file PASSWORD-FILE \
	--encrypted file ENCRYPTED-FILE
```
