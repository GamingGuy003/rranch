### Untrusted

<code>AUTH</code> Authenticates Client

```rust
pub fn auth(&mut self) -> Result<(), std::io::Error>
```

* Value:
  
  * <code>AUTHKEY</code>

* Returns:
  
  * <code>UNTRUSTED_MODE</code>
  
  * <code>ALREADY_AUTHENTICATED</code>
  
  * <code>AUTH_OK</code>
  
  * <code>INV_AUTH_KEY</code>

---

### Trusted

<code>SET_MACHINE_TYPE</code>

```rust
pub fn set_type(&mut self) -> Result<(), std::io::Error>
```

* Value:
  
  * <code>CONTROLLER</code>
  
  * <code>BUILD</code>

* Returns:
  
  * <code>CMD_OK</code>
  
  * <code>INV_MACHINE_TYPE</code>
  
  * <code>AUTH_REQUIRED</code>

---

<code>SET_MACHINE_NAME</code> 

```rust
pub fn set_name(&mut self) -> Result<(), std::io::Error>
```

* Value:
  
  * <code>NAME</code>

* Returns:
  
  * <code>CMD_OK</code>

---

<code>CHECKOUT_PACKAGE</code>

```rust
pub fn checkout(&mut self, pkgname: &str) -> Result<(), std::io::Error>
```

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>INV_PKG_NAME</code>
  
  - <code>{JSON:PKGBUILD}</code>

---

<code>SUBMIT_PACKAGE</code>

- Value:
  
  - <code>{JSON:PKGBUILD}</code>

- Returns:
  
  - <code>INV_PKG_BUILD</code>
  
  - <code>CMD_OK</code>

---

<code>RELEASE_BUILD</code>

```rust
pub fn build(&mut self, rb: bool, pkg: &str) -> Result<(), std::io::Error>
```

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>RELEASE_ENV_UNAVAILABLE</code>
  
  - <code>INV_PKG_NAME</code>
  
  - <code>BUILD_REQ_SUBMIT_IMMEDIATELY</code>
  
  - <code>BUILD_REQ_QUEUED</code>
  
  - <code>PKG_BUILD_DAMAGED</code>

---

<code>CROSS_BUILD</code>

```rust
pub fn build(&mut self, rb: bool, pkg: &str) -> Result<(), std::io::Error>
```

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>CROSS_ENV_UNAVAILABLE</code>
  
  - <code>INV_PKG_NAME</code>
  
  - <code>BUILD_REQ_SUBMIT_IMMEDIATELY</code>
  
  - <code>BUILD_REQ_QUEUED</code>
  
  - <code>PKG_BUILD_DAMAGED</code>

---

<code>VIEW_LOG</code>

- Value:
  
  - <code>JOB_ID</code>

- Returns:
  
  - <code>INV_JOB_ID</code>
  
  - <code>NO_LOG</code>
  
  - <code>{JSON:LOG}</code>

---

<code>VIEW_SYS_EVENTS</code>

- Value:

- Returns:
  
  - <code>{JSON:EVENTS}</code>

---

<code>GET_DEPENDERS</code>

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>INV_PKG_NAME</code>
  
  - <code>{JSON:DEPENDERS}</code>

---

<code>REBUILD_DEPENDERS</code>

```rust
pub fn rebuild_dependers(&mut self, pkg: &str) -> Result<(), std::io::Error>
```

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>INV_PKG_NAME</code>
  
  - <code>RELEASE_ENV_UNAVAILABLE</code>
  
  - <code>CROSS_ENV_UNAVAILABLE</code>
  
  - <code>CMD_OK</code>

---

<code>COMPLETED_JOBS_STATUS</code>

- Value:

- Returns:
  
  - <code>{JSON:[JOB]}</code>

---

<code>RUNNING_JOBS_STATUS</code>

- Value:

- Returns:
  
  - <code>{JSON:[JOB]}</code>

---

<code>QUEUED_JOBS_STATUS</code>

- Value:

- Returns:
  
  - <code>{JSON:[JOB]}</code>

---

<code>CONNECTED_CONTROLLERS</code>

- Value:

- Returns:
  
  - <code>{JSON:CONNECTED_CONTROLLERS}</code>

---

<code>CONNECTED_BUILDBOTS</code>

- Value:

- Returns:
  
  - <code>{JSON:CONNECTED_BUILDBOTS}</code>

---

<code>MANAGED_PACKAGES</code>

- Value:

- Returns:
  
  - <code>{JSON:MANAGED_PACKAGES}</code>

---

