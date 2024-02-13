{
  lib,
  rustPlatform,
  fetchFromGitHub,
  pkg-config,
  alsa-lib,
}:
rustPlatform.buildRustPackage {
  pname = "rat-rs";
  version = "unstable-2024-02-13";

  src = fetchFromGitHub {
    owner = "yavko";
    repo = "rat-rs";
    rev = "3a8d45aac074888287e3d1546db63ae5b29d198d";
    hash = "sha256-4/gAAsueIdJfPAmYz7O0z1cwUr7+8721yKwEhhvDI8s=";
  };

  cargoHash = "sha256-StO0wsjGuAW0xj3+K2Bnot98/VxtEyTUID2SGLGZKJo=";

  nativeBuildInputs = [
    pkg-config
    rustPlatform.bindgenHook
  ];

  buildInputs = [
    alsa-lib
  ];

  meta = {
    description = "Rust rewrite of https://github.com/thinkingsand/rat/, 0 unsafe code, 0 dependencies";
    homepage = "https://github.com/yavko/rat-rs";
    license = lib.licenses.gpl3Only;
    maintainers = with lib.maintainers; [NotAShelf];
    mainProgram = "rat-rs";
  };
}
