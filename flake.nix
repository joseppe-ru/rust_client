{
  description = "Home Assistant Rust TUI & Daemon";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
  };

  outputs = { self, nixpkgs }:
    let
      system = "x86_64-linux";
      pkgs = nixpkgs.legacyPackages.${system};
    in
    {
      packages.${system}.default = pkgs.rustPlatform.buildRustPackage {
        pname = "ha-tui";
        version = "0.1.0";
        src = ./ha_tui;

        cargoLock = {
          lockFile = ./ha_tui/Cargo.lock;
        };

        postInstall = ''
          mv $out/bin/tui $out/bin/ha-tui
        '';
      };

      # Das fertige Modul für Home Manager
      homeManagerModules.default = { config, lib, pkgs, ... }:
        let
          cfg = config.services.ha-tui;
          pkg = self.packages.${pkgs.stdenv.hostPlatform.system}.default;
        in
        {
          options.services.ha-tui.enable = lib.mkEnableOption "Home Assistant Rust TUI & Daemon";

          config = lib.mkIf cfg.enable {
            # Macht 'ha-tui' im PATH verfügbar
            home.packages = [ pkg ];

            # Erstellt und startet den User-Service automatisch
            systemd.user.services.rust_client = {
              Unit = {
                Description = "Home Assistant Rust Client Daemon";
                After = [ "network-online.target" ];
              };
              Service = {
                ExecStart = "${pkg}/bin/daemon";
                Restart = "on-failure";
                RestartSec = "5s";
              };
              Install = {
                WantedBy = [ "default.target" ];
              };
            };
          };
        };
    };
}
