# Generated by rust2rpm 27
%bcond check 1

Name:           quantum_launcher
Version:        0.3.1
Release:        %autorelease
Summary:        Simple Minecraft Launcher written in Rust

SourceLicense:  GPL-3.0-only
# FIXME: paste output of %%cargo_license_summary here
License:        GPLv3
# LICENSE.dependencies contains a full license breakdown

URL:            https://mrmayman.github.io/quantumlauncher
Source:         https://github.com/apicalshark/quantum-launcher/archive/refs/tags/v0.3.1.tar.gz     

BuildRequires:  cargo-rpm-macros >= 24

%global _description %{expand:
A simple Minecraft Launcher written in Rust.}

%description %{_description}

%prep
%autosetup -c -n quantum_launcher-%{version}
%cargo_prep

%generate_buildrequires
%cargo_generate_buildrequires

%build
%cargo_build
%{cargo_license_summary}
%{cargo_license} > LICENSE.dependencies

%install
%cargo_install

%if %{with check}
%check
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/quantum_launcher

%changelog
%autochangelog
