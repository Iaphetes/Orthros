{
  description = "Flake for Orthros-BE";

  # Flake inputs
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay"; # A helper for Rust + Nix
    nixgl.url = "github:guibou/nixGL"; # Allows you to run OpenGL and or Vulkan applications in a nix shell
  };

  # Flake outputs
  outputs = { self, nixpkgs, rust-overlay, nixgl, ... }:
    let
      # Overlays enable you to customize the Nixpkgs attribute set
      overlays = [
        # Makes a `rust-bin` attribute available in Nixpkgs
        (import rust-overlay)
        nixgl.overlay
        # Provides a `rustToolchain` attribute for Nixpkgs that we can use to
        # create a Rust environment
        (self: super: {
          rustToolchain = super.rust-bin.nightly.latest.default;
        })
      ];

      # Systems supported
      allSystems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      forAllSystems = f: nixpkgs.lib.genAttrs allSystems (system: f {
        pkgs = import nixpkgs { inherit overlays system; };
      });
    in
    {
      # Development environment output
      devShells = forAllSystems ({ pkgs }: {
        default =  pkgs.mkShell {
          # The Nix packages provided in the environment
          packages = (with pkgs; [
             # Fluff
             cargo-mommy
             onefetch
             # Bevy
             pkg-config
             alsa-lib
             vulkan-tools
             vulkan-headers
             vulkan-loader
             vulkan-validation-layers
             udev
             clang
             lld
             # If using an intel GPU
             pkgs.nixgl.nixVulkanIntel
             # If on x11
             xorg.libX11
             xorg.libXcursor
             xorg.libXi
             xorg.libXrandr
             # If on wayland
             libxkbcommon
             wayland
            # Rust
            rustup
            rustToolchain
          ]) ++ pkgs.lib.optionals pkgs.stdenv.isDarwin (with pkgs; [ libiconv ]);
          shellHook = ''
            # Required
            export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.lib.makeLibraryPath [
              pkgs.alsaLib
              pkgs.udev
              pkgs.vulkan-loader
              pkgs.libxkbcommon
              pkgs.libllvm
              pkgs.libdrm
              pkgs.libelf
              pkgs.elfutils
              pkgs.xorg.libxcb
              pkgs.zstd
             # If using an intel GPU
             pkgs.nixgl.nixVulkanIntel
              pkgs.amdvlk
             # If on x11
             pkgs.xorg.libX11
             pkgs.xorg.libXcursor
             pkgs.xorg.libXi
             pkgs.xorg.libXrandr
            pkgs.xorg.libxshmfence
            pkgs.amdvlk
            pkgs.xorg.xcbutilkeysyms
             pkgs.vulkan-tools
             pkgs.vulkan-headers
             pkgs.vulkan-loader
             pkgs.vulkan-validation-layers
            pkgs.lld
            ]}"
            # Aliases and other fluff/ease of use
            alias runIntel="nixVulkanIntel cargo run"
            onefetch
            echo "Welcome to nix-hell uh nix-shell!"
          '';
        };
      });
    };
}
