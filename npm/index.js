#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');

const PLATFORMS = {
  'darwin-arm64': '@elzup/hash-omikuji-darwin-arm64',
  'darwin-x64': '@elzup/hash-omikuji-darwin-x64',
  'linux-x64': '@elzup/hash-omikuji-linux-x64',
  'win32-x64': '@elzup/hash-omikuji-win32-x64',
};

function getBinaryPath() {
  const platform = os.platform();
  const arch = os.arch();
  const platformKey = `${platform}-${arch === 'arm64' ? 'arm64' : 'x64'}`;

  const packageName = PLATFORMS[platformKey];
  if (!packageName) {
    console.error(`Unsupported platform: ${platform}-${arch}`);
    process.exit(1);
  }

  const binaryName = platform === 'win32' ? 'hash-omikuji.exe' : 'hash-omikuji';

  try {
    const packagePath = require.resolve(`${packageName}/package.json`);
    return path.join(path.dirname(packagePath), binaryName);
  } catch (e) {
    console.error(`Platform package not found: ${packageName}`);
    console.error('');
    console.error('The pre-built binary for your platform is not available.');
    console.error('Please build from source:');
    console.error('  1. Install Rust: https://rustup.rs/');
    console.error('  2. Clone the repo and run: cargo build --release');
    process.exit(1);
  }
}

function main() {
  const binaryPath = getBinaryPath();
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
