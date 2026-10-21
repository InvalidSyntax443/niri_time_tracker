{
  description = "Time tracker apps for niri";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };

        cargoToml = builtins.fromTOML (builtins.readFile ./Cargo.toml);
        pkg = cargoToml.package or { };

        # Не тащим target/ и .git/ в сборку.
        src = pkgs.lib.cleanSourceWith {
          src = self;
          filter =
            path: type:
            let
              s = toString path;
            in
            !(pkgs.lib.hasSuffix "/target" s || pkgs.lib.hasInfix "/target/" s)
            && !(pkgs.lib.hasSuffix "/.git" s || pkgs.lib.hasInfix "/.git/" s)
            && pkgs.lib.cleanSourceFilter path type;
        };

        myapp = pkgs.rustPlatform.buildRustPackage {
          pname = pkg.name or "niri_tracker";
          version = pkg.version or "0.1.0";

          inherit src;

          # Это hash зависимостей из Cargo.lock.
          # Сначала оставьте fakeHash, потом замените на реальный.
          cargoHash = pkgs.lib.fakeHash;

          # Зависимости, нужные для сборки/линовки.
          # Если у вас чистый Rust без C-зависимостей, можно убрать лишнее.
          nativeBuildInputs = with pkgs; [
            pkg-config
            sqlite
          ];

          buildInputs = with pkgs; [
            openssl
          ];

          # Запускать ли cargo test во время сборки.
          doCheck = true;

          meta = with pkgs.lib; {
            description = pkg.description or "My Rust app";
            homepage = pkg.homepage or "https://github.com/USER/REPO";
            license = licenses.mit; # поменяйте под свой license
            platforms = platforms.all;
            mainProgram = pkg.name or "myapp";
          };
        };
      in
      {
        packages = {
          default = myapp;
          inherit myapp;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = myapp;
        };

        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            rustc
            cargo
            clippy
            rustfmt
            pkg-config
            openssl
          ];

          shellHook = ''
            echo "Dev shell for ${myapp.pname}"
          '';
        };

        checks.myapp = myapp;
      }
    );
}
