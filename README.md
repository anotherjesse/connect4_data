To generate training data for connect 4, I thought I'd try generating using a rust library on the fly...

## Setup

1. install rust (rustup)
2. create virtualenv
3. pip install maturin

    maturin develop
    🔗 Found pyo3 bindings
    🐍 Found CPython 3.12 at /Users/jesse/ai/connect4/.env/bin/python
    📡 Using build options features from pyproject.toml
    Compiling connect4 v0.1.0 (/Users/jesse/ai/connect4)
        Finished dev [unoptimized + debuginfo] target(s) in 0.29s
    📦 Built wheel for CPython 3.12 to /var/folders/6r/kg0cjzns5tj3b_0qnn8hpc000000gn/T/.tmpaBXmVy/connect4-0.1.0-cp312-cp312-macosx_11_0_arm64.whl
    ✏️  Setting installed package as editable
    🛠 Installed connect4-0.1.0

## Usage

    python

    >>> import connect4
    >>> g = connect4.GameIterator()
    >>> next(g)
    [4, 2, 2, 6, 5, 6, 7, 3, 2, 4, 4, 2, 6, 3, 3, 4, 4, 3, 3, 3, 5, 7, 5, 8]
    >>> next(g)
    [7, 5, 6, 3, 6, 4, 3, 2, 8]
    >>> next(g)
    [1, 2, 7, 2, 1, 5, 3, 1, 5, 4, 4, 3, 1, 1, 1, 6, 5, 5, 5, 3, 7, 3, 6, 8]
    >>> next(g)
    [5, 4, 4, 4, 4, 4, 7, 1, 1, 5, 6, 4, 5, 1, 7, 2, 2, 1, 7, 1, 3, 7, 7, 5, 3, 8]
