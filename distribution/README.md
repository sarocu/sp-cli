# Building Installer Packages
## MacOS
The packaging script for the SuperPlus CLI largely comes from: https://github.com/KosalaHerath/macos-installer-builder

To create packages:
```bash
make clean
make release

cd distribution
./build-macos-x64 sp 0.1.0
```

The build script takes the form of `build-macos-x64 [package name] [package version]`; it will request an Apple Developer Installer Certficate.