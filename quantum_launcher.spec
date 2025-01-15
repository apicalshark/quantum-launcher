Name:           quantum_launcher
Version:        0.3.1
Release:        %autorelease
Summary:        Simple Minecraft Launcher written in Rust

SourceLicense:  GPL-3.0-only
License:        GPLv3

URL:            https://mrmayman.github.io/quantumlauncher
Source:        {{{ git_dir_pack }}}

BuildRequires:  cargo-rpm-macros >= 24

%global _description %{expand:
A simple Minecraft Launcher written in Rust.}

%description %{_description}

%prep
{{{ git_dir_setup_macro }}}
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
%cargo_test --all-targets --all-features -- --show-output
%cargo_test
%endif

%files
%license LICENSE
%license LICENSE.dependencies
%doc README.md
%{_bindir}/quantum_launcher

%changelog
%autochangelog