<code>MANAGED_PKGBUILDS</code>

- Value:

- Returns:
  
  - <code>{JSON:MANAGED_PKGBUILDS}</code>

---

<code>CLEAR_COMPLETED_JOBS</code>

```rust
pub fn clear_completed(&mut self) -> Result<(), std::io::Error>
```

- Value:

- Returns:
  
  - <code>JOBS_CLEARED</code>

---

<code>CANCEL_QUEUED_JOB</code>

```rust
pub fn cancel_job(&mut self, job_id: &str) -> Result<(), std::io::Error>
```

- Value:
  
  - <code>JOB_ID</code>

- Returns:
  
  - <code>INV_JOB_ID</code>
  
  - <code>JOB_CANCELED</code>

---

<code>CANCEL_ALL_QUEUED_JOBS</code>

```rust
pub fn cancel_all_jobs(&mut self) -> Result<(), std::io::Error>
```

- Value:

- Returns:
  
  - <code>JOBS_CANCELED</code>

---

<code>SUBMIT_SOLUTION_RB</code>

- Value:
  
  - <code>{SOLUTION}</code>

- Returns:
  
  - <code>RELEASE_ENV_UNAVAILABLE</code>
  
  - <code>INV_SOL</code>
  
  - <code>PKG_BUILD_MISSING {NAME}</code>
  
  - <code>BATCH_QUEUED</code>

---

<code>SUBMIT_SOLUTION_CB</code>

- Value:
  
  - <code>{SOLUTION}</code>

- Returns:
  
  - <code>CROSS_ENV_UNAVAILABLE</code>
  
  - <code>INV_SOL</code>
  
  - <code>PKG_BUILD_MISSING {NAME}</code>
  
  - <code>BATCH_QUEUED</code>

---

<code>GET_CLIENT_INFO</code>

- Value:
  
  - <code>CLIENT_NAME</code>

- Returns:
  
  - <code>INV_CLIENT_NAME</code>
  
  - <code>{JSON:CLIENT_INFO}</code>

---

<code>GET_LOCKED_PACKAGES</code>

- Value:

- Returns:
  
  - <code>{JSON:PACKAGES}</code>

---

<code>DELETE_PKGBUILD</code>

- Value:
  
  - <code>PKG_NAME</code>

- Returns:
  
  - <code>INV_CMD</code>
  
  - <code>INV_PKG_NAME</code>
  
  - <code>REQUIRED_PKG</code>
  
  - <code>CMD_OK</code>

---

<code>GET_MANAGED_EXTRA_SOURCES</code>

- Value:

- Returns:
  
  - <code>{JSON:[EXTRA_SOURCE]}</code>

---

<code>REMOVE_EXTRA_SOURCE</code>

- Value:
  
  - <code>ES_NAME</code>

- Returns:
  
  - <code>INV_ES_NAME</code>
  
  - <code>CMD_OK</code>

---

<code>TRANSFER_EXTRA_SOURCE</code>

* Value:
  
  * <code>{JSON:EXTRA_SOURCE}</code>

* Returns:
  
  * <code>BYTE_COUNT_ERR</code>
  
  * <code>CMD_OK</code>

---

<code>COMPLETE_TRANSFER</code>

* Value:

* Returns:
  
  * <code>ERR_COULD_NOT_INSERT</code>
  
  * <code>CMD_OK</code>

---

### Buildbot Only

<code>SET_MACHINE_INFORMATION</code>

- Value:

- Returns:

---

<code>SIG_READY</code>

- Value:

- Returns:

---

<code>PONG</code>

- Value:

- Returns:

---

<code>GET_DEPLOYMENT_CONFIG</code>

- Value:

- Returns:

---

<code>REPORT_STATUS_UPDATE</code>

- Value:

- Returns:

---

<code>REPORT_STATUS_UPDATE</code>

- Value:

- Returns:

---

<code>SUBMIT_LOG</code>

- Value:

- Returns:

---

<code>FILE_TRANSFER_MODE</code>

- Value:

- Returns:

---

<code>REPORT_SYS_EVENTS</code>

- Value:

- Returns:

---

<code>EXTRA_SOURCE_INFO</code>

- Value:

- Returns:

---

<code>FETCH_EXTRA_SOURCE</code>

- Value:

- Returns:

---

### Other

<code>!INVALID</code>

- Value:

- Returns:

---

### JSON

<code>Extra Source</code>

```rust
struct ExtraSource {
    id: String,
    filename: String,
    description: String,
}
```

---

<code>Jobs</code>

```rust

```
