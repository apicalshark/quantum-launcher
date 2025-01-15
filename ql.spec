# Package metadata
Name: quantum-launcher
Version: 0.3.1
Release: 1%{?dist}
Summary: A fast Minecraft launcher written in Rust
License: GPLv3
URL: https://mrmayman.github.io/quantumlauncher
Source0: https://github.com/Mrmayman/quantum-launcher/archive/refs/heads/main.zip
Buildrequires: rustup

# Description of the package
%description
A minimalistic Minecraft launcher for Windows and Linux written in Rust with the iced framework.

# Preparation step
%prep
%setup -q -n quantum-launcher-%{version}

# Build step
%build

# Initialize rustup and set the toolchain to nightly
rustup-init
rustup override set nightly

# Build the project in release mode
cd quantum-launcher-%{version} && cargo build --release

# Installation step
%install

# Check if the previous command was successful
if [ $? -ne 0 ]; then
    echo "An Error has occurred"
    exit 1
fi

# Create the directory for the binary and copy the built binary to the build root's bin directory
mkdir -p %{buildroot}/usr/bin
cp -dfp target/*/release/quantum_launcher %{buildroot}/usr/bin/quantum-launcher
chmod 755 %{buildroot}/usr/bin/quantum-launcher

# Clean up step
%clean
rm -rf %{buildroot}

# Files included in the package
%files
# Include the launcher binary in the package
/usr/bin/quantum-launcher
