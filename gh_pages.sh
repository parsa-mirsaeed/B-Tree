rm -rf docs/*

dx build --release

cp -r target/dx/b-tree/release/web/public/* docs/