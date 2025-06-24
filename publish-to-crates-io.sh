#!/bin/bash

# Exit on error and undefined variables
set -eu

echo -e "\n=== MANUAL STEPS REQUIRED ==="
echo "1. Ensure main branch is up-to-date"
echo "2. Update version and dependencies in Cargo.toml"
echo "3. If not bugfix. Update dependency version for rustemo in the tutorial calculator1."
echo "4. Check dependency versions in examples and docs by searching for 'rustemo.='."
echo "5. Update CHANGELOG.md"
echo -e "\nAfter completing these steps, press enter to continue..."
read

# Get version from Cargo.toml
VERSION=$(grep -m1 '^version' Cargo.toml | cut -d '"' -f2)

echo "Detected version: $VERSION"
read -p "Continue with this version? [y/N] " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "Aborting."
    exit 1
fi

# Create release branch
echo -e "\nCreating release branch..."
git checkout -b "release/$VERSION"

# Commit changes
echo -e "\nCommitting version/changelog updates..."
git commit -am "release $VERSION"

# Push branch
echo -e "\nPushing release branch..."
git push -u origin "release/$VERSION"

echo -e "\n=== ACTION REQUIRED ==="
echo "Wait for CI tests to complete on the release branch."
echo "Press enter when tests pass to continue..."
read

# Publish rustemo
echo -e "\nPublishing rustemo library..."
cargo publish --dry-run -p rustemo
cargo package --list -p rustemo
echo -e "\nCheck files above in target/package, then press enter to publish..."
read
cargo publish -p rustemo || {
    echo "Trying to login to crates.io"
    echo "You will be prompted for API token."
    echo "To create API token visit https://crates.io/me"
    cargo login
    cargo publish -p rustemo
}

# Publish rustemo-compiler
echo -e "\nPublishing rustemo-compiler..."
cargo publish --dry-run -p rustemo-compiler
cargo package --list -p rustemo-compiler
echo -e "\nCheck files above or in target/package, then press enter to publish..."
read
cargo publish -p rustemo-compiler

# Merge to main
echo -e "\nMerging to main..."
git checkout main
git merge "release/$VERSION"

# Create tag
echo -e "\nCreating signed tag..."
git tag -s "$VERSION"

# Push everything
echo -e "\nPushing changes..."
git push
git push origin "$VERSION"

echo -e "\n=== FINAL MANUAL STEP ==="
echo "Create a release on GitHub:"
echo "1. Go to repository releases page"
echo "2. Draft a new release for tag $VERSION"
echo "3. Use changelog content as description"
echo -e "\nPublishing complete!"
