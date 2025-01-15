Name:           quantum_launcher
Version:        0.3.1
Release:        %autorelease
Summary:        Simple Minecraft Launcher written in Rust

SourceLicense:  GPL-3.0-only
License:        GPLv3
URL:            https://mrmayman.github.io/quantumlauncher
Source:        {{{ git_dir_pack }}}

BuildRequires:  rustup

%global _description %{expand:
A simple Minecraft Launcher written in Rust.}

%description %{_description}

%prep
{{{ git_dir_setup_macro }}}
rustup toolchain install nightly && rustup default nightly
cargo fetch

%build
cargo build --profile release

%install
install -Dm755 target/release/quantum_launcher %{buildroot}%{_bindir}/quantum_launcher

%files
%license LICENSE*
%doc README.md
%{_bindir}/quantum_launcher

%changelog
%autochangelog