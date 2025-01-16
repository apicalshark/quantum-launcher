Name:           quantum-launcher
Version:        0.3.1
Release:        1%{?dist}
Summary:        Simple Minecraft Launcher written in Rust

License:        GPLv3
URL:            https://mrmayman.github.io/quantumlauncher
Source:        {{{ git_dir_pack }}}

BuildRequires:  rust cargo perl

%global _description %{expand:
A simple Minecraft Launcher written in Rust.}

%description %{_description}

%prep
{{{ git_dir_setup_macro }}}
cargo fetch

%build
cargo build --profile release

%install
install -Dm755 target/release/quantum_launcher %{buildroot}%{_bindir}/quantum-launcher
cp -pdf assets/freedesktop/quantum-launcher.desktop %{buildroot}/usr/share/applications/quantum-launcher.desktop
chmod 644 %{buildroot}/usr/share/applications/quantum-launcher.desktop
cp -pdf assets/icon/ql_logo.png %{buildroot}/usr/share/pixmaps/ql_logo.png
chmod 644 %{buildroot}/usr/share/pixmaps/ql_logo.png

%postun
case '"$1"' in
    0)
        # Post uninstall
        rm -f %{buildroot}/usr/share/pixmaps/ql_logo.png
        rm -f %{buildroot}/usr/share/applications/quantum-launcher.desktop
        
    ;;
    1)  
        # Post upgrade
        :
    ;;

%files
%license LICENSE
%doc README.md
%{_bindir}/quantum-launcher
/usr/share/applications/quantum-launcher.desktop
/usr/share/pixmaps/ql_logo.png

%changelog
%autochangelog