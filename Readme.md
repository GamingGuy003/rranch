## Rranch (Rusty Branch)

This project is an improved rewrite of the [AcaciaLinux branch client](https://github.com/AcaciaLinux/branch) in rust because why not and for a few QOL improvements. The code is a mess so good luck every trying to maintain it. All features might work as expected though I do not know.

## Commands

* **-h / --help** Displays help

* **-c / --checkout [name]** Fetches pkgbuild

* **-s / --submit [path]** Submits pkgbuild

* **-rb / --releasebuild [name]** Releasebuilds pkg

* **-cb / --crossbuild [name]** Crossbuilds pkg

* **-jl / --job-log [job_id]** Log for job

* **-sl / --sys-log** Fetches syslog

* **-depds / --dependers [name]** Dependers

* **-deps / --dependencies [name]** Dependencies

* **-rd / --rebuilddependers [name]** Rebuild dependers

* **-js / --job-status** Shows jobs

* **-wj / --watch-jobs [interval]** Periodic jobstatus

* **-ll / --latest-log** Latest job log

* **-cs / --client-status** Shows active clients

* **-ci / --client-info [name]** Shows client info

* **-mpkg / --managed-pkgs** Shows pkg status

* **-mpkgbs / --managed pkgbs** Shows pkgb status

* **-d / --diff** pkgs / pkgbs diff

* **-cc / --cancel-completed** Clear completed jobs

* **-cq / cancel-queued [job_id]** Cancels queued job

* **-caq / --cancel-all-queued** Cancels all queued jobs

* **-ssr / --submit-solution-release [path]** Submits release solution

* **-ssc / --submit-solution-cross [path]** Submits cross solution

* **-e / --edit [name]** Opens pkgb with editor

* **-rm / --remove-pkg** Removes pkg

* **-es / --extrasources** SHows extrasources

* **-res / --remove-extrasource [es_id]** Removes extrasource

* **-ses / --submit-extrasource [path]** Submits extrasource

* **-ex / --export** Exports all pkgbs

* **-im / --import [path]** Imports all pkgbs

* **-cf / --configure** Configures client

## Install

On Unix Systems you can cd into the local repo and run <code>make all</code> or <code>make redeploy</code> to build a release binary and install it to /usr/bin. On other Systems you might have to do the two steps manually.

## Config

The default config (~/.config/rranch.toml) which rranch generates on first startup should look similar to this:

```toml
[master]
# api server
addr = "localhost"
port = 27015
# api authkey
authkey = "default"

[client]
# clientname
name = "a-rranch-client"
# clienttype
type = "CONTROLLER"
# loglevel (INFO | DEBUG | TRACE | NONE)
loglevel = "INFO"
# editor for -cf | -e
editor = "vim"
# protocol version (should not be changed)
protver = 0
```
