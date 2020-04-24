# This Makefile builds packages for distribution
sp/target/debug/sp:
	cd sp; cargo build

release:
	cd sp; cargo build --release
clean:
	rm -rf sp/target

package-mac:
	echo "Building Packages for MacOS"; 