# This script is used by me(@sntdevco) for packaging releases
rm -rf tarball
mkdir tarball
mkdir target/tarball
cargo build --release
cp ./install-debian.sh ./tarball
cp ./target/release/dockinfo ./tarball
tar -cvf ./target/tarball/linux-amd64-bin.tar ./tarball
rm -rf ./tarball
