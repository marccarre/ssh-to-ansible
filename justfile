setup:
    command -v pre-commit >/dev/null 2>&1 || brew install pre-commit
    pre-commit install
    pre-commit install --hook-type commit-msg
    pre-commit run --all-files

lint:
    pre-commit run --all-files
