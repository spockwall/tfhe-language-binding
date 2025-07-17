from setuptools import setup, find_packages
from setuptools_rust import RustExtension, Binding

setup(
    name="fhe-language-binding",
    version="0.2.47",
    author="spockwall",
    author_email="e1e1e1n9n9n9@gmail.com",
    description="This is an test package for Python Fhe Http ",
    # long_description=open("README.md").read(),
    long_description_content_type="text/markdown",
    url="https://github.com/spockwall/fhe-language-binding",  # Replace with your project's URL
    packages=find_packages(),
    install_requires=[
        "ply>=3.11",
        "setuptools-rust>=1.9.0",
        "setuptools>=68.2.2",
        "wheel>=0.43.0",
    ],
    rust_extensions=[
        RustExtension(
            "fhe_http.fhe",
            "Cargo.toml",
            binding=Binding.PyO3,
        ),
    ],
    classifiers=[
        "Programming Language :: Python :: 3.10",
        "License :: OSI Approved :: MIT License",
        "Operating System :: OS Independent",
    ],
    # use_scm_version=True,  # Use setuptools_scm for versioning
)
