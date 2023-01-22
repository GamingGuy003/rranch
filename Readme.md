## Rranch (Rusty Branch)

This project is an improved rewrite of the [AcaciaLinux branch client](https://github.com/AcaciaLinux/branch) in rust because why not and for a few QOL improvements. The code is a mess so good luck every trying to maintain it. All features might work as expected though i do not know.



## Commands

* **-h / --help:** Shows a handy help text for every possible cli option.

* **-ds / --debugshell:** Runs a debugshell on the remote server.

* **-c / --checkout [name]:** Checks out the specified packagebuild from the server.

* **-e / --edit [name]:** Checks out packagebuild, opens it in a editor and submits the changed packagebuild.

* **-t / --template:** Creates a template packagbuild.

* **-s / --submit [filename]:** Submits the specified packagebuild file to the server.

* **-rb / --releasebuild [name]:** Requests a releasebuild for the sepcified package.

* **-cb / --crossbuild [name]:** Requests a crossbuild for the sepcified package.

* **-vl / --viewlog [job_id]:** Requests build log of the specified completed job.

* **-vll / --viewlastlog:** Requests the log of the last job that completed.

* **-st / --status:** Requests a list of running / completed / queued jobs.

* **-w / --watch [interval]:** Watches the joblist in the given interval in seconds.

* **-cs / --clientstatus:** Requests a list of clients connected to the server.

* **-cj / --clearjobs:** Clears the completed jobs from the server.

* **-caj / --cancelalljobs:** Cancels all currently queued jobs.

* **-cn / --canceljob [job_id]:** Cancels specified currently queued job.

* **-mp / --managedpkgs:** Requests list of managed packages.

* **-mk / --managedpkgbuilds:** Requests list of managed packagebuilds.

* **-dp / --differencepkgs:** Requests difference between packagebuilds and packages.

* **-sys / --viewsyslog:** Requests buildbot system logs from server.

* **-vd / --viewdependers [name]:** Requests all dependers for specified package..

* **-vdp / --viewdependencies [name]:** Shows if dependencies of a package have a pkgbuild / pkg.

* **-rd / --rebuilddependers [name]:** Rebuilds dependers of specified package.

* **-rbs / --releasebuildsol [sol_file]:** Submits a branch release solution to the server.

* **-cbs / --crossbuildsol [sol_file]:** Submits a branch cross solution to the server.

## Config

The default config (rranch.toml) should look similar to this:

```toml
#master config
[master]
addr = "localhost"
port = 27015
authkey = "key"

#client config
[client]
name = "client-name"
#an optional value you can omit if you just want to connect as controller
type = "CONTROLLER"
#info / debug / trace / none
loglevel = "trace"
#the editor to use for the edit command
editor = "vim"
```
