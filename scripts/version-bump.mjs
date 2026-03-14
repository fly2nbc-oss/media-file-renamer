import fs from "node:fs";
import { execSync } from "node:child_process";

const VALID_MODES = new Set(["major", "minor", "patch", "auto"]);
const mode = process.argv[2];
const dryRun = process.argv.includes("--dry-run");

if (!VALID_MODES.has(mode)) {
  console.error("Usage: node scripts/version-bump.mjs <major|minor|patch|auto>");
  process.exit(1);
}

const pkgPath = "package.json";
const lockPath = "package-lock.json";
const cargoPath = "src-tauri/Cargo.toml";
const tauriConfigPath = "src-tauri/tauri.conf.json";

const pkg = readJson(pkgPath);
const currentVersion = pkg.version;
assertSemver(currentVersion);

const effectiveMode = mode === "auto" ? deriveModeFromGit() : mode;
const nextVersion = bumpSemver(currentVersion, effectiveMode);

if (nextVersion === currentVersion) {
  console.log(`No version change needed (${currentVersion}).`);
  process.exit(0);
}

if (dryRun) {
  console.log(`[dry-run] Version bump (${effectiveMode}): ${currentVersion} -> ${nextVersion}`);
  process.exit(0);
}

pkg.version = nextVersion;
writeJson(pkgPath, pkg);

if (fs.existsSync(lockPath)) {
  const lock = readJson(lockPath);
  lock.version = nextVersion;
  if (lock.packages && lock.packages[""]) {
    lock.packages[""].version = nextVersion;
  }
  writeJson(lockPath, lock);
}

updateCargoPackageVersion(cargoPath, nextVersion);

const tauriConf = readJson(tauriConfigPath);
tauriConf.version = nextVersion;
writeJson(tauriConfigPath, tauriConf);

console.log(`Version bumped (${effectiveMode}): ${currentVersion} -> ${nextVersion}`);

function deriveModeFromGit() {
  const latestTag = getLatestTag();
  const range = latestTag ? `${latestTag}..HEAD` : "HEAD";
  const log = runGit(`git log ${range} --format=%s%n%b%n----`);
  const text = log.trim();

  if (!text) {
    throw new Error(
      "No commits found to derive version bump. Use explicit mode (major|minor|patch)."
    );
  }

  if (hasBreakingChange(text)) return "major";
  if (hasFeatureCommit(text)) return "minor";
  return "patch";
}

function hasBreakingChange(text) {
  if (/BREAKING CHANGE:/i.test(text)) return true;
  const lines = text.split("\n");
  return lines.some((line) => /^[a-z]+(\(.+\))?!:/i.test(line));
}

function hasFeatureCommit(text) {
  const lines = text.split("\n");
  return lines.some((line) => /^feat(\(.+\))?:/i.test(line));
}

function bumpSemver(version, kind) {
  const [major, minor, patch] = version.split(".").map((n) => Number.parseInt(n, 10));
  if (kind === "major") return `${major + 1}.0.0`;
  if (kind === "minor") return `${major}.${minor + 1}.0`;
  return `${major}.${minor}.${patch + 1}`;
}

function assertSemver(version) {
  if (!/^\d+\.\d+\.\d+$/.test(version)) {
    throw new Error(`Unsupported version format: ${version}. Expected MAJOR.MINOR.PATCH.`);
  }
}

function readJson(path) {
  return JSON.parse(fs.readFileSync(path, "utf8"));
}

function writeJson(path, value) {
  fs.writeFileSync(path, `${JSON.stringify(value, null, 2)}\n`);
}

function updateCargoPackageVersion(path, version) {
  const content = fs.readFileSync(path, "utf8");
  const lines = content.split("\n");
  let inPackage = false;
  let updated = false;

  for (let i = 0; i < lines.length; i += 1) {
    const line = lines[i];
    if (/^\[package\]\s*$/.test(line)) {
      inPackage = true;
      continue;
    }
    if (/^\[.+\]\s*$/.test(line)) {
      inPackage = false;
    }
    if (inPackage && /^\s*version\s*=\s*".*"\s*$/.test(line)) {
      lines[i] = `version = "${version}"`;
      updated = true;
      break;
    }
  }

  if (!updated) {
    throw new Error(`Could not update package version in ${path}`);
  }

  fs.writeFileSync(path, lines.join("\n"));
}

function getLatestTag() {
  try {
    return runGit("git describe --tags --abbrev=0").trim();
  } catch {
    return "";
  }
}

function runGit(command) {
  return execSync(command, {
    encoding: "utf8",
    stdio: ["ignore", "pipe", "pipe"],
  });
}
