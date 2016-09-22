# RZBackup

https://github.com/wellbehavedsoftware/wbs-backup

James Pharaoh <james@wellbehavedsoftware.com>

This is a partial Rust clone of [ZBackup](http://zbackup.org/), along with some
unique features of its own.

This project is free software and available under the [Apache 2.0 licence]
(https://www.apache.org/licenses/LICENSE-2.0).

Binaries for ubuntu are available can be downloaded [here]
(https://dist.wellbehavedsoftware.com/rzbackup/).

## List of features

* Rust library for access to ZBackup repositories
* Supports encrypted and non encrypted formats
* RandomAccess implements Read and Seek to provide efficient random access
* Client/server utilities to efficiently restore multiple backups, sharing
  chunk cache
* Multi-threaded restore
* Command line decrypt utility, mostly useful for debugging

## Library usage

In cargo.toml:

```toml
[dependencies]
rzbackup = '1.0'
```

Example code, for demonstration (won't compile):

```rust
extern crate rzbackup;

use rzbackup::Repository;
use rzbackup::RandomAccess;

fn main () {

	let mut repository =
		Repository::open (
			"/path/to/repository",
			Some ("/path/to/password/file"));

	repository.restore (
		"/backup-name",
		output_stream ());

	let mut random_access =
		RandomAccess::new (
			repository,
			"/backup-name");

	do_something_with_random_access (
		random_access);

}
```

## Command usage

### Server

The server process listens for client connections and streams backups over a
socket. It has a large cache and so will be more efficient than running separate
restore processes for backed up data with lots of shared deduplicated content.

```sh
rzbackup-server LISTEN-ADDRESS:PORT REPOSITORY [PASSWORD-FILE]
```

### Client

The client connects to the server and streams a backup to standard output. It
can also tell the server to reload its indexes, which will be necessary if new
backups have been made.

```sh
rzbackup-client reindex SERVER-ADDRESS:PORT
rzbackup-client restore SERVER-ADDRESS:PORT BACKUP-NAME > OUTPUT-FILE
```

### Restore

The restore command is able to perform a one-off restore. It is basically
equivalent to ZBackup's own `restore` command.

```sh
rzbackup-restore REPOSITORY PASSWORD-FILE BACKUP > OUTPUT-FILE
rzbackup-restore REPOSITORY '' BACKUP > OUTPUT-FILE
```

### Decrypt

This is mostly useful for debugging. It allows you to show the decrypted
contents of any backup, index or bundle file in a ZBackup repository.

```sh
rzbackup-decrypt REPOSITORY PASSWORD-FILE ENCRYPTED-FILE > OUTPUT-FILE
```
