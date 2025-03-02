alias db := debug_build
alias rb := release_build
alias ob := cpu_optimized_build
alias ni := normal_install
alias oi := native_install

# debug build
debug_build:
  cargo build

# release build
release_build:
  cargo build --release

# release build with native cpu optimization
cpu_optimized_build:
  RUSTFLAGS="-C target-cpu=native" cargo build --release

# normal install with release build
normal_install:
  cargo install --path .

# optimize for native cpu and install
native_install:
  RUSTFLAGS="-C target-cpu=native" cargo install --path .
