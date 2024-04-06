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
    >>> connect4.play_game(20)
    [1, 6, 6, 7, 1, 1, 4, 3, 2, 5, 4, 3, 1, 3, 2, 6, 6, 7, 2, 6]
    >>> connect4.play_game(20)
    [2, 5, 4, 1, 2, 2, 4, 1, 6, 1, 1, 6, 7, 7, 3, 4, 5, 4, 3, 8]
