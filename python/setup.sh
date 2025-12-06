#!/bin/bash
# Setup script for Python environment

set -e

echo "Setting up Python environment..."

# Create virtual environment if it doesn't exist
if [ ! -d "venv" ]; then
    echo "Creating virtual environment..."
    python3 -m venv venv
fi

# Activate virtual environment
source venv/bin/activate

# Install dependencies
echo "Installing dependencies..."
pip install -r requirements.txt

# Apply patch to torchfree_ocr
echo "Applying patch to torchfree_ocr..."
python patch_torchfree_ocr.py

echo "✅ Setup complete!"
