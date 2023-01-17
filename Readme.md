## Rranch (Rusty Branch)

This project is a rewrite in rust of the [AcaciaLinux branch client](https://github.com/AcaciaLinux/branch) just because why not. The code is a mess so good luck every trying to maintain it; All features should work as expected though.



## Commands

* **-h / --help:** Shows a handy help text for every possible cli option.

* **-ds / --debugshell:** Runs a debugshell on the remote server.

* **-c / --checkout [name]:** Checks out the specified packagebuild from the server.

* **-s / --submit [filename]:** Submits the specified packagebuild file to the server.

* **-rb / --releasebuild [name]:** Requests a releasebuild for the sepcified package.

* **-cb / --crossbuild [name]:** Requests a crossbuild for the sepcified package.

* **-vl / --viewlog [job_id]:** Requests build log of the specified completed job.

* **-st / --status:** Requests a list of running / completed / queued jobs.

* **-cs / --clientstatus:** Requests a list of clients connected to the server.

* **-cj / --clearjobs:** Clears the completed jobs from the server.

* **-caj / --cancelalljobs:** Cancels all currently queued jobs.

* **-cn / --canceljob [job_id]:** Cancels specified currently queued job.

* **-mp / --managedpkgs:** Requests list of managed packages.

* **-mk / --managedpkgbuilds:** Requests list of managed packagebuilds.

* **-dp / --differencepkgs:** Requests difference between packagebuilds and packages.

* **-sys / --viewsyslog:** Requests buildbot system logs from server.

* **-vt / --viewtree [name]:** Requests dependency tree for specified package.

* **-rd / --rebuilddependers [name]:** Rebuilds dependers of specified package.

* **-rbs / --releasebuildsol [sol_file]:** Submits a branch release solution to the server.

* **-cbs / --crossbuildsol [sol_file]:** Submits a branch cross solution to the server.