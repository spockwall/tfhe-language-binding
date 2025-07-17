# Fhe-Language-Binding SetUp

### Description

- This setup script integrates py_fhe_http and assembler into a packages and upload the package to [PyPI](https://pypi.org/) and [TestPyPI](https://test.pypi.org/).
- For the uploaded package on TestPyPI, please check: [fhe-language-binding](https://test.pypi.org/project/fhe-language-binding/)
- For the uploaded package on PyPI, please check: [fhe-language-binding](https://pypi.org/project/fhe-language-binding/)

### How to build package and upload to PyPI

```shellscript=
$ cd fhe-language-binding/fhe_python
$ python setup.py sdist bdist_wheel
$ twine check dist/*

// upload to test pypi
$ twine upload --skip-existing --repository testpypi dist/*

// upload to pypi
$ twine upload dist/*
```

### Information

- For more details, please check: [How to upload your package to the Python Package Index (PyPI) test server](https://kynan.github.io/blog/2020/05/23/how-to-upload-your-package-to-the-python-package-index-pypi-test-server)
