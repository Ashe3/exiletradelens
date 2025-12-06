#!/usr/bin/env python3
"""
Patch script to disable MD5 check in torchfree_ocr library.
Run this after installing torchfree_ocr to prevent re-downloading models.
"""

import os
import sys
from pathlib import Path

def patch_torchfree_ocr():
    """Patch the torchfree_ocr library to disable buggy MD5 check."""
    
    # Find the torchfree_ocr installation
    try:
        import torchfree_ocr
        lib_path = Path(torchfree_ocr.__file__).parent / 'torchfree_ocr.py'
    except ImportError:
        print("ERROR: torchfree_ocr not installed")
        sys.exit(1)
    
    if not lib_path.exists():
        print(f"ERROR: Could not find {lib_path}")
        sys.exit(1)
    
    print(f"Patching {lib_path}...")
    
    # Read the file
    content = lib_path.read_text()
    
    # Check if already patched
    if "# Disabled MD5 check due to bug" in content:
        print("Already patched! No changes needed.")
        return
    
    # Find and replace the MD5 check block
    original = """        else: # check if all files are intact
            sums = []
            for filename in os.listdir(self.character_storage_directory):
                if filename.endswith(".txt"):
                    sums.append(calculate_md5(os.path.join(self.character_storage_directory, filename)))
            if sums != character_md5sum:
                shutil.rmtree(self.character_storage_directory)
                LOGGER.warning('Re-downloading character text files, please wait. '
                                'This may take several minutes depending upon your network connection.')
                download_and_unzip_all(character_folder['url'], MODULE_PATH, True)
                LOGGER.info('Download complete.')"""
    
    patched = """        # Disabled MD5 check due to bug with unsorted file list comparison
        # else: # check if all files are intact
        #     sums = []
        #     for filename in os.listdir(self.character_storage_directory):
        #         if filename.endswith(".txt"):
        #             sums.append(calculate_md5(os.path.join(self.character_storage_directory, filename)))
        #     if sums != character_md5sum:
        #         shutil.rmtree(self.character_storage_directory)
        #         LOGGER.warning('Re-downloading character text files, please wait. '
        #                         'This may take several minutes depending upon your network connection.')
        #         download_and_unzip_all(character_folder['url'], MODULE_PATH, True)
        #         LOGGER.info('Download complete.')"""
    
    if original not in content:
        print("WARNING: Could not find exact code block to patch.")
        print("The library might have been updated or already modified.")
        sys.exit(1)
    
    # Apply patch
    content = content.replace(original, patched)
    lib_path.write_text(content)
    
    print("✅ Successfully patched torchfree_ocr!")
    print("Models will no longer be re-downloaded on each run.")

if __name__ == "__main__":
    patch_torchfree_ocr()
