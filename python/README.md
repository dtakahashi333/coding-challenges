### Create a new Miniforge environment
Use `-n` or `--name` option to specify an evironment name. The Python version can be specified using `python=`.

```sh
mamba create --name myenv python=3.10
```

Create a new Mamba environment using `environment.yml`.
```sh
mamba create -f environment.yml
```

Add dependencies using `environment.yml` to an existing Mamba environment.
```sh
mamba update -f environment.yml
```

Activate a Mamba environment.
```sh
mamba activate myenv
```

## pytest
Make tests folder package by adding __init__.py inside.

Include PYTHONPATH=src in pytest.ini.

```sh
pytest tests
```
