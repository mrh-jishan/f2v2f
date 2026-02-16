#!/usr/bin/env python3
"""
Setup script for f2v2f Python bindings

This script helps build and install the f2v2f library with Python bindings.
"""

from setuptools import setup, find_packages
from pathlib import Path

# Read README
readme_path = Path(__file__).parent.parent.parent / "README.md"
long_description = readme_path.read_text() if readme_path.exists() else ""

setup(
    name="f2v2f",
    version="0.1.0",
    description="File to Video to File - Encode files as artistic videos and decode back",
    long_description=long_description,
    long_description_content_type="text/markdown",
    author="Your Name",
    author_email="your.email@example.com",
    url="https://github.com/yourusername/f2v2f",
    license="MIT",
    py_modules=["f2v2f"],
    python_requires=">=3.8",
    classifiers=[
        "Development Status :: 3 - Alpha",
        "Intended Audience :: Developers",
        "License :: OSI Approved :: MIT License",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
        "Programming Language :: Python :: 3.10",
        "Programming Language :: Python :: 3.11",
        "Topic :: Multimedia :: Video",
        "Topic :: Software Development :: Libraries :: Python Modules",
    ],
    zip_safe=False,
)
