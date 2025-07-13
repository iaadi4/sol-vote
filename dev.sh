#!/bin/bash

set -e

echo "🔧 Installing dependencies..."
yarn install

echo "🔨 Building the program..."
anchor build

echo "🧪 Running tests..."
anchor test

echo "✅ All done!"
