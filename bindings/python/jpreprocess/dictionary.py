from contextlib import ExitStack
import sys
import atexit

# from https://github.com/r9y9/pyopenjtalk/pull/74

if sys.version_info >= (3, 9):
    from importlib.resources import as_file, files
else:
    from importlib_resources import as_file, files

_file_manager = ExitStack()
atexit.register(_file_manager.close)
_file_ref = files(__package__)


def dictionary_path(version: str) -> str:
    return str(_file_manager.enter_context(
        as_file(_file_ref / version / "naist-jdic")))


def download_dictionary(version: str) -> str:
    from urllib.request import urlopen
    import tarfile
    import tempfile

    if version == "latest":
        url = f"https://github.com/jpreprocess/jpreprocess/releases/latest/download/naist-jdic-jpreprocess.tar.gz"
    else:
        url = f"https://github.com/jpreprocess/jpreprocess/releases/download/{version}/naist-jdic-jpreprocess.tar.gz"

    target_dir = _file_ref / version

    with tempfile.TemporaryFile() as file:
        print('Downloading: "{}"'.format(url))
        with urlopen(url) as response:
            try:
                from tqdm.auto import tqdm
                with tqdm.wrapattr(file, "write", total=getattr(response, "length", None)) as tar:
                    for chunk in response:
                        tar.write(chunk)
            except ImportError:
                for chunk in response:
                    file.write(chunk)
        file.seek(0)
        print("Extracting tar file")
        with tarfile.open(mode="r|gz", fileobj=file) as f, as_file(target_dir) as dir:
            f.extractall(path=dir)
        print("done")

    return str(target_dir)
