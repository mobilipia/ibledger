#!/usr/bin/env python
"""Setup Script for the Vega Python Integration Tests."""
import setuptools

INSTALL_REQUIRES = ["vega-launcher==0.1.3"]

PYTHON_REQUIRES = ">=3.6"

with open("README.md", "r") as readme:
    LONG_DESCRIPTION = readme.read()

setuptools.setup(
    name="vega-integration-tests",
    version="0.1.0",
    author="The Vega team",
    author_email="contact@vega.com",
    description="Vega Python Integration Tests",
    long_description=LONG_DESCRIPTION,
    long_description_content_type="text/markdown",
    url="https://github.com/vega/vega-python-client",
    packages=setuptools.find_packages(),
    install_requires=INSTALL_REQUIRES,
    python_requires=PYTHON_REQUIRES,
    classifiers=[
        "Programming Language :: Python :: 3",
        "License :: OSI Approved :: Apache Software License",
        "Operating System :: OS Independent",
        "Topic :: Security :: Cryptography",
    ],
)
