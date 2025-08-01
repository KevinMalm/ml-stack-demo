pip install build
python -m build -o $TGT/.dist
pip install $TGT/.dist/ml_stack_py-0.1.0-py3-none-any.whl --force-reinstall
