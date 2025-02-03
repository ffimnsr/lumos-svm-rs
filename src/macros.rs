
#[macro_export]
macro_rules! handle_tty_output {
  ($output:expr, $context:expr) => {
    let output = $output
      .take()
      .with_context(|| format!("Failed to open {}", stringify!($output)))?;
    let pb = $context.pb.clone();
    std::thread::spawn(move || {
      let reader = BufReader::new(output);
      for line in reader.lines().map_while(Result::ok) {
        let _ = pb.println(line);
      }
    });
  };
}

#[macro_export]
macro_rules! get_tty_output {
  ($verbose:expr) => {
    if $verbose {
      std::process::Stdio::piped()
    } else {
      std::process::Stdio::null()
    }
  };
}
