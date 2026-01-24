//! ANTLR Grammar Validation
//!
//! This crate provides automated validation of the ANTLR grammar file against test corpus.
//! It is NOT used in production - only for development and CI.
//!
//! Requirements:
//! - Java (JRE) must be installed and in PATH
//! - Internet connection for downloading ANTLR jar (first run only)

#[cfg(test)]
mod tests {
    use std::env;
    use std::fs;
    use std::path::{Path, PathBuf};
    use std::process::Command;
    use walkdir::WalkDir;

    const ANTLR_VERSION: &str = "4.13.1";
    const ANTLR_JAR_URL: &str = "https://www.antlr.org/download/antlr-4.13.1-complete.jar";
    const GRAMMAR_FILE: &str = "grammar/hexyg.g4";
    const CORPUS_DIR: &str = "tests/corpus/valid";

    #[test]
    fn test_grammar_validity() -> anyhow::Result<()> {
        println!("=== ANTLR Grammar Validation ===");

        // 1. Check for Java
        check_java_installed()?;

        // 2. Setup paths
        let root_dir = find_project_root()?;
        let target_dir = root_dir.join("target");
        let cache_dir = target_dir.join("antlr_cache");
        let jar_path = cache_dir.join(format!("antlr-{}-complete.jar", ANTLR_VERSION));

        // 3. Download ANTLR if needed
        if !jar_path.exists() {
            println!("Downloading ANTLR jar to {:?}...", jar_path);
            fs::create_dir_all(&cache_dir)?;
            download_file(ANTLR_JAR_URL, &jar_path)?;
            println!("Downloaded successfully");
        } else {
            println!("Using cached ANTLR jar: {:?}", jar_path);
        }

        // 4. Generate Parser in a temporary directory
        let temp_dir = tempfile::tempdir()?;
        let gen_dir = temp_dir.path();

        let grammar_path = root_dir.join(GRAMMAR_FILE);
        if !grammar_path.exists() {
            anyhow::bail!("Grammar file not found at {:?}", grammar_path);
        }

        println!("Generating Java parser from {:?}...", grammar_path);
        let output = Command::new("java")
            .arg("-jar")
            .arg(&jar_path)
            .arg("-Dlanguage=Java")
            .arg("-o")
            .arg(gen_dir)
            .arg("-no-listener")
            .arg("-no-visitor")
            .arg(&grammar_path)
            .output()?;

        if !output.status.success() {
            eprintln!("ANTLR generation failed!");
            eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            anyhow::bail!("ANTLR generation failed");
        }
        println!("Parser generated successfully");

        // 5. Compile Java Parser
        println!("Compiling Java parser...");

        let java_files: Vec<PathBuf> = fs::read_dir(gen_dir)?
            .filter_map(|entry| entry.ok())
            .map(|e| e.path())
            .filter(|p| p.extension().map_or(false, |ext| ext == "java"))
            .collect();

        if java_files.is_empty() {
            anyhow::bail!("No Java files generated");
        }

        let output = Command::new("javac")
            .arg("-cp")
            .arg(&jar_path)
            .args(&java_files)
            .output()?;

        if !output.status.success() {
            eprintln!("Java compilation failed!");
            eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
            eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
            anyhow::bail!("Java compilation failed");
        }
        println!("Compilation successful");

        // 6. Run TestRig (Grun) on valid corpus files
        let corpus_path = root_dir.join(CORPUS_DIR);
        if !corpus_path.exists() {
            anyhow::bail!("Corpus directory not found: {:?}", corpus_path);
        }

        let mut test_count = 0;
        let mut failed = false;

        println!("\n=== Validating Corpus Files ===");

        for entry in WalkDir::new(&corpus_path) {
            let entry = entry?;
            if !entry.file_type().is_file() {
                continue;
            }

            // Only process .hex files
            if entry.path().extension().map_or(false, |ext| ext == "hex") {
                test_count += 1;
                print!("Validating {:?}... ", entry.path().file_name().unwrap());

                // Construct classpath (platform-dependent separator)
                let classpath = if cfg!(windows) {
                    format!(".;{}", jar_path.display())
                } else {
                    format!(".:{}", jar_path.display())
                };

                let output = Command::new("java")
                    .current_dir(gen_dir) // Run inside gen dir where .class files are
                    .arg("-cp")
                    .arg(&classpath)
                    .arg("org.antlr.v4.gui.TestRig")
                    .arg("hexyg") // Grammar name
                    .arg("hex_file") // Start rule
                    .arg("-diagnostics") // Show parse diagnostics
                    .stdin(std::fs::File::open(entry.path())?)
                    .output()?;

                if !output.status.success() {
                    println!("FAILED");
                    eprintln!("\nError parsing {:?}", entry.path());
                    eprintln!("STDOUT: {}", String::from_utf8_lossy(&output.stdout));
                    eprintln!("STDERR: {}", String::from_utf8_lossy(&output.stderr));
                    failed = true;
                } else {
                    // Check for parse errors in output
                    //let stdout = String::from_utf8_lossy(&output.stdout);
                    let stderr = String::from_utf8_lossy(&output.stderr);

                    // Check for actual parse errors (not just ambiguity warnings)
                    // Ambiguity warnings are acceptable, but syntax errors are not
                    if stderr.contains("token recognition error")
                        || stderr.contains("mismatched input")
                        || stderr.contains("no viable alternative")
                        || stderr.contains("missing")
                        || stderr.contains("extraneous input")
                    {
                        println!("FAILED");
                        eprintln!("\nParse errors in {:?}", entry.path());
                        eprintln!("STDERR: {}", stderr);
                        failed = true;
                    } else if stderr.contains("reportAmbiguity") || stderr.contains("reportAttemptingFullContext") {
                        // Ambiguity warnings - acceptable but inform user
                        println!("OK (with ambiguity warnings)");
                    } else {
                        println!("OK");
                    }
                }
            }
        }

        println!("\n=== Summary ===");
        println!("Files tested: {}", test_count);

        if test_count == 0 {
            anyhow::bail!("No .hex files found in corpus directory");
        }

        if failed {
            anyhow::bail!("Some corpus files failed validation");
        }

        println!("All tests passed!");
        Ok(())
    }

    fn check_java_installed() -> anyhow::Result<()> {
        let output = Command::new("java").arg("-version").output();

        match output {
            Ok(output) => {
                if output.status.success() {
                    let version = String::from_utf8_lossy(&output.stderr); // java -version outputs to stderr
                    println!("Java found: {}", version.lines().next().unwrap_or("unknown"));
                    Ok(())
                } else {
                    anyhow::bail!("Java command failed")
                }
            }
            Err(_) => anyhow::bail!(
                "Java is not installed or not in PATH. Required for grammar verification.\n\
                 Please install Java JRE and ensure 'java' command is available."
            ),
        }
    }

    fn find_project_root() -> anyhow::Result<PathBuf> {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR")?;
        let mut path = PathBuf::from(manifest_dir);

        // We are in crates/hexyg-grammar-check, so root is ../../
        if path.pop() && path.pop() {
            Ok(path)
        } else {
            anyhow::bail!("Could not find project root")
        }
    }

    fn download_file(url: &str, path: &Path) -> anyhow::Result<()> {
        println!("Downloading from {}...", url);
        let response = reqwest::blocking::get(url)?;

        if !response.status().is_success() {
            anyhow::bail!("Failed to download: HTTP {}", response.status());
        }

        let mut file = fs::File::create(path)?;
        let content = response.bytes()?;
        std::io::copy(&mut content.as_ref(), &mut file)?;

        println!("Download complete: {} bytes", content.len());
        Ok(())
    }
}
