# Todos

#### <code>package_ops.rs</code>

- [x] ```rust
  pub fn checkout(&mut self, pkg_name: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn submit(&mut self, path: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn submit_sol(&mut self, rb: bool, path: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_packages(&mut self) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_packagebuilds(&mut self) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_diff(&mut self) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_dependers(&mut self, pkg_name: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_dependencies(&mut self, pkg_name: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn get_extra_sources(&mut self) -> Result<(), std::io::Error>
  ```

#### <code>jobs.rs</code>

- [x] ```rust
  pub fn build(&mut self, rb: bool, pkg_name: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn cancel_job(&mut self, job_id: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn cancel_all_jobs(&mut self) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn clear_completed(&mut self) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn rebuild_dependers(&mut self, pkg_name: &str) -> Result<(), std::io::Error>
  ```

- [x] ```rust
  pub fn edit(&mut self, pkg_name: &str, editor: &str) -> Result<(), std::io::Error>
  ```

- [ ] ```rust
  pub fn export_all(&mut self)
  ```

- [ ] ```rust
  pub fn import_folder(&mut self, path: &str)
  ```

- [ ] ```rust
  pub fn submit_extra_source(&mut self, path: &str)
  ```

- [ ] ```rust
  pub fn remove_extra_source(&mut self, es_name: &str)
  ```

#### <code>info.rs</code>

- [ ] ```rust
  pub fn build_status(&mut self)
  ```

- [ ] ```rust
  pub fn client_status(&mut self)
  ```

- [ ] ```rust
  pub fn client_info(&mut self, client_name: &str)
  ```

- [ ] ```rust
  pub fn sys_log(&mut self)
  ```

- [ ] ```rust
  pub fn build_log(&mut self, job_id: &str)
  ```

#### <code>debugshell.rs</code>

- [x] ```rust
  pub fn debug_shell(&mut self) -> Result<(), std::io::Error>
  ```


