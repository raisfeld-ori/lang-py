from setuptools import setup, find_packages

with open("readme.md", "r") as readme:
    setup(
        name="lang-py compiler",
        version="0.0.1",
        license="MIT",
        description="a python compiler that compiles .py code into other language's code",
        package_dir={"": "code"},
        packages=find_packages(where="code"),
        long_description=readme.read(),
        long_description_content_type="text/markdown",
        install_requires=["windows-curses >= 2.3.1", "maturin >= 1.1.0"],
        extras_require={"dev": ["twine>=4.0.2"]},
        author="ori raisfeld",
        python_requires=">=3.10.0",
        url="https://github.com/raisfeld-ori/lang-py-compiler",
        classifiers=[
            "License :: MIT license",
            "Programming Language :: python 3.11, rust 1.70",
            "Operating system :: Windows"
        ],
    )