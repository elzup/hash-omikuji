#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

function getBinaryName() {
  const platform = os.platform();
  const arch = os.arch();

  let binaryName = 'sha-omikuji';

  if (platform === 'win32') {
    binaryName += '.exe';
  }

  // Map to binary directory structure
  let platformDir;
  if (platform === 'darwin') {
    platformDir = arch === 'arm64' ? 'darwin-arm64' : 'darwin-x64';
  } else if (platform === 'linux') {
    platformDir = 'linux-x64';
  } else if (platform === 'win32') {
    platformDir = 'win32-x64';
  } else {
    console.error(`Unsupported platform: ${platform}`);
    process.exit(1);
  }

  return path.join(__dirname, 'bin', platformDir, binaryName);
}

function main() {
  const binaryPath = getBinaryName();

  // Check if binary exists
  if (!fs.existsSync(binaryPath)) {
    console.error(`Binary not found: ${binaryPath}`);
    console.error('');
    console.error('The pre-built binary for your platform is not available.');
    console.error('Please build from source:');
    console.error('  1. Install Rust: https://rustup.rs/');
    console.error('  2. Clone the repo and run: cargo build --release');
    process.exit(1);
  }

  // Pass all arguments to the binary
  const args = process.argv.slice(2);

  const child = spawn(binaryPath, args, {
    stdio: 'inherit',
    env: process.env,
  });

  child.on('error', (err) => {
    console.error(`Failed to start binary: ${err.message}`);
    process.exit(1);
  });

  child.on('exit', (code) => {
    process.exit(code || 0);
  });
}

main();
