#![allow(clippy::module_name_repetitions)]

use std::process::ExitCode;

use clap::{Parser, Subcommand};

/// kantei -- device compliance runner for Android.
#[derive(Parser)]
#[command(name = "kantei", about = "Run compliance profiles against Android devices")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Run compliance checks against a device.
    Check {
        /// Device serial number (as shown by `adb devices`).
        serial: String,
        /// Path to a custom YAML profile. If omitted, uses GrapheneOS Hardened.
        #[arg(long)]
        profile: Option<String>,
        /// ADB server host.
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// ADB server port.
        #[arg(long, default_value_t = 5037)]
        port: u16,
    },
    /// List available built-in profiles.
    ListProfiles,
    /// Generate a compliance report for a device.
    Report {
        /// Device serial number.
        serial: String,
        /// Output format.
        #[arg(long, default_value = "json")]
        format: ReportFormat,
        /// Path to a custom YAML profile.
        #[arg(long)]
        profile: Option<String>,
        /// ADB server host.
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        /// ADB server port.
        #[arg(long, default_value_t = 5037)]
        port: u16,
    },
}

#[derive(Clone, Debug, clap::ValueEnum)]
enum ReportFormat {
    Json,
    Text,
}

fn load_profile(path: Option<&str>) -> Result<kantei::ComplianceProfile, String> {
    match path {
        Some(p) => {
            let yaml = std::fs::read_to_string(p)
                .map_err(|e| format!("failed to read profile {p}: {e}"))?;
            kantei::ComplianceProfile::from_yaml(&yaml)
                .map_err(|e| format!("failed to parse profile {p}: {e}"))
        }
        None => Ok(kantei_android::grapheneos_profile()),
    }
}

fn run_check(serial: &str, profile_path: Option<&str>, host: &str, port: u16) -> ExitCode {
    let profile = match load_profile(profile_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let transport = kantei_android::AdbTransport::new(host, port, serial);
    let report = profile.report(&transport);

    println!(
        "Profile: {} v{}",
        report.profile_name,
        profile.meta.version
    );
    println!("Device:  {}", report.device_id);
    println!();

    for result in &report.results {
        let icon = if result.status == kantei::CheckStatus::Pass {
            "\x1b[32mPASS\x1b[0m"
        } else if result.status == kantei::CheckStatus::Fail {
            "\x1b[31mFAIL\x1b[0m"
        } else {
            "\x1b[33mERR \x1b[0m"
        };

        println!(
            "  [{icon}] {} ({:?}) -- {}",
            result.check_id, result.severity, result.title
        );
        println!("         {}", result.evidence);
    }

    println!();
    println!(
        "Total: {} | Passed: {} | Failed: {} | Errors: {}",
        report.total, report.passed, report.failed, report.errors
    );

    let crit = report.critical_failures();
    if !crit.is_empty() {
        println!(
            "\x1b[31mCritical failures: {}\x1b[0m",
            crit.len()
        );
    }

    println!("Hash: {}", report.compliance_hash);

    if report.is_compliant() {
        println!("\x1b[32mCompliant\x1b[0m");
        ExitCode::SUCCESS
    } else {
        println!("\x1b[31mNon-compliant\x1b[0m");
        ExitCode::FAILURE
    }
}

fn run_report(
    serial: &str,
    profile_path: Option<&str>,
    format: &ReportFormat,
    host: &str,
    port: u16,
) -> ExitCode {
    let profile = match load_profile(profile_path) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("Error: {e}");
            return ExitCode::FAILURE;
        }
    };

    let transport = kantei_android::AdbTransport::new(host, port, serial);
    let report = profile.report(&transport);

    match format {
        ReportFormat::Json => match report.to_json() {
            Ok(json) => {
                println!("{json}");
                if report.is_compliant() {
                    ExitCode::SUCCESS
                } else {
                    ExitCode::FAILURE
                }
            }
            Err(e) => {
                eprintln!("Error serializing report: {e}");
                ExitCode::FAILURE
            }
        },
        ReportFormat::Text => run_check(serial, profile_path, host, port),
    }
}

fn list_profiles() -> ExitCode {
    println!("Built-in profiles:");
    println!();
    println!("  grapheneos-hardened    GrapheneOS Hardened Device (6 checks)");
    println!("                        NIST 800-53: AC-3, SC-28, SI-2, CM-7");
    println!("                        CIS Android: 1.1");
    println!();
    println!("Use --profile <path> to load a custom YAML profile.");
    ExitCode::SUCCESS
}

fn main() -> ExitCode {
    let cli = Cli::parse();

    match cli.command {
        Command::Check {
            serial,
            profile,
            host,
            port,
        } => run_check(&serial, profile.as_deref(), &host, port),
        Command::ListProfiles => list_profiles(),
        Command::Report {
            serial,
            format,
            profile,
            host,
            port,
        } => run_report(&serial, profile.as_deref(), &format, &host, port),
    }
}
