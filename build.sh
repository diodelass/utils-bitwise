#!/bin/sh
echo "putting binary files in subdirectory 'bin' here."
mkdir -p bin

echo "building not..."
rustc not.rs -o bin/not

echo "building and..."
rustc and.rs -o bin/and

echo "building or..."
rustc or.rs -o bin/or

echo "building xor..."
rustc xor.rs -o bin/xor

echo "building nand..."
rustc nand.rs -o bin/nand

echo "building nor..."
rustc nor.rs -o bin/nor

echo "building xnor..."
rustc xnor.rs -o bin/xnor

echo "done."